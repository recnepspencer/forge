#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiEvidenceAuthorityGeneration {
    value: u64,
}

impl UiEvidenceAuthorityGeneration {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn as_u64(self) -> u64 {
        self.value
    }
}
