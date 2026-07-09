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
    support_posture: WorthQueryGraphObligationSupportPosture,
) -> WorthQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(policy_gate_registration(label, support_posture))
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with policy gate")
}

pub(super) fn policy_gate_registration(
    label: &str,
    support_posture: WorthQueryGraphObligationSupportPosture,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::operating_context_gate(
        WorthQueryGraphObligationRuleIdentity::new(
            "test.graph-obligation-policy-gate",
            label,
            "v1",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection("Task").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(support_posture)
}

pub(super) fn policy_gate_registration_for_collection(
    label: &str,
    collection: &str,
    support_posture: WorthQueryGraphObligationSupportPosture,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::operating_context_gate(
        WorthQueryGraphObligationRuleIdentity::new(
            "test.graph-obligation-policy-gate",
            label,
            "v1",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection(collection).unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(support_posture)
}

pub(super) fn task_insert_command(id: &str) -> WorthQueryWriteCommand {
    WorthQueryWriteCommand::InsertAspects {
        collection: crate::runtime::WorthQueryMutationTargetCollectionIdentity::new(
            "write-command-declared",
            "Task",
        ),
        aspects: vec![
            WorthQueryAdmittedAspectValue::new(
                test_aspect_touch("identity.id"),
                test_string_aspect_value(id),
            )
            .unwrap(),
            WorthQueryAdmittedAspectValue::new(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Policy gated task"),
            )
            .unwrap(),
        ],
        symbolic_aspect_references: Vec::new(),
        metadata: WorthQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
        symbolic_target_reference: None,
    }
}

pub(super) fn task_graph_program(
    id: &str,
) -> (
    Vec<WorthQueryWriteCommand>,
    WorthQueryGraphCompositionBreadth,
    WorthQueryGraphCompositionProgram,
) {
    let mut graph = WorthQueryGraphCompositionBuilder::new();
    graph
        .insert_entity("task", "Task", |entity| {
            entity
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value(id),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Policy gated graph task"),
                )
        })
        .unwrap();
    graph.finish().unwrap()
}

pub(super) fn supported_policy_gate_runtime(label: &str) -> WorthQueryRuntime {
    runtime_with_policy_gate(
        label,
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::ScalarMutation,
        ),
    )
}

pub(super) fn supported_batch_policy_gate_runtime(label: &str) -> WorthQueryRuntime {
    runtime_with_policy_gate(
        label,
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    )
}

pub(super) fn supported_graph_policy_gate_runtime(label: &str) -> WorthQueryRuntime {
    runtime_with_policy_gate(
        label,
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::GraphComposition,
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
