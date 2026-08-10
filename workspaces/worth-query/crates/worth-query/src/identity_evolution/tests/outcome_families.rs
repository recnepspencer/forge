use super::{basis_digest, metadata, query_digest};
use crate::identity_evolution::{
    admit_identity_evolution_query, admit_identity_evolution_query_for_scenario,
    execute_admitted_identity_evolution_query, BranchLocalityClass,
    IdentityEvolutionAmbiguityBundle, IdentityEvolutionAmbiguityReason,
    IdentityEvolutionComplexityContract, IdentityEvolutionDenialReason,
    IdentityEvolutionDeniedBundle, IdentityEvolutionExecutionFamily,
    IdentityEvolutionIdentityBreakBundle, IdentityEvolutionIdentityBreakReason,
    IdentityEvolutionOutcomeFamily, IdentityEvolutionQueryContext, IdentityEvolutionResultBundle,
    IdentityEvolutionSyntheticScenario, LineageTraversalDescriptor,
};

#[test]
fn ambiguity_and_denial_remain_distinct_result_families() {
    let ambiguity = IdentityEvolutionAmbiguityBundle::new(
        metadata(
            IdentityEvolutionOutcomeFamily::Ambiguity,
            IdentityEvolutionComplexityContract::denied_or_deferred("ambiguity"),
            BranchLocalityClass::CrossBranchDenied,
        ),
        IdentityEvolutionAmbiguityReason::MultipleAuthoritativeContinuities,
    );
    let denied = IdentityEvolutionDeniedBundle::new(
        metadata(
            IdentityEvolutionOutcomeFamily::Denied,
            IdentityEvolutionComplexityContract::denied_or_deferred("denied"),
            BranchLocalityClass::CrossBranchDenied,
        ),
        IdentityEvolutionDenialReason::RecursiveTraversalDeferred,
    );

    let ambiguity_bundle = IdentityEvolutionResultBundle::ambiguity(ambiguity);
    let denied_bundle = IdentityEvolutionResultBundle::denied(denied);

    assert!(ambiguity_bundle.as_ambiguity().is_some());
    assert!(ambiguity_bundle.as_denied().is_none());
    assert!(denied_bundle.as_denied().is_some());
    assert!(denied_bundle.as_ambiguity().is_none());
}

#[test]
fn identity_break_remains_distinct_from_denial() {
    let identity_break = IdentityEvolutionIdentityBreakBundle::new(
        metadata(
            IdentityEvolutionOutcomeFamily::IdentityBreak,
            IdentityEvolutionComplexityContract::denied_or_deferred("identity_break"),
            BranchLocalityClass::CrossBranchAuthoritative,
        ),
        IdentityEvolutionIdentityBreakReason::ExplicitIdentityBreak,
    );
    let denied = IdentityEvolutionDeniedBundle::new(
        metadata(
            IdentityEvolutionOutcomeFamily::Denied,
            IdentityEvolutionComplexityContract::denied_or_deferred("denied"),
            BranchLocalityClass::CrossBranchDenied,
        ),
        IdentityEvolutionDenialReason::RecursiveTraversalDeferred,
    );

    let identity_break_bundle = IdentityEvolutionResultBundle::identity_break(identity_break);
    let denied_bundle = IdentityEvolutionResultBundle::denied(denied);

    assert!(identity_break_bundle.as_identity_break().is_some());
    assert!(identity_break_bundle.as_denied().is_none());
    assert!(denied_bundle.as_denied().is_some());
    assert!(denied_bundle.as_identity_break().is_none());
}

#[test]
fn correspondence_context_admits_as_distinct_shape() {
    let context =
        crate::identity_evolution::IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
            query_digest("correspondence"),
            crate::identity_evolution::IdentityEvolutionComparisonBasisFamily::BranchToBranch,
            basis_digest("left"),
            basis_digest("right"),
            crate::identity_evolution::CorrespondenceIdentityComparison::advisory_between(
                "left-id",
                "right-id",
            ),
        );

    let admitted = admit_identity_evolution_query(context).expect("comparison should now admit");
    assert!(admitted.correspondence_identity_comparison().is_some());
    assert!(admitted.traversal_descriptor().is_none());
}

#[test]
fn lineage_traversal_admission_requires_anchor_identity() {
    let context = IdentityEvolutionQueryContext::lineage_traversal_for_test(
        query_digest("lineage"),
        basis_digest("basis"),
        LineageTraversalDescriptor::direct_predecessor(""),
    );

    let error =
        admit_identity_evolution_query(context).expect_err("empty anchors must be rejected");
    assert_eq!(
        error.failure_class(),
        &crate::identity_evolution::IdentityEvolutionAdmissionFailureClass::MissingLineageAnchor
    );
}

#[test]
fn split_successor_execution_shapes_plural_result() {
    let admitted =
        admit_identity_evolution_query(IdentityEvolutionQueryContext::lineage_traversal_for_test(
            query_digest("split"),
            basis_digest("basis"),
            LineageTraversalDescriptor::direct_split_successors("anchor"),
        ))
        .expect("split traversal should admit");

    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("split traversal should execute");

    assert_eq!(
        artifact.family(),
        &IdentityEvolutionExecutionFamily::DirectSplitSuccessors
    );
    assert!(artifact
        .result_bundle()
        .as_plural_identity_successor_set()
        .is_some());
    assert_eq!(artifact.counters().split_successor_fanout_width(), 2);
    assert_eq!(artifact.counters().executor_rediscovery_count(), 0);
}

#[test]
fn branch_local_execution_keeps_locality_explicit() {
    let admitted =
        admit_identity_evolution_query(IdentityEvolutionQueryContext::lineage_traversal_for_test(
            query_digest("branch-local"),
            basis_digest("basis"),
            LineageTraversalDescriptor::branch_local_direct_evolution("anchor"),
        ))
        .expect("branch-local traversal should admit");

    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("branch-local traversal should execute");

    assert_eq!(
        artifact.result_bundle().metadata().branch_locality_class(),
        BranchLocalityClass::BranchLocalOnly
    );
    assert_eq!(artifact.counters().branch_local_boundary_check_count(), 1);
}

#[test]
fn branch_crossing_probe_shapes_denial_bundle() {
    let admitted = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::lineage_traversal_for_test(
            query_digest("branch-cross"),
            basis_digest("basis"),
            LineageTraversalDescriptor::branch_local_direct_evolution("anchor"),
        ),
        IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied,
    )
    .expect("branch-local traversal should admit before execution shaping");

    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("execution should return a denial bundle, not an error");

    assert!(artifact.result_bundle().as_denied().is_some());
    assert_eq!(
        artifact.result_bundle().metadata().branch_locality_class(),
        BranchLocalityClass::CrossBranchDenied
    );
    assert_eq!(artifact.counters().unsupported_lineage_denial_count(), 1);
}
