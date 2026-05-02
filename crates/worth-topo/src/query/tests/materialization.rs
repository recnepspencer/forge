use super::support::seeded_sheet_disk_workspace;
use crate::facade::worth_milestone_one_runtime_builder;
use crate::read_stage::{open_topology_read_view, stage_topology_read_from_view};
use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};

#[test]
fn query_materializer_rebuilds_minimal_topology_from_production_runtime_rows() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "worth-query-materializer-minimal",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let staged = stage_topology_read_from_view(
        &open_topology_read_view(&runtime, &verified.read_basis).expect("read view should open"),
    )
    .expect("read stage should succeed");
    let (mut workspace, assembly, read_basis) =
        seeded_sheet_disk_workspace("worth-query-materializer-minimal");
    let snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &read_basis)
        .expect("query snapshot should decode");
    let materialized_view = snapshot.materialized;

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
