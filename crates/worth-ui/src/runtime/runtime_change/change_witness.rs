#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiRuntimeInstanceWitness {
    raw: u64,
}

impl WorthUiRuntimeInstanceWitness {
    pub(crate) fn from_raw(raw: u64) -> Self {
        Self { raw }
    }

    pub fn raw(self) -> u64 {
        self.raw
    }
}
