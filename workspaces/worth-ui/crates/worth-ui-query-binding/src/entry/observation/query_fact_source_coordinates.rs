#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryAllocationSourceGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryAllocationSourceOrder(u64);

impl WorthUiQueryAllocationSourceGeneration {
    pub(crate) fn from_value(value: u64) -> Self {
        Self(value)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl WorthUiQueryAllocationSourceOrder {
    pub(crate) fn from_value(value: u64) -> Self {
        Self(value)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}
