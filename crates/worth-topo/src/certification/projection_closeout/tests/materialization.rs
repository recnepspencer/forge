use super::topology_reads::support::seeded_sheet_disk_workspace;
use crate::projection::runtime_boundary::read_stage::{
    open_topology_read_view, stage_topology_read_from_view,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::milestone_one_runtime_builder;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

#[test]
fn query_materializer_rebuilds_minimal_topology_from_production_runtime_rows() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query-materializer-minimal",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let staged = stage_topology_read_from_view(
        &open_topology_read_view(&runtime, &verified.read_basis()).expect("read view should open"),
    )
    .expect("read stage should succeed");
    let (mut workspace, surfaces, read_basis) =
        seeded_sheet_disk_workspace("query-materializer-minimal");
    let _ = read_basis;
    let materialized_view =
        crate::certification::support::current_head_materialized_topology::current_head_materialized_topology(
            &mut workspace,
            &surfaces,
        )
        .expect("current-head materialized topology should decode");

    assert_eq!(
        materialized_view.topology(),
        staged.materialized().topology()
    );
    assert_eq!(
        materialized_view.report().breadth.topology_relation_count,
        staged
            .materialized()
            .report()
            .breadth
            .topology_relation_count
    );
    assert_eq!(
        materialized_view.report().breadth.topology_entity_count,
        staged.materialized().report().breadth.topology_entity_count
    );
    assert_eq!(
        materialized_view.report().whole_view_materialization,
        staged.materialized().report().whole_view_materialization
    );
    assert_eq!(
        materialized_view.report().fallback_class,
        staged.materialized().report().fallback_class
    );
}
