#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiVisualSnapshotIdentity(u64);

impl UiVisualSnapshotIdentity {
    pub(crate) const fn issued_by_runtime(value: u64) -> Self {
        Self(value)
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }
}
