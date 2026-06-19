use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    adversarial_loop_reconstruction_subject, AdversarialLoopReconstructionScenario,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanDeniedLoopCandidateKind, PlanarBooleanLoopCandidateBoundary,
    PlanarBooleanLoopCandidateBoundaryInput,
};

#[test]
fn closed_walks_with_mixed_lineage_stop_as_denied_loop_candidates() {
    let subject = adversarial_loop_reconstruction_subject(
        AdversarialLoopReconstructionScenario::LineageContradiction,
    );
    let outcomes = subject.classify();

    let boundary = PlanarBooleanLoopCandidateBoundary::promote(
        PlanarBooleanLoopCandidateBoundaryInput::from_walk_outcomes(&outcomes),
    );

    assert!(boundary.loop_candidates().rows().is_empty());
    assert_eq!(boundary.counters().closed_walks_considered(), 1);
    assert_eq!(boundary.counters().denied_loop_candidates_emitted(), 1);
    let denied = boundary
        .denied_loop_candidates()
        .rows()
        .first()
        .expect("mixed-lineage closed walk should localize a denied loop candidate");
    assert_eq!(
        denied.kind(),
        PlanarBooleanDeniedLoopCandidateKind::LineageContradiction
    );
}
