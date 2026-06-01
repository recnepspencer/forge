use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::*;
use crate::projection::equivalence_contract_from_diagnostics_rows;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::runtime_boundary::read_stage::open_topology_read_view;
use crate::validation::reference_integrity::milestone_one_runtime_builder;

#[test]
fn snapshot_read_only_assembly_synthesizes_complete_query_shaped_derived_rows() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-surfaces-historical-derived-rows",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let read_view =
        open_topology_read_view(&runtime, &verified.read_basis()).expect("read view should open");
    let adapters = TopologyRuntimeAdapters::snapshot_read_only(
        read_view,
        verified.read_basis().snapshot().clone(),
    );
    let mut workspace = topology_runtime(
        adapters,
        "topology-declared-query-surfaces-historical-derived-rows",
    )
    .expect("query workspace should build");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("query surfaces should declare");

    assert!(workspace.materialize(surfaces.diagnostics()).is_empty());
    assert!(workspace
        .materialize(surfaces.equivalence_contract())
        .is_empty());

    let rows = historical_rows::historical_snapshot_rows(
        &surfaces,
        &mut workspace,
        &verified.read_basis(),
    )
    .expect("historical rows should synthesize from query-native surfaces");
    let snapshot = surfaces
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis())
        .expect("historical snapshot should decode");

    assert_eq!(rows.materialized_rows.len(), 1);
    assert_eq!(rows.interpreted_rows.len(), 1);
    assert_eq!(rows.validation_rows.len(), 1);
    assert_eq!(rows.diagnostics_rows.len(), 1);
    assert_eq!(rows.equivalence_rows.len(), 1);
    assert_eq!(rows.naming_attachments, snapshot.naming_attachments);
    assert_eq!(
        equivalence_contract_from_diagnostics_rows(&rows.diagnostics_rows)
            .expect("diagnostics rows should decode equivalence"),
        snapshot.equivalence_contract
    );
}
