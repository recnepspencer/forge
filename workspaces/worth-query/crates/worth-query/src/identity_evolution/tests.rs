use super::{
    admit_identity_evolution_query, admit_identity_evolution_query_for_scenario,
    compare_identity_evolution_denial_replay, compare_identity_evolution_result_replay,
    execute_admitted_identity_evolution_query,
    runtime_backed_direct_identity_evolution_support_profile, AdvisoryIdentityCandidateSet,
    BranchLocalityClass, CorrespondenceIdentityComparison, IdentityComparisonIntent,
    IdentityEvolutionAdmissionFailureClass, IdentityEvolutionAmbiguityBundle,
    IdentityEvolutionAmbiguityReason, IdentityEvolutionCertificationDenialEvidence,
    IdentityEvolutionCertificationResultEvidence, IdentityEvolutionComparisonBasisFamily,
    IdentityEvolutionComplexityContract, IdentityEvolutionComplexityReport,
    IdentityEvolutionComplexityStatus, IdentityEvolutionDenialReason,
    IdentityEvolutionDeniedBundle, IdentityEvolutionExecutionFamily,
    IdentityEvolutionIdentityBreakBundle, IdentityEvolutionIdentityBreakReason,
    IdentityEvolutionMetadata, IdentityEvolutionOutcomeFamily, IdentityEvolutionQueryContext,
    IdentityEvolutionQueryFamily, IdentityEvolutionReplayArtifact,
    IdentityEvolutionReplayParityClass, IdentityEvolutionResultBundle,
    IdentityEvolutionSyntheticScenario, InspectorIdentityArtifact, InspectorIdentityClassification,
    LineageTraversalDescriptor, LineageTraversalFamily, PluralIdentitySuccessorSet,
    PromotionOrMergeAuthorityState, SingularIdentityContinuityResult,
};
use crate::identity::{BasisDigest, CanonicalQueryDigest, LineageDigest};

fn query_digest(label: &str) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("query:{label}")])
}

fn basis_digest(label: &str) -> BasisDigest {
    BasisDigest::from_parts(&[format!("basis:{label}")])
}

fn lineage_digest(label: &str) -> LineageDigest {
    LineageDigest::from_parts(&[format!("lineage:{label}")])
}

fn metadata(
    outcome_family: IdentityEvolutionOutcomeFamily,
    contract: IdentityEvolutionComplexityContract,
    branch_locality_class: BranchLocalityClass,
) -> IdentityEvolutionMetadata {
    IdentityEvolutionMetadata::from_parts(
        query_digest("phase-1"),
        basis_digest("read"),
        lineage_digest("direct"),
        outcome_family,
        basis_digest("anchor-branch"),
        basis_digest("origin-branch"),
        basis_digest("divergence-root"),
        branch_locality_class,
        PromotionOrMergeAuthorityState::NotRequired,
        IdentityEvolutionComplexityReport::from_contract(contract),
    )
}

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
        IdentityEvolutionDenialReason::RecursiveTraversalDeferred.as_str(),
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
            super::IdentityEvolutionDeferredScopeMarker::RecursiveTraversal,
            super::IdentityEvolutionDeferredScopeMarker::BroadCollectionDiscovery,
            super::IdentityEvolutionDeferredScopeMarker::StoreBackedParity,
            super::IdentityEvolutionDeferredScopeMarker::IdentityAwareNonInspectorViews,
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
    let report = IdentityEvolutionComplexityReport::from_contract(contract.clone());

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
            CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
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

#[test]
fn query_minted_authority_survives_planning_replay_and_inspection() {
    let canonical = crate::harness::fixtures::canonical_bundles::runtime_detail_bundle();
    let validated = crate::harness::fixtures::validated_bundles::runtime_detail_bundle();
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();

    let direct = canonical.query().authority();
    let fluent = validated.query().canonical_authority();
    let planned = preflight.plan().query().canonical_authority();
    assert_eq!(direct, fluent);
    assert_eq!(fluent, planned);

    let context = IdentityEvolutionQueryContext::lineage_traversal(
        &planned,
        preflight.basis(),
        LineageTraversalDescriptor::direct_replacement("entity:authority-continuity"),
    );
    let admitted = admit_identity_evolution_query(context).expect("authority-bound admission");
    let execution = execute_admitted_identity_evolution_query(&admitted)
        .expect("authority-bound identity evolution");
    let evidence =
        IdentityEvolutionCertificationResultEvidence::from_execution_artifact(&execution);
    let replay = IdentityEvolutionReplayArtifact::from_result_evidence(&evidence);
    let inspection = InspectorIdentityArtifact::from_result_evidence(&evidence);

    assert_eq!(evidence.query_authority(), &planned);
    assert_eq!(replay.query_authority(), Some(&planned));
    assert_eq!(inspection.query_authority(), &planned);
    assert_eq!(evidence.basis_proof(), preflight.basis().proof());
    assert_eq!(replay.basis_proof(), Some(preflight.basis().proof()));
    assert_eq!(inspection.basis_proof(), preflight.basis().proof());
    assert_eq!(replay.query_digest(), planned.digest().as_str());
}

#[test]
fn equal_digest_text_cannot_collapse_distinct_basis_generations() {
    let left_preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let right_preflight =
        crate::harness::fixtures::execution_preflights::runtime_preflight_with_snapshot_identity(
            crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
        );
    let collision_label = "hostile-equal-rendered-basis-digest";
    let left_digest = BasisDigest::from_collision_for_test(
        collision_label,
        left_preflight.basis().proof().identity().clone(),
    );
    let right_digest = BasisDigest::from_collision_for_test(
        collision_label,
        right_preflight.basis().proof().identity().clone(),
    );
    let left = left_preflight.basis().clone().replace_proof_for_test(
        crate::basis::ResolvedBasisProof::from_identity_and_digest_for_test(
            left_preflight.basis().proof().identity().clone(),
            left_digest,
        ),
    );
    let right = right_preflight.basis().clone().replace_proof_for_test(
        crate::basis::ResolvedBasisProof::from_identity_and_digest_for_test(
            right_preflight.basis().proof().identity().clone(),
            right_digest,
        ),
    );

    assert_eq!(
        left.proof().digest().as_str(),
        right.proof().digest().as_str()
    );
    assert_ne!(left.proof(), right.proof());
    assert!(!crate::basis::snapshot_resolution_report(&left).certifies(&right));

    let authority = left_preflight.plan().query().canonical_authority();
    let context = IdentityEvolutionQueryContext::correspondence_identity_comparison(
        &authority,
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        &left,
        &right,
        CorrespondenceIdentityComparison::advisory_between("left", "right"),
    );
    let (left_proof, right_proof) = context
        .correspondence_basis_proofs()
        .expect("comparison retains both sealed generations");
    assert_ne!(left_proof.identity(), right_proof.identity());

    let admitted = admit_identity_evolution_query(context).expect("structurally distinct bases");
    let execution = execute_admitted_identity_evolution_query(&admitted)
        .expect("collision-safe comparison execution");
    let inspection = InspectorIdentityArtifact::from_result_bundle(execution.result_bundle());
    assert_eq!(inspection.basis_proof().identity(), left.proof().identity());
}

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
    let context = IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
        query_digest("correspondence"),
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        basis_digest("left"),
        basis_digest("right"),
        CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
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
        &IdentityEvolutionAdmissionFailureClass::MissingLineageAnchor
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

#[test]
fn result_evidence_exposes_required_digests() {
    let admitted =
        admit_identity_evolution_query(IdentityEvolutionQueryContext::lineage_traversal_for_test(
            query_digest("replacement-evidence"),
            basis_digest("basis"),
            LineageTraversalDescriptor::direct_replacement("anchor"),
        ))
        .expect("replacement should admit");
    let artifact =
        execute_admitted_identity_evolution_query(&admitted).expect("replacement should execute");
    let evidence = IdentityEvolutionCertificationResultEvidence::from_execution_artifact(&artifact);

    assert_eq!(evidence.query_digest().as_str(), artifact.query_digest());
    assert_eq!(evidence.basis_digest().as_str(), artifact.basis_digest());
    assert_eq!(
        evidence.lineage_digest().as_str(),
        artifact.lineage_digest()
    );
    assert_eq!(evidence.result_digest(), artifact.result_digest());
    assert!(!evidence.branch_locality_digest().as_str().is_empty());
    assert!(!evidence.complexity_contract_digest().as_str().is_empty());
    assert!(!evidence.failure_digest().as_str().is_empty());
    assert!(!evidence
        .counter_snapshot()
        .counter_snapshot_digest()
        .as_str()
        .is_empty());
}

#[test]
fn denial_evidence_exposes_required_digests() {
    let admitted = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::lineage_traversal_for_test(
            query_digest("branch-crossing-evidence"),
            basis_digest("basis"),
            LineageTraversalDescriptor::branch_local_direct_evolution("anchor"),
        ),
        IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied,
    )
    .expect("branch-local denial should admit");
    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("branch-local denial should execute");
    let evidence = IdentityEvolutionCertificationDenialEvidence::from_execution_artifact(&artifact);

    assert_eq!(evidence.query_digest().as_str(), artifact.query_digest());
    assert_eq!(evidence.basis_digest().as_str(), artifact.basis_digest());
    assert_eq!(
        evidence.lineage_digest().as_str(),
        artifact.lineage_digest()
    );
    assert_eq!(evidence.result_digest(), artifact.result_digest());
    assert!(!evidence.failure_digest().as_str().is_empty());
    assert!(!evidence
        .counter_snapshot()
        .counter_snapshot_digest()
        .as_str()
        .is_empty());
}

#[test]
fn inspector_identity_artifact_preserves_identity_break_classification() {
    let bundle =
        IdentityEvolutionResultBundle::identity_break(IdentityEvolutionIdentityBreakBundle::new(
            metadata(
                IdentityEvolutionOutcomeFamily::IdentityBreak,
                IdentityEvolutionComplexityContract::denied_or_deferred("identity_break"),
                BranchLocalityClass::BranchLocalOnly,
            ),
            IdentityEvolutionIdentityBreakReason::ExplicitIdentityBreak,
        ));

    let artifact = InspectorIdentityArtifact::from_result_bundle(&bundle);
    assert_eq!(
        artifact.classification(),
        InspectorIdentityClassification::IdentityBreak
    );
    assert!(artifact.identity_break());
    assert_eq!(
        artifact.branch_locality_class(),
        BranchLocalityClass::BranchLocalOnly
    );
}

#[test]
fn typed_reason_taxonomy_stays_closed_inside_result_bundles() {
    let ambiguity = IdentityEvolutionAmbiguityBundle::new(
        metadata(
            IdentityEvolutionOutcomeFamily::Ambiguity,
            IdentityEvolutionComplexityContract::denied_or_deferred("ambiguity"),
            BranchLocalityClass::CrossBranchDenied,
        ),
        IdentityEvolutionAmbiguityReason::AmbiguousCorrespondenceCandidates,
    );
    let denial = IdentityEvolutionDeniedBundle::new(
        metadata(
            IdentityEvolutionOutcomeFamily::Denied,
            IdentityEvolutionComplexityContract::denied_or_deferred("denied"),
            BranchLocalityClass::CrossBranchDenied,
        ),
        IdentityEvolutionDenialReason::ComplexityContractViolationDenied,
    );
    let identity_break = IdentityEvolutionIdentityBreakBundle::new(
        metadata(
            IdentityEvolutionOutcomeFamily::IdentityBreak,
            IdentityEvolutionComplexityContract::denied_or_deferred("identity_break"),
            BranchLocalityClass::CrossBranchAuthoritative,
        ),
        IdentityEvolutionIdentityBreakReason::ExplicitIdentityBreak,
    );

    assert_eq!(
        ambiguity.ambiguity_reason(),
        IdentityEvolutionAmbiguityReason::AmbiguousCorrespondenceCandidates
    );
    assert_eq!(
        denial.denial_reason(),
        IdentityEvolutionDenialReason::ComplexityContractViolationDenied
    );
    assert_eq!(
        identity_break.identity_break_reason(),
        IdentityEvolutionIdentityBreakReason::ExplicitIdentityBreak
    );
    assert_ne!(
        ambiguity.ambiguity_digest().as_str(),
        denial.denial_digest().as_str()
    );
}

#[test]
fn replay_artifact_stays_equivalent_for_same_result_evidence() {
    let admitted =
        admit_identity_evolution_query(IdentityEvolutionQueryContext::lineage_traversal_for_test(
            query_digest("replay"),
            basis_digest("basis"),
            LineageTraversalDescriptor::direct_replacement("anchor"),
        ))
        .expect("replacement should admit");
    let artifact =
        execute_admitted_identity_evolution_query(&admitted).expect("replacement should execute");
    let evidence = IdentityEvolutionCertificationResultEvidence::from_execution_artifact(&artifact);
    let replay = compare_identity_evolution_result_replay(&evidence, &evidence);

    assert_eq!(
        replay.parity_class(),
        IdentityEvolutionReplayParityClass::ReplayEquivalent
    );
    assert!(!replay.replay_digest().as_str().is_empty());
}

#[test]
fn replay_artifact_detects_divergent_denial_classification() {
    let left = IdentityEvolutionCertificationDenialEvidence::compile_fail(
        "compile-fail-left",
        &query_digest("compile-left"),
        &basis_digest("basis-left"),
    );
    let right = IdentityEvolutionCertificationDenialEvidence::compile_fail(
        "compile-fail-right",
        &query_digest("compile-left"),
        &basis_digest("basis-left"),
    );
    let replay = compare_identity_evolution_denial_replay(&left, &right);

    assert_eq!(
        replay.parity_class(),
        IdentityEvolutionReplayParityClass::ReplayDivergent
    );
}
