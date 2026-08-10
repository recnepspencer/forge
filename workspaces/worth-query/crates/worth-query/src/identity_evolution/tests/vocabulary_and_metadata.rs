use super::{basis_digest, metadata, query_digest};
use crate::identity_evolution::{
    runtime_backed_direct_identity_evolution_support_profile, AdvisoryIdentityCandidateSet,
    BranchLocalityClass, IdentityEvolutionAmbiguityReason, IdentityEvolutionComparisonBasisFamily,
    IdentityEvolutionComplexityContract, IdentityEvolutionComplexityStatus,
    IdentityEvolutionIdentityBreakReason, IdentityEvolutionOutcomeFamily,
    IdentityEvolutionQueryContext, IdentityEvolutionQueryFamily, IdentityEvolutionResultBundle,
    InspectorIdentityClassification, LineageTraversalDescriptor, LineageTraversalFamily,
    PluralIdentitySuccessorSet, PromotionOrMergeAuthorityState, SingularIdentityContinuityResult,
};

#[test]
fn identity_evolution_family_vocabulary_is_stable() {
    assert_eq!(
        IdentityEvolutionQueryFamily::LineageTraversal.as_str(),
        "lineage_traversal"
    );
    assert_eq!(
        IdentityEvolutionQueryFamily::CorrespondenceIdentityComparison.as_str(),
        "correspondence_identity_comparison"
    );
    assert_eq!(
        LineageTraversalFamily::DirectMergeSuccessor.as_str(),
        "direct_merge_successor"
    );
    assert_eq!(
        IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet.as_str(),
        "advisory_identity_candidate_set"
    );
    assert_eq!(
        IdentityEvolutionOutcomeFamily::IdentityBreak.as_str(),
        "identity_break"
    );
    assert_eq!(
        IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative.as_str(),
        "preview_to_authoritative"
    );
    assert_eq!(
        IdentityEvolutionAmbiguityReason::AmbiguousCorrespondenceCandidates.as_str(),
        "ambiguous_correspondence_candidates"
    );
    assert_eq!(
        crate::identity_evolution::IdentityEvolutionDenialReason::RecursiveTraversalDeferred
            .as_str(),
        "recursive_traversal_deferred"
    );
    assert_eq!(
        IdentityEvolutionIdentityBreakReason::ExplicitIdentityBreak.as_str(),
        "explicit_identity_break"
    );
}

#[test]
fn direct_only_support_profile_is_explicit() {
    let support_profile = runtime_backed_direct_identity_evolution_support_profile();

    assert_eq!(
        support_profile.admitted_traversal_families(),
        &[
            LineageTraversalFamily::DirectPredecessor,
            LineageTraversalFamily::DirectSuccessor,
            LineageTraversalFamily::DirectReplacement,
            LineageTraversalFamily::DirectSplitSuccessors,
            LineageTraversalFamily::DirectMergeSuccessor,
            LineageTraversalFamily::BranchLocalDirectEvolution,
        ]
    );
    assert_eq!(
        support_profile.admitted_comparison_basis_families(),
        &[
            IdentityEvolutionComparisonBasisFamily::BranchToBranch,
            IdentityEvolutionComparisonBasisFamily::CurrentToHistorical,
            IdentityEvolutionComparisonBasisFamily::HistoricalToHistorical,
            IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative,
        ]
    );
    assert_eq!(
        support_profile.deferred_scope_markers(),
        &[
            crate::identity_evolution::IdentityEvolutionDeferredScopeMarker::RecursiveTraversal,
            crate::identity_evolution::IdentityEvolutionDeferredScopeMarker::BroadCollectionDiscovery,
            crate::identity_evolution::IdentityEvolutionDeferredScopeMarker::StoreBackedParity,
            crate::identity_evolution::IdentityEvolutionDeferredScopeMarker::IdentityAwareNonInspectorViews,
        ]
    );
    assert_eq!(
        support_profile
            .lineage_complexity_contracts()
            .iter()
            .map(IdentityEvolutionComplexityContract::contract_name)
            .collect::<Vec<_>>(),
        vec![
            "direct_predecessor",
            "direct_successor",
            "direct_replacement",
            "direct_split_successors",
            "direct_merge_successor",
            "branch_local_direct_evolution",
        ]
    );
    assert_eq!(
        support_profile
            .comparison_complexity_contracts()
            .iter()
            .map(IdentityEvolutionComplexityContract::contract_name)
            .collect::<Vec<_>>(),
        vec![
            "branch_to_branch_identity_comparison",
            "current_to_historical_identity_comparison",
            "historical_to_historical_identity_comparison",
            "preview_to_authoritative_identity_comparison",
        ]
    );
    assert_eq!(
        support_profile.admitted_inspector_consumable_identity_classifications(),
        &[
            InspectorIdentityClassification::IdentitySummary,
            InspectorIdentityClassification::AuthoritativeContinuity,
            InspectorIdentityClassification::AdvisoryCandidates,
            InspectorIdentityClassification::Ambiguity,
            InspectorIdentityClassification::IdentityBreak,
            InspectorIdentityClassification::Denied,
        ]
    );
    assert!(!support_profile.profile_digest().is_empty());
}

#[test]
fn result_family_cardinality_stays_separated() {
    let singular = SingularIdentityContinuityResult::new(
        metadata(
            IdentityEvolutionOutcomeFamily::SingularIdentityContinuity,
            IdentityEvolutionComplexityContract::direct_lineage(
                LineageTraversalFamily::DirectSuccessor,
            ),
            BranchLocalityClass::CrossBranchAuthoritative,
        ),
        "authoritative-identity",
    );
    let plural = PluralIdentitySuccessorSet::new(
        metadata(
            IdentityEvolutionOutcomeFamily::PluralIdentitySuccessorSet,
            IdentityEvolutionComplexityContract::direct_lineage(
                LineageTraversalFamily::DirectSplitSuccessors,
            ),
            BranchLocalityClass::BranchLocalOnly,
        ),
        vec!["a".into(), "b".into()],
    );
    let advisory = AdvisoryIdentityCandidateSet::new(
        metadata(
            IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet,
            IdentityEvolutionComplexityContract::correspondence_identity_comparison(
                IdentityEvolutionComparisonBasisFamily::BranchToBranch,
            ),
            BranchLocalityClass::CrossBranchDenied,
        ),
        vec!["candidate".into()],
    );

    let singular_bundle = IdentityEvolutionResultBundle::singular_identity_continuity(singular);
    let plural_bundle = IdentityEvolutionResultBundle::plural_identity_successor_set(plural);
    let advisory_bundle = IdentityEvolutionResultBundle::advisory_identity_candidate_set(advisory);

    assert!(singular_bundle.as_singular_identity_continuity().is_some());
    assert!(singular_bundle.as_plural_identity_successor_set().is_none());
    assert!(plural_bundle.as_plural_identity_successor_set().is_some());
    assert!(plural_bundle.as_advisory_identity_candidate_set().is_none());
    assert!(advisory_bundle
        .as_advisory_identity_candidate_set()
        .is_some());
    assert!(advisory_bundle.as_singular_identity_continuity().is_none());
}

#[test]
fn complexity_contract_and_report_digests_are_stable() {
    let contract = IdentityEvolutionComplexityContract::direct_lineage(
        LineageTraversalFamily::DirectPredecessor,
    );
    let report = crate::identity_evolution::IdentityEvolutionComplexityReport::from_contract(
        contract.clone(),
    );

    assert_eq!(
        contract.verified_or_debt_status(),
        IdentityEvolutionComplexityStatus::Verified
    );
    assert_eq!(report.status(), IdentityEvolutionComplexityStatus::Verified);
    assert_eq!(
        contract.digest().as_str(),
        report.complexity_contract_digest().as_str()
    );
}

#[test]
fn branch_locality_fields_participate_in_metadata_digest() {
    let metadata = metadata(
        IdentityEvolutionOutcomeFamily::PluralIdentitySuccessorSet,
        IdentityEvolutionComplexityContract::direct_lineage(
            LineageTraversalFamily::BranchLocalDirectEvolution,
        ),
        BranchLocalityClass::BranchLocalOnly,
    );

    assert_eq!(
        metadata.branch_locality_class(),
        BranchLocalityClass::BranchLocalOnly
    );
    assert_eq!(
        metadata.promotion_or_merge_authority_state(),
        PromotionOrMergeAuthorityState::NotRequired
    );
    assert!(!metadata.branch_locality_digest().as_str().is_empty());
    assert!(!metadata.metadata_digest().as_str().is_empty());
}

#[test]
fn query_context_keeps_lineage_and_correspondence_shapes_distinct() {
    let lineage_context = IdentityEvolutionQueryContext::lineage_traversal_for_test(
        query_digest("lineage"),
        basis_digest("left"),
        LineageTraversalDescriptor::direct_predecessor("anchor"),
    );
    let correspondence_context =
        IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
            query_digest("correspondence"),
            IdentityEvolutionComparisonBasisFamily::BranchToBranch,
            basis_digest("left"),
            basis_digest("right"),
            crate::identity_evolution::CorrespondenceIdentityComparison::advisory_between(
                "left-id", "right-id",
            ),
        );

    assert_eq!(
        lineage_context.family(),
        IdentityEvolutionQueryFamily::LineageTraversal
    );
    assert!(lineage_context.lineage_traversal_descriptor().is_some());
    assert!(correspondence_context
        .correspondence_identity_comparison_descriptor()
        .is_some());
}
