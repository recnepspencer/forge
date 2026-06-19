use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanBornLoopSet,
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeBoundaryInput,
    PlanarBooleanDegenerateLoopOutcomeKind, PlanarBooleanLoopContainmentEvidencePosture,
    PlanarBooleanLoopContainmentEvidencePostureKind,
    PlanarBooleanLoopContainmentEvidencePostureSet, PlanarBooleanLoopRoleOutcome,
    PlanarBooleanLoopRoleOutcomeKind, PlanarBooleanLoopRoleOutcomeSet,
};

use super::tests_support::{
    collinear_geometry, empty_born_loops, empty_source_loop_carriers, empty_split_fragments,
    preserved_containment, preserved_role, reconstructed_loops, role_outcomes, triangle_geometry,
};

#[test]
fn two_fragment_loops_stop_as_tiny_cardinality_before_identity_minting() {
    let boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            &reconstructed_loops(vec![PlanarBooleanAdmittedReconstructedLoop::new(
                "reconstructed-loop".to_string(),
                "candidate".to_string(),
                "source-loop".to_string(),
                "source-face".to_string(),
                "local-frame".to_string(),
                "precision-basis".to_string(),
                vec!["fragment-a".to_string(), "fragment-b".to_string()],
                vec!["vertex-a".to_string(), "vertex-b".to_string()],
            )]),
            &empty_born_loops(),
            &role_outcomes(vec![preserved_role("reconstructed-loop")]),
            &containment_postures(vec![preserved_containment("reconstructed-loop")]),
            &empty_source_loop_carriers(),
            &empty_split_fragments(),
        ),
    );

    assert_eq!(
        boundary.outcomes().rows()[0].kind(),
        PlanarBooleanDegenerateLoopOutcomeKind::DeniedTinyCardinality
    );
    assert_eq!(boundary.counters().loops_consumed(), 1);
    assert_eq!(boundary.counters().tiny_cardinality_outcomes_emitted(), 1);
    assert_eq!(boundary.counters().admitted_for_identity_minting(), 0);
}

#[test]
fn duplicate_split_vertices_stop_as_self_touching_before_identity_minting() {
    let boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            &reconstructed_loops(vec![PlanarBooleanAdmittedReconstructedLoop::new(
                "reconstructed-loop".to_string(),
                "candidate".to_string(),
                "source-loop".to_string(),
                "source-face".to_string(),
                "local-frame".to_string(),
                "precision-basis".to_string(),
                vec![
                    "fragment-a".to_string(),
                    "fragment-b".to_string(),
                    "fragment-c".to_string(),
                    "fragment-d".to_string(),
                ],
                vec![
                    "vertex-a".to_string(),
                    "vertex-b".to_string(),
                    "vertex-c".to_string(),
                    "vertex-a".to_string(),
                ],
            )]),
            &empty_born_loops(),
            &role_outcomes(vec![preserved_role("reconstructed-loop")]),
            &containment_postures(vec![preserved_containment("reconstructed-loop")]),
            &empty_source_loop_carriers(),
            &empty_split_fragments(),
        ),
    );

    assert_eq!(
        boundary.outcomes().rows()[0].kind(),
        PlanarBooleanDegenerateLoopOutcomeKind::DeniedSelfTouching
    );
    assert_eq!(boundary.counters().loops_consumed(), 1);
    assert_eq!(boundary.counters().self_touching_outcomes_emitted(), 1);
    assert_eq!(boundary.counters().admitted_for_identity_minting(), 0);
}

#[test]
fn ambiguous_role_evidence_stops_as_policy_required() {
    let (source_loop_carriers, split_fragments) = triangle_geometry();
    let boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            &reconstructed_loops(vec![PlanarBooleanAdmittedReconstructedLoop::new(
                "reconstructed-loop".to_string(),
                "candidate".to_string(),
                "source-loop".to_string(),
                "source-face".to_string(),
                "local-frame".to_string(),
                "precision-basis".to_string(),
                vec![
                    "fragment-a".to_string(),
                    "fragment-b".to_string(),
                    "fragment-c".to_string(),
                ],
                vec![
                    "vertex-a".to_string(),
                    "vertex-b".to_string(),
                    "vertex-c".to_string(),
                ],
            )]),
            &empty_born_loops(),
            &role_outcomes(vec![PlanarBooleanLoopRoleOutcome::new(
                "role-outcome".to_string(),
                "reconstructed-loop".to_string(),
                crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
                Vec::new(),
                vec!["source-loop".to_string()],
                None,
                PlanarBooleanLoopRoleOutcomeKind::MissingSourceRoleEvidence,
            )]),
            &containment_postures(vec![preserved_containment("reconstructed-loop")]),
            &source_loop_carriers,
            &split_fragments,
        ),
    );

    assert_eq!(
        boundary.outcomes().rows()[0].kind(),
        PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredRoleEvidence
    );
    assert_eq!(boundary.counters().loops_consumed(), 1);
    assert_eq!(boundary.counters().policy_required_outcomes_emitted(), 1);
    assert_eq!(boundary.counters().admitted_for_identity_minting(), 0);
}

#[test]
fn collinear_loop_walk_stops_as_zero_area_before_identity_minting() {
    let (source_loop_carriers, split_fragments) = collinear_geometry();
    let boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            &reconstructed_loops(vec![PlanarBooleanAdmittedReconstructedLoop::new(
                "reconstructed-loop".to_string(),
                "candidate".to_string(),
                "source-loop".to_string(),
                "source-face".to_string(),
                "local-frame".to_string(),
                "precision-basis".to_string(),
                vec![
                    "fragment-a".to_string(),
                    "fragment-b".to_string(),
                    "fragment-c".to_string(),
                ],
                vec![
                    "vertex-a".to_string(),
                    "vertex-b".to_string(),
                    "vertex-c".to_string(),
                ],
            )]),
            &empty_born_loops(),
            &role_outcomes(vec![preserved_role("reconstructed-loop")]),
            &containment_postures(vec![preserved_containment("reconstructed-loop")]),
            &source_loop_carriers,
            &split_fragments,
        ),
    );

    assert_eq!(
        boundary.outcomes().rows()[0].kind(),
        PlanarBooleanDegenerateLoopOutcomeKind::DeniedZeroArea
    );
    assert_eq!(boundary.counters().loops_consumed(), 1);
    assert_eq!(boundary.counters().zero_area_outcomes_emitted(), 1);
    assert_eq!(boundary.counters().admitted_for_identity_minting(), 0);
}

#[test]
fn missing_geometry_evidence_stops_as_policy_required_before_identity_minting() {
    let boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            &reconstructed_loops(vec![PlanarBooleanAdmittedReconstructedLoop::new(
                "reconstructed-loop".to_string(),
                "candidate".to_string(),
                "source-loop".to_string(),
                "source-face".to_string(),
                "local-frame".to_string(),
                "precision-basis".to_string(),
                vec![
                    "fragment-a".to_string(),
                    "fragment-b".to_string(),
                    "fragment-c".to_string(),
                ],
                vec![
                    "vertex-a".to_string(),
                    "vertex-b".to_string(),
                    "vertex-c".to_string(),
                ],
            )]),
            &empty_born_loops(),
            &role_outcomes(vec![preserved_role("reconstructed-loop")]),
            &containment_postures(vec![preserved_containment("reconstructed-loop")]),
            &empty_source_loop_carriers(),
            &empty_split_fragments(),
        ),
    );

    assert_eq!(
        boundary.outcomes().rows()[0].kind(),
        PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredGeometryEvidence
    );
    assert_eq!(boundary.counters().loops_consumed(), 1);
    assert_eq!(
        boundary
            .counters()
            .geometry_policy_required_outcomes_emitted(),
        1
    );
    assert_eq!(boundary.counters().admitted_for_identity_minting(), 0);
}

fn containment_postures(
    rows: Vec<PlanarBooleanLoopContainmentEvidencePosture>,
) -> PlanarBooleanLoopContainmentEvidencePostureSet {
    PlanarBooleanLoopContainmentEvidencePostureSet::new(
        "containment-set".to_string(),
        "request".to_string(),
        rows,
    )
}
