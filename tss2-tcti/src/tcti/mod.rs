pub mod error;

pub mod tcti {

    use std::ffi::CStr;

    use tss2_tcti_sys::tpm2_tss;

    use super::error::TctiError;

    #[macro_export]
    macro_rules! tcti_rc {
        ($x:expr) => {
            (10 << tpm2_tss::TSS2_RC_LAYER_SHIFT) | $x
        };
    }

    #[macro_export]
    macro_rules! define_api_symbols {
        ($tcti:ty) => {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn Tss2_Tcti_Info() -> *const tpm2_tss::TSS2_TCTI_INFO {
                <$tcti as TctiLib>::info()
            }
        };
    }

    macro_rules! check_not_null {
        ( $( $ptr:expr ),* ) => {
            {
                'result: {
                    $(
                        if $ptr.is_null() {
                            eprintln!("Error: unexpected null pointer: {:?}", stringify!($ptr));
                            break 'result Err(TctiError::BadReference);
                        }
                    )*
                    Ok(())
                }
            }
        }
    }

    macro_rules! check_state {
        ($tcti:expr, $error:expr, $($expected_state:expr),*) => {{
            match $tcti.get_state() {
                None => Ok(()),
                Some(state) => {
                    'result: {
                        $(
                            if state == $expected_state {
                                break 'result Ok(());
                            }
                        )*
                        eprintln!(
                            "Error: Expected tcti state to be one of {:?}, but state is {:?}",
                            &[$($expected_state),*], state
                        );
                        Err($error)
                    }
                }
            }
        }};
    }

    macro_rules! return_if_error {
        ($result:expr) => {
            match $result {
                Ok(x) => x,
                Err(error) => return TctiError::from(error).discriminant(),
            }
        };
    }

    /// Verifies that a bytes object is null-terminated and casts to *const i8
    const fn as_char_str(bytes: &[u8]) -> *const i8 {
        let last_byte = match bytes.last() {
            Some(byte) => byte,
            None => panic!("Cstr must not be empty. It must at least contain a null-byte."),
        };

        if *last_byte != 0 {
            panic!("Cstr must be null-terminated. Add \\0 at the end, i.e. \"hello world\\0\"");
        }

        bytes as *const _ as *const i8
    }

    /*
     * The TCTI trait enables us to create new tctis in Rust. To make them ABI-compatible, they must check a view boxes:
     *  - have a symbol Tss2_Tcti_Info() which returns TSS2_TCTI_INFO*
     *  - [dynamic loading] the TSS2_TCTI_INFO struct contains the init function
     *  - [static loading] the init function is made available via a header
     *  - the init functions initialized the context in a way that the transmit/receive/... function pointers are set in the tcti context
     *
     * To archive that, we have:
     *  - a Rust implementation of the public API for convenience (changed function signatures)
     *  - C wrappers which are used for ABI compatibility, they call their Rust counter-parts
     *
     * This also means, that implementations of the TctiLib trait must guarantee that their structs are repr(C) and that the first struct member
     * is common.
     *
     * Lifetime:
     * In C, TCTI contexts life from calling init() to calling finalize(). In Rust, we want a lifetime which is tied to its scope.
     * Therefore, we initialize on object creation and finalize on out-of-scope. Any further (manual) calls to init/finalize are NOPs.
     *
     * For reference, this is how the tcti-device context looks like:
     *
     *   TSS2_TCTI_DEVICE_CONTEXT                                     .tcti_device           TSS2_TCTI_CONTEXT (opaque)
     *   TSS2_TCTI_COMMON_CONTEXT                                     | .common
     *   TSS2_TCTI_CONTEXT_COMMON_V2/TSS2_TCTI_CONTEXT_COMMON_CURRENT | | .v2                 \
     *   TSS2_TCTI_CONTEXT_COMMON_V1                                  | | | .v1               | public API
     *   uint64_t                                                     | | | | .magic          |
     *   uint32_t                                                     | | | | .version        |
     *   TSS2_TCTI_TRANSMIT_FCN                                       | | | | .transmit       |
     *   TSS2_TCTI_RECEIVE_FCN                                        | | | | .receive        |
     *   TSS2_TCTI_FINALIZE_FCN                                       | | | | .finalize       |
     *   TSS2_TCTI_CANCEL_FCN                                         | | | | .cancel         |
     *   TSS2_TCTI_GET_POLL_HANDLES_FCN                               | | | | .getPollHandles |
     *   TSS2_TCTI_SET_LOCALITY_FCN                                   | | | | .setLocality    |
     *   TSS2_TCTI_MAKE_STICKY_FCN                                    | | | .makeSticky       /
     *   tcti_state_t                                                 | | .state
     *   tpm_header_t                                                 | | .header
     *   uint8_t                                                      | | .locality
     *   bool                                                         | | .partial_read_supported
     *   bool                                                         | | .partial
     *   int                                                          | .fd
     */

    pub struct Info<'a> {
        pub name: &'a [u8],
        pub description: &'a [u8],
        pub config_help: &'a [u8],
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub enum State {
        #[default]
        NotInitialized = 0,
        Transmit,
        Receive,
        Finalized,
    }

    #[repr(C)]
    #[derive(Debug, Default)]
    struct Header {
        tag: u8,
        size: u32,
        code: u32,
    }

    pub type Api = tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_V2;

    pub trait Tcti: Sized {
        /// Creates a new [Tcti] object, initializing it.
        fn new(conf: &str) -> Result<Self, TctiError>;

        /// Transmit TPM command.
        fn transmit(&mut self, command: &[u8]) -> Result<(), TctiError>;

        /// Receive TPM response.
        fn receive(&mut self) -> Result<Vec<u8>, TctiError>;

        /// Finalizes the object. Called from ABI. Do not call from Rust code.
        fn finalize(&mut self) {}

        /// Cancel TPM command.
        fn cancel(&mut self) -> Result<(), TctiError> {
            Err(TctiError::NotImplemented)
        }

        fn get_poll_handles(&mut self) -> Result<&[tpm2_tss::TSS2_TCTI_POLL_HANDLE], TctiError> {
            Err(TctiError::NotImplemented)
        }

        fn set_locality(&mut self, _locality: u8) -> Result<(), TctiError> {
            Err(TctiError::NotImplemented)
        }

        fn make_sticky(&mut self) -> Result<(), TctiError> {
            Err(TctiError::NotImplemented)
        }
    }

    impl<T: TctiLib> Tcti for T {
        fn new(conf: &str) -> Result<Self, TctiError> {
            TctiLib::new(conf)
        }

        fn transmit(&mut self, command: &[u8]) -> Result<(), TctiError> {
            TctiLib::transmit(self, command)
        }

        fn receive(&mut self) -> Result<Vec<u8>, TctiError> {
            TctiLib::receive(self)
        }
    }

    /// Trait for implementing an ABI-compliant tcti.
    ///
    /// This trait can then be used by both Rust and C code (which expects a
    /// well-behaved library).
    ///
    ///
    /// Implementations need to fulfill these criteria:
    ///  1. [init_inner()](TctiLib::init_inner), [transmit_inner()](TctiLib::transmit_inner), [receive_inner()](TctiLib::receive_inner) must be implemented. These are called from both the ABI-layer and Rust.
    ///  1. First member must be of type [Api](crate::tcti::tcti::Api) (used internally).
    ///  1. If you want [TctiLib] to run the state machine for you: there must be a member of type [State]; [get_state()](TctiLib::get_state), [set_state()](TctiLib::set_state) must be implemented.
    ///  1. [define_api_symbols] must be called on the type.
    ///
    /// ```rust
    /// pub mod lib {
    ///     use tss2_tcti::tcti::error::TctiError;
    ///     use tss2_tcti::tcti::tcti::{Api, Info, State, Tcti, TctiLib};
    ///     use tss2_tcti::tctildr::tcti_loader::TctiLoader;
    ///     use tss2_tcti_sys::tpm2_tss;
    ///     use tss2_tcti::define_api_symbols;
    ///
    ///     #[repr(C)]
    ///     #[derive(Debug)]
    ///     pub struct TctiFoobar {
    ///         api: Api,
    ///         state: State,
    ///         child_tcti: Option<TctiLoader>,
    ///     }
    ///
    ///     impl TctiLib for TctiFoobar {
    ///         const INFO: Info<'static> = Info {
    ///             name: b"tcti-foobar\0",
    ///             description: b"A demo tcti written in Rust.\0",
    ///             config_help: b"Child TCTI config string as expected by TctiLdr.\0",
    ///         };
    ///         const MAGIC: u64 = 0x44a50b8745675fe5;
    ///
    ///         fn new(conf: &str) -> Result<Self, TctiError> {
    ///             let mut tcti = Self {
    ///                 api: Self::get_api_static(),
    ///                 state: State::NotInitialized,
    ///                 child_tcti: None,
    ///             };
    ///
    ///             tcti.init(conf)?;
    ///
    ///             Ok(tcti)
    ///         }
    ///
    ///         fn init_inner(&mut self, conf: &str) -> Result<(), TctiError> {
    ///             self.api = TctiFoobar::get_api_static();
    ///             self.child_tcti = Some(<TctiLoader as Tcti>::new(conf)?);
    ///             self.state = State::Transmit;
    ///             Ok(())
    ///         }
    ///
    ///         fn transmit_inner(&mut self, command: &[u8]) -> Result<(), TctiError> {
    ///             if let Some(child_tcti) = self.child_tcti.as_mut() {
    ///                 return child_tcti.transmit(command);
    ///             }
    ///
    ///             todo!()
    ///         }
    ///
    ///         fn receive_inner(&mut self) -> Result<Vec<u8>, TctiError> {
    ///             if let Some(child_tcti) = self.child_tcti.as_mut() {
    ///                 return child_tcti.receive();
    ///             }
    ///
    ///             todo!()
    ///         }
    ///
    ///         fn get_state(&self) -> Option<State> {
    ///             Some(self.state)
    ///         }
    ///         fn set_state(&mut self, state: State) {
    ///             self.state = state;
    ///         }
    ///     }
    ///
    ///     define_api_symbols!(TctiFoobar);
    /// }
    /// ```
    pub trait TctiLib: Sized {
        /// Provide information about this tcti to applications which will
        /// dynamically load your tcti.
        ///
        /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
        /// <strong>Warning</strong>:
        /// You have to manually NULL-terminate the strings by ending them with `\0`.
        /// </p>
        ///
        /// # Example
        ///
        /// ```
        /// # use tss2_tcti::tcti::tcti::Info;
        /// const INFO: Info<'static> = Info {
        ///     name: b"tcti-demo\0",
        ///     description: b"A demo tcti written in Rust.\0",
        ///     config_help: b"Child TCTI config string as expected by TctiLdr.\0",
        /// };
        /// ```
        const INFO: Info<'static>;
        const INFO_RAW: tpm2_tss::TSS2_TCTI_INFO = tpm2_tss::TSS2_TCTI_INFO {
            version: tpm2_tss::TCTI_VERSION, //Self::get_api_static().v1.version,
            name: as_char_str(Self::INFO.name),
            description: as_char_str(Self::INFO.description),
            config_help: as_char_str(Self::INFO.config_help),
            init: Some(init_c::<Self>),
        };
        /// Used to detect tcti types in memory. Set this to an arbitrary value
        /// which does not collide with other tcti magic values.
        ///
        /// # Example
        /// **Warning**: do not copy the value below but generate a new (random) value!
        ///
        /// ```
        /// const MAGIC: u64 = 0x44a50b8745675fe5;
        /// ```
        const MAGIC: u64;

        /// Return the info struct which
        /// [define_api_symbols!()](define_api_symbols) will redefine as a
        /// global variable (specified symbol for dynamic loading).
        fn info() -> *const tpm2_tss::TSS2_TCTI_INFO {
            &Self::INFO_RAW as *const _
        }

        /// Create new context (Rust only). Must internally call
        /// [init()](TctiLib::init) (which calls
        /// [init_inner()](TctiLib::init_inner)).
        ///
        /// # State machine
        /// If surrounding code is to handle the state machine, make sure to
        /// initialize the state member to [State::NotInitialized].
        fn new(conf: &str) -> Result<Self, TctiError>;

        /// Wrapper for [init_inner()](TctiLib::init_inner).
        ///
        /// Called from both ABI layer ([init_c()]) and Rust
        /// ([TctiLib::init_inner()]). Do not call, call [new()](TctiLib::new)
        /// instead.
        fn init(&mut self, conf: &str) -> Result<(), TctiError> {
            // when memory is zeroized, state is State::NotInitialized = 0
            // TODO check_state!(self, TctiError::BadSequence, State::NotInitialized)?; // TODO panic?

            let result = self.init_inner(conf);

            self.set_state(State::Transmit);
            result
        }

        /// Wrapper for [transmit_inner()](TctiLib::transmit_inner).
        fn transmit(&mut self, command: &[u8]) -> Result<(), TctiError> {
            check_state!(self, TctiError::BadSequence, State::Transmit)?;

            // TODO maybe there are error messages that should result in a state change?
            self.transmit_inner(command)?;

            self.set_state(State::Receive);
            Ok(())
        }
        /// Wrapper for [receive_inner()](TctiLib::receive_inner).
        fn receive(&mut self) -> Result<Vec<u8>, TctiError> {
            check_state!(self, TctiError::BadSequence, State::Receive)?;

            // TODO maybe there are error messages that should result in a state change?
            let response = self.receive_inner()?;

            self.set_state(State::Transmit);
            Ok(response)
        }

        /// Wrapper for [finalize_inner()](TctiLib::finalize_inner).
        /// Should not be called by the user.
        fn finalize(&mut self) {
            // print error but do not return error_code
            //let _ignore_error_code = check_state!(self, FinalizeError::BadSequence, State::Transmit);

            self.finalize_inner();

            self.set_state(State::Finalized);
        }

        /// Wrapper for [cancel_inner()](TctiLib::cancel_inner).
        fn cancel(&mut self) -> Result<(), TctiError> {
            check_state!(self, TctiError::BadSequence, State::Receive)?;

            self.cancel_inner()
        }
        /// Wrapper for [get_poll_handles_inner()](TctiLib::get_poll_handles_inner).
        fn get_poll_handles(&mut self) -> Result<&[tpm2_tss::TSS2_TCTI_POLL_HANDLE], TctiError> {
            // no state check
            self.get_poll_handles_inner()
        }
        /// Wrapper for [set_locality_inner()](TctiLib::set_locality_inner).
        fn set_locality(&mut self, locality: u8) -> Result<(), TctiError> {
            check_state!(self, TctiError::BadSequence, State::Transmit)?;
            self.set_locality_inner(locality)
        }
        /// Wrapper for [make_sticky_inner()](TctiLib::make_sticky_inner).
        fn make_sticky(&mut self) -> Result<(), TctiError> {
            // No state check
            self.make_sticky_inner()
        }

        /// Initialize tcti.
        ///
        /// Called from both ABI layer ([init_c()]) and Rust
        /// ([new()](TctiLib::new) -> [init()](TctiLib::init) ->
        /// [init_inner()](TctiLib::init_inner)).
        ///
        /// # State machine
        /// The following assumes that the surrounding code handles the state
        /// machine.
        ///
        /// If called from C interface, memory is zeroized. That is, if there is
        /// a member of type [State], it will default to
        /// [State::NotInitialized].
        ///
        /// If called from Rust, [new()](TctiLib::new) should make sure that any
        /// state is initialized to [State::NotInitialized].
        fn init_inner(&mut self, conf: &str) -> Result<(), TctiError>;

        /// Transmit TPM command.
        ///
        /// Called from both ABI layer ([transmit_c()] -> [TctiLib::transmit()]) and Rust ([TctiLib::transmit()]).
        fn transmit_inner(&mut self, command: &[u8]) -> Result<(), TctiError>;

        /// Receive TPM command.
        ///
        /// Called from both ABI layer ([receive_c()] -> [TctiLib::receive()]) and Rust ([TctiLib::receive()]).
        fn receive_inner(&mut self) -> Result<Vec<u8>, TctiError>;

        /// Finalize tcti.
        ///
        /// Called from both ABI layer ([finalize_c()] -> [TctiLib::finalize()]) and Rust ([TctiLib::finalize()]).
        fn finalize_inner(&mut self) {}

        /// Cancel TPM command.
        ///
        /// The default implementation returns [Err](Err)([TctiError::NotImplemented]).
        ///
        /// Called from both ABI layer ([cancel_c()] -> [TctiLib::cancel()]) and Rust ([TctiLib::cancel()]).
        fn cancel_inner(&mut self) -> Result<(), TctiError> {
            Err(TctiError::NotImplemented)
        }

        /// Get poll hangles.
        ///
        /// The default implementation returns [Err](Err)([TctiError::NotImplemented]).
        ///
        /// Called from both ABI layer ([get_poll_handles_c()] -> [TctiLib::get_poll_handles()]) and Rust ([TctiLib::get_poll_handles()]).
        fn get_poll_handles_inner(
            &mut self,
        ) -> Result<&[tpm2_tss::TSS2_TCTI_POLL_HANDLE], TctiError> {
            Err(TctiError::NotImplemented)
        }

        /// Set TPM locality.
        ///
        /// The default implementation returns [Err](Err)([TctiError::NotImplemented]).
        ///
        /// Called from both ABI layer ([set_locality_c()] -> [TctiLib::set_locality()]) and Rust ([TctiLib::set_locality()]).
        fn set_locality_inner(&mut self, _locality: u8) -> Result<(), TctiError> {
            Err(TctiError::NotImplemented)
        }

        /// Make sticky.
        ///
        /// The default implementation returns [Err](Err)([TctiError::NotImplemented]).
        ///
        /// Called from both ABI layer ([make_sticky_c()] -> [TctiLib::make_sticky()]) and Rust ([TctiLib::make_sticky()]).
        fn make_sticky_inner(&mut self) -> Result<(), TctiError> {
            Err(TctiError::NotImplemented)
        }

        /// # Safety
        ///
        /// This function should only be called with a tcti context which
        /// conforms to [tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_CURRENT]. NULL
        /// pointer
        /// is permissible (Err(TctiError::BadContext)).
        unsafe fn from_ptr<'a>(
            tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
        ) -> Result<&'a mut Self, TctiError> {
            check_not_null!(tcti_context)?;

            let magic = unsafe { *(tcti_context as *mut u64) };
            if magic != Self::MAGIC {
                eprintln!(
                    "Magic number wrong. Expected {:#08x} but got {:#08x}",
                    Self::MAGIC,
                    magic,
                );
                return Err(TctiError::BadContext {
                    magic: Some(magic),
                    expected_magic: Some(Self::MAGIC),
                });
            }

            let tcti = unsafe { &mut *(tcti_context as *mut Self) };

            Ok(tcti)
        }

        fn get_api_static() -> Api {
            Api {
                v1: tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_V1 {
                    magic: Self::MAGIC,
                    version: tpm2_tss::TCTI_VERSION,
                    transmit: Some(transmit_c::<Self>),
                    receive: Some(receive_c::<Self>),
                    finalize: Some(finalize_c::<Self>),
                    cancel: Some(cancel_c::<Self>),
                    getPollHandles: Some(get_poll_handles_c::<Self>),
                    setLocality: Some(set_locality_c::<Self>),
                    ..Default::default()
                },
                makeSticky: Some(make_sticky_c::<Self>),
            }
        }

        fn get_api(&self) -> tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_CURRENT {
            let api_ptr = self as *const _ as *const tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_CURRENT;

            unsafe { std::ptr::read(api_ptr) }
        }

        /// Return your member of type [State] if the surrounding code should
        /// take care of the default tcti state machine. Return [None] if you do
        /// not want any state handling/checking.
        ///
        /// # Example
        /// ```
        /// # use tss2_tcti::tcti::tcti::{Api, Info, State, TctiLib};
        /// # use tss2_tcti::tcti::error::TctiError;
        /// pub struct TctiFoobar {
        ///    api: Api,
        ///    state: State,
        ///    // ...
        /// }
        ///
        /// impl TctiLib for TctiFoobar {
        ///     # const INFO: Info<'static> = todo!();
        ///     # const MAGIC: u64 = todo!();
        ///     # fn new(conf: &str) -> Result<Self, TctiError> {todo!()}
        ///     # fn init_inner(&mut self, conf: &str) -> Result<(), TctiError> {todo!()}
        ///     # fn transmit_inner(&mut self, command: &[u8]) -> Result<(), TctiError> {todo!()}
        ///     # fn receive_inner(&mut self) -> Result<Vec<u8>, TctiError> {todo!()}
        ///     # fn set_state(&mut self, state: State) {todo!()}
        ///     fn get_state(&self) -> Option<State> {
        ///         Some(self.state)
        ///     }
        ///
        ///     // ...
        /// }
        /// ```
        fn get_state(&self) -> Option<State>;
        /// Set your member of type [State]. See
        /// [get_state()](TctiLib::get_state).
        ///
        /// # Example
        /// ```
        /// # use tss2_tcti::tcti::tcti::{Api, Info, State, TctiLib};
        /// # use tss2_tcti::tcti::error::TctiError;
        /// pub struct TctiFoobar {
        ///    api: Api,
        ///    state: State,
        ///    // ...
        /// }
        ///
        /// impl TctiLib for TctiFoobar {
        ///     # const INFO: Info<'static> = todo!();
        ///     # const MAGIC: u64 = todo!();
        ///     # fn new(conf: &str) -> Result<Self, TctiError> {todo!()}
        ///     # fn init_inner(&mut self, conf: &str) -> Result<(), TctiError> {todo!()}
        ///     # fn transmit_inner(&mut self, command: &[u8]) -> Result<(), TctiError> {todo!()}
        ///     # fn receive_inner(&mut self) -> Result<Vec<u8>, TctiError> {todo!()}
        ///     # fn get_state(&self) -> Option<State> {todo!()}
        ///     fn set_state(&mut self, state: State) {
        ///         self.state = state;
        ///     }
        ///
        ///     // ...
        /// }
        /// ```
        fn set_state(&mut self, state: State);
    }

    pub unsafe extern "C" fn init_c<T: TctiLib>(
        tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
        size: *mut usize,
        config: *const ::std::os::raw::c_char,
    ) -> tpm2_tss::TSS2_RC {
        if tcti_context.is_null() && size.is_null() {
            eprintln!("TCTI context and size cannot both be NULL.");
            return tcti_rc!(tpm2_tss::TSS2_BASE_RC_BAD_VALUE);
        }

        // if context is NULL, return size
        if tcti_context.is_null() {
            unsafe { *size = std::mem::size_of::<T>() };
            eprintln!("TCTI context is NULL. Return size: {} bytes", unsafe {
                *size
            });
            return 0;
        }

        let config = if config.is_null() {
            ""
        } else {
            let config_c_str = unsafe { CStr::from_ptr(config) };
            config_c_str.to_str().unwrap()
        };

        // cast and initialize context
        let tcti = unsafe { &mut *(tcti_context as *mut T) };
        // implicitly sets State to NotInitialized
        std::ptr::write_bytes(tcti_context, 0, *size);
        match tcti.init(config) {
            Ok(()) => 0,
            Err(error) => TctiError::from(error).discriminant(),
        }
    }

    pub unsafe extern "C" fn transmit_c<T: TctiLib>(
        tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
        size: usize,
        command: *const u8,
    ) -> tpm2_tss::TSS2_RC {
        return_if_error!(check_not_null!(command));

        let tcti = return_if_error!(unsafe { T::from_ptr(tcti_context) });
        let command = unsafe { std::slice::from_raw_parts(command, size) };
        return_if_error!(tcti.transmit(command));

        0
    }

    pub unsafe extern "C" fn receive_c<T: TctiLib>(
        tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
        size: *mut usize,
        response: *mut u8,
        _timeout: i32, // TODO
    ) -> tpm2_tss::TSS2_RC {
        return_if_error!(check_not_null!(size));

        let tcti = return_if_error!(unsafe { T::from_ptr(tcti_context) });

        if response.is_null() {
            *size = 4096; // TODO get header/whole response and save in tcti for next call?
            return 0;
        }

        let response = unsafe { std::slice::from_raw_parts_mut(response, *size) };

        // TODO what if TRY_AGAIN (partial read/size not enough etc.
        let child_response = return_if_error!(tcti.receive());

        eprintln!(
            "response.len(): {:#?},  child_response.len(): {:#?}",
            unsafe { *size },
            child_response.len()
        );
        // TODO gracefully handle response.len() < child_response.len()
        response[..child_response.len()].copy_from_slice(child_response.as_slice());

        0
    }

    pub unsafe extern "C" fn finalize_c<T: TctiLib>(
        tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
    ) {
        let tcti = match unsafe { T::from_ptr(tcti_context) } {
            Ok(tcti) => tcti,
            Err(_) => {
                // abort due to invalid magic
                eprint!("Aborting finalization of TCTI since magic is invalid.");
                return;
            }
        };

        tcti.finalize()
    }

    pub unsafe extern "C" fn cancel_c<T: TctiLib>(
        tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
    ) -> tpm2_tss::TSS2_RC {
        let tcti = return_if_error!(unsafe { T::from_ptr(tcti_context) });

        return_if_error!(tcti.cancel());
        0
    }

    pub unsafe extern "C" fn get_poll_handles_c<T: TctiLib>(
        tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
        handles: *mut tpm2_tss::TSS2_TCTI_POLL_HANDLE,
        num_handles: *mut usize,
    ) -> tpm2_tss::TSS2_RC {
        return_if_error!(check_not_null!(num_handles));

        let tcti = return_if_error!(unsafe { T::from_ptr(tcti_context) });
        let handles_src = return_if_error!(tcti.get_poll_handles());

        if handles.is_null() {
            *num_handles = handles_src.len();
        } else {
            // TODO gracefully handle *num_handles < handles_src.len()

            let handles = unsafe { std::slice::from_raw_parts_mut(handles, *num_handles) };
            handles.copy_from_slice(handles_src);
        };

        0
    }

    pub unsafe extern "C" fn set_locality_c<T: TctiLib>(
        tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
        locality: u8,
    ) -> tpm2_tss::TSS2_RC {
        let tcti = return_if_error!(unsafe { T::from_ptr(tcti_context) });
        return_if_error!(tcti.set_locality(locality));

        0
    }

    pub unsafe extern "C" fn make_sticky_c<T: TctiLib>(
        tcti_context: *mut tpm2_tss::TSS2_TCTI_CONTEXT,
        _handle: *mut tpm2_tss::TPM2_HANDLE,
        _sticky: u8,
    ) -> tpm2_tss::TSS2_RC {
        let tcti = return_if_error!(unsafe { T::from_ptr(tcti_context) });
        // TODO handle, sticky
        return_if_error!(tcti.make_sticky());

        0
    }
}
