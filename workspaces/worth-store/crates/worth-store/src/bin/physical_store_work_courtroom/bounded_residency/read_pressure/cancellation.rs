use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, PhysicalRecordId, ServingPhysicalRuntime,
};

use super::super::BoundedResidencyConfiguration;
use super::{positioned_reads, read_limits, work_belongs_to_runtime};

pub(in crate::bounded_residency) struct ResidencyCancellationEvidence {
    pub(in crate::bounded_residency) physical_work: u64,
    pub(in crate::bounded_residency) first_operation: u64,
    pub(in crate::bounded_residency) last_operation: u64,
    pub(in crate::bounded_residency) runtime_bound: bool,
    pub(in crate::bounded_residency) unread_payload_bytes: u64,
    pub(in crate::bounded_residency) open_media_effects: u64,
    pub(in crate::bounded_residency) cancellation_media_effects: u64,
}

pub(in crate::bounded_residency) fn prove_cancellation(
    serving: &ServingPhysicalRuntime,
    record: PhysicalRecordId,
    configuration: BoundedResidencyConfiguration,
) -> Result<ResidencyCancellationEvidence, String> {
    serving
        .certification_physical_residency()
        .drain_unpinned_clean_frames();
    let locator = ExternalPhysicalRecordLocator::new(serving.store_identity(), record);
    let before_open = positioned_reads(serving);
    let session = serving
        .records()
        .open_external(locator, read_limits(configuration))
        .map_err(|failure| format!("cancellable C.6 read open failed: {failure:?}"))?;
    let after_open = positioned_reads(serving);
    let cancelled = session.cancel();
    let observation = cancelled.observation();
    let first = observation.first_physical_work();
    let last = observation.last_physical_work();
    Ok(ResidencyCancellationEvidence {
        physical_work: observation.physical_work_count(),
        first_operation: first.map_or(0, |identity| identity.operation().get()),
        last_operation: last.map_or(0, |identity| identity.operation().get()),
        runtime_bound: first.is_some_and(|identity| work_belongs_to_runtime(serving, identity))
            && last.is_some_and(|identity| work_belongs_to_runtime(serving, identity)),
        unread_payload_bytes: cancelled.unread_payload_bytes(),
        open_media_effects: after_open.saturating_sub(before_open),
        cancellation_media_effects: positioned_reads(serving).saturating_sub(after_open),
    })
}
