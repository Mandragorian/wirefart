use std::os::unix::io::AsRawFd;

use nix::ioctl_readwrite_bad;

use crate::ctypes::{c_short, ifreq, IFName};

const TUNSETIFF: u32 = 0x400454ca;

ioctl_readwrite_bad!(tun_setiff, TUNSETIFF, ifreq);

pub struct Tuntap {
    file: std::fs::File,
}

impl Tuntap {
    pub fn allocate_and_attach(dev: Option<IFName>, flags: c_short) -> Result<Self, String> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun")
            .map_err(|_| String::from("Could not open /dev/net/tun"))?;


        let mut ifr = ifreq::new(dev.unwrap_or([0; 16]), flags);

        let fd = file.as_raw_fd();
        unsafe {
            tun_setiff(fd, &mut ifr)
        }
        .map_err(|_| String::from("could not allocate tun/tap device"))?;
        Ok(Self{
            file,
        })
    }
}

impl std::io::Read for Tuntap {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl std::io::Write for Tuntap {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
