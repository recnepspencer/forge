#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedLoopWinding {
    Clockwise,
    CounterClockwise,
}

impl CertifiedLoopWinding {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clockwise => "clockwise",
            Self::CounterClockwise => "counter-clockwise",
        }
    }
}
