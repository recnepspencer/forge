use worth_query::facade::runtime::{CausalInspectionReason, CausalObservationAnchor, CausalObservationAnchorCounters, CausalObservationAnchorDigest, CausalObservationMissingReferencePosture, QueryObservationReceipt};

fn main() {
    let observation_receipt: QueryObservationReceipt = todo!();
    let anchor_digest: CausalObservationAnchorDigest = todo!();
    let counters: CausalObservationAnchorCounters = todo!();

    let _ = CausalObservationAnchor {
        observation_receipt,
        inspection_reason: CausalInspectionReason::ChangedResult,
        lower_runtime_evidence_family_count: 1,
        missing_reference_posture: CausalObservationMissingReferencePosture::Complete,
        anchor_digest,
        counters,
    };
}
