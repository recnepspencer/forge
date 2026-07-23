#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManifestDiscoveryCounterSnapshot {
    blocks_read: u64,
    comparisons: u64,
    bytes_read: u64,
}

impl ManifestDiscoveryCounterSnapshot {
    pub const fn blocks_read(self) -> u64 {
        self.blocks_read
    }
    pub const fn comparisons(self) -> u64 {
        self.comparisons
    }
    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }

    pub(in crate::physical_runtime::record_serving) fn observe_block(&mut self, bytes: usize) {
        self.blocks_read = self.blocks_read.saturating_add(1);
        self.bytes_read = self.bytes_read.saturating_add(bytes as u64);
    }

    pub(in crate::physical_runtime::record_serving) fn observe_comparisons(
        &mut self,
        count: usize,
    ) {
        self.comparisons = self.comparisons.saturating_add(count as u64);
    }
}
