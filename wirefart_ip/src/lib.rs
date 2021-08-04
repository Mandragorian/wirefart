use std::io::Read;

#[derive(Debug)]
pub enum IPError {
    NotImplemented,
    IOError,
}

pub struct IPv4BufferBuilder {}

impl IPv4BufferBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn prepare_buffer(&self) -> IPv4Buffer {
        IPv4Buffer::new()
    }
}

pub struct IPv4Buffer{
    buf: Vec<u8>,
}

impl IPv4Buffer {
    fn new() -> Self {
        let mut buf = Vec::with_capacity(1500);
        buf.resize(1500, 0);
        Self {
            buf,
        }
    }

    pub fn read<R: Read>(mut self, reader: &mut R) -> Result<IPv4Packet, IPError> {
        reader.read(&mut self.buf)
            .map_err(|_| IPError::IOError)?;

        match (self.buf[0] >> 4) & 0xf {
            4 =>  {
                Ok(IPv4Packet {
                    buf: self.buf,
                })
            }
            _ => Err(IPError::NotImplemented)
        }
    }
}

pub struct IPv4Packet {
    pub buf: Vec<u8>,
}

impl IPv4Packet {
    pub fn version(&self) -> u8 {
        4
    }

    pub fn header_len(&self) -> u8 {
        (self.buf[0] & 0xf) * 4
    }

    pub fn total_len(&self) -> u16 {
        ((self.buf[2] as u16) << 8 ) + self.buf[3] as u16
    }

    pub fn payload(&self) -> &[u8] {
        &self.buf[self.header_len() as usize .. self.total_len() as usize]
    }

    pub fn header(&self) -> &[u8] {
        &self.buf[..self.header_len() as usize]
    }

    pub fn buf(&self) -> &[u8] {
        &self.buf
    }

    pub fn source_address(&self) -> u32 {
        let mut result: u32 = 0;
        result += (self.buf[12] as u32) << 24;
        result += (self.buf[13] as u32) << 16;
        result += (self.buf[14] as u32) << 8;
        result += self.buf[15] as u32;
        result
    }

    pub fn destination_address(&self) -> u32 {
        let mut result: u32 = 0;
        result += (self.buf[16] as u32) << 24;
        result += (self.buf[17] as u32) << 16;
        result += (self.buf[18] as u32) << 8;
        result += self.buf[19] as u32;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wirefart_test::mocks::MockWireReader;

    #[test]
    fn basic_test() {
        let data = [69, 0, 0, 84, 41, 71, 64, 0, 64, 1, 36, 13, 192, 168, 54, 1, 192, 168, 54, 3, 8, 0, 7, 213, 0, 48, 0, 49, 84, 133, 9, 97, 0, 0, 0, 0, 196, 16, 15, 0, 0, 0, 0, 0, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55];
        let mut mock = MockWireReader::new(&data, 1500);

        let builder = IPv4BufferBuilder::new();
        let buffer = builder.prepare_buffer();
        let packet = buffer.read(&mut mock).unwrap();

        assert_eq!(packet.version(), 4);
        assert_eq!(packet.source_address(), 3232249345);
        assert_eq!(packet.destination_address(), 3232249347);
    }

    #[test]
    fn test_payload() {
        let data = [69, 0, 0, 84, 41, 71, 64, 0, 64, 1, 36, 13, 192, 168, 54, 1, 192, 168, 54, 3, 8, 0, 7, 213, 0, 48, 0, 49, 84, 133, 9, 97, 0, 0, 0, 0, 196, 16, 15, 0, 0, 0, 0, 0, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55];
        let mut mock = MockWireReader::new(&data, 1500);

        let builder = IPv4BufferBuilder::new();
        let buffer = builder.prepare_buffer();
        let packet = buffer.read(&mut mock).unwrap();

        assert_eq!(packet.payload(), [8, 0, 7, 213, 0, 48, 0, 49, 84, 133, 9, 97, 0, 0, 0, 0, 196, 16, 15, 0, 0,   0, 0, 0, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55]);
    }

    #[test]
    fn test_header() {
        let data = [69, 0, 0, 84, 41, 71, 64, 0, 64, 1, 36, 13, 192, 168, 54, 1, 192, 168, 54, 3, 8, 0, 7, 213, 0, 48, 0, 49, 84, 133, 9, 97, 0, 0, 0, 0, 196, 16, 15, 0, 0, 0, 0, 0, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55];
        let mut mock = MockWireReader::new(&data, 1500);

        let builder = IPv4BufferBuilder::new();
        let buffer = builder.prepare_buffer();
        let packet = buffer.read(&mut mock).unwrap();

        assert_eq!(packet.header(), [69, 0, 0, 84, 41, 71, 64, 0, 64, 1, 36, 13, 192, 168, 54, 1, 192, 168, 54, 3]);
    }
}
