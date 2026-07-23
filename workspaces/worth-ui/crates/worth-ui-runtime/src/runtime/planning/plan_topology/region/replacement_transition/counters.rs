#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiPlanRegionStorageCounters {
    region_construction_count: usize,
    exact_comparison_count: usize,
    fingerprint_rejection_count: usize,
    trie_node_copy_count: usize,
    storage_pointer_copy_count: usize,
    reuse_count: usize,
    retirement_count: usize,
}

impl WorthUiPlanRegionStorageCounters {
    pub(crate) fn record_region_construction(&mut self) {
        self.region_construction_count += 1;
    }

    pub(crate) fn record_exact_comparison(&mut self) {
        self.exact_comparison_count += 1;
    }

    pub(crate) fn record_fingerprint_rejection(&mut self) {
        self.fingerprint_rejection_count += 1;
    }

    pub(crate) fn record_trie_node_copy(&mut self) {
        self.trie_node_copy_count += 1;
    }

    pub(crate) fn record_storage_pointer_copies(&mut self, count: usize) {
        self.storage_pointer_copy_count += count;
    }

    pub(crate) fn record_reuse(&mut self) {
        self.reuse_count += 1;
    }

    pub(crate) fn record_retirement(&mut self) {
        self.retirement_count += 1;
    }

    pub fn region_construction_count(self) -> usize {
        self.region_construction_count
    }

    pub fn exact_comparison_count(self) -> usize {
        self.exact_comparison_count
    }

    pub fn fingerprint_rejection_count(self) -> usize {
        self.fingerprint_rejection_count
    }

    pub fn trie_node_copy_count(self) -> usize {
        self.trie_node_copy_count
    }

    pub fn storage_pointer_copy_count(self) -> usize {
        self.storage_pointer_copy_count
    }

    pub fn reuse_count(self) -> usize {
        self.reuse_count
    }

    pub fn retirement_count(self) -> usize {
        self.retirement_count
    }
}
