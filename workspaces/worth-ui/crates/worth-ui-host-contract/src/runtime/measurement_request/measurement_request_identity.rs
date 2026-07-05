#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiMeasurementRequestIdentity {
    digest: u64,
}

impl UiMeasurementRequestIdentity {
    pub const fn new(digest: u64) -> Self {
        Self { digest }
    }

    pub const fn as_u64(self) -> u64 {
        self.digest
    }
}
