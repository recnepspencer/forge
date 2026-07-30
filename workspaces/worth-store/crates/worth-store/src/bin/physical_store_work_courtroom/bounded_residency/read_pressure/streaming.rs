mod pressure_schedule;
mod reconciliation;
mod record_copy;

use worth_store::physical_runtime::{PhysicalRecordId, ServingPhysicalRuntime};

use super::super::configuration::BoundedResidencyConfiguration;

pub(in crate::bounded_residency) struct BoundedReadPressureEvidence {
    pub(in crate::bounded_residency) cold_read_effects: u64,
    pub(in crate::bounded_residency) hot_read_effects: u64,
    pub(in crate::bounded_residency) refault_effects: u64,
    pub(in crate::bounded_residency) cold_metadata_effects: u64,
    pub(in crate::bounded_residency) hot_metadata_effects: u64,
    pub(in crate::bounded_residency) refault_metadata_effects: u64,
    pub(in crate::bounded_residency) cold_read_work: u64,
    pub(in crate::bounded_residency) hot_read_work: u64,
    pub(in crate::bounded_residency) refault_work: u64,
    pub(in crate::bounded_residency) read_work: u64,
    pub(in crate::bounded_residency) positioned_read_effects: u64,
    pub(in crate::bounded_residency) metadata_read_effects: u64,
    pub(in crate::bounded_residency) metadata_read_work_declared: u64,
    pub(in crate::bounded_residency) metadata_read_work_dispatched: u64,
    pub(in crate::bounded_residency) metadata_read_work_terminal: u64,
    pub(in crate::bounded_residency) range_read_work_declared: u64,
    pub(in crate::bounded_residency) range_read_work_dispatched: u64,
    pub(in crate::bounded_residency) range_read_work_terminal: u64,
    pub(in crate::bounded_residency) first_operation: u64,
    pub(in crate::bounded_residency) last_operation: u64,
    pub(in crate::bounded_residency) runtime_bound: bool,
    pub(in crate::bounded_residency) peak_resident_bytes: u64,
    pub(in crate::bounded_residency) peak_admitted_bytes: u64,
    pub(in crate::bounded_residency) faults: u64,
    pub(in crate::bounded_residency) source_loads: u64,
    pub(in crate::bounded_residency) hits: u64,
    pub(in crate::bounded_residency) evictions: u64,
    pub(in crate::bounded_residency) caller_copy_operations: u64,
    pub(in crate::bounded_residency) caller_copied_bytes: u64,
    pub(in crate::bounded_residency) store_copy_operations: u64,
    pub(in crate::bounded_residency) store_copied_bytes: u64,
    pub(in crate::bounded_residency) peak_copy_width: u64,
    pub(in crate::bounded_residency) store_maximum_copy_width: u64,
    pub(in crate::bounded_residency) streaming_scratch_bytes: u64,
    pub(in crate::bounded_residency) largest_record_bytes: u64,
}

pub(in crate::bounded_residency) fn prove_reads(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    configuration: BoundedResidencyConfiguration,
) -> Result<BoundedReadPressureEvidence, String> {
    serving
        .certification_physical_residency()
        .drain_unpinned_clean_frames();
    let baseline = reconciliation::ReadPressureBaseline::capture(serving);
    let schedule = pressure_schedule::execute(serving, records, configuration)?;
    reconciliation::reconcile(serving, configuration, baseline, schedule)
}
