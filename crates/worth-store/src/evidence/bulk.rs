use std::sync::atomic::{AtomicU64, Ordering};

use super::{StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct BulkCounters {
    bulk_program_plan_count: AtomicU64,
    bulk_source_manifest_member_count: AtomicU64,
    bulk_source_manifest_stream_pass_count: AtomicU64,
    bulk_transform_partition_count: AtomicU64,
    bulk_chunk_plan_count: AtomicU64,
    bulk_chunk_execute_count: AtomicU64,
    bulk_checkpoint_write_count: AtomicU64,
    bulk_chunk_witness_write_count: AtomicU64,
    bulk_resume_index_lookup_count: AtomicU64,
    bulk_chunk_resume_count: AtomicU64,
    bulk_chunk_commit_count: AtomicU64,
    bulk_chunk_width_units: AtomicU64,
    bulk_peak_in_flight_memory_units: AtomicU64,
    bulk_fallback_path_count: AtomicU64,
    bulk_fallback_breadth_units: AtomicU64,
}

impl StoreCounters {
    pub fn record_bulk_source_manifest(&self, member_count: u64, stream_pass_count: u64) {
        self.bulk
            .bulk_program_plan_count
            .fetch_add(1, Ordering::Relaxed);
        self.bulk
            .bulk_source_manifest_member_count
            .fetch_add(member_count, Ordering::Relaxed);
        self.bulk
            .bulk_source_manifest_stream_pass_count
            .fetch_add(stream_pass_count, Ordering::Relaxed);
    }
    pub fn record_bulk_chunk_plan(&self, chunk_count: u64) {
        self.bulk
            .bulk_chunk_plan_count
            .fetch_add(chunk_count, Ordering::Relaxed);
    }
    pub fn record_bulk_transform_partition(&self, partition_count: u64) {
        self.bulk
            .bulk_transform_partition_count
            .fetch_add(partition_count, Ordering::Relaxed);
    }
    pub fn record_bulk_checkpoint_write(&self) {
        self.bulk
            .bulk_checkpoint_write_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_bulk_chunk_witness_write(&self) {
        self.bulk
            .bulk_chunk_witness_write_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_bulk_resume_index_lookup(&self) {
        self.bulk
            .bulk_resume_index_lookup_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_bulk_chunk_resume(&self) {
        self.bulk
            .bulk_chunk_resume_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_bulk_chunk_commit(&self) {
        self.bulk
            .bulk_chunk_commit_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_bulk_chunk_execute(
        &self,
        width_units: u64,
        memory_units: u64,
        fallback_breadth_units: u64,
        used_fallback_path: bool,
    ) {
        self.bulk
            .bulk_chunk_execute_count
            .fetch_add(1, Ordering::Relaxed);
        self.bulk
            .bulk_chunk_width_units
            .fetch_add(width_units, Ordering::Relaxed);
        let mut current_peak = self
            .bulk
            .bulk_peak_in_flight_memory_units
            .load(Ordering::Relaxed);
        while memory_units > current_peak {
            match self.bulk.bulk_peak_in_flight_memory_units.compare_exchange(
                current_peak,
                memory_units,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => current_peak = observed,
            }
        }
        if used_fallback_path {
            self.bulk
                .bulk_fallback_path_count
                .fetch_add(1, Ordering::Relaxed);
            self.bulk
                .bulk_fallback_breadth_units
                .fetch_add(fallback_breadth_units, Ordering::Relaxed);
        }
    }
}

pub(super) fn write_snapshot(counters: &BulkCounters, snapshot: &mut StoreCounterSnapshot) {
    macro_rules! load {
        ($field:ident) => {
            snapshot.$field = counters.$field.load(Ordering::Relaxed);
        };
    }
    load!(bulk_program_plan_count);
    load!(bulk_source_manifest_member_count);
    load!(bulk_source_manifest_stream_pass_count);
    load!(bulk_transform_partition_count);
    load!(bulk_chunk_plan_count);
    load!(bulk_chunk_execute_count);
    load!(bulk_checkpoint_write_count);
    load!(bulk_chunk_witness_write_count);
    load!(bulk_resume_index_lookup_count);
    load!(bulk_chunk_resume_count);
    load!(bulk_chunk_commit_count);
    load!(bulk_chunk_width_units);
    load!(bulk_peak_in_flight_memory_units);
    load!(bulk_fallback_path_count);
    load!(bulk_fallback_breadth_units);
}
