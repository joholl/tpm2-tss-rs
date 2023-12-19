pub mod tcti_loader {
    use std::ptr::{null, null_mut};

    use crate::tcti::{
        error::TctiError,
        tcti::{Api, Tcti},
    };
    use log::warn;
    use tss2_tcti_sys::tpm2_tss;

    /// Tcti for loading other tctis dynamically.
    ///
    /// TctiLoader is special. Since this is a wrapper for C code, we do not need a C interface.
    #[repr(C)]
    #[derive(Debug)]
    pub struct TctiLoader {
        ctx: Vec<u8>,
    }

    impl TctiLoader {
        fn get_api(&self) -> Api {
            let api_ptr = self.ctx_ptr() as *const _ as *const Api;
            unsafe { std::ptr::read(api_ptr) }
        }

        fn ctx_ptr(&self) -> *const tpm2_tss::TSS2_TCTI_OPAQUE_CONTEXT_BLOB {
            self.ctx.as_ptr() as *const _
        }

        fn ctx_mut_ptr(&mut self) -> *mut tpm2_tss::TSS2_TCTI_OPAQUE_CONTEXT_BLOB {
            self.ctx.as_mut_ptr() as *mut _
        }
    }

    impl Drop for TctiLoader {
        fn drop(&mut self) {
            let finalize_fn = self.get_api().v1.finalize.unwrap();

            unsafe {
                finalize_fn(self.ctx_mut_ptr());
            }
        }
    }

    impl Tcti for TctiLoader {
        /// # Examples
        /// ```
        /// use crate::tss2_tcti::tcti::tcti::Tcti;
        /// use crate::tss2_tcti::tctildr::tcti_loader::TctiLoader;
        ///
        /// // Make sure that
        /// //  a) libtpms is installed
        /// //  b) libtss2‑tcti‑libtpms.so is installed (comes with tpm2-tss since v3.0)
        /// let mut tcti = TctiLoader::new("libtpms").unwrap();
        ///
        /// // TPM2_Startup
        /// tcti.transmit(b"\x80\x01\x00\x00\x00\x0c\x00\x00\x01\x44\x00\x00").unwrap();
        /// let response = tcti.receive().unwrap();
        /// assert_eq!(response, b"\x80\x01\x00\x00\x00\x0a\x00\x00\x00\x00");
        ///
        /// // TPM2_GetCap (TPM_PT_MANUFACTURER: IBM)
        /// tcti.transmit(b"\x80\x01\x00\x00\x00\x16\x00\x00\x01\x7a\x00\x00\x00\x06\x00\x00\x01\x05\x00\x00\x00\x01").unwrap();
        /// let response = tcti.receive().unwrap();
        /// assert_eq!(response, b"\x80\x01\x00\x00\x00\x1b\x00\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x01\x05\x49\x42\x4d\x00");
        /// ```
        fn new(name_conf: &str) -> Result<Self, TctiError> {
            let mut size: usize = 0;
            let name_conf_cstr =
                std::ffi::CString::new(name_conf).expect("Failed to convert to CString");

            match unsafe { tpm2_tss::Tss2_Tcti_TctiLdr_Init(null_mut(), &mut size, null()) } {
                0 => (),
                _ => panic!("Unexpected Error."),
            };
            let mut tcti_loader = Self { ctx: vec![0; size] };

            let return_code = unsafe {
                tpm2_tss::Tss2_Tcti_TctiLdr_Init(
                    tcti_loader.ctx_mut_ptr(),
                    &mut size,
                    name_conf_cstr.as_ptr(),
                )
            };

            let error: TctiError = match return_code {
                0 => return Ok(tcti_loader),
                0xa001 => TctiError::GeneralFailure,
                0xa002 => TctiError::NotImplemented,
                0xa003 => TctiError::BadContext {
                    magic: None,
                    expected_magic: None,
                },
                0xa005 => TctiError::BadReference,
                0xa006 => TctiError::InsufficientBuffer,
                0xa007 => TctiError::BadSequence,
                0xa009 => TctiError::TryAgain,
                0xa010 => TctiError::IoError,
                0xa011 => TctiError::BadValue,
                0xa012 => TctiError::NotPermitted,
                0xa021 => TctiError::NotSupported,
                0xa023 => TctiError::Memory,
                0xaFFF => TctiError::BadHandle,
                error_code => TctiError::Unknown(error_code),
            };

            warn!("Child tcti returned error: {error:?}");
            Err(error)
        }

        /// Transmit byte array to child tcti
        fn transmit(&mut self, command: &[u8]) -> Result<(), TctiError> {
            let transmit_fn = self.get_api().v1.transmit.unwrap();

            println!("{:#?}", self.ctx_ptr());

            let return_code =
                unsafe { transmit_fn(self.ctx_mut_ptr(), command.len(), command.as_ptr()) };
            let error: TctiError = match return_code {
                0 => return Ok(()),
                error_code => error_code
                    .try_into()
                    .expect("TCTI returned invalid return code."),
            };

            warn!("Child tcti returned error: {error:?}");
            Err(error)
        }

        /// Transmit byte array from child tcti
        fn receive(&mut self) -> Result<Vec<u8>, TctiError> {
            let receive_fn = self.get_api().v1.receive.unwrap();

            let mut size = 0;
            let timeout = 0;

            let return_code = unsafe {
                receive_fn(
                    self.ctx_mut_ptr(),
                    &mut size as *mut usize,
                    std::ptr::null_mut(),
                    timeout,
                )
            };
            if return_code != 0 {
                let error: TctiError = return_code.into();

                warn!("Child tcti returned error: {error:?}");
                return Err(error);
            }

            let mut response = Vec::with_capacity(size);
            let mut size = response.capacity();
            let timeout = 0;

            let return_code = unsafe {
                receive_fn(
                    self.ctx_mut_ptr(),
                    &mut size as *mut usize,
                    response.as_mut_ptr(),
                    timeout,
                )
            };
            if return_code != 0 {
                let error: TctiError = return_code.into();

                warn!("Child tcti returned error: {error:?}");
                return Err(error);
            }

            unsafe { response.set_len(size) };

            Ok(response)
        }

        fn cancel(&mut self) -> Result<(), TctiError> {
            let cancel_fn = self.get_api().v1.cancel.unwrap();
            let return_code = unsafe { cancel_fn(self.ctx_mut_ptr()) };
            let error: TctiError = match return_code {
                0 => return Ok(()),
                error_code => error_code.into(),
            };

            warn!("Child tcti returned error: {error:?}");
            Err(error)
        }

        fn get_poll_handles(&mut self) -> Result<&[tpm2_tss::TSS2_TCTI_POLL_HANDLE], TctiError> {
            // let get_poll_handles_fn = self.get_api().v1.getPollHandles.unwrap();
            // match unsafe { get_poll_handles_fn(self.ctx_mut_ptr()) } {
            //     0 => Ok(()),
            //     error_code => Err(error_code),
            // }
            todo!()
        }

        fn set_locality(&mut self, locality: u8) -> Result<(), TctiError> {
            let set_locality_fn = self.get_api().v1.setLocality.unwrap();
            let return_code = unsafe { set_locality_fn(self.ctx_mut_ptr(), locality) };
            let error: TctiError = match return_code {
                0 => return Ok(()),
                error_code => error_code.into(),
            };

            warn!("Child tcti returned error: {error:?}");
            Err(error)
        }

        fn make_sticky(&mut self) -> Result<(), TctiError> {
            // let make_sticky_fn = self.get_api().v2.makeSticky.unwrap();
            // match unsafe { make_sticky_fn(self.ctx_mut_ptr()) } {
            //     0 => Ok(()),
            //     error_code => Err(error_code),
            // }
            todo!()
        }

        /// Do not call. The TctiLoader object is finalized when it is dropped.
        fn finalize(&mut self) {}
    }

    // #[cfg(test)]
    // mod tests {
    //     // Note this useful idiom: importing names from outer (for mod tests) scope.
    //     use super::*;

    //     #[test]
    //     fn test_setup() {
    //         assert_eq!(add(1, 2), 3);
    //     }

    //     #[test]
    //     fn test_bad_add() {
    //         // This assert would fire and test will fail.
    //         // Please note, that private functions can be tested too!
    //         assert_eq!(bad_add(1, 2), 3);
    //     }
    // }
}
