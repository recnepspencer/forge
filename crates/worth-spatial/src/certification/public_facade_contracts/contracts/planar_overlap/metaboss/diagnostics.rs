use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld, PlanarDiagnosticSubject, PlanarDiagnosticTopologyEvidence,
    PlanarDiagnosticTriggerLocality,
};

use crate::public_api_planar_diagnostics::contract_subject::causal_reference;

pub(crate) fn certify_tiny_rotation_diagnostic(
    source_digest: &str,
) -> worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleReceipt {
    PlanarDiagnosticBundle::explain_planar_failure(PlanarDiagnosticSubject::motion_failure(
        source_digest,
    ))
    .with_topology_declared_surface(PlanarDiagnosticTopologyEvidence::declared_surface(
        "topology:coplanar-storm:declared-surface",
    ))
    .with_query_causal_inspection(causal_reference("coplanar-storm-tiny-rotation"))
    .with_motion_posture(
        crate::public_api_planar_motion_posture::contract_subject::cancellation_motion_receipt(
            "coplanar-storm-tiny-rotation-motion",
        ),
    )
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle()))
    .expect("tiny rotation diagnostic plan")
    .certify()
    .expect("tiny rotation diagnostic receipt")
}

pub(crate) fn assert_tiny_rotation_diagnostic(
    receipt: &worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleReceipt,
    source_digest: &str,
) {
    assert_eq!(
        receipt.trigger_locality(),
        PlanarDiagnosticTriggerLocality::MotionOrRotationPosture
    );
    assert_eq!(receipt.basis().subject().source_digest(), source_digest);
    assert!(receipt
        .basis()
        .subject()
        .evidence()
        .iter()
        .any(|row| !row.evidence_digest().is_empty()));
    assert!(receipt.counters().source_receipts_inspected() >= 2);
}

fn diagnostic_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld,
> {
    forge_query::facade::ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarDiagnosticBundleQueryDomain)
        .with_operating_context(PlanarDiagnosticBundleQueryWorld::new(
            "coplanar-storm-tiny-rotation",
        ))
        .validate()
        .expect("validated coplanar storm diagnostic domain")
        .admit()
        .expect("admitted coplanar storm diagnostic domain")
}
