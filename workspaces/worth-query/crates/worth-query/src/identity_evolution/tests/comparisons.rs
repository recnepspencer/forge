use super::{basis_digest, query_digest};
use crate::identity_evolution::{
    admit_identity_evolution_query, admit_identity_evolution_query_for_scenario,
    execute_admitted_identity_evolution_query, BranchLocalityClass,
    CorrespondenceIdentityComparison, IdentityComparisonIntent,
    IdentityEvolutionAdmissionFailureClass, IdentityEvolutionComparisonBasisFamily,
    IdentityEvolutionExecutionFamily, IdentityEvolutionQueryContext,
    IdentityEvolutionSyntheticScenario,
};

#[test]
fn comparison_context_exposes_basis_family_and_intent() {
    let context = IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
        query_digest("comparison"),
        IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative,
        basis_digest("preview"),
        basis_digest("authoritative"),
        CorrespondenceIdentityComparison::authoritative_between("left-id", "right-id"),
    );

    let (basis_family, _, _, comparison) = context
        .correspondence_identity_comparison_descriptor()
        .expect("comparison descriptor should exist");
    assert_eq!(
        basis_family,
        IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative
    );
    assert_eq!(
        comparison.intent(),
        IdentityComparisonIntent::AuthoritativeContinuityRequired
    );
}

#[test]
fn comparison_admission_requires_distinct_bases() {
    let digest = basis_digest("same");
    let context = IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
        query_digest("comparison"),
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        digest.clone(),
        digest,
        CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
    );

    let error =
        admit_identity_evolution_query(context).expect_err("same-basis comparison must deny");
    assert_eq!(
        error.failure_class(),
        &IdentityEvolutionAdmissionFailureClass::ComparisonBasisPairingRequired
    );
}

#[test]
fn advisory_comparison_shapes_candidate_set() {
    let admitted = admit_identity_evolution_query(
        IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
            query_digest("comparison"),
            IdentityEvolutionComparisonBasisFamily::BranchToBranch,
            basis_digest("left"),
            basis_digest("right"),
            CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
        ),
    )
    .expect("comparison should admit");

    let artifact =
        execute_admitted_identity_evolution_query(&admitted).expect("comparison should execute");

    assert_eq!(
        artifact.family(),
        &IdentityEvolutionExecutionFamily::BranchToBranchComparison
    );
    assert!(artifact
        .result_bundle()
        .as_advisory_identity_candidate_set()
        .is_some());
    assert_eq!(artifact.counters().correspondence_candidate_count(), 2);
    assert_eq!(
        artifact
            .counters()
            .lineage_to_correspondence_fallback_count(),
        0
    );
}

#[test]
fn authoritative_comparison_denies_when_authority_is_unavailable() {
    let admitted = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
            query_digest("comparison"),
            IdentityEvolutionComparisonBasisFamily::BranchToBranch,
            basis_digest("left"),
            basis_digest("right"),
            CorrespondenceIdentityComparison::authoritative_between("left-id", "right-id"),
        ),
        IdentityEvolutionSyntheticScenario::AdvisoryAsAuthoritativeDenied,
    )
    .expect("comparison should admit");

    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("comparison should shape denial");

    assert!(artifact.result_bundle().as_denied().is_some());
    assert_eq!(
        artifact.counters().advisory_as_authoritative_denial_count(),
        1
    );
    assert_eq!(artifact.counters().branch_crossing_denial_count(), 1);
}

#[test]
fn ambiguous_comparison_shapes_ambiguity_bundle() {
    let admitted = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
            query_digest("comparison"),
            IdentityEvolutionComparisonBasisFamily::HistoricalToHistorical,
            basis_digest("historical-left"),
            basis_digest("historical-right"),
            CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
        ),
        IdentityEvolutionSyntheticScenario::AmbiguousCorrespondence,
    )
    .expect("comparison should admit");

    let artifact =
        execute_admitted_identity_evolution_query(&admitted).expect("comparison should execute");

    assert!(artifact.result_bundle().as_ambiguity().is_some());
    assert_eq!(artifact.counters().ambiguous_correspondence_count(), 1);
}

#[test]
fn branch_local_comparison_preserves_branch_locality_metadata() {
    let admitted = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
            query_digest("comparison"),
            IdentityEvolutionComparisonBasisFamily::CurrentToHistorical,
            basis_digest("current"),
            basis_digest("historical"),
            CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
        ),
        IdentityEvolutionSyntheticScenario::BranchLocalComparison,
    )
    .expect("comparison should admit");

    let artifact =
        execute_admitted_identity_evolution_query(&admitted).expect("comparison should execute");

    assert_eq!(
        artifact.result_bundle().metadata().branch_locality_class(),
        BranchLocalityClass::BranchLocalOnly
    );
    assert_eq!(artifact.counters().executor_rediscovery_count(), 0);
}
