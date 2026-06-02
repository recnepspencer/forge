use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{PrimitiveRealizationStrategy, PrimitiveStabilityClass};
use worth_kernel::facade::{
    authoring::construction::*,
    diagnostics::{family::*, query::*},
};

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
    let prepared_result =
        session.prepare_result(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 8,
        }));
    let prepared_outcome =
        session.prepare_outcome(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 8,
        }));
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

#[test]
fn kernel_public_facade_exports_branch_preview_runtime_report() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-branch-preview".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_branch_preview_runtime_report(
        &mut workspace,
        PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(1.0)),
    )
    .expect("branch preview report");

    assert_eq!(report.family(), PrimitiveConstructionFamily::SimplexSolid);
    assert!(report.authority_chain_report().query_gap_rows().is_empty());
    assert_eq!(
        report.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        report.attempted_realization_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );
    assert_eq!(
        report.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_ne!(
        report.outcome().outcome_digest(),
        report.preview_lane().admission_digest()
    );
    assert_ne!(
        report.outcome().outcome_digest(),
        report.branch_lane().admission_digest()
    );
    assert_ne!(
        report.preview_lane().admission_digest(),
        report.branch_lane().admission_digest()
    );
}
