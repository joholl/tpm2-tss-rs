// from dataclasses import fields
// from tpmstream.spec.commands.commands_handles import command_handle_types
// from tpmstream.spec.commands.responses_handles import response_handle_types
// print("\n".join(f"0x{int(k):08x}: {len(fields(v))}" for k, v in command_handle_types.items()))
// print("\n".join(f"0x{int(k):08x}: {len(fields(v))}" for k, v in response_handle_types.items()))

pub mod rm {
    use std::{collections::HashMap, fmt};

    use log::warn;
    use std::io::Read;
    use tss2_tcti::tcti::error::TctiError;

    use crate::bytes::bytes::Stream;

    const TPM_HEADER_SIZE: usize = 10;

    #[derive(Debug)]
    pub struct Handle {
        pub value: u32,
        pub size: u32,
        pub cc: u32,
        pub handles: Vec<u32>,
    }

    #[derive(Debug)]
    pub struct Command {
        pub tag: u16,
        pub size: u32,
        pub cc: u32,
        pub handles: Vec<u32>,
    }

    impl Command {
        pub fn new(buf: &[u8]) -> Result<Command, TctiError> {
            let mut stream = Stream::new(buf);
            println!("{:?}", stream.read_u16().unwrap());
            println!("{:?}", stream.read_u32().unwrap());
            println!("{:?}", stream.read_u32().unwrap());

            if buf.len() < TPM_HEADER_SIZE {
                return Err(TctiError::GeneralFailure);
            }

            let nr_handles_cmd: HashMap<u32, u8> = [
                (0x0000011f, 2),
                (0x00000120, 2),
                (0x00000121, 1),
                (0x00000122, 2),
                (0x00000124, 1),
                (0x00000125, 1),
                (0x00000126, 1),
                (0x00000127, 1),
                (0x00000128, 1),
                (0x00000129, 1),
                (0x0000012a, 1),
                (0x0000012b, 1),
                (0x0000012c, 1),
                (0x0000012d, 1),
                (0x0000012e, 1),
                (0x0000012f, 2),
                (0x00000130, 1),
                (0x00000131, 1),
                (0x00000132, 1),
                (0x00000133, 2),
                (0x00000134, 2),
                (0x00000135, 2),
                (0x00000136, 2),
                (0x00000137, 2),
                (0x00000138, 2),
                (0x00000139, 1),
                (0x0000013a, 1),
                (0x0000013b, 1),
                (0x0000013c, 1),
                (0x0000013d, 1),
                (0x0000013e, 1),
                (0x0000013f, 1),
                (0x00000140, 1),
                (0x00000141, 0),
                (0x00000142, 0),
                (0x00000143, 0),
                (0x00000144, 0),
                (0x00000145, 0),
                (0x00000146, 0),
                (0x00000147, 2),
                (0x00000148, 2),
                (0x00000149, 3),
                (0x0000014a, 2),
                (0x0000014b, 2),
                (0x0000014c, 2),
                (0x0000014d, 3),
                (0x0000014e, 2),
                (0x0000014f, 2),
                (0x00000150, 2),
                (0x00000151, 2),
                (0x00000152, 2),
                (0x00000153, 1),
                (0x00000154, 1),
                (0x00000155, 1),
                (0x00000156, 1),
                (0x00000157, 1),
                (0x00000158, 1),
                (0x00000159, 1),
                (0x0000015b, 1),
                (0x0000015c, 1),
                (0x0000015d, 1),
                (0x0000015e, 1),
                (0x00000160, 2),
                (0x00000161, 0),
                (0x00000162, 1),
                (0x00000163, 1),
                (0x00000164, 1),
                (0x00000165, 1),
                (0x00000167, 0),
                (0x00000168, 1),
                (0x00000169, 1),
                (0x0000016a, 1),
                (0x0000016b, 1),
                (0x0000016c, 1),
                (0x0000016d, 1),
                (0x0000016e, 1),
                (0x0000016f, 1),
                (0x00000170, 1),
                (0x00000171, 1),
                (0x00000172, 1),
                (0x00000173, 1),
                (0x00000174, 1),
                (0x00000176, 2),
                (0x00000177, 1),
                (0x00000178, 0),
                (0x00000179, 0),
                (0x0000017a, 0),
                (0x0000017b, 0),
                (0x0000017c, 0),
                (0x0000017d, 0),
                (0x0000017e, 0),
                (0x0000017f, 1),
                (0x00000180, 1),
                (0x00000181, 0),
                (0x00000182, 1),
                (0x00000183, 1),
                (0x00000184, 3),
                (0x00000185, 2),
                (0x00000186, 0),
                (0x00000187, 1),
                (0x00000188, 1),
                (0x00000189, 1),
                (0x0000018a, 0),
                (0x0000018b, 1),
                (0x0000018c, 1),
                (0x0000018d, 1),
                (0x0000018e, 0),
                (0x0000018f, 1),
                (0x00000190, 1),
                (0x00000191, 1),
                (0x00000192, 3),
                (0x00000193, 1),
                (0x00000194, 1),
                (0x00000195, 3),
                (0x00000196, 1),
                (0x00000197, 2),
                (0x00000198, 1),
            ]
            .iter()
            .cloned()
            .collect();

            let tag = u16::from_be_bytes(buf[0..2].try_into().unwrap());
            let size = u32::from_be_bytes(buf[2..6].try_into().unwrap());
            let cc = u32::from_be_bytes(buf[6..10].try_into().unwrap());
            let nr_handles = match nr_handles_cmd.get(&cc) {
                Some(nr_handles) => *nr_handles,
                None => {
                    warn!("Unknown command code {:08x}. Do not process command.", cc);
                    0
                }
            } as usize;

            let handles = (0..nr_handles)
                .map(|i| u32::from_be_bytes(buf[(10 + i * 4)..(14 + i * 4)].try_into().unwrap()))
                .collect::<Vec<_>>();

            Ok(Command {
                tag,
                size,
                cc,
                handles,
            })
        }
    }

    impl fmt::Display for Command {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Command {{\n")?;

            write!(f, "    tag:     {:04x}\n", self.tag)?;
            write!(f, "    size:    {:08x}\n", self.size)?;
            write!(f, "    cc:      {:08x}\n", self.cc)?;
            write!(
                f,
                "    handles: {}\n",
                self.handles
                    .iter()
                    .map(|&h| format!("{:08x}", h))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
            write!(f, "}}\n")?;

            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct Response {
        pub cc: u32,
        pub tag: u16,
        pub size: u32,
        pub rc: u32,
        pub handles: Vec<u32>,
    }

    impl Response {
        pub fn new(buf: &[u8], cc: u32) -> Result<Response, TctiError> {
            if buf.len() < TPM_HEADER_SIZE {
                return Err(TctiError::GeneralFailure);
            }

            let nr_handles_rsp: HashMap<u32, u8> = [
                (0x0000011f, 0),
                (0x00000120, 0),
                (0x00000121, 0),
                (0x00000122, 0),
                (0x00000124, 0),
                (0x00000125, 0),
                (0x00000126, 0),
                (0x00000127, 0),
                (0x00000128, 0),
                (0x00000129, 0),
                (0x0000012a, 0),
                (0x0000012b, 0),
                (0x0000012c, 0),
                (0x0000012d, 0),
                (0x0000012e, 0),
                (0x0000012f, 0),
                (0x00000130, 0),
                (0x00000131, 1),
                (0x00000132, 0),
                (0x00000133, 0),
                (0x00000134, 0),
                (0x00000135, 0),
                (0x00000136, 0),
                (0x00000137, 0),
                (0x00000138, 0),
                (0x00000139, 0),
                (0x0000013a, 0),
                (0x0000013b, 0),
                (0x0000013c, 0),
                (0x0000013d, 0),
                (0x0000013e, 0),
                (0x0000013f, 0),
                (0x00000140, 0),
                (0x00000141, 0),
                (0x00000142, 0),
                (0x00000143, 0),
                (0x00000144, 0),
                (0x00000145, 0),
                (0x00000146, 0),
                (0x00000147, 0),
                (0x00000148, 0),
                (0x00000149, 0),
                (0x0000014a, 0),
                (0x0000014b, 0),
                (0x0000014c, 0),
                (0x0000014d, 0),
                (0x0000014e, 0),
                (0x0000014f, 0),
                (0x00000150, 0),
                (0x00000151, 0),
                (0x00000152, 0),
                (0x00000153, 0),
                (0x00000154, 0),
                (0x00000155, 0),
                (0x00000156, 0),
                (0x00000157, 1),
                (0x00000158, 0),
                (0x00000159, 0),
                (0x0000015b, 1),
                (0x0000015c, 0),
                (0x0000015d, 0),
                (0x0000015e, 0),
                (0x00000160, 0),
                (0x00000161, 1),
                (0x00000162, 0),
                (0x00000163, 0),
                (0x00000164, 0),
                (0x00000165, 0),
                (0x00000167, 1),
                (0x00000168, 0),
                (0x00000169, 0),
                (0x0000016a, 0),
                (0x0000016b, 0),
                (0x0000016c, 0),
                (0x0000016d, 0),
                (0x0000016e, 0),
                (0x0000016f, 0),
                (0x00000170, 0),
                (0x00000171, 0),
                (0x00000172, 0),
                (0x00000173, 0),
                (0x00000174, 0),
                (0x00000176, 1),
                (0x00000177, 0),
                (0x00000178, 0),
                (0x00000179, 0),
                (0x0000017a, 0),
                (0x0000017b, 0),
                (0x0000017c, 0),
                (0x0000017d, 0),
                (0x0000017e, 0),
                (0x0000017f, 0),
                (0x00000180, 0),
                (0x00000181, 0),
                (0x00000182, 0),
                (0x00000183, 0),
                (0x00000184, 0),
                (0x00000185, 0),
                (0x00000186, 1),
                (0x00000187, 0),
                (0x00000188, 0),
                (0x00000189, 0),
                (0x0000018a, 0),
                (0x0000018b, 0),
                (0x0000018c, 0),
                (0x0000018d, 0),
                (0x0000018e, 0),
                (0x0000018f, 0),
                (0x00000190, 0),
                (0x00000191, 1),
                (0x00000192, 0),
                (0x00000193, 0),
                (0x00000194, 0),
                (0x00000195, 0),
                (0x00000196, 0),
                (0x00000197, 0),
                (0x00000198, 0),
            ]
            .iter()
            .cloned()
            .collect();

            let tag = u16::from_be_bytes(buf[0..2].try_into().unwrap());
            let size = u32::from_be_bytes(buf[2..6].try_into().unwrap());
            let rc = u32::from_be_bytes(buf[6..10].try_into().unwrap());
            let nr_handles = match nr_handles_rsp.get(&cc) {
                Some(nr_handles) => *nr_handles,
                None => {
                    warn!("Unknown command code {:08x}. Do not process command.", cc);
                    0
                }
            } as usize;

            let handles = (0..nr_handles)
                .map(|i| u32::from_be_bytes(buf[(10 + i * 4)..(14 + i * 4)].try_into().unwrap()))
                .collect::<Vec<_>>();

            Ok(Response {
                cc,
                tag,
                size,
                rc,
                handles,
            })
        }
    }

    impl fmt::Display for Response {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Response {{\n")?;

            write!(f, "    tag:     {:04x}\n", self.tag)?;
            write!(f, "    size:    {:08x}\n", self.size)?;
            write!(f, "    rc:      {:08x}\n", self.rc)?;
            write!(
                f,
                "    handles: {}\n",
                self.handles
                    .iter()
                    .map(|&h| format!("{:08x}", h))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
            write!(f, "}}\n")?;

            Ok(())
        }
    }
}
