use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld, PlanarDiagnosticDenialKind, PlanarDiagnosticSubject,
    PlanarDiagnosticTriggerLocality, PlanarDiagnosticTruthEffect,
};

#[test]
fn kernel_consumes_planar_diagnostic_bundle_without_reopening_truth() {
    let receipt = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::binding_failure("kernel-binding:stale-planar-rebind"),
    )
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "kernel-planar-diagnostic-binding",
    )))
    .expect("kernel diagnostic plan")
    .certify()
    .expect("kernel diagnostic receipt");

    assert_eq!(
        receipt.trigger_locality(),
        PlanarDiagnosticTriggerLocality::BindingOrRebinding
    );
    assert_eq!(
        receipt.truth_effect(),
        PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth
    );
    assert_eq!(receipt.counters().source_receipts_inspected(), 1);
    assert_eq!(receipt.counters().causal_references_resolved(), 0);
    assert_eq!(receipt.counters().denied_evidence_rows(), 0);
}

#[test]
fn kernel_rejects_materialized_causal_archive_diagnostic_claim() {
    let denial = match PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::predicate_failure("kernel-predicate:archive-overclaim"),
    )
    .request_materialized_causal_archive()
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "kernel-planar-diagnostic-archive",
    ))) {
        Ok(_) => panic!("kernel must not accept support-gated causal archives in phase 18"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarDiagnosticDenialKind::MaterializedCausalArchiveNotSupported
    );
}

fn diagnostic_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarDiagnosticBundleQueryDomain)
        .with_operating_context(PlanarDiagnosticBundleQueryWorld::new(world))
        .validate()
        .expect("validated planar diagnostic kernel domain")
        .admit()
        .expect("admitted planar diagnostic kernel domain")
}
