use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarDirtyInputKind, PlanarOpenInputKind,
};
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticSubject;
use worth_spatial::facade::planar_overlap::{
    CoplanarOverlapNoOptionsCause, CoplanarOverlapUserDecision, CoplanarOverlapUserOutcome,
    CoplanarOverlapUserOutcomeKind,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateCoincidencePolicy,
};

use super::diagnostics::{assert_tiny_rotation_diagnostic, certify_tiny_rotation_diagnostic};
use super::platform_storm_subject::mismatched_operator_stage_link_error;
use super::storm_extraction_subject::certify_projected_storm_extraction_bundle;
use crate::public_api_planar_clean_fail_boundary::clean_fail_fixture::{
    certify_clean_fail_boundary, diagnostic, dirty_input_with_kind, dirty_recovery,
    open_input_with_kind, unbounded_recovery,
};
use crate::public_api_planar_predicate::proof_fixture::{admitted_handle, orient_basis};
use worth_kernel::workload_composition::{WorkloadCatalog, WorkloadTopologyBreadth};
use worth_spatial::facade::planar_overlap::CoplanarOverlapDenial;

pub(crate) fn assert_mb_m6_outcome_matrix(movement_denial: &CoplanarOverlapDenial) {
    let outcomes = vec![
        certified_overlap_outcome(),
        policy_required_outcome(),
        dirty_input_outcome(),
        unsupported_input_outcome(),
        denied_movement_outcome(movement_denial),
        predicate_uncertain_outcome(),
        integrity_mismatch_outcome(),
    ];

    assert_one_kind(
        &outcomes,
        CoplanarOverlapUserOutcomeKind::ContractsCertified,
    );
    assert_one_kind(
        &outcomes,
        CoplanarOverlapUserOutcomeKind::PolicyDecisionRequired,
    );
    assert_one_no_options(&outcomes, CoplanarOverlapNoOptionsCause::DirtyInput);
    assert_one_no_options(&outcomes, CoplanarOverlapNoOptionsCause::UnsupportedInput);
    assert_one_no_options(
        &outcomes,
        CoplanarOverlapNoOptionsCause::DeniedMovementOrRotation,
    );
    assert_one_no_options(&outcomes, CoplanarOverlapNoOptionsCause::PredicateUncertain);
    assert_one_no_options(&outcomes, CoplanarOverlapNoOptionsCause::IntegrityMismatch);

    for outcome in outcomes {
        assert!(!outcome.message().is_empty());
        assert!(
            !outcome.message().contains('_'),
            "user-facing outcome messages must not leak machine tokens: {}",
            outcome.message()
        );
        assert!(
            !contains_machine_slug(outcome.message()),
            "user-facing outcome messages must explain causes in prose: {}",
            outcome.message()
        );
        assert!(!outcome.evidence_digest().is_empty());
        match outcome.kind() {
            CoplanarOverlapUserOutcomeKind::ContractsCertified => {
                assert!(outcome.decisions().is_empty());
                assert_eq!(outcome.boolean_result(), None);
                assert_eq!(outcome.imprint_action(), None);
            }
            CoplanarOverlapUserOutcomeKind::PolicyDecisionRequired => {
                assert_eq!(
                    outcome.message(),
                    "Signed area needs a user policy decision before overlap imprint."
                );
                assert_eq!(
                    outcome.decisions(),
                    &[
                        CoplanarOverlapUserDecision::TreatCandidateLoopAsInsideFace,
                        CoplanarOverlapUserDecision::TreatCandidateLoopAsOutsideFace,
                        CoplanarOverlapUserDecision::PauseForManualInspection,
                    ]
                );
                assert_eq!(
                    outcome
                        .decisions()
                        .iter()
                        .map(|decision| decision.label())
                        .collect::<Vec<_>>(),
                    vec![
                        "Treat the candidate loop as inside this face.",
                        "Treat the candidate loop as outside this face.",
                        "Pause boolean certification for manual inspection.",
                    ]
                );
                assert_eq!(outcome.boolean_result(), None);
                assert_eq!(outcome.imprint_action(), None);
            }
            CoplanarOverlapUserOutcomeKind::NoOptions => {
                assert!(outcome.decisions().is_empty());
                assert!(outcome.no_options_cause().is_some());
                assert_eq!(outcome.boolean_result(), None);
                assert_eq!(outcome.imprint_action(), None);
            }
        }
    }
}

fn contains_machine_slug(message: &str) -> bool {
    message
        .split_whitespace()
        .any(|word| word.matches('-').count() >= 3)
}

fn certified_overlap_outcome() -> CoplanarOverlapUserOutcome {
    let bundle = projected_overlap_bundle("mb-m6-matrix-certified-overlap");
    let receipt = bundle
        .receipts()
        .iter()
        .find(|receipt| receipt.policy_required_exits().is_empty())
        .expect("storm extraction should include admitted overlap receipts");
    assert!(receipt.policy_required_exits().is_empty());
    CoplanarOverlapUserOutcome::from_overlap_receipt(receipt)
}

fn policy_required_outcome() -> CoplanarOverlapUserOutcome {
    let bundle = projected_overlap_bundle("mb-m6-matrix-policy-required");
    let receipt = bundle
        .receipts()
        .iter()
        .find(|receipt| !receipt.policy_required_exits().is_empty())
        .expect("storm extraction should include policy-required receipts");
    assert_eq!(receipt.policy_required_exits().len(), 1);
    CoplanarOverlapUserOutcome::from_overlap_receipt(receipt)
}

fn projected_overlap_bundle(
    world: &'static str,
) -> worth_spatial::facade::projected_overlap_faces::CoplanarOverlapExtractionBundle {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .with_topology_breadth(WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 })
        .declared(format!("MB-M6-1 outcome matrix projected storm {world}"))
        .build()
        .expect("outcome matrix projected storm should build");
    certify_projected_storm_extraction_bundle(
        world,
        built.projected_workload(),
        built.transform_receipts(),
    )
}

fn dirty_input_outcome() -> CoplanarOverlapUserOutcome {
    let world = "mb-m6-matrix-dirty-input";
    let source = "mb-m6-matrix:dirty:self-intersecting-loop";
    let receipt = certify_clean_fail_boundary(
        world,
        dirty_input_with_kind(world, source, PlanarDirtyInputKind::SelfIntersectingLoop),
        dirty_recovery(world, source),
        diagnostic(world, PlanarDiagnosticSubject::topology_failure(source)),
    );
    CoplanarOverlapUserOutcome::from_clean_fail_boundary(&receipt)
}

fn unsupported_input_outcome() -> CoplanarOverlapUserOutcome {
    let world = "mb-m6-matrix-unsupported-input";
    let source = "mb-m6-matrix:unsupported:open-planar-domain";
    let receipt = certify_clean_fail_boundary(
        world,
        open_input_with_kind(world, source, PlanarOpenInputKind::OpenPlanarDomain),
        unbounded_recovery(world, source),
        diagnostic(
            world,
            PlanarDiagnosticSubject::unsupported_planar_class(source),
        ),
    );
    CoplanarOverlapUserOutcome::from_clean_fail_boundary(&receipt)
}

fn denied_movement_outcome(denial: &CoplanarOverlapDenial) -> CoplanarOverlapUserOutcome {
    let diagnostic = certify_tiny_rotation_diagnostic(denial.reason());
    assert_tiny_rotation_diagnostic(&diagnostic, denial.reason());
    CoplanarOverlapUserOutcome::from_overlap_denial(denial, &diagnostic)
}

fn predicate_uncertain_outcome() -> CoplanarOverlapUserOutcome {
    let handle = admitted_handle("mb-m6-matrix-predicate-uncertain");
    let basis = orient_basis(
        "movement:predicate-uncertain",
        [[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]],
    )
    .with_coincidence_policy(PlanarPredicateCoincidencePolicy::DenyCertifiedZeroBeforeRepair);
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    let error = planar_predicate_authority_facts(&entry, &handle)
        .expect_err("certified zero must require policy or repair before boolean work");
    CoplanarOverlapUserOutcome::from_predicate_authority_error(&error)
}

fn integrity_mismatch_outcome() -> CoplanarOverlapUserOutcome {
    let error = mismatched_operator_stage_link_error()
        .expect_err("matrix integrity branch should come from mismatched platform evidence");
    CoplanarOverlapUserOutcome::from_storm_workload_error(error)
}

fn assert_one_kind(outcomes: &[CoplanarOverlapUserOutcome], kind: CoplanarOverlapUserOutcomeKind) {
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.kind() == kind)
            .count(),
        1
    );
}

fn assert_one_no_options(
    outcomes: &[CoplanarOverlapUserOutcome],
    cause: CoplanarOverlapNoOptionsCause,
) {
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| {
                outcome.kind() == CoplanarOverlapUserOutcomeKind::NoOptions
                    && outcome.no_options_cause() == Some(cause)
            })
            .count(),
        1
    );
}
