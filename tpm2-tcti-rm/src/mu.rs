pub mod mu {

    use crate::bytes::bytes::Stream;

    const TPM_HEADER_SIZE: usize = 10;

    pub trait Marshalable {
        fn from_be_bytes(stream: &mut Stream) -> Result<Self, std::io::Error>
        where
            Self: Sized;
    }

    pub struct Command<Handles, Params> {
        pub tag: u16,
        pub size: u32,
        pub cc: u32,
        pub handles: Handles,
        pub params: Params,
    }

    impl<Handles, Params> Marshalable for Command<Handles, Params>
    where
        Handles: Marshalable,
        Params: Marshalable,
    {
        fn from_be_bytes(stream: &mut Stream) -> Result<Command<Handles, Params>, std::io::Error> {
            // TODO error handling: too little bytes left

            //Err("Failed parsing.")

            let cmd = Command {
                tag: stream.read_u16()?,
                size: stream.read_u32()?,
                cc: stream.read_u32()?,
                handles: Handles::from_be_bytes(stream)?,
                params: Params::from_be_bytes(stream)?,
            };

            Ok(cmd)
        }
    }

    pub struct StartupHandles {}
    pub struct StartupParams {}

    type Startup = Command<[u32; 0], StartupParams>;
}
