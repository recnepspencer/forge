use super::{basis_digest, metadata, query_digest};
use crate::identity_evolution::{
    admit_identity_evolution_query, admit_identity_evolution_query_for_scenario,
    compare_identity_evolution_denial_replay, compare_identity_evolution_result_replay,
    execute_admitted_identity_evolution_query, BranchLocalityClass,
    IdentityEvolutionAmbiguityBundle, IdentityEvolutionAmbiguityReason,
    IdentityEvolutionCertificationDenialEvidence, IdentityEvolutionCertificationResultEvidence,
    IdentityEvolutionComplexityContract, IdentityEvolutionDenialReason,
    IdentityEvolutionDeniedBundle, IdentityEvolutionIdentityBreakBundle,
    IdentityEvolutionIdentityBreakReason, IdentityEvolutionIdentityBreakReason as BreakReason,
    IdentityEvolutionOutcomeFamily, IdentityEvolutionQueryContext,
    IdentityEvolutionReplayParityClass, IdentityEvolutionResultBundle, InspectorIdentityArtifact,
    InspectorIdentityClassification, LineageTraversalDescriptor,
};

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
        crate::identity_evolution::IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied,
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
        BreakReason::ExplicitIdentityBreak,
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
        BreakReason::ExplicitIdentityBreak
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
