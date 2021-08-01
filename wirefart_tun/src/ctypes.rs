type c_int = i32;
type c_char = i8;
type c_ulong = u64;
type c_ushort = u16;
pub type c_short = i16;
type c_uchar = u8;
type sa_family_t = u16;

#[derive(Copy, Clone)]
#[repr(C)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [c_char; 14],
}

#[derive(Copy, Clone)]
#[repr(C)]
struct ifmap {
    mem_start: c_ulong,
    mem_end: c_ulong,
    base_addr: c_ushort,
    irq: c_uchar,
    dma: c_uchar,
    port: c_uchar,
}

#[repr(C)]
union ifr_ifru {
    ifr_addr: sockaddr,
    ifr_dstaddr: sockaddr,
    ifr_broadaddr: sockaddr,
    ifr_netmask: sockaddr,
    ifr_hwaddr: sockaddr,
    ifr_flags: c_short,
    ifr_ifindex: c_int,
    ifr_metric: c_int,
    ifr_mtu: c_int,
    ifr_map: ifmap,
    ifr_slave: [c_char; 16],
    ifr_newname: [c_char; 16],
    ifr_data: *mut c_char,
}

pub type IFName = [u8; 16];

#[repr(C)]
pub struct ifreq {
    name: IFName,
    ifru: ifr_ifru,
}

impl ifreq {
    pub fn new(name: IFName, flags: c_short) -> Self {
        Self {
            name,
            ifru: ifr_ifru {
                ifr_flags: flags
            },
        }
    }
}
