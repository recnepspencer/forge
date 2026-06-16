mod basis_projection;
mod bridge;
mod effect;
mod inspection;
mod neighbors;
mod read;
mod routing;
mod runtime;
mod write_authority;

use serde_json::json;

pub(in crate::intent_admission::certification) use basis_projection::{
    certified_basis_observation_intent_fixture, certified_projection_consumption_admitted_fixture,
    certified_projection_consumption_warning_fixture,
};
pub(crate) use bridge::certification_bridge;
pub(in crate::intent_admission::certification) use effect::certified_effect_intent_fixture;
pub(in crate::intent_admission::certification) use inspection::certified_inspection_advisory_redaction_fixture;
pub(in crate::intent_admission::certification) use neighbors::{
    certified_deferred_intent_fixture, certified_unsupported_intent_fixture,
    CertifiedDeferredIntentFixture, CertifiedUnsupportedIntentFixture,
};
pub(in crate::intent_admission::certification) use read::{
    certified_read_intent_fixture, read_delegation_parity_fixture, CertifiedReadIntentFixture,
};
pub(in crate::intent_admission::certification) use routing::{
    certified_routing_intent_fixture, routing_delegation_parity_fixture,
    CertifiedRoutingIntentFixture, RoutingDelegationParityFixture,
};
pub(crate) use runtime::{
    certification_runtime, certification_runtime_with_invariant_violation_authority,
    certification_task_live_request, certification_task_schema,
};

use crate::facade::runtime::{
    admit_runtime_intent_request, ForgeQueryEffectDeclaration, ForgeQueryEffectTrigger,
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdvisoryDecision,
    ForgeQueryIntentDeclaration, ForgeQueryRawIntentAdmissionRequest, ForgeQueryWriteCommand,
};
use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity,
};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

pub(super) fn certification_commit_identity_for(
    namespace: impl AsRef<str>,
    evidence: impl AsRef<str>,
) -> ForgeQueryCommitIdentity {
    ForgeQueryCommitIdentity::from_relational_commit_id(stable_certification_position(
        namespace, evidence,
    ))
}

pub(super) fn certification_snapshot_identity(
    label: impl AsRef<str>,
) -> ForgeQuerySnapshotIdentity {
    certification_snapshot_identity_for("certification-snapshot", label)
}

pub(super) fn certification_snapshot_identity_for(
    namespace: impl AsRef<str>,
    evidence: impl AsRef<str>,
) -> ForgeQuerySnapshotIdentity {
    let snapshot_id = stable_certification_position(namespace.as_ref(), evidence.as_ref());
    let version_id =
        stable_certification_position(format!("{}:version", namespace.as_ref()), evidence.as_ref());
    ForgeQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(snapshot_id, version_id),
    )
}

pub(super) fn certification_entity_identity(label: impl AsRef<str>) -> ForgeQueryEntityIdentity {
    crate::memory_workspace::admit_authored_entity_label(label)
}

fn stable_certification_position(namespace: impl AsRef<str>, evidence: impl AsRef<str>) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in namespace.as_ref().bytes().chain(evidence.as_ref().bytes()) {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211_u64);
    }
    acc
}

#[derive(Clone)]
pub(super) struct CertifiedAdmittedIntentFixture {
    pub(super) request: ForgeQueryRawIntentAdmissionRequest,
    pub(super) decision: ForgeQueryIntentAdmissionDecision,
    pub(super) plan: crate::intent_admission::ForgeQueryAdmittedIntentPlan,
    pub(super) handoff: crate::intent_admission::ForgeQueryAdmittedIntentExecutionHandoff,
    pub(super) binding: crate::intent_admission::ForgeQueryAuthoritativeIntentExecutionBinding,
    pub(super) trace: crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope,
    pub(super) receipt: crate::runtime::ForgeQueryIntentReceipt,
}

#[derive(Clone)]
pub(super) struct CertifiedAdvisoryIntentFixture {
    pub(super) request: ForgeQueryRawIntentAdmissionRequest,
    pub(super) decision: ForgeQueryIntentAdmissionDecision,
    pub(super) trace: crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope,
}

#[derive(Clone)]
pub(super) struct CertifiedViolationIntentFixture {
    pub(super) request: ForgeQueryRawIntentAdmissionRequest,
    pub(super) decision: ForgeQueryIntentAdmissionDecision,
    pub(super) trace: crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope,
}

#[derive(Clone)]
pub(super) struct CertifiedFailureIntentFixture {
    pub(super) request: ForgeQueryRawIntentAdmissionRequest,
    pub(super) failure_digest: String,
    pub(super) execution_provenance_chain_digest: String,
    pub(super) trace: crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope,
}

pub(super) struct LegacyDelegationParityFixture {
    pub(super) authoritative_legacy: crate::runtime::ForgeQueryIntentReceipt,
    pub(super) authoritative_canonical: crate::runtime::ForgeQueryIntentReceipt,
    pub(super) effect_legacy: crate::runtime::ForgeQueryEffectIntentReceipt,
    pub(super) effect_canonical: crate::runtime::ForgeQueryEffectIntentReceipt,
    pub(super) read_current_legacy: crate::runtime::ForgeQueryReadResult,
    pub(super) read_current_canonical: crate::runtime::ForgeQueryReadResult,
    pub(super) read_basis_legacy: crate::runtime::ForgeQueryReadResult,
    pub(super) read_basis_canonical: crate::runtime::ForgeQueryReadResult,
}

pub(super) fn certified_admitted_intent_fixture() -> CertifiedAdmittedIntentFixture {
    let declaration = authoritative_declaration("certification-admitted-intent");
    let request =
        ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(declaration.clone())
            .expect("authoritative request should build");
    let _eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let decision = admit_runtime_intent_request(request.clone());
    match &decision {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            let mut runtime = certification_runtime();
            let plan = plan.clone();
            let handoff = plan.clone().into_execution_handoff();
            let authoritative_handoff = match handoff.clone() {
                Some(crate::intent_admission::ForgeQueryAdmittedIntentExecutionHandoff::Authoritative(
                    handoff,
                )) => handoff,
                other => panic!("expected authoritative handoff, got {other:?}"),
            };
            let binding =
                runtime.prepare_authoritative_intent_execution_binding(authoritative_handoff);
            let receipt = runtime
                .execute_intent(declaration)
                .expect("admitted certification intent should execute");
            let trace = receipt.decision_trace_envelope().clone();
            CertifiedAdmittedIntentFixture {
                request,
                decision,
                plan,
                handoff: handoff.expect("authoritative admitted plan should still mint a handoff"),
                binding,
                trace,
                receipt,
            }
        }
        other => panic!("expected admitted decision, got {other:?}"),
    }
}

pub(super) fn certified_advisory_intent_fixture() -> CertifiedAdvisoryIntentFixture {
    let declaration = authoritative_declaration("certification-advisory-intent");
    let request =
        ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(declaration)
            .expect("authoritative request should build");
    let eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let advisory = ForgeQueryIntentAdvisoryDecision::new(
        request.family(),
        request.entrypoint(),
        "materialized-detail-advisory",
        "full execution is intentionally deferred",
        request.request_digest(),
        eligibility.eligibility_digest(),
    );
    let decision = ForgeQueryIntentAdmissionDecision::Advisory(advisory);
    let review = ForgeQueryRuntimeIntentAdmissionReviewData::from_decision(
        request.clone(),
        decision.clone(),
    );
    let trace = review
        .decision_trace_envelope()
        .expect("advisory review should preserve a trace")
        .clone();
    CertifiedAdvisoryIntentFixture {
        request,
        decision,
        trace,
    }
}

pub(super) fn certified_violation_intent_fixture() -> CertifiedViolationIntentFixture {
    let declaration = authoritative_declaration("certification-violation-intent");
    let request = ForgeQueryRawIntentAdmissionRequest::effect_runtime_entrypoint(declaration)
        .expect("violation request should still build");
    let _eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let decision = admit_runtime_intent_request(request.clone());
    let review = ForgeQueryRuntimeIntentAdmissionReviewData::from_decision(
        request.clone(),
        decision.clone(),
    );
    let trace = review
        .decision_trace_envelope()
        .expect("violation review should preserve a trace")
        .clone();
    CertifiedViolationIntentFixture {
        request,
        decision,
        trace,
    }
}

pub(super) fn certified_failure_intent_fixture() -> CertifiedFailureIntentFixture {
    let declaration = authoritative_declaration("certification-failure-intent");
    let request =
        ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(declaration.clone())
            .expect("authoritative request should build");
    let mut runtime = certification_runtime_with_invariant_violation_authority();
    let error = runtime
        .execute_intent(declaration)
        .expect_err("invariant violation should preserve denial evidence");
    match error {
        crate::runtime::ForgeQueryRuntimeError::IntentCommitDenied { evidence, .. } => {
            let trace = evidence
                .decision_trace_envelope()
                .expect("execution-time denial should retain a trace")
                .clone();
            CertifiedFailureIntentFixture {
                request,
                failure_digest: evidence
                    .denial_digest()
                    .terminal_projection_for_reporting()
                    .to_string(),
                execution_provenance_chain_digest: evidence
                    .execution_provenance()
                    .expect("execution-time denial should retain provenance")
                    .execution_provenance_chain_digest()
                    .to_string(),
                trace,
            }
        }
        other => panic!("expected execution-time denial evidence, got {other:?}"),
    }
}

pub(super) fn legacy_delegation_parity_fixture() -> LegacyDelegationParityFixture {
    let read_fixture = read_delegation_parity_fixture();
    let declaration = authoritative_declaration("certification-authoritative-parity-intent");
    let mut delegated_runtime = certification_runtime();
    let authoritative_legacy = delegated_runtime
        .execute_intent(declaration.clone())
        .expect("legacy authoritative entrypoint should execute");

    let mut canonical_runtime = certification_runtime();
    let canonical_handoff = canonical_runtime
        .admit_authoritative_intent_for_execution(declaration.clone())
        .expect("canonical authoritative handoff should admit");
    let canonical_binding =
        canonical_runtime.prepare_authoritative_intent_execution_binding(canonical_handoff);
    let authoritative_canonical = canonical_runtime
        .execute_authoritative_intent_execution_binding(canonical_binding)
        .expect("canonical authoritative execution should succeed");

    let mut delegated_effect_runtime = certification_runtime();
    let delegated_live = delegated_effect_runtime
        .declare_live_view::<serde_json::Value>(
            "certification.effect-live",
            certification_task_live_request(),
            certification_task_schema(),
        )
        .expect("live view should declare");
    let delegated_effect = delegated_effect_runtime
        .declare_effect::<serde_json::Value>(ForgeQueryEffectDeclaration::write_intent(
            "effects.certification.reconcile",
            ForgeQueryEffectTrigger::live_view(&delegated_live, ["title.value"]),
            "strategy.intent.reconcile",
        ))
        .expect("effect should declare");
    delegated_effect_runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: certification_entity_identity("task-1"),
            aspect_path: "title.value".to_string(),
            value: json!("title from delegated effect"),
        })
        .expect("delegated effect write should queue");
    let effect_legacy = delegated_effect_runtime
        .execute_next_effect_write_intent(&delegated_effect, "1.0", "effect.intent.input.v1")
        .expect("legacy effect entrypoint should execute");

    let mut canonical_effect_runtime = certification_runtime();
    let canonical_live = canonical_effect_runtime
        .declare_live_view::<serde_json::Value>(
            "certification.effect-live",
            certification_task_live_request(),
            certification_task_schema(),
        )
        .expect("canonical live view should declare");
    let canonical_effect = canonical_effect_runtime
        .declare_effect::<serde_json::Value>(ForgeQueryEffectDeclaration::write_intent(
            "effects.certification.reconcile",
            ForgeQueryEffectTrigger::live_view(&canonical_live, ["title.value"]),
            "strategy.intent.reconcile",
        ))
        .expect("canonical effect should declare");
    canonical_effect_runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: certification_entity_identity("task-1"),
            aspect_path: "title.value".to_string(),
            value: json!("title from delegated effect"),
        })
        .expect("canonical effect write should queue");
    let (pending_delivery, canonical_handoff) = canonical_effect_runtime
        .admit_next_effect_write_intent_for_execution(
            canonical_effect.name(),
            "1.0",
            "effect.intent.input.v1",
        )
        .expect("canonical effect handoff should admit");
    let canonical_binding = canonical_effect_runtime
        .prepare_effect_intent_execution_binding(canonical_handoff, &pending_delivery);
    let effect_canonical = canonical_effect_runtime
        .execute_effect_intent_execution_binding(canonical_binding)
        .expect("canonical effect execution should succeed");

    LegacyDelegationParityFixture {
        authoritative_legacy,
        authoritative_canonical,
        effect_legacy,
        effect_canonical,
        read_current_legacy: read_fixture.current_legacy,
        read_current_canonical: read_fixture.current_canonical,
        read_basis_legacy: read_fixture.basis_legacy,
        read_basis_canonical: read_fixture.basis_canonical,
    }
}

pub(super) fn authoritative_declaration(name: &str) -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        name,
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1", "title": "Certification committed title"}),
    )
}
