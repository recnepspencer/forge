use serde_json::json;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::WorthTopologyEntityKind;

use super::*;
use crate::facade::worth_milestone_one_runtime_builder;
use crate::query::equivalence_contract_from_diagnostics_rows;
use crate::query::{worth_topology_runtime, WorthTopologyRuntimeAdapters};
use crate::read_stage::open_topology_read_view;

fn current_head_workspace(
    runtime: forge_relational::facade::runtime::RelationalRuntime,
    name: &str,
) -> (
    forge_query::facade::ForgeQueryWorkspace,
    WorthTopologyQueryAssembly,
) {
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, name).expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    (workspace, assembly)
}

#[test]
fn snapshot_read_only_assembly_synthesizes_complete_query_shaped_derived_rows() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-historical-derived-rows",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let read_view =
        open_topology_read_view(&runtime, &verified.read_basis).expect("read view should open");
    let adapters = WorthTopologyRuntimeAdapters::snapshot_read_only(
        read_view,
        verified.read_basis.snapshot().clone(),
    );
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth-topology-query-assembly-historical-derived-rows",
    )
    .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");

    assert!(workspace.materialize(assembly.diagnostics()).is_empty());
    assert!(workspace
        .materialize(assembly.equivalence_contract())
        .is_empty());

    let rows =
        historical_rows::historical_derived_rows(&assembly, &mut workspace, &verified.read_basis)
            .expect("historical rows should synthesize from query-native surfaces");
    let snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis)
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

#[test]
fn current_head_snapshot_decoder_rejects_malformed_retained_validation_rows() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-current-head-validation-decode",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let (mut workspace, assembly) = current_head_workspace(
        runtime,
        "worth-topology-query-assembly-current-head-validation-decode",
    );
    workspace
        .insert("WorthTopologyEntity", |builder| {
            builder
                .metadata(
                    crate::query::WorthTopologyQueryMutationEvidence::metadata_key(),
                    crate::query::WorthTopologyQueryMutationEvidence::from_read_basis(
                        &verified.read_basis,
                    ),
                )
                .aspect("topology.kind", WorthTopologyEntityKind::Vertex.kind_name())
                .aspect(
                    "topology.structure",
                    "worth-topology-query-assembly.current-head.extra-vertex",
                )
                .aspect(
                    "naming.persistent_name",
                    "worth-topology-query-assembly.current-head.extra-vertex",
                )
        })
        .expect("current-head mutation should retain derived rows");
    let snapshot = assembly
        .snapshot(&mut workspace)
        .expect("current-head snapshot should decode");
    let materialized_rows = workspace.materialize(assembly.materialized());
    let interpreted_rows = workspace.materialize(assembly.interpreted());
    let diagnostics_rows = workspace.materialize(assembly.diagnostics());
    let equivalence_rows = workspace.materialize(assembly.equivalence_contract());

    let error = snapshot_decode::snapshot_from_query_rows(
        snapshot.naming_attachments,
        &materialized_rows,
        &interpreted_rows,
        &[json!({ "not": "a retained validation row" })],
        &diagnostics_rows,
        &equivalence_rows,
    )
    .expect_err("snapshot decoding must fail closed on malformed retained validation rows");

    assert!(error.to_string().contains("topology validation"));
}
