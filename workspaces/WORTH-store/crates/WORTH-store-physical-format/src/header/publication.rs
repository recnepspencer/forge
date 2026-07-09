#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPublicationState {
    Unpublished,
    Published,
}

impl PhysicalPublicationState {
    pub const fn code(self) -> u8 {
        match self {
            Self::Unpublished => 0,
            Self::Published => 1,
        }
    }

    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Unpublished),
            1 => Some(Self::Published),
            _ => None,
        }
    }
}
