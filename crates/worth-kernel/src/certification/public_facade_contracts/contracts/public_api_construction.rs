use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{authoring::construction::*, diagnostics::family::*};

#[test]
fn kernel_public_facade_exports_query_construction_entry_surface() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api".to_string(),
    )
    .expect("workspace");
    let mut session = primitive_construction_authoring(&mut workspace).expect("authoring session");
    let report = session.authority_chain_report();
    let write_contract = session
        .admit_query_family(forge_query::facade::ForgeQueryRuntimeFacadeFamily::Write)
        .expect("write contract");
    let inspect_contract = session
        .admit_query_family(forge_query::facade::ForgeQueryRuntimeFacadeFamily::Inspect)
        .expect("inspect contract");
    let prepared_result = session
        .author(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 8,
        }))
        .and_then(|entry| entry.prepare_result());
    let prepared_outcome = session
        .author(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 8,
        }))
        .map(|entry| entry.prepare_outcome());
    let temporal_error = session
        .admit_query_family(forge_query::facade::ForgeQueryRuntimeFacadeFamily::Temporal)
        .expect_err("temporal family should stay unsupported here");

    assert_eq!(session.query_front_door(), "ForgeQueryWorkspace");
    assert_eq!(report.required_query_family_contracts().len(), 2);
    assert!(report.query_gap_rows().is_empty());
    assert_eq!(
        write_contract.family(),
        forge_query::facade::ForgeQueryRuntimeFacadeFamily::Write
    );
    assert_eq!(
        inspect_contract.family(),
        forge_query::facade::ForgeQueryRuntimeFacadeFamily::Inspect
    );
    assert!(prepared_result.is_ok());
    assert!(prepared_outcome.is_ok());
    assert!(matches!(
        temporal_error,
        WorthKernelAuthorityError::QueryRuntime(_)
    ));
}

#[test]
fn kernel_public_facade_exports_phase_three_family_ladder() {
    let coverage = primitive_construction_family_coverage_report();

    assert_eq!(
        coverage
            .row_for(PrimitiveConstructionFamily::ShellWithHole)
            .expect("shell row")
            .status(),
        PrimitiveConstructionFamilyCoverageStatus::AdmittedPlanarConstruction
    );
    assert_eq!(
        coverage
            .row_for(PrimitiveConstructionFamily::WireBody)
            .expect("wire row")
            .status(),
        PrimitiveConstructionFamilyCoverageStatus::AdmittedPlanarConstruction
    );
}
