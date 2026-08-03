use worth_store::physical_runtime::ServingPhysicalRuntime;
use worth_store_physical_backend::MediaOperationRole;

pub(super) fn positioned_reads(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedRead)
}

pub(super) fn metadata_reads(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::ReadMetadata)
}
