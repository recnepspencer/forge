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
use super::proof::{certify_policy_required_overlap, certify_representative_overlap};
use crate::public_api_planar_clean_fail_boundary::clean_fail_fixture::{
    certify_clean_fail_boundary, diagnostic, dirty_input_with_kind, dirty_recovery,
    open_input_with_kind, unbounded_recovery,
};
use crate::public_api_planar_predicate::proof_fixture::{admitted_handle, orient_basis};
use worth_spatial::facade::planar_overlap::CoplanarOverlapDenial;

pub(crate) fn assert_mb_m6_outcome_matrix(movement_denial: &CoplanarOverlapDenial) {
    let outcomes = vec![
        certified_overlap_outcome(),
        policy_required_outcome(),
        dirty_input_outcome(),
        unsupported_input_outcome(),
        denied_movement_outcome(movement_denial),
        predicate_uncertain_outcome(),
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

    for outcome in outcomes {
        assert!(!outcome.message().is_empty());
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

fn certified_overlap_outcome() -> CoplanarOverlapUserOutcome {
    let receipt = certify_representative_overlap("mb-m6-matrix-certified-overlap");
    assert!(receipt.policy_required_exits().is_empty());
    CoplanarOverlapUserOutcome::from_overlap_receipt(&receipt)
}

fn policy_required_outcome() -> CoplanarOverlapUserOutcome {
    let receipt = certify_policy_required_overlap("mb-m6-matrix-policy-required");
    assert_eq!(receipt.policy_required_exits().len(), 1);
    CoplanarOverlapUserOutcome::from_overlap_receipt(&receipt)
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
