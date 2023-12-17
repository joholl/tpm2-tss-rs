pub mod bytes;
pub mod mu;
pub mod rm;

pub mod lib {
    use core::panic;
    use std::collections::HashMap;

    use log::warn;
    use tss2_tcti::define_api_symbols;
    use tss2_tcti::tcti::error::TctiError;
    use tss2_tcti::tcti::tcti::{Api, Info, State, Tcti, TctiLib};
    use tss2_tcti::tctildr::tcti_loader::TctiLoader;
    use tss2_tcti_sys::tpm2_tss;

    use crate::rm::rm::{Command, Response};

    #[repr(C)]
    #[derive(Debug)]
    pub struct TctiFoobar {
        api: Api,
        state: State,
        child_tcti: Option<TctiLoader>,
        last_command_code: Option<u32>,
    }

    impl TctiFoobar {
        fn get_child_tcti(&mut self) -> &mut TctiLoader {
            match &mut self.child_tcti {
                Some(child_tcti) => child_tcti,
                None => panic!("Do not call this on uninitialized context."),
            }
        }

        fn get_last_command_code(&mut self) -> u32 {
            match self.last_command_code {
                Some(last_command_code) => last_command_code,
                None => panic!("Do not call this before the first command was transmitted."),
            }
        }

        fn process_command(&mut self, buf: &[u8]) -> Result<Vec<u8>, TctiError> {
            let cmd = Command::new(buf)?;
            self.last_command_code = Some(cmd.cc);

            println!("{}", cmd);

            for handle in cmd.handles.iter() {
                match *handle {
                    h if h >> 24 == 0x00 => (), // PCR
                    h if h >> 24 == 0x01 => (), // NV index
                    h if h >> 24 == 0x02 => (), // HMAC session
                    h if h >> 24 == 0x03 => (), // policy session
                    h if h >> 24 == 0x40 => (), // permanent values
                    h if h >> 24 == 0x80 => (), // transient objects
                    h if h >> 24 == 0x81 => (), // persistent objects
                    h => panic!("Unknown handle type: {:08x}", h),
                }
            }

            Ok(buf.to_vec())
        }

        fn process_response(&mut self, buf: &[u8]) -> Result<Vec<u8>, TctiError> {
            let rsp = Response::new(buf, self.get_last_command_code())?;

            println!("{}", rsp);

            Ok(buf.to_vec())
        }
    }

    impl TctiLib for TctiFoobar {
        const INFO: Info<'static> = Info {
            name: b"tpm2_tcti-foobar\0",
            description: b"A demo tcti written in Rust.\0",
            config_help: b"Child TCTI config string as expected by TctiLdr.\0",
        };
        const MAGIC: u64 = 0x72657372736d6772;

        fn new(conf: &str) -> Result<Self, TctiError> {
            let mut tcti = Self {
                api: Self::get_api_static(),
                state: State::NotInitialized,
                child_tcti: None,
                last_command_code: None,
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
            let command = self.process_command(&command)?;

            self.get_child_tcti().transmit(&command)
        }

        fn receive_inner(&mut self) -> Result<Vec<u8>, TctiError> {
            let mut response = self.get_child_tcti().receive()?;

            self.process_response(&mut response)
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
