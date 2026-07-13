use worth_query::facade::foundation::ScopedInspectionBasis;
use worth_query::facade::runtime::{
    CausalInspection, CausalInspectionExplanationFamily, CausalInspectionMaterializationPolicy,
    CausalInspectionRedactionPolicy, CausalInspectionRichness, CausalInspectionSupportRowPosture,
    QueryCausalInspectionArtifact, QueryObservationReceipt,
};
use worth_runtime_bridge::facade::RuntimeBridge;

fn common_path_compiles(
    receipt: QueryObservationReceipt,
    inspection_basis: ScopedInspectionBasis,
    bridge: &RuntimeBridge,
) -> Result<
    QueryCausalInspectionArtifact,
    worth_query::facade::runtime::CausalInspectionMaterializationError,
> {
    let plan = CausalInspection::for_observation(receipt, inspection_basis)
        .why_changed()
        .reference_only()
        .include_all_retained_evidence()
        .plan()
        .expect("public common path should expose plan errors as typed values");

    plan.materialize_with_bridge(bridge)
}

fn plan_inspection_compiles(
    receipt: QueryObservationReceipt,
    inspection_basis: ScopedInspectionBasis,
) {
    let plan = CausalInspection::for_observation(receipt, inspection_basis)
        .why_changed()
        .materialized_detail()
        .redaction(CausalInspectionRedactionPolicy::DigestOnly)
        .materialization(CausalInspectionMaterializationPolicy::DigestReferenceOnly)
        .plan()
        .expect("public plan should compile");

    let _ = plan.support_posture();
    let _ = plan.required_evidence();
    let _ = plan.decision_trace();
    let _ = plan.estimated_cost().anchor_derivation_count();
    let _ = plan.estimated_cost().evidence_reference_resolution_count();
    let _ = plan.estimated_cost().admission_count();
    let _ = plan.estimated_cost().bridge_envelope_assembly_count();
    let _ = plan.estimated_cost().evidence_reference_count();
    let _ = plan.explain().reason();
    let _ = plan.redaction_policy();
    let _ = plan.materialization_policy();
    let _ = plan.requested_richness();
    let _ = plan.explanation_family();
    let _ = plan.anchor_for_reporting();
    let _ = plan.reference_set_digest();
    let _ = plan.request_for_reporting();
    let _ = plan.admission_digest();
}

fn support_discovery_compiles() {
    let support = CausalInspection::support();
    let explanation = support.explain();

    let _ = explanation.supported_row_count();
    let _ = explanation.advisory_row_count();
    let _ = explanation.deferred_row_count();
    for row in support.rows() {
        let _ = row.explanation_family();
        let _ = row.default_richness();
        let _ = row.posture();
        let _ = row.note();
    }
}

fn artifact_exploration_compiles(artifact: QueryCausalInspectionArtifact) {
    let _ = artifact.primary_result();
    let _ = artifact.warnings();
    let _ = artifact.decision_trace().query_decision_for_reporting();
    let _ = artifact.decision_trace().bridge_envelope_for_reporting();
    let _ = artifact.decision_trace().bridge_denial_for_reporting();
    let _ = artifact.authority_bindings();
    let _ = artifact.evidence();
    let _ = artifact.integrity().artifact_for_reporting();
    let _ = artifact.integrity().causal_identity_for_reporting();
    let _ = artifact
        .integrity()
        .bridge_readmission_proof_for_reporting();
    let _ = artifact.performance_envelope();
    let _ = artifact.receipt();
    let _ = artifact.denial_reason();
    let _ = artifact.advisory_reason();
}

fn advanced_vocabulary_remains_available() {
    let _ = CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation;
    let _ = CausalInspectionRichness::ReferenceOnly;
    let _ = CausalInspectionSupportRowPosture::Supported;
}

#[test]
fn causal_inspection_public_dx_signatures_are_referenced() {
    let _ = common_path_compiles
        as fn(
            QueryObservationReceipt,
            ScopedInspectionBasis,
            &RuntimeBridge,
        ) -> Result<
            QueryCausalInspectionArtifact,
            worth_query::facade::runtime::CausalInspectionMaterializationError,
        >;
    let _ = plan_inspection_compiles as fn(QueryObservationReceipt, ScopedInspectionBasis);
    let _ = support_discovery_compiles as fn();
    let _ = artifact_exploration_compiles as fn(QueryCausalInspectionArtifact);
    let _ = advanced_vocabulary_remains_available as fn();
}
