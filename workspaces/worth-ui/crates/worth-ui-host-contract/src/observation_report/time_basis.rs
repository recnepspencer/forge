#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostObservationTimeBasis {
    HostMonotonicTick(u64),
    HostWallClockMicros(i128),
    PresentationRelativeTick(u64),
}

impl UiHostObservationTimeBasis {
    pub const fn diagnostic_value(self) -> u64 {
        match self {
            Self::HostMonotonicTick(value) => value ^ 0x11,
            Self::HostWallClockMicros(value) => value as u64 ^ 0x22,
            Self::PresentationRelativeTick(value) => value ^ 0x33,
        }
    }
}
