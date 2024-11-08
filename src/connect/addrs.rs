use std::net::{Ipv4Addr, SocketAddr};

pub const PORT: u16 = 7678;
pub static MULTICAST_IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);

pub static BIND_SERVER: Ipv4Addr = {
    if cfg!(target_os = "windows") {
        Ipv4Addr::UNSPECIFIED
    } else {
        MULTICAST_IPV4
    }
};

pub static SOCKET_ADDR: SocketAddr = SocketAddr::new(std::net::IpAddr::V4(BIND_SERVER), PORT);

#[test]
fn ip_is_multicast() {
    assert!(MULTICAST_IPV4.is_multicast())
}
