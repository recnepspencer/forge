#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnsupportedQoSClaim {
    P99Latency,
    P999Latency,
    HardwareQueueDepth,
    MediaQoS,
    BackgroundWorkPacing,
}

impl UnsupportedQoSClaim {
    pub const fn canonical_physical_isolation_non_claims() -> [Self; 5] {
        [
            Self::P99Latency,
            Self::P999Latency,
            Self::HardwareQueueDepth,
            Self::MediaQoS,
            Self::BackgroundWorkPacing,
        ]
    }
}
