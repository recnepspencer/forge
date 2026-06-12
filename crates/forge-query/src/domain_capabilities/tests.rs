use forge_proof::TransitionOutcome;
use serde_json::json;

use super::authoring::{
    ForgeQueryAdmissionContributionAuthoring, ForgeQueryAftermathContributionAuthoring,
    ForgeQueryContinuityContributionAuthoring, ForgeQueryExplanationContributionAuthoring,
    ForgeQueryInvariantCapabilityContributionAuthoring, ForgeQuerySupportContributionAuthoring,
    ForgeQueryWorkflowContributionAuthoring,
};
use super::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use super::payloads::{ForgeQuerySupportContributionPayload, ForgeQuerySupportContributionPosture};
use super::proof_integration::create_requested_domain_capability_contribution;
use super::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryDomainCapabilityTargetKind, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use super::test_support::{
    admitted_plan, admitted_plan_target, declaration_target, lower_runtime_envelope,
    lower_runtime_target, success,
};
use crate::runtime::ForgeQueryIntentDeclaration;
use crate::target_binding::{
    ForgeQueryAdmittedIntentPlanBindingTarget, ForgeQueryBindingTargetWitness,
    ForgeQueryIntentDeclarationBindingTarget, ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget,
};

#[test]
fn requested_contribution_denies_empty_semantic_code() {
    let declaration = sample_declaration("rotate");
    let requested = ForgeQueryAdmissionContributionAuthoring::advisory("", "needs clarification")
        .for_intent_declaration(&declaration);

    match evaluate_requested_domain_capability_contribution(requested) {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.kind(),
                super::ForgeQueryDomainCapabilityProgressionDenialKind::EmptySemanticCode
            );
            assert_eq!(denial.category(), "admission");
            assert_eq!(
                denial.target_kind(),
                ForgeQueryDomainCapabilityTargetKind::IntentDeclaration
            );
        }
        _ => panic!("expected empty semantic code denial"),
    }
}

#[test]
fn request_digest_survives_successful_progression() {
    let declaration = sample_declaration("move");
    let requested = ForgeQueryAdmissionContributionAuthoring::advisory(
        "arbitration.requires_clarification",
        "multiple candidates remain admissible",
    )
    .for_intent_declaration(&declaration);
    let request_digest = requested.payload().request_digest().to_string();
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    let admitted = success(admit_eligible_domain_capability_contribution(eligible));
    let ready = success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            admitted,
            ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(&declaration),
        ),
    );

    assert_eq!(ready.payload().request_digest(), request_digest);
}

#[test]
fn declaration_drift_requires_rebind() {
    let declaration = sample_declaration("offset");
    let requested = ForgeQueryAdmissionContributionAuthoring::violation(
        "spatial.target_binding_changed",
        "the original intent declaration no longer matches the contribution binding",
    )
    .for_intent_declaration(&declaration);
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    let admitted = success(admit_eligible_domain_capability_contribution(eligible));
    let rebound_target = declaration_target("different-intent-digest");
    let rebound_digest = rebound_target.target_digest().to_string();

    match prepare_admitted_domain_capability_contribution_for_materialization(
        admitted,
        rebound_target,
    ) {
        TransitionOutcome::RebindRequired(rebind) => {
            assert_eq!(rebind.category(), "admission");
            assert_eq!(rebind.current_target_digest(), rebound_digest);
        }
        _ => panic!("expected rebind-required outcome"),
    }
}

#[test]
fn lower_runtime_boundary_drift_is_stale_not_rebind() {
    let requested = create_requested_domain_capability_contribution(
        lower_runtime_target("envelope-a"),
        ForgeQuerySupportContributionPayload::new(
            ForgeQuerySupportContributionPosture::DeclarationSupport,
            "runtime.boundary.support",
            "support posture comes from a lower-runtime seam",
        ),
    );
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    let admitted = success(admit_eligible_domain_capability_contribution(eligible));

    match prepare_admitted_domain_capability_contribution_for_materialization(
        admitted,
        lower_runtime_target("envelope-b"),
    ) {
        TransitionOutcome::Stale(stale) => {
            assert_eq!(stale.category(), "support-traceability");
            assert_ne!(stale.bound_target_digest(), stale.current_target_digest());
        }
        _ => panic!("expected stale outcome"),
    }
}

#[test]
fn every_category_reaches_admitted_form_through_the_same_lifecycle() {
    let declaration_target = declaration_target("intent-declaration-a");
    let admitted_plan_target = admitted_plan_target("admitted-plan-a");
    let envelope_target = lower_runtime_target("boundary-envelope-a");

    assert_admitted(
        ForgeQueryAdmissionContributionAuthoring::advisory(
            "admission.advisory",
            "admission detail",
        )
        .bind_to_declaration_target(declaration_target.clone()),
    );
    assert_admitted(
        ForgeQuerySupportContributionAuthoring::declaration_support(
            "support.declaration",
            "support detail",
        )
        .bind_to_declaration_target(declaration_target.clone()),
    );
    assert_admitted(
        ForgeQueryInvariantCapabilityContributionAuthoring::capability_gap(
            "invariant.capability_gap",
            "invariant detail",
        )
        .bind_to_lower_runtime_boundary_target(envelope_target.clone()),
    );
    assert_admitted(
        ForgeQueryWorkflowContributionAuthoring::promotion_eligible(
            "workflow.promotion_eligible",
            "workflow detail",
        )
        .bind_to_admitted_plan_target(admitted_plan_target.clone()),
    );
    assert_admitted(
        ForgeQueryContinuityContributionAuthoring::preserved(
            "continuity.preserved",
            "continuity detail",
        )
        .bind_to_admitted_plan_target(admitted_plan_target.clone()),
    );
    assert_admitted(
        ForgeQueryAftermathContributionAuthoring::declares_residue(
            "aftermath.residue",
            "aftermath detail",
        )
        .bind_to_lower_runtime_boundary_target(envelope_target),
    );
    assert_admitted(
        ForgeQueryExplanationContributionAuthoring::requires_context(
            "explanation.context",
            "explanation detail",
        )
        .bind_to_declaration_target(declaration_target),
    );
}

#[test]
fn compatibility_targets_share_digests_with_the_shared_binding_core() {
    let declaration = sample_declaration("bind");
    let legacy_declaration =
        ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(&declaration);
    let shared_declaration =
        ForgeQueryIntentDeclarationBindingTarget::for_intent_declaration(&declaration);

    assert_eq!(
        ForgeQueryBindingTargetWitness::binding_digest(&legacy_declaration),
        shared_declaration.binding_digest()
    );
    assert_eq!(
        ForgeQueryBindingTargetWitness::target_digest(&legacy_declaration),
        shared_declaration.target_digest()
    );

    let plan = admitted_plan("admitted-plan-a.request-a.eligibility-a.decision-a");
    let plan_target =
        ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(&plan);
    let shared_plan = ForgeQueryAdmittedIntentPlanBindingTarget::for_admitted_intent_plan(&plan);
    assert_eq!(
        ForgeQueryBindingTargetWitness::binding_digest(&plan_target),
        shared_plan.binding_digest()
    );

    let envelope = lower_runtime_envelope("boundary-envelope-a");
    let envelope_target =
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
            &envelope,
        );
    let shared_envelope =
        ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget::for_lower_runtime_boundary_envelope(
            &envelope,
        );
    assert_eq!(
        ForgeQueryBindingTargetWitness::binding_digest(&envelope_target),
        shared_envelope.binding_digest()
    );
}

fn sample_declaration(name: &str) -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        name,
        format!("worth.spatial.{name}"),
        "1",
        "worth.spatial.intent",
        json!({ "entity": format!("edge:{name}") }),
    )
}

fn assert_admitted<P, T>(requested: super::ForgeQueryRequestedDomainCapabilityContribution<P, T>)
where
    P: super::ForgeQueryDomainCapabilityPayload,
    T: super::ForgeQueryDomainCapabilityTargetBinding,
{
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    let _admitted = success(admit_eligible_domain_capability_contribution(eligible));
}
