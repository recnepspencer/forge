#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryStoreFootprint {
    total_store_pages: u64,
}

impl RecoveryStoreFootprint {
    pub const fn admitted_persisted_pages(total_store_pages: u64) -> Self {
        Self { total_store_pages }
    }

    pub(crate) const fn empty() -> Self {
        Self::admitted_persisted_pages(0)
    }

    pub const fn total_store_pages(self) -> u64 {
        self.total_store_pages
    }
}
