use std::io::Read;
use std::io::Result;

pub struct MockWireReader {
    data: Vec<u8>,
    ind: usize,
    len: usize,
}

impl MockWireReader {
    pub fn new(buf: &[u8], len: usize) -> Self {
        if len < buf.len() {
            panic!("Wire mock MTU less than provided buffer length");
        }

        let mut data: Vec<u8> = Vec::new();
        data.resize(len, 0);
        data.as_mut_slice()[..buf.len()].copy_from_slice(buf);
        Self {
            data,
            ind: 0,
            len,
        }
    }

    pub fn from_buffer(buf: &[u8]) -> Self {
        Self::new(buf, buf.len())
    }
}

impl Read for MockWireReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len_to_read = std::cmp::min(buf.len(), self.len - self.ind);

        if len_to_read > 0 {
            buf.clone_from_slice(&self.data[self.ind..self.ind + len_to_read]);
            self.ind += len_to_read;
            return Ok(len_to_read)
        }
        Ok(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let data = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
        let mut mock = MockWireReader::from_buffer(&data);

        let mut buf = [0u8; 8];
        mock.read(&mut buf).unwrap();
        assert_eq!(buf, [0,1,2,3,4,5,6,7]);

        mock.read(&mut buf).unwrap();
        assert_eq!(buf, [8,9,10,11,12,13,14,15]);
    }

    #[test]
    fn different_length_test() {
        let data = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
        let mut mock = MockWireReader::from_buffer(&data);

        let mut buf = [0u8; 3];
        mock.read(&mut buf).unwrap();
        assert_eq!(buf, [0,1,2]);

        let mut buf = [0u8; 4];
        mock.read(&mut buf).unwrap();
        assert_eq!(buf, [3,4,5,6]);

        let mut buf = [0u8; 9];
        mock.read(&mut buf).unwrap();
        assert_eq!(buf, [7,8,9,10,11,12,13,14,15]);
    }

    #[test]
    fn extend_with_zeros() {
        let data = [0,1,2,3,4,5,6,7];
        let mut mock = MockWireReader::new(&data, 100);

        let mut buf = [0u8; 100];
        mock.read(&mut buf).unwrap();

        for i in 0..8 {
            assert_eq!(buf[i], i as u8);
        }
        assert_eq!(buf[8..], [0u8; 92]);
    }
}
