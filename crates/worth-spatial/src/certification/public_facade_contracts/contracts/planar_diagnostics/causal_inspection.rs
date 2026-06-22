use forge_query::facade::{
    CausalInspectionExplanationFamily, CausalInspectionMaterializationPolicy,
    CausalInspectionRichness,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticEvidenceKind,
    PlanarDiagnosticSubject, PlanarDiagnosticTriggerLocality, PlanarDiagnosticTruthEffect,
};

use super::contract_subject::{causal_reference, diagnostic_planar_parts, topology_surface};
use super::runtime_handles::diagnostic_handle;

#[test]
fn planar_causal_inspection_explains_cross_runtime_failure_without_reopening_truth() {
    let parts = diagnostic_planar_parts("planar-diagnostic-cross-runtime");
    let contracts =
        PlanarDiagnosticBundleContracts::new(diagnostic_handle("planar-diagnostic-cross-runtime"));
    let plan = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::topology_failure("topology-to-spatial:validation-crossing"),
    )
    .with_retained_planar_facts(parts.retained)
    .with_projection_consumed_planar_facts(parts.projected)
    .with_topology_declared_surface(topology_surface("planar-diagnostic-cross-runtime"))
    .with_query_causal_inspection(causal_reference("planar-diagnostic-cross-runtime"))
    .inspect_failure_locality()
    .compile(&contracts)
    .expect("cross-runtime diagnostic plan");
    assert_eq!(plan.inspected_diagnostic_rows(), 13);

    let receipt = plan.certify().expect("cross-runtime diagnostic receipt");

    assert_eq!(
        receipt.trigger_locality(),
        PlanarDiagnosticTriggerLocality::TopologyContract
    );
    assert_eq!(
        receipt.truth_effect(),
        PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth
    );
    assert_eq!(receipt.counters().source_receipts_inspected(), 5);
    assert_eq!(receipt.counters().topology_surfaces_inspected(), 1);
    assert_eq!(receipt.counters().causal_references_resolved(), 1);
    assert_eq!(receipt.counters().denied_evidence_rows(), 0);

    let causal = receipt
        .basis()
        .causal_evidence()
        .expect("receipt should retain causal evidence");
    assert_eq!(causal.richness(), CausalInspectionRichness::ReferenceOnly);
    assert_eq!(
        causal.explanation_family(),
        CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation
    );
    assert_eq!(
        causal.materialization_policy(),
        CausalInspectionMaterializationPolicy::DigestReferenceOnly
    );
    assert!(!causal.reference_digest().is_empty());
    assert!(!causal.anchor_digest().is_empty());
    assert!(!causal.reference_set_digest().is_empty());
    assert!(!causal.request_digest().is_empty());
    assert!(!causal.admission_digest().is_empty());
    assert!(receipt.basis().subject().evidence().iter().any(|evidence| {
        evidence.kind() == PlanarDiagnosticEvidenceKind::QueryCausalInspection
            && evidence.evidence_digest() == causal.reference_digest()
    }));
}
