#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiHandleSlotGeneration {
    value: u64,
}

impl WorthUiHandleSlotGeneration {
    pub(crate) fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn as_u64(self) -> u64 {
        self.value
    }
}
