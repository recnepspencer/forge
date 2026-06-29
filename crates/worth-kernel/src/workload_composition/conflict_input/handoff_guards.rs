use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

use super::{ConflictInputAdmissionError, ConflictInputAdmissionErrorKind};

pub(super) fn require_honest_lookup_handoff(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    handoff: &EvidenceLookupConsumedWorkloadHandoff,
) -> Result<(), ConflictInputAdmissionError> {
    if handoff.workload_stage_index_identity() != authority.stage_index_identity() {
        return Err(ConflictInputAdmissionError::new(
            ConflictInputAdmissionErrorKind::StageIndexMismatch,
            "lookup-backed conflict input requires matching spatial touch authority and workload stage-index identity",
        ));
    }
    if handoff.stage_receipt_identity() != authority.evidence_identity() {
        return Err(ConflictInputAdmissionError::new(
            ConflictInputAdmissionErrorKind::WrongAuthority,
            "lookup-backed conflict input requires one matching spatial touch authority and stage receipt identity",
        ));
    }
    if handoff.counters().raw_row_scan_count() != 0 {
        return Err(ConflictInputAdmissionError::new(
            ConflictInputAdmissionErrorKind::RawRowScanDenied,
            "lookup-backed conflict input rejects raw evidence row scans",
        ));
    }
    if handoff.counters().broad_receipt_scan_count() != 0 {
        return Err(ConflictInputAdmissionError::new(
            ConflictInputAdmissionErrorKind::BroadReceiptScanDenied,
            "lookup-backed conflict input rejects broad receipt scans",
        ));
    }
    if handoff.counters().caller_owned_scan_count() != 0 {
        return Err(ConflictInputAdmissionError::new(
            ConflictInputAdmissionErrorKind::CallerOwnedScanDenied,
            "lookup-backed conflict input rejects caller-owned scans",
        ));
    }
    Ok(())
}
