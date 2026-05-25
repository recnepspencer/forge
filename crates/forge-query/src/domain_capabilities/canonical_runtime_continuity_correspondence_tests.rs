use forge_proof::TransitionOutcome;

use super::test_support::{admitted_plan_target, ready_payload, success};
use super::{
    materialize_correspondence_evidence_resolved, ForgeQueryContinuityContributionAuthoring,
    ForgeQueryContinuityContributionPayload, ForgeQueryContinuityContributionPosture,
};
use crate::correspondence::{
    CorrespondenceCostPosture, StructuralCandidateDiscoveryPlan,
    StructuralCandidateOrderingContract,
};

#[test]
fn continuity_correspondence_materializer_builds_lineage_continuity() {
    let resolved = success(materialize_correspondence_evidence_resolved(
        ready_continuity(
            ForgeQueryContinuityContributionAuthoring::correspondence_lineage_only(
                "edge:12",
                "edge:14",
                StructuralCandidateDiscoveryPlan::IndexBackedBounded,
                4,
                "continuity.correspondence.lineage_only",
                "lineage evidence supplies the authoritative counterpart",
            ),
        ),
    ));

    assert_eq!(resolved.outcome().family_name(), "lineage_continuity");
    let lineage = resolved
        .outcome()
        .as_lineage_continuity()
        .expect("lineage continuity outcome");
    assert_eq!(lineage.canonical_subject(), "edge:12");
    assert_eq!(lineage.authoritative_counterpart(), "edge:14");
}

#[test]
fn continuity_correspondence_materializer_builds_structural_ambiguity() {
    let resolved = success(materialize_correspondence_evidence_resolved(
        ready_continuity(
            ForgeQueryContinuityContributionAuthoring::correspondence_structural_only(
                ["edge:14", "edge:15"],
                StructuralCandidateDiscoveryPlan::IndexBackedBounded,
                4,
                StructuralCandidateOrderingContract::StableFingerprintThenLineageHintOrder,
                "continuity.correspondence.structural_only",
                "structural evidence yields two bounded candidates",
            ),
        ),
    ));

    assert_eq!(
        resolved.outcome().family_name(),
        "advisory_structural_ambiguous"
    );
    assert_eq!(
        resolved.cost_posture(),
        &CorrespondenceCostPosture::StructuralAmbiguityBounded
    );
    assert_eq!(
        resolved
            .outcome()
            .as_advisory_structural_ambiguous()
            .expect("ambiguity outcome")
            .candidate_set()
            .candidates(),
        &["edge:14".to_string(), "edge:15".to_string()]
    );
}

#[test]
fn continuity_correspondence_materializer_builds_mixed_disagreement() {
    let resolved = success(materialize_correspondence_evidence_resolved(
        ready_continuity(
            ForgeQueryContinuityContributionAuthoring::correspondence_mixed(
                "edge:12",
                "edge:14",
                ["edge:99"],
                StructuralCandidateDiscoveryPlan::IndexBackedBounded,
                4,
                StructuralCandidateOrderingContract::StableFingerprintThenLineageHintOrder,
                "continuity.correspondence.mixed",
                "structural evidence disagrees with lineage continuity",
            ),
        ),
    ));

    assert_eq!(
        resolved.outcome().family_name(),
        "lineage_structural_disagreement"
    );
    let disagreement = resolved
        .outcome()
        .as_lineage_structural_disagreement()
        .expect("disagreement outcome");
    assert_eq!(disagreement.lineage_counterpart(), "edge:14");
    assert_eq!(disagreement.structural_counterpart(), "edge:99");
}

#[test]
fn continuity_correspondence_materializer_denies_missing_semantics() {
    let outcome = materialize_correspondence_evidence_resolved(ready_continuity_payload(
        ForgeQueryContinuityContributionPayload::new(
            ForgeQueryContinuityContributionPosture::CorrespondenceOnly,
            "continuity.correspondence.missing",
            "correspondence posture without correspondence evidence",
        ),
    ));

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
}

#[test]
fn continuity_correspondence_materializer_denies_mutation_posture() {
    let outcome = materialize_correspondence_evidence_resolved(ready_continuity(
        ForgeQueryContinuityContributionAuthoring::preserved_rebind(
            "edge:12",
            "edge:14",
            "continuity.identity.preserved",
            "authoritative continuity evidence should stay on the mutation lane",
        ),
    ));

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture
    ));
}

fn ready_continuity(
    authoring: ForgeQueryContinuityContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyContinuityContribution<
    super::ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready_continuity_payload(
        authoring
            .bind_to_admitted_plan_target(admitted_plan_target("plan-continuity-correspondence"))
            .payload()
            .payload()
            .clone(),
    )
}

fn ready_continuity_payload(
    payload: ForgeQueryContinuityContributionPayload,
) -> super::ForgeQueryMaterializationReadyContinuityContribution<
    super::ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready_payload(
        admitted_plan_target("plan-continuity-correspondence"),
        payload,
    )
}
