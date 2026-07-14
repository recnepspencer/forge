use crate::media::DurableMediaReport;

use super::super::engine::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub(crate) fn record_physical_chunk_export(&self, chunk_width: u64) {
        self.counters.record_physical_chunk_export(chunk_width);
    }

    pub fn durable_media_report(&self) -> DurableMediaReport {
        self.persistence.durable_media_report()
    }

    pub fn record_bulk_source_manifest(&self, member_count: u64, stream_pass_count: u64) {
        self.counters
            .record_bulk_source_manifest(member_count, stream_pass_count);
    }

    pub fn record_bulk_chunk_plan(&self, chunk_count: u64) {
        self.counters.record_bulk_chunk_plan(chunk_count);
    }

    pub fn record_bulk_chunk_execute(
        &self,
        width_units: u64,
        memory_units: u64,
        fallback_breadth_units: u64,
        used_fallback_path: bool,
    ) {
        self.counters.record_bulk_chunk_execute(
            width_units,
            memory_units,
            fallback_breadth_units,
            used_fallback_path,
        );
    }

    pub fn record_bulk_chunk_resume(&self) {
        self.counters.record_bulk_chunk_resume();
    }

    pub fn record_bulk_chunk_commit(&self) {
        self.counters.record_bulk_chunk_commit();
    }
}
