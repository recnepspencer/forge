#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublicBridgeReaderLanePosture {
    Closed,
    Open,
}

impl WorthQueryPublicBridgeReaderLanePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
        }
    }
}
