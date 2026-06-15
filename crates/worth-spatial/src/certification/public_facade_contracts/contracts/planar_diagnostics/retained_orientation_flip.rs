use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticEvidenceKind,
    PlanarDiagnosticSubject, PlanarDiagnosticTriggerLocality, PlanarDiagnosticTruthEffect,
};

use super::contract_subject::{causal_reference, diagnostic_planar_parts};
use super::runtime_handles::diagnostic_handle;
use crate::public_api_planar_motion_posture::contract_subject::cancellation_motion_receipt;

#[test]
fn retained_orientation_flip_localizes_exact_step() {
    let parts = diagnostic_planar_parts("planar-diagnostic-retained-orientation-flip");
    let contracts = PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "planar-diagnostic-retained-orientation-flip",
    ));
    let plan = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::retained_transform_failure(
            "retained-transform:orientation-flip-step",
        ),
    )
    .with_retained_planar_facts(parts.retained)
    .with_projection_consumed_planar_facts(parts.projected)
    .with_motion_posture(cancellation_motion_receipt(
        "planar-diagnostic-retained-orientation-flip-motion",
    ))
    .with_query_causal_inspection(causal_reference(
        "planar-diagnostic-retained-orientation-flip",
    ))
    .inspect_failure_locality()
    .compile(&contracts)
    .expect("final boss diagnostic plan");
    assert_eq!(plan.inspected_diagnostic_rows(), 13);

    let receipt = plan.certify().expect("final boss diagnostic receipt");

    assert_eq!(
        receipt.trigger_locality(),
        PlanarDiagnosticTriggerLocality::RetainedTransformStep
    );
    assert_eq!(
        receipt.truth_effect(),
        PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth
    );
    assert_eq!(receipt.counters().causal_references_resolved(), 1);
    assert_eq!(receipt.counters().source_receipts_inspected(), 5);
    assert_eq!(
        receipt
            .basis()
            .subject()
            .evidence()
            .iter()
            .filter(
                |evidence| evidence.kind() == PlanarDiagnosticEvidenceKind::BasisLifecycleReceipt
            )
            .count(),
        2
    );
    assert!(receipt.basis().subject().evidence().iter().any(|evidence| {
        evidence.kind() == PlanarDiagnosticEvidenceKind::ProjectionConsumptionReceipt
    }));
    assert!(receipt.basis().subject().evidence().iter().any(|evidence| {
        evidence.kind() == PlanarDiagnosticEvidenceKind::QueryCausalInspection
    }));
    assert!(!receipt.declaration_digest().is_empty());
    assert!(!receipt.diagnostic_bundle_digest().is_empty());
    assert_ne!(
        receipt.declaration_digest(),
        receipt.diagnostic_bundle_digest()
    );
}
