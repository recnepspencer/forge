use super::payloads::{
    ForgeQueryAdmissionContributionPayload, ForgeQueryAdmissionContributionPosture,
    ForgeQueryAftermathContributionPayload, ForgeQueryAftermathContributionPosture,
    ForgeQueryContinuityContributionPayload, ForgeQueryContinuityContributionPosture,
    ForgeQueryExplanationContributionPayload, ForgeQueryExplanationContributionPosture,
    ForgeQueryInvariantCapabilityContributionPayload,
    ForgeQueryInvariantCapabilityContributionPosture, ForgeQuerySupportContributionPayload,
    ForgeQuerySupportContributionPosture, ForgeQueryWorkflowContributionPayload,
    ForgeQueryWorkflowContributionPosture,
};
use super::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use super::test_support::{
    admitted_plan_target, declaration_target, lower_runtime_target, ready_payload,
};
use super::{
    materialize_canonical_admission_artifact, materialize_canonical_aftermath_artifact,
    materialize_canonical_continuity_artifact, materialize_canonical_explanation_artifact,
    materialize_canonical_invariant_capability_artifact,
    materialize_canonical_support_traceability_artifact, materialize_canonical_workflow_artifact,
    ForgeQueryDomainCapabilityPayload, ForgeQueryDomainCapabilityTargetBinding,
};

#[test]
fn equivalent_admission_meaning_canonicalizes_to_same_digest() {
    let left = materialize_canonical_admission_artifact(declaration_ready_contribution(
        "intent-a",
        ForgeQueryAdmissionContributionPayload::new(
            ForgeQueryAdmissionContributionPosture::Advisory,
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    ));
    let right = materialize_canonical_admission_artifact(declaration_ready_contribution(
        "intent-a",
        ForgeQueryAdmissionContributionPayload::new(
            ForgeQueryAdmissionContributionPosture::Advisory,
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    ));

    assert_eq!(
        left.materialization_digest(),
        right.materialization_digest()
    );
    assert_eq!(
        left.semantic_identity_digest(),
        right.semantic_identity_digest()
    );
    assert_eq!(left.canonical_family(), "intent-admission");
    let semantics = left
        .contribution()
        .payload()
        .target()
        .semantics()
        .intent_declaration()
        .expect("declaration semantics should be preserved");
    assert_eq!(semantics.0, "test.intent");
    assert_eq!(semantics.1, "test.strategy");
}

#[test]
fn different_target_or_semantic_posture_changes_canonical_digest() {
    let advisory = materialize_canonical_admission_artifact(declaration_ready_contribution(
        "intent-a",
        ForgeQueryAdmissionContributionPayload::new(
            ForgeQueryAdmissionContributionPosture::Advisory,
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    ));
    let violation = materialize_canonical_admission_artifact(declaration_ready_contribution(
        "intent-a",
        ForgeQueryAdmissionContributionPayload::new(
            ForgeQueryAdmissionContributionPosture::Violation,
            "spatial.arbitration.invalid_target",
            "requested target violates domain law",
        ),
    ));
    let other_target = materialize_canonical_admission_artifact(declaration_ready_contribution(
        "intent-b",
        ForgeQueryAdmissionContributionPayload::new(
            ForgeQueryAdmissionContributionPosture::Advisory,
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    ));

    assert_ne!(
        advisory.materialization_digest(),
        violation.materialization_digest()
    );
    assert_ne!(
        advisory.materialization_digest(),
        other_target.materialization_digest()
    );
}

#[test]
fn all_categories_materialize_profile_independent_canonical_artifacts() {
    let support =
        materialize_canonical_support_traceability_artifact(lower_runtime_ready_contribution(
            "boundary-a",
            ForgeQuerySupportContributionPayload::new(
                ForgeQuerySupportContributionPosture::DeclarationTraceability,
                "runtime.boundary.traceability",
                "reconstructed boundary evidence stays attributable",
            ),
        ));
    let invariant =
        materialize_canonical_invariant_capability_artifact(declaration_ready_contribution(
            "intent-c",
            ForgeQueryInvariantCapabilityContributionPayload::new(
                ForgeQueryInvariantCapabilityContributionPosture::CapabilityGap,
                "graph.face_inner_loop_insertion",
                "topology substrate is unavailable",
            ),
        ));
    let workflow = materialize_canonical_workflow_artifact(admitted_plan_ready_contribution(
        "plan-a",
        ForgeQueryWorkflowContributionPayload::new(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            "preview.confirmation.required",
            "destructive promotion requires confirmation",
        ),
    ));
    let continuity = materialize_canonical_continuity_artifact(admitted_plan_ready_contribution(
        "plan-b",
        ForgeQueryContinuityContributionPayload::new(
            ForgeQueryContinuityContributionPosture::Split,
            "continuity.identity.split",
            "edge split produces two descendant identities",
        ),
    ));
    let aftermath = materialize_canonical_aftermath_artifact(lower_runtime_ready_contribution(
        "boundary-b",
        ForgeQueryAftermathContributionPayload::new(
            ForgeQueryAftermathContributionPosture::EstablishesFact,
            "aftermath.projection.fact_established",
            "projection establishes relation aftermath",
        ),
    ));
    let explanation = materialize_canonical_explanation_artifact(declaration_ready_contribution(
        "intent-d",
        ForgeQueryExplanationContributionPayload::new(
            ForgeQueryExplanationContributionPosture::ExplainsFallback,
            "spatial.reorient.fallback",
            "used canonical perpendicular fallback",
        ),
    ));

    assert_eq!(support.canonical_family(), "declaration-support");
    assert_eq!(invariant.canonical_family(), "capability-invariant");
    assert_eq!(workflow.canonical_family(), "preview-workflow");
    assert_eq!(continuity.canonical_family(), "continuity-lineage");
    assert_eq!(aftermath.canonical_family(), "consequence-aftermath");
    assert_eq!(explanation.canonical_family(), "explanation-inspection");

    for digest in [
        support.materialization_digest(),
        invariant.materialization_digest(),
        workflow.materialization_digest(),
        continuity.materialization_digest(),
        aftermath.materialization_digest(),
        explanation.materialization_digest(),
    ] {
        assert!(!digest.is_empty());
    }

    let workflow_plan = workflow
        .contribution()
        .payload()
        .target()
        .semantics()
        .admitted_intent_plan()
        .expect("admitted-plan semantics should be preserved");
    assert_eq!(
        workflow_plan.0,
        crate::intent_admission::ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent
    );
    let support_boundary = support
        .contribution()
        .payload()
        .target()
        .semantics()
        .lower_runtime_boundary()
        .expect("lower-runtime semantics should be preserved");
    assert_eq!(
        support_boundary.0,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule
    );
}

fn declaration_ready_contribution<P>(
    target_digest: &str,
    payload: P,
) -> super::ForgeQueryMaterializationReadyDomainCapabilityContribution<
    P,
    ForgeQueryDeclarationBoundContributionTarget,
>
where
    P: ForgeQueryDomainCapabilityPayload,
    (P, ForgeQueryDeclarationBoundContributionTarget):
        super::proof_integration::AllowedContributionBinding<
            P,
            ForgeQueryDeclarationBoundContributionTarget,
        >,
{
    ready_payload(declaration_target(target_digest), payload)
}

fn admitted_plan_ready_contribution<P>(
    target_digest: &str,
    payload: P,
) -> super::ForgeQueryMaterializationReadyDomainCapabilityContribution<
    P,
    ForgeQueryAdmittedPlanBoundContributionTarget,
>
where
    P: ForgeQueryDomainCapabilityPayload,
    (P, ForgeQueryAdmittedPlanBoundContributionTarget):
        super::proof_integration::AllowedContributionBinding<
            P,
            ForgeQueryAdmittedPlanBoundContributionTarget,
        >,
{
    ready_payload(admitted_plan_target(target_digest), payload)
}

fn lower_runtime_ready_contribution<P>(
    target_digest: &str,
    payload: P,
) -> super::ForgeQueryMaterializationReadyDomainCapabilityContribution<
    P,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
>
where
    P: ForgeQueryDomainCapabilityPayload,
    (P, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget):
        super::proof_integration::AllowedContributionBinding<
            P,
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
{
    ready_payload(lower_runtime_target(target_digest), payload)
}
