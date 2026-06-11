use forge_proof::TransitionOutcome;

use crate::domain_capabilities::certification_closeout_test_support::{
    admitted_basis_observation_plan, admitted_projection_consumption_plan, intent_declaration,
    lower_runtime_envelope, projection_binding, projection_source, replay_gap_inputs,
};
use crate::domain_capabilities::forge_query_domain;
use forge_relational::facade::runtime::InvariantCatalog;

fn success<T>(
    outcome: crate::domain_capabilities::ForgeQueryDomainCapabilityTransitionOutcome<T>,
) -> T {
    match outcome {
        TransitionOutcome::Success(value) => value,
        _ => panic!("expected success"),
    }
}

fn projection_contract_request() -> crate::domain_capabilities::ForgeQueryProjectionContractRequest
{
    crate::domain_capabilities::ForgeQueryProjectionContractRequest::new(
        projection_source(),
        projection_binding(),
        crate::projection_consumption::ProjectMaterializedFacts::declare()
            .display_field("field.visible"),
    )
}

fn store_backed_replay_gap_request(
) -> crate::domain_capabilities::ForgeQueryLowerRuntimeExplanationRequest {
    let (reference_set, target) = replay_gap_inputs();
    crate::domain_capabilities::ForgeQueryLowerRuntimeExplanationRequest::explains_store_backed_replay_gap(
        reference_set,
        target,
        vec![crate::runtime::CausalEvidenceFamily::QueryInspection],
        crate::runtime::CausalInspectionRedactionPolicy::PreserveDetail,
        crate::runtime::CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
}

#[test]
fn public_common_and_checked_lanes_converge_for_named_categories() {
    let declaration = intent_declaration();
    let admitted_plan = admitted_basis_observation_plan();
    let projection_plan = admitted_projection_consumption_plan();
    let lower_runtime = lower_runtime_envelope("domain-capability-public-lane");

    let support_common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_traceability("traceability.edge_split")
        .because("public common and checked support lanes must agree")
        .materialize()
        .expect("common support lane should materialize");
    let support_checked = success(
        forge_query_domain("worth.spatial")
            .for_intent(&declaration)
            .supports_traceability("traceability.edge_split")
            .because("public common and checked support lanes must agree")
            .try_materialize()
            .into_transition_outcome(),
    );
    assert_eq!(support_common, support_checked);

    let admission_common = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&admitted_plan)
        .advises("admission.routing_gap")
        .because("public common and checked admission lanes must agree")
        .materialize()
        .expect("common admission lane should materialize");
    let admission_checked = success(
        forge_query_domain("worth.spatial")
            .for_admitted_intent_plan(&admitted_plan)
            .advises("admission.routing_gap")
            .because("public common and checked admission lanes must agree")
            .try_materialize()
            .into_transition_outcome(),
    );
    assert_eq!(admission_common, admission_checked);

    let workflow_common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .plans_preview_mutation(
            "workflow.preview_mutation",
            crate::facade::runtime::BridgePreviewSessionIdentity::new(
                "preview-session:public-lane",
            ),
        )
        .because("public common and checked workflow lanes must agree")
        .materialize()
        .expect("common workflow lane should materialize");
    let workflow_checked = success(
        forge_query_domain("worth.spatial")
            .for_intent(&declaration)
            .plans_preview_mutation(
                "workflow.preview_mutation",
                crate::facade::runtime::BridgePreviewSessionIdentity::new(
                    "preview-session:public-lane",
                ),
            )
            .because("public common and checked workflow lanes must agree")
            .try_materialize()
            .into_transition_outcome(),
    );
    assert_eq!(workflow_common, workflow_checked);

    let continuity_common = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&admitted_plan)
        .preserves_continuity("continuity.edge_split", "edge:before", "edge:after")
        .because("public common and checked continuity lanes must agree")
        .materialize()
        .expect("common continuity lane should materialize");
    let continuity_checked = success(
        forge_query_domain("worth.spatial")
            .for_admitted_intent_plan(&admitted_plan)
            .preserves_continuity("continuity.edge_split", "edge:before", "edge:after")
            .because("public common and checked continuity lanes must agree")
            .try_materialize()
            .into_transition_outcome(),
    );
    assert_eq!(continuity_common, continuity_checked);

    let aftermath_common = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&projection_plan)
        .consumes_projection_contract(
            "aftermath.projection_contract",
            projection_contract_request(),
        )
        .because("public common and checked aftermath lanes must agree")
        .materialize()
        .expect("common aftermath lane should materialize");
    let aftermath_checked = success(
        forge_query_domain("worth.spatial")
            .for_admitted_intent_plan(&projection_plan)
            .consumes_projection_contract(
                "aftermath.projection_contract",
                projection_contract_request(),
            )
            .because("public common and checked aftermath lanes must agree")
            .try_materialize()
            .into_transition_outcome(),
    );
    assert_eq!(aftermath_common, aftermath_checked);

    let explanation_common = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&lower_runtime)
        .explains_store_backed_replay_gap(
            "explanation.store_backed_replay",
            store_backed_replay_gap_request(),
        )
        .because("public common and checked explanation lanes must agree")
        .materialize_artifact()
        .expect("common explanation lane should materialize");
    let explanation_checked = success(
        forge_query_domain("worth.spatial")
            .for_lower_runtime_boundary_envelope(&lower_runtime)
            .explains_store_backed_replay_gap(
                "explanation.store_backed_replay",
                store_backed_replay_gap_request(),
            )
            .because("public common and checked explanation lanes must agree")
            .try_materialize_artifact()
            .into_transition_outcome(),
    );
    assert_eq!(explanation_common, explanation_checked);

    let invariant_common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .register_invariant_catalog("invariant.edge_split", InvariantCatalog::default())
        .because("public common and checked invariant lanes must agree")
        .materialize()
        .expect("common invariant lane should materialize");
    let invariant_checked = success(
        forge_query_domain("worth.spatial")
            .for_intent(&declaration)
            .register_invariant_catalog("invariant.edge_split", InvariantCatalog::default())
            .because("public common and checked invariant lanes must agree")
            .try_materialize()
            .into_transition_outcome(),
    );
    assert_eq!(invariant_common, invariant_checked);
}
