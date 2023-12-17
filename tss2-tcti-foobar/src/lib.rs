pub mod lib {
    use tss2_tcti::define_api_symbols;
    use tss2_tcti::tcti::error::TctiError;
    use tss2_tcti::tcti::tcti::{Api, Info, State, Tcti, TctiLib};
    use tss2_tcti::tctildr::tcti_loader::TctiLoader;
    use tss2_tcti_sys::tpm2_tss;

    #[repr(C)]
    #[derive(Debug)]
    pub struct TctiFoobar {
        api: Api,
        state: State,
        child_tcti: Option<TctiLoader>,
    }

    impl TctiLib for TctiFoobar {
        const INFO: Info<'static> = Info {
            name: b"tpm2_tcti-foobar\0",
            description: b"A demo tcti written in Rust.\0",
            config_help: b"Child TCTI config string as expected by TctiLdr.\0",
        };
        const MAGIC: u64 = 0x44a50b8745675fe5;

        fn new(conf: &str) -> Result<Self, TctiError> {
            let mut tcti = Self {
                api: Self::get_api_static(),
                state: State::NotInitialized,
                child_tcti: None,
            };

            tcti.init(conf)?;

            Ok(tcti)
        }

        fn init_inner(&mut self, conf: &str) -> Result<(), TctiError> {
            self.api = TctiFoobar::get_api_static();
            self.child_tcti = Some(<TctiLoader as Tcti>::new(conf)?);
            self.state = State::Transmit;
            Ok(())
        }

        fn transmit_inner(&mut self, command: &[u8]) -> Result<(), TctiError> {
            println!("TX: {:?}", command);

            if let Some(child_tcti) = self.child_tcti.as_mut() {
                return child_tcti.transmit(command);
            }

            todo!()
        }

        fn receive_inner(&mut self) -> Result<Vec<u8>, TctiError> {
            if let Some(child_tcti) = self.child_tcti.as_mut() {
                return child_tcti.receive();
            }

            todo!()
        }

        fn get_state(&self) -> Option<State> {
            Some(self.state)
        }
        fn set_state(&mut self, state: State) {
            self.state = state;
        }
    }

    define_api_symbols!(TctiFoobar);
}
