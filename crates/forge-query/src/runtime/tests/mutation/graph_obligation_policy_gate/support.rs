pub(super) use crate::runtime::tests::support::*;

use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey,
};
use crate::policy_basis::{
    admit_policy_tenant_context, AdmittedPolicyTenantContext, BranchAccessGrant, PolicyEpoch,
    PolicyExecutionModeRequest, PolicyRuleSnapshot,
};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};

pub(super) fn policy_context(label: &str, narrows_projection: bool) -> AdmittedPolicyTenantContext {
    policy_context_with_policy_posture(label, narrows_projection, false)
}

pub(super) fn policy_context_with_policy_posture(
    label: &str,
    narrows_projection: bool,
    admits_non_disclosing_use: bool,
) -> AdmittedPolicyTenantContext {
    policy_context_for_mode(
        label,
        narrows_projection,
        admits_non_disclosing_use,
        PolicyExecutionModeRequest::GraphMutation,
    )
}

pub(super) fn policy_context_for_mode(
    label: &str,
    narrows_projection: bool,
    admits_non_disclosing_use: bool,
    mode: PolicyExecutionModeRequest,
) -> AdmittedPolicyTenantContext {
    let canonical = canonical_task_query();
    let policy = PolicyRuleSnapshot::synthetic_authority_with_posture(
        format!("policy-{label}"),
        format!("rules-{label}"),
        PolicyEpoch::Synthetic(7),
        true,
        narrows_projection,
        admits_non_disclosing_use,
    );
    let tenant = TenantBindingSnapshot::synthetic_direct(
        format!("tenant-{label}"),
        format!("branch-{label}"),
        format!("schema-{label}"),
        TenantBasisEpoch::Synthetic(3),
    );
    let branch = BranchAccessGrant::synthetic_granted(format!("branch-{label}"), &policy);
    let schema = SchemaVariantSnapshot::synthetic_authority(
        format!("tenant-{label}"),
        format!("schema-{label}"),
        "compatible",
    );
    admit_policy_tenant_context(canonical.query(), policy, tenant, branch, schema, mode)
        .expect("policy/tenant context should admit")
}

pub(super) fn runtime_with_policy_gate(
    label: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
) -> ForgeQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(policy_gate_registration(label, support_posture))
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with policy gate")
}

pub(super) fn policy_gate_registration(
    label: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::operating_context_gate(
        ForgeQueryGraphObligationRuleIdentity::new(
            "test.graph-obligation-policy-gate",
            label,
            "v1",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection("Task").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(support_posture)
}

pub(super) fn policy_gate_registration_for_collection(
    label: &str,
    collection: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::operating_context_gate(
        ForgeQueryGraphObligationRuleIdentity::new(
            "test.graph-obligation-policy-gate",
            label,
            "v1",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection(collection).unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(support_posture)
}

pub(super) fn task_insert_command(id: &str) -> ForgeQueryWriteCommand {
    ForgeQueryWriteCommand::InsertAspects {
        collection: "Task".to_string(),
        aspects: vec![
            ForgeQueryAspectValue::new("identity.id", id).unwrap(),
            ForgeQueryAspectValue::new("title.value", "Policy gated task").unwrap(),
        ],
        symbolic_aspect_references: Vec::new(),
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
        symbolic_target_reference: None,
    }
}

pub(super) fn task_graph_program(
    id: &str,
) -> (
    Vec<ForgeQueryWriteCommand>,
    ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram,
) {
    let mut graph = ForgeQueryGraphCompositionBuilder::new();
    graph
        .insert_entity("task", "Task", |entity| {
            entity
                .aspect("identity.id", id)
                .aspect("title.value", "Policy gated graph task")
        })
        .unwrap();
    graph.finish().unwrap()
}

pub(super) fn supported_policy_gate_runtime(label: &str) -> ForgeQueryRuntime {
    runtime_with_policy_gate(
        label,
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::ScalarMutation,
        ),
    )
}

pub(super) fn supported_batch_policy_gate_runtime(label: &str) -> ForgeQueryRuntime {
    runtime_with_policy_gate(
        label,
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    )
}

pub(super) fn supported_graph_policy_gate_runtime(label: &str) -> ForgeQueryRuntime {
    runtime_with_policy_gate(
        label,
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::GraphComposition,
        ),
    )
}

pub(super) fn canonical_task_query() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("title", "value").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("title", "value", "title").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}
