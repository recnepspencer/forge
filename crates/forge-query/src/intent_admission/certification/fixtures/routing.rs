use crate::facade::runtime::{
    ForgeQueryExistingEntityTarget, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthProbeResult, ForgeQueryExistingTruthTargetBinding, ForgeQueryWorkspace,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryRawIntentAdmissionRequest,
};

use super::runtime::certification_runtime;

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedRoutingIntentFixture {
    pub(in crate::intent_admission::certification) request: ForgeQueryRawIntentAdmissionRequest,
    pub(in crate::intent_admission::certification) decision: ForgeQueryIntentAdmissionDecision,
    pub(in crate::intent_admission::certification) handoff_digest: String,
    pub(in crate::intent_admission::certification) binding_digest: String,
    pub(in crate::intent_admission::certification) trace_digest: String,
    pub(in crate::intent_admission::certification) result: ForgeQueryExistingTruthProbeResult,
}

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct RoutingDelegationParityFixture {
    pub(in crate::intent_admission::certification) workspace_legacy_trace_digest: String,
    pub(in crate::intent_admission::certification) workspace_legacy_provenance_digest: String,
    pub(in crate::intent_admission::certification) workspace_legacy_probe_digest: String,
    pub(in crate::intent_admission::certification) workspace_canonical:
        ForgeQueryExistingTruthProbeResult,
    pub(in crate::intent_admission::certification) runtime_legacy_trace_digest: String,
    pub(in crate::intent_admission::certification) runtime_legacy_provenance_digest: String,
    pub(in crate::intent_admission::certification) runtime_legacy_probe_digest: String,
    pub(in crate::intent_admission::certification) runtime_canonical:
        ForgeQueryExistingTruthProbeResult,
}

pub(in crate::intent_admission::certification) fn certified_routing_intent_fixture(
) -> CertifiedRoutingIntentFixture {
    let mut workspace = certification_runtime()
        .workspace("certification-routing-intent")
        .expect("routing certification workspace should open");
    let binding = seeded_probe_binding(&mut workspace);
    let request = ForgeQueryExistingTruthProbeRequest::new(binding, ["identity.id", "title.value"])
        .expect("routing certification request should build");
    let runtime = workspace.into_runtime();
    let review = runtime
        .probe_existing_intent(request)
        .review()
        .expect("routing certification review should succeed");
    let request = review.request().clone();
    let decision = review.decision().clone();
    let admitted = review
        .admit()
        .expect("routing certification admitted path should resolve");
    let handoff_digest = admitted.handoff().handoff_digest().to_string();
    let binding_digest = admitted.execution_binding().binding_digest().to_string();
    let result = admitted
        .execute()
        .expect("routing certification admitted execution should succeed");
    let trace_digest = result
        .receipt()
        .decision_trace_envelope()
        .expect("routing certification result should retain a trace")
        .trace_digest()
        .to_string();
    CertifiedRoutingIntentFixture {
        request,
        decision,
        handoff_digest,
        binding_digest,
        trace_digest,
        result,
    }
}

pub(in crate::intent_admission::certification) fn routing_delegation_parity_fixture(
) -> RoutingDelegationParityFixture {
    let mut delegated_workspace = certification_runtime()
        .workspace("certification-routing-workspace-legacy")
        .expect("workspace routing parity workspace should open");
    let delegated_binding = seeded_probe_binding(&mut delegated_workspace);
    let workspace_legacy_request =
        ForgeQueryExistingTruthProbeRequest::new(delegated_binding, ["identity.id", "title.value"])
            .expect("workspace routing legacy request should build");
    let workspace_legacy = delegated_workspace
        .probe_existing(
            workspace_legacy_request.binding().clone(),
            workspace_legacy_request
                .aspect_paths()
                .iter()
                .map(String::as_str),
        )
        .expect("workspace routing legacy path should execute");
    let workspace_legacy_canonical = delegated_workspace
        .probe_existing_intent(workspace_legacy_request.clone())
        .execute()
        .expect("workspace routing delegated evidence should execute");

    let mut canonical_workspace = certification_runtime()
        .workspace("certification-routing-workspace-canonical")
        .expect("workspace routing canonical workspace should open");
    let canonical_binding = seeded_probe_binding(&mut canonical_workspace);
    let canonical_request =
        ForgeQueryExistingTruthProbeRequest::new(canonical_binding, ["identity.id", "title.value"])
            .expect("workspace routing canonical request should build");
    let workspace_canonical = canonical_workspace
        .probe_existing_intent(canonical_request)
        .review()
        .expect("workspace routing canonical review should succeed")
        .admit()
        .expect("workspace routing canonical admission should succeed")
        .execute()
        .expect("workspace routing canonical path should execute");

    let mut delegated_runtime_workspace = certification_runtime()
        .workspace("certification-routing-runtime-legacy")
        .expect("runtime routing legacy workspace should open");
    let delegated_runtime_binding = seeded_probe_binding(&mut delegated_runtime_workspace);
    let delegated_runtime_request = ForgeQueryExistingTruthProbeRequest::new(
        delegated_runtime_binding,
        ["identity.id", "title.value"],
    )
    .expect("runtime routing legacy request should build");
    let delegated_runtime = delegated_runtime_workspace.into_runtime();
    let runtime_legacy = delegated_runtime
        .probe_existing(delegated_runtime_request.clone())
        .expect("runtime routing legacy path should execute");
    let runtime_legacy_canonical = delegated_runtime
        .probe_existing_intent(delegated_runtime_request)
        .execute()
        .expect("runtime routing delegated evidence should execute");

    let mut canonical_runtime_workspace = certification_runtime()
        .workspace("certification-routing-runtime-canonical")
        .expect("runtime routing canonical workspace should open");
    let canonical_runtime_binding = seeded_probe_binding(&mut canonical_runtime_workspace);
    let canonical_runtime_request = ForgeQueryExistingTruthProbeRequest::new(
        canonical_runtime_binding,
        ["identity.id", "title.value"],
    )
    .expect("runtime routing canonical request should build");
    let canonical_runtime = canonical_runtime_workspace.into_runtime();
    let runtime_canonical = canonical_runtime
        .probe_existing_intent(canonical_runtime_request)
        .review()
        .expect("runtime routing canonical review should succeed")
        .admit()
        .expect("runtime routing canonical admission should succeed")
        .execute()
        .expect("runtime routing canonical path should execute");

    RoutingDelegationParityFixture {
        workspace_legacy_trace_digest: workspace_legacy_canonical
            .receipt()
            .decision_trace_envelope()
            .expect("workspace routing delegated evidence should retain a trace")
            .trace_digest()
            .to_string(),
        workspace_legacy_provenance_digest: workspace_legacy_canonical
            .receipt()
            .execution_provenance_chain_digest()
            .expect("workspace routing delegated evidence should retain provenance")
            .to_string(),
        workspace_legacy_probe_digest: workspace_legacy.probe_digest().to_string(),
        workspace_canonical,
        runtime_legacy_trace_digest: runtime_legacy_canonical
            .receipt()
            .decision_trace_envelope()
            .expect("runtime routing delegated evidence should retain a trace")
            .trace_digest()
            .to_string(),
        runtime_legacy_provenance_digest: runtime_legacy_canonical
            .receipt()
            .execution_provenance_chain_digest()
            .expect("runtime routing delegated evidence should retain provenance")
            .to_string(),
        runtime_legacy_probe_digest: runtime_legacy.probe_digest().to_string(),
        runtime_canonical,
    }
}

fn seeded_probe_binding(
    workspace: &mut ForgeQueryWorkspace,
) -> ForgeQueryExistingTruthTargetBinding {
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("routing certification seed insert should execute");
    let authority_label =
        crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
            .expect("routing certification authority label should build");
    let authority =
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            authority_label,
        )
        .expect("routing certification authority identity should build");
    workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                authority,
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("routing certification existing entity target should build")
            .in_target_collection("Task")
            .expect("routing certification target collection should build"),
        )
        .expect("routing certification binding should build")
}
