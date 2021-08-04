use wirefart_tun::tuntap::Tuntap;
use wirefart_ip::IPv4BufferBuilder;

fn main() {
    let mut tun = Tuntap::allocate_and_attach(Some([b't', b'u', b'n', b't', b'e', b's', b't', 0, 0, 0, 0 ,0 ,0 ,0 ,0 , 0]), 0x00001 | 0x1000).unwrap();
    let builder = IPv4BufferBuilder::new();

    loop {
        let buffer = builder.prepare_buffer();
        let packet = if let Ok(p) = buffer.read(&mut tun) {
            p
        } else {
            continue;
        };
        println!("{}", packet.version());
        println!("{:?}", packet.buf());
        println!("{:?}", packet.payload());
        println!("===========");
    }
}
