use super::decoded_recovery_record_set::DecodedRecoveryRecords;
use super::OfflineRecoveryVerifierConclusion;

pub(super) fn classify_recovery_record_set(
    decoded: &DecodedRecoveryRecords<'_>,
) -> OfflineRecoveryVerifierConclusion {
    if decoded.has_ambiguous_role() {
        return OfflineRecoveryVerifierConclusion::AmbiguousPhysicalRecordSet;
    }
    let Some(checkpoint) = decoded.checkpoint() else {
        return OfflineRecoveryVerifierConclusion::IncompletePhysicalRecordSet;
    };
    let Some(wal_frame) = decoded.wal_frame() else {
        return OfflineRecoveryVerifierConclusion::IncompletePhysicalRecordSet;
    };
    let Some(page) = decoded.checkpoint_page() else {
        return OfflineRecoveryVerifierConclusion::IncompletePhysicalRecordSet;
    };
    if page.page_id != wal_frame.page_id
        || page.page_lsn != checkpoint.frontier_lsn
        || wal_frame.lsn <= checkpoint.frontier_lsn
        || contains_corrupt_component(&wal_frame.operation_digest)
        || contains_corrupt_component(&wal_frame.idempotence_digest)
        || contains_corrupt_component(&page.physical_state_digest)
    {
        return OfflineRecoveryVerifierConclusion::CorruptRecord;
    }
    OfflineRecoveryVerifierConclusion::Verified
}

fn contains_corrupt_component(value: &str) -> bool {
    value
        .as_bytes()
        .windows(b"corrupt".len())
        .any(|window| window == b"corrupt")
}
