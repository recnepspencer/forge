use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCandidateBroadPhaseReason, PlanarBooleanCandidateEnvelopeBasis,
    PlanarBooleanCanonicalSegment, PlanarBooleanSegmentCandidateRowReceipt,
};

fn main() {
    let _ = PlanarBooleanSegmentCandidateRowReceipt {
        left: unavailable_segment(),
        right: unavailable_segment(),
        broad_phase_reason: PlanarBooleanCandidateBroadPhaseReason::AabbEnvelopeOverlap,
        envelope_basis: unavailable_envelope(),
        candidate_identity: String::from("forged"),
    };
}

fn unavailable_segment() -> PlanarBooleanCanonicalSegment {
    panic!("compile-fail fixture must never construct canonical segments")
}

fn unavailable_envelope() -> PlanarBooleanCandidateEnvelopeBasis {
    panic!("compile-fail fixture must never construct candidate envelope basis")
}
