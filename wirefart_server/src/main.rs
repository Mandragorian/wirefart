use std::io::Read;

use wirefart_tun::tuntap::Tuntap;

fn main() {
    let mut tun = Tuntap::allocate_and_attach(Some([b't', b'u', b'n', b't', b'e', b's', b't', 0, 0, 0, 0 ,0 ,0 ,0 ,0 , 0]), 0x00001 | 0x1000).unwrap();
    let mut buf = [0u8; 1024];

    loop {
        tun.read(&mut buf).unwrap();
        println!("{:?}", buf);
        println!("===========");
    }
}
