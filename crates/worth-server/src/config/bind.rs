use std::net::SocketAddr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthServerBindAddress(SocketAddr);

impl WorthServerBindAddress {
    pub fn new(address: SocketAddr) -> Self {
        Self(address)
    }

    pub fn socket_addr(self) -> SocketAddr {
        self.0
    }
}

impl From<SocketAddr> for WorthServerBindAddress {
    fn from(value: SocketAddr) -> Self {
        Self::new(value)
    }
}

impl From<([u8; 4], u16)> for WorthServerBindAddress {
    fn from(value: ([u8; 4], u16)) -> Self {
        SocketAddr::from(value).into()
    }
}
