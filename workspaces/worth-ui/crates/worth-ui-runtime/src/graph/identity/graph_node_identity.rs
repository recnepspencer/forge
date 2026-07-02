#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UiGraphNodeIdentity {
    digest: u64,
}

impl UiGraphNodeIdentity {
    pub(crate) const fn new(digest: u64) -> Self {
        Self { digest }
    }

    pub fn digest(self) -> u64 {
        self.digest
    }
}
