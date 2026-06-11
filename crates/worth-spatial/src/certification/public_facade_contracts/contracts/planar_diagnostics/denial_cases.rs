use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticDenialKind,
    PlanarDiagnosticSubject,
};

use super::contract_subject::{causal_reference, topology_surface};
use super::runtime_handles::diagnostic_handle;

#[test]
fn planar_diagnostics_reject_missing_topology_and_causal_evidence() {
    let missing_topology = match PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::topology_failure("topology:no-declared-surface"),
    )
    .with_query_causal_inspection(causal_reference("diagnostic-missing-topology"))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "diagnostic-missing-topology",
    ))) {
        Ok(_) => panic!("topology diagnostics must require declared topology surface evidence"),
        Err(error) => error,
    };
    assert_eq!(
        missing_topology.kind(),
        PlanarDiagnosticDenialKind::MissingTopologyDeclaredSurface
    );

    let missing_causal = match PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::topology_failure("topology:no-causal-reference"),
    )
    .with_topology_declared_surface(topology_surface("diagnostic-missing-causal"))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "diagnostic-missing-causal",
    ))) {
        Ok(_) => panic!("cross-runtime diagnostics must require Query causal references"),
        Err(error) => error,
    };
    assert_eq!(
        missing_causal.kind(),
        PlanarDiagnosticDenialKind::MissingCausalInspectionReference
    );
}

#[test]
fn planar_diagnostics_reject_materialized_causal_archive_claim() {
    let denial = match PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::predicate_failure("predicate:archive-overclaim"),
    )
    .request_materialized_causal_archive()
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "diagnostic-archive-overclaim",
    ))) {
        Ok(_) => panic!("phase 18 must not claim materialized causal archive support"),
        Err(error) => error,
    };
    assert_eq!(
        denial.kind(),
        PlanarDiagnosticDenialKind::MaterializedCausalArchiveNotSupported
    );
}
