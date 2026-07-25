#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManifestDiscoveryCounterSnapshot {
    blocks_read: u64,
    comparisons: u64,
    bytes_read: u64,
    work_count: u64,
    first_work: Option<crate::physical_runtime::PhysicalWorkIdentity>,
    last_work: Option<crate::physical_runtime::PhysicalWorkIdentity>,
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
    pub const fn work_count(self) -> u64 {
        self.work_count
    }
    pub const fn first_work(self) -> Option<crate::physical_runtime::PhysicalWorkIdentity> {
        self.first_work
    }
    pub const fn last_work(self) -> Option<crate::physical_runtime::PhysicalWorkIdentity> {
        self.last_work
    }

    pub(in crate::physical_runtime::record_serving) fn observe_block(
        &mut self,
        bytes: usize,
        work: super::super::super::residency::frame_work_trace::FrameWorkTrace,
    ) {
        self.blocks_read = self.blocks_read.saturating_add(1);
        self.bytes_read = self.bytes_read.saturating_add(bytes as u64);
        self.observe_work(work);
    }

    pub(in crate::physical_runtime::record_serving) fn observe_comparisons(
        &mut self,
        count: usize,
    ) {
        self.comparisons = self.comparisons.saturating_add(count as u64);
    }

    pub(in crate::physical_runtime::record_serving) fn observe_failed_work(
        &mut self,
        work: super::super::super::residency::frame_work_trace::FrameWorkTrace,
    ) {
        self.observe_work(work);
    }

    fn observe_work(
        &mut self,
        work: super::super::super::residency::frame_work_trace::FrameWorkTrace,
    ) {
        self.work_count = self.work_count.saturating_add(work.count());
        self.first_work = self.first_work.or(work.first());
        self.last_work = work.last().or(self.last_work);
    }
}
