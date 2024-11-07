use std::net::{Ipv4Addr, SocketAddr};

pub const PORT: u16 = 7678;
pub static IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);

pub static SOCKED_ADDR: SocketAddr = SocketAddr::new(std::net::IpAddr::V4(IPV4), PORT);

#[test]
fn ip_is_multicast() {
    assert!(IPV4.is_multicast())
}