#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPublicBridgeReaderLanePosture {
    Closed,
    Open,
}

impl ForgeQueryPublicBridgeReaderLanePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
        }
    }
}
