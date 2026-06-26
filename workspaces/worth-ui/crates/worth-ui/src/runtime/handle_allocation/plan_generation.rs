#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiHandlePlanGeneration {
    value: u64,
}

impl WorthUiHandlePlanGeneration {
    pub(crate) fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn as_u64(self) -> u64 {
        self.value
    }
}
