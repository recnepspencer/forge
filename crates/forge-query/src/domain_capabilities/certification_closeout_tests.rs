use forge_proof::TransitionOutcome;
use forge_relational::facade::runtime::InvariantCatalog;

use super::certification_closeout_test_support::{
    admission_digest, admitted_basis_observation_plan, admitted_projection_consumption_plan,
    admitted_ready, intent_declaration, lower_runtime_envelope, projection_binding,
    projection_source, replay_gap_inputs, success,
};
use crate::domain_capabilities::authoring::{
    ForgeQueryExplanationContributionAuthoring, ForgeQueryInvariantCapabilityContributionAuthoring,
    ForgeQuerySupportContributionAuthoring, ForgeQueryWorkflowContributionAuthoring,
};
use crate::domain_capabilities::canonical_runtime::{
    materialize_intent_admission_support_traceability_report,
    materialize_intent_declaration_support_traceability_artifact,
    materialize_projection_consumption_contract, materialize_query_causal_inspection_artifact,
    materialize_query_invariant_catalog_registration_artifact,
    materialize_query_workflow_declaration, materialize_runtime_admission_decision,
    materialize_runtime_continuity_evidence,
};
use crate::domain_capabilities::certification::{
    certify_domain_capabilities, forge_query_domain_capability_certification_surface,
    forge_query_domain_capability_compile_fail_boundary_digest,
    forge_query_domain_capability_public_surface_inventory,
    forge_query_domain_capability_representative_report,
    forge_query_domain_capability_slope_report,
};
use crate::domain_capabilities::{
    evaluate_requested_domain_capability_contribution, forge_query_domain,
};
use crate::runtime::{
    CausalEvidenceFamily, CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
};

#[test]
fn certification_bundle_matches_equivalent_public_and_proof_outputs() {
    let bundle = certify_domain_capabilities();
    let declaration = intent_declaration();
    let lower_runtime = lower_runtime_envelope("domain-capability-closeout");
    let (reference_set, target) = replay_gap_inputs();

    let support = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_traceability("traceability.edge_split")
        .because("declaration-scoped support remains declaration scoped")
        .materialize()
        .expect("support common lane should materialize");
    let support_proof = success(
        materialize_intent_declaration_support_traceability_artifact(admitted_ready(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "worth.spatial.traceability.edge_split",
                "declaration-scoped support remains declaration scoped",
            )
            .for_intent_declaration(&declaration),
        )),
    );
    assert_eq!(support, support_proof);
    assert_eq!(
        bundle.output_digest("support_artifact_digest"),
        Some(support.materialization_digest())
    );

    let workflow = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .plans_preview_mutation("workflow.preview_mutation", "preview-session:certification")
        .because("preview mutation planning should preserve canonical workflow semantics")
        .materialize()
        .expect("workflow common lane should materialize");
    let workflow_proof = success(materialize_query_workflow_declaration(admitted_ready(
        ForgeQueryWorkflowContributionAuthoring::promotion_eligible_mutation_lowering(
            "worth.spatial.workflow.preview_mutation",
            "preview mutation planning should preserve canonical workflow semantics",
            "preview-session:certification",
        )
        .for_intent_declaration(&declaration),
    )));
    assert_eq!(workflow.workflow_declaration(), &workflow_proof);
    assert_eq!(
        bundle.output_digest("workflow_artifact_digest"),
        Some(
            workflow
                .workflow_declaration()
                .report()
                .declaration_digest()
        )
    );

    let explanation = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&lower_runtime)
        .explains_store_backed_replay_gap(
            "explanation.store_backed_replay",
            crate::domain_capabilities::ForgeQueryLowerRuntimeExplanationRequest::explains_store_backed_replay_gap(
                reference_set.clone(),
                target.clone(),
                vec![CausalEvidenceFamily::QueryInspection],
                CausalInspectionRedactionPolicy::PreserveDetail,
                CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
            ),
        )
        .because("store-backed replay should preserve denied explanation identity")
        .materialize_artifact()
        .expect("explanation common lane should materialize");
    let explanation_proof = success(materialize_query_causal_inspection_artifact(
        admitted_ready(
            ForgeQueryExplanationContributionAuthoring::store_backed_replay_gap_explanation(
                "worth.spatial.explanation.store_backed_replay",
                "store-backed replay should preserve denied explanation identity",
                reference_set,
                target,
                vec![CausalEvidenceFamily::QueryInspection],
                CausalInspectionRedactionPolicy::PreserveDetail,
                CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
            )
            .for_lower_runtime_boundary_envelope(&lower_runtime),
        ),
    ));
    assert_eq!(explanation, explanation_proof);
    assert_eq!(
        bundle.output_digest("explanation_artifact_digest"),
        Some(explanation.artifact_digest())
    );

    let invariant = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .register_invariant_catalog("invariant.edge_split", InvariantCatalog::default())
        .because("geometry kernel must reject invalid edge splits")
        .materialize()
        .expect("invariant common lane should materialize");
    let invariant_proof = success(materialize_query_invariant_catalog_registration_artifact(
        admitted_ready(
            ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(
                InvariantCatalog::default(),
                "worth.spatial.invariant.edge_split",
                "geometry kernel must reject invalid edge splits",
            )
            .for_intent_declaration(&declaration),
        ),
    ));
    assert_eq!(invariant, invariant_proof);
}

#[test]
fn certification_bundle_preserves_support_scope_and_posture_distinction() {
    let bundle = certify_domain_capabilities();
    let declaration = intent_declaration();
    let admitted_plan = admitted_basis_observation_plan();

    let traceability = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_traceability("traceability.edge_split")
        .because("declaration-scoped support remains declaration scoped")
        .materialize()
        .expect("traceability support should materialize");
    let plain_support = success(
        materialize_intent_declaration_support_traceability_artifact(admitted_ready(
            ForgeQuerySupportContributionAuthoring::declaration_support(
                "worth.spatial.traceability.edge_split",
                "declaration-scoped support remains declaration scoped",
            )
            .for_intent_declaration(&declaration),
        )),
    );
    let admitted_plan_support = success(materialize_intent_admission_support_traceability_report(
        admitted_ready(
            ForgeQuerySupportContributionAuthoring::declaration_support(
                "worth.spatial.support.runtime_floor",
                "runtime floor remains explicitly supported",
            )
            .for_admitted_intent_plan(&admitted_plan),
        ),
    ));

    assert_eq!(traceability.intent_name(), declaration.name());
    assert_ne!(
        traceability.materialization_digest(),
        plain_support.materialization_digest()
    );
    assert_ne!(
        traceability.materialization_digest(),
        admitted_plan_support.decision_support_traceability_digest()
    );
    assert_ne!(
        bundle
            .output_digest("support_artifact_digest")
            .expect("bundle should emit support artifact digest"),
        bundle
            .output_digest("support_traceability_digest")
            .expect("bundle should emit support traceability digest")
    );
}

#[test]
fn certification_bundle_matches_admission_continuity_and_aftermath_outputs() {
    let bundle = certify_domain_capabilities();
    let admitted_plan = admitted_basis_observation_plan();
    let projection_plan = admitted_projection_consumption_plan();

    let admission = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&admitted_plan)
        .advises("admission.routing_gap")
        .because("runtime routing still needs clarification")
        .materialize()
        .expect("admission common lane should materialize");
    let admission_proof = success(materialize_runtime_admission_decision(admitted_ready(
        crate::domain_capabilities::ForgeQueryAdmissionContributionAuthoring::advisory(
            "worth.spatial.admission.routing_gap",
            "runtime routing still needs clarification",
        )
        .for_admitted_intent_plan(&admitted_plan),
    )));
    assert_eq!(admission, admission_proof);
    assert_eq!(
        bundle.output_digest("admission_artifact_digest"),
        Some(admission_digest(&admission))
    );

    let continuity = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&admitted_plan)
        .preserves_continuity("continuity.edge_split", "edge:before", "edge:after")
        .because("edge split preserves one authoritative successor")
        .materialize()
        .expect("continuity common lane should materialize");
    let continuity_proof = success(materialize_runtime_continuity_evidence(admitted_ready(
        crate::domain_capabilities::ForgeQueryContinuityContributionAuthoring::preserved_rebind(
            "edge:before",
            "edge:after",
            "worth.spatial.continuity.edge_split",
            "edge split preserves one authoritative successor",
        )
        .for_admitted_intent_plan(&admitted_plan),
    )));
    assert_eq!(continuity, continuity_proof);
    assert_eq!(
        bundle.output_digest("continuity_artifact_digest"),
        Some(continuity.continuity_resolution_digest())
    );

    let aftermath = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&projection_plan)
        .consumes_projection_contract(
            "aftermath.projection_contract",
            crate::domain_capabilities::ForgeQueryProjectionContractRequest::new(
                projection_source(),
                projection_binding(),
                crate::projection_consumption::ProjectMaterializedFacts::declare()
                    .display_field("field.visible"),
            ),
        )
        .because("projection aftermath should bind a stable contract")
        .materialize()
        .expect("aftermath common lane should materialize");
    let aftermath_proof = success(materialize_projection_consumption_contract(admitted_ready(
        crate::domain_capabilities::ForgeQueryAftermathContributionAuthoring::consumes_projection_contract(
            "worth.spatial.aftermath.projection_contract",
            "projection aftermath should bind a stable contract",
            projection_source(),
            projection_binding(),
            crate::projection_consumption::ProjectMaterializedFacts::declare()
                .display_field("field.visible"),
        )
        .for_admitted_intent_plan(&projection_plan),
    )));
    assert_eq!(aftermath, aftermath_proof);
    assert_eq!(
        bundle.output_digest("aftermath_artifact_digest"),
        Some(aftermath.contract_digest())
    );
}

#[test]
fn certification_bundle_boundary_and_failure_outputs_track_live_surfaces() {
    let bundle = certify_domain_capabilities();
    let declaration = intent_declaration();
    let denial = match evaluate_requested_domain_capability_contribution(
        ForgeQuerySupportContributionAuthoring::declaration_support(
            "worth.spatial.support.failure",
            "",
        )
        .for_intent_declaration(&declaration),
    ) {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("expected typed denial"),
    };

    assert_eq!(
        bundle.output_digest("public_boundary_digest"),
        Some(
            forge_query_domain_capability_public_surface_inventory()
                .public_surface_digest()
                .as_str()
        )
    );
    assert_eq!(
        bundle.output_digest("compile_fail_boundary_digest"),
        Some(forge_query_domain_capability_compile_fail_boundary_digest().as_str())
    );
    assert_eq!(
        bundle.output_digest("failure_digest"),
        Some(denial.failure_digest().as_str())
    );
}

#[test]
fn certification_bundle_populates_stable_width_and_slope_outputs() {
    let bundle = certify_domain_capabilities();
    let representative = forge_query_domain_capability_representative_report();
    let slopes = forge_query_domain_capability_slope_report(&representative);
    let contribution_width = slopes.counter_snapshot().contribution_width().to_string();
    let trace_width = slopes.counter_snapshot().trace_width().to_string();
    let category_width = slopes.counter_snapshot().category_width().to_string();
    let support_width = slopes.counter_snapshot().support_width().to_string();

    assert_eq!(
        bundle.output_digest("counter_snapshot"),
        Some(slopes.counter_snapshot().digest())
    );
    assert_eq!(
        bundle.output_digest("contribution_width"),
        Some(contribution_width.as_str())
    );
    assert_eq!(
        bundle.output_digest("trace_width"),
        Some(trace_width.as_str())
    );
    assert_eq!(
        bundle.output_digest("category_width"),
        Some(category_width.as_str())
    );
    assert_eq!(
        bundle.output_digest("support_width"),
        Some(support_width.as_str())
    );
    assert_eq!(
        bundle.output_digest("contribution_materialization_slope_digest"),
        Some(slopes.contribution_materialization_slope_digest())
    );
    assert_eq!(
        bundle.output_digest("trace_materialization_slope_digest"),
        Some(slopes.trace_materialization_slope_digest())
    );
    assert_eq!(
        bundle.output_digest("category_materialization_slope_digest"),
        Some(slopes.category_materialization_slope_digest())
    );
    assert_eq!(
        bundle.output_digest("support_materialization_slope_digest"),
        Some(slopes.support_materialization_slope_digest())
    );
    assert!(slopes.counter_snapshot().contribution_width() > 0);
    assert!(slopes.counter_snapshot().trace_width() > 0);
    assert!(slopes.counter_snapshot().category_width() >= 7);
    assert!(slopes.counter_snapshot().support_width() >= 2);
}

#[test]
fn certification_surface_keeps_ordinary_lane_distinct_from_lower_lanes() {
    let surface = forge_query_domain_capability_certification_surface();
    let inventory = forge_query_domain_capability_public_surface_inventory();

    assert_eq!(
        surface.public_surface_digest(),
        inventory.public_surface_digest()
    );
    assert_eq!(surface.category_count(), inventory.rows().len());
    for row in inventory.rows() {
        assert_ne!(row.ordinary_lane(), row.inspectable_lane());
        assert_ne!(row.inspectable_lane(), row.proof_lane());
        assert_ne!(row.proof_lane(), row.raw_lane());
        assert!(!row.ordinary_lane().contains("raw"));
        assert!(!row.ordinary_lane().contains("proof"));
    }
}
