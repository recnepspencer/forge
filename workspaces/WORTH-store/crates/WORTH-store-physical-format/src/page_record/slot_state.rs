#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotDirectoryEntryState {
    Occupied,
    Deleted,
    Moved,
    Free,
    Reserved,
}

impl SlotDirectoryEntryState {
    pub const fn code(self) -> u8 {
        match self {
            Self::Occupied => 1,
            Self::Deleted => 2,
            Self::Moved => 3,
            Self::Free => 4,
            Self::Reserved => 5,
        }
    }

    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Occupied),
            2 => Some(Self::Deleted),
            3 => Some(Self::Moved),
            4 => Some(Self::Free),
            5 => Some(Self::Reserved),
            _ => None,
        }
    }
}
