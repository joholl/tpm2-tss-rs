pub mod bytes {
    use std::io::Read;
    use std::marker::PhantomData;

    pub struct Stream<'a, T = &'a [u8]>
    where
        T: Read,
    {
        source: T,
        phantom: PhantomData<&'a T>, // TODO to mitigate compiler bug?
    }

    impl<T> Stream<'_, T>
    where
        T: Read,
    {
        pub fn new(source: T) -> Self {
            Stream {
                source: source,
                phantom: PhantomData,
            }
        }

        pub fn read_u8(&mut self) -> Result<u8, std::io::Error> {
            let mut buf = [0; 1];
            self.source.read_exact(&mut buf)?;
            Ok(u8::from_be_bytes(buf))
        }

        pub fn read_u16(&mut self) -> Result<u16, std::io::Error> {
            let mut buf = [0; 2];
            self.source.read_exact(&mut buf)?;
            Ok(u16::from_be_bytes(buf))
        }

        pub fn read_u32(&mut self) -> Result<u32, std::io::Error> {
            let mut buf = [0; 4];
            self.source.read_exact(&mut buf)?;
            Ok(u32::from_be_bytes(buf))
        }
    }
}
