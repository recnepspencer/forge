use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_support_reports_authoritative_current_head_posture() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let support = adapters.support();

    assert!(support.current_head_live_reads_supported());
    assert!(!support.current_head_materialization_supported());
    assert!(support.post_write_materialization_supported());
    assert!(!support.historical_basis_supported());
    assert!(support.authoritative_writes_supported());
    assert!(support.query_edit_execution_supported());
    assert!(!support.query_edit_family_supported(
        crate::edit::WorthTopologyEditFamily::AttachBoundaryMembership
    ));
    assert!(!support.query_edit_family_supported(
        crate::edit::WorthTopologyEditFamily::AttachShellOrWireMembership
    ));
    assert!(!support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::RewireLoopEndpoint));
    assert!(!support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::RewireLoopSuccessor));
    assert!(!support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::SpliceRadialAdjacency));
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::CreateTopologyEntity));
    assert!(support.query_edit_family_supported(
        crate::edit::WorthTopologyEditFamily::DetachBoundaryMembership
    ));
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::DetachRadialAdjacency));
    assert!(support.query_edit_family_supported(
        crate::edit::WorthTopologyEditFamily::DetachShellOrWireMembership
    ));
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::RetireTopologyEntity));
}

#[test]
fn current_head_runtime_reads_seeded_topology_without_query_import() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    worth_schema::facade::seed_minimal_topology(&mut runtime, "worth-query-runtime")
        .expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.runtime").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let persistent_name_rows = workspace.read(assembly.persistent_names());

    assert!(!entity_rows.is_empty());
    assert!(!relation_rows.is_empty());
    assert!(!persistent_name_rows.is_empty());
    assert!(entity_rows.iter().any(|row| {
        row.payload
            .get("naming")
            .and_then(|value| value.get("persistent_name"))
            .and_then(|value| value.as_str())
            .is_some()
    }));
}

#[test]
fn current_head_runtime_writes_topology_through_real_runtime() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.authoritative").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

    let receipt = workspace
        .insert("WorthTopologyEntity", |builder| {
            builder
                .aspect("topology.kind", "worth.vertex")
                .aspect(
                    "topology.structure",
                    "worth-query-runtime-write.added_vertex",
                )
                .aspect(
                    "naming.persistent_name",
                    "worth-query-runtime-write.added_vertex",
                )
        })
        .expect("entity create should commit");

    let entity_rows = workspace.read(assembly.entities());
    let materialized_rows = workspace.materialize(assembly.materialized());
    let materialized: crate::facade::MaterializedTopologyView =
        serde_json::from_value(materialized_rows[0].clone()).expect("materialized topology row");
    assert!(entity_rows.iter().any(|row| {
        row.payload
            .get("naming")
            .and_then(|value| value.get("persistent_name"))
            .and_then(|value| value.as_str())
            .is_some_and(|name| name == "worth-query-runtime-write.added_vertex")
    }));
    assert!(receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert!(!materialized.topology().vertices.is_empty());
}

#[test]
fn current_head_runtime_denies_unsupported_insert_collections() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.denials").expect("workspace");

    let error = workspace
        .insert("WorthPersistentName", |builder| {
            builder
                .aspect("topology.kind", "worth.naming.persistent_name")
                .aspect("naming.persistent_name", "worth.current-head.denials.name")
        })
        .expect_err("unsupported insert collections must fail closed");

    assert!(error
        .to_string()
        .contains("does not admit insert collection `WorthPersistentName`"));
}

#[test]
fn snapshot_read_only_runtime_support_reports_historical_read_only_posture() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        worth_schema::facade::seed_minimal_topology(&mut runtime, "worth-query-runtime-snapshot")
            .expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let adapters = WorthTopologyRuntimeAdapters::snapshot_read_only(read_view, seeded.snapshot);
    let support = adapters.support();

    assert!(!support.current_head_live_reads_supported());
    assert!(!support.current_head_materialization_supported());
    assert!(!support.post_write_materialization_supported());
    assert!(support.historical_basis_supported());
    assert!(!support.authoritative_writes_supported());
    assert!(!support.query_edit_execution_supported());
}

#[test]
fn snapshot_read_only_runtime_reads_seeded_topology_and_denies_writes() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        worth_schema::facade::seed_minimal_topology(&mut runtime, "worth-query-runtime-snapshot")
            .expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let adapters = WorthTopologyRuntimeAdapters::snapshot_read_only(read_view, seeded.snapshot);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.snapshot-read-only.runtime").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

    assert!(!workspace.read(assembly.entities()).is_empty());

    let error = workspace
        .insert("WorthTopologyEntity", |builder| {
            builder
                .aspect("topology.kind", "worth.vertex")
                .aspect(
                    "topology.structure",
                    "worth.snapshot-read-only.runtime.vertex",
                )
                .aspect(
                    "naming.persistent_name",
                    "worth.snapshot-read-only.runtime.vertex",
                )
        })
        .expect_err("snapshot runtime must deny authoritative writes");

    assert!(error
        .to_string()
        .contains("snapshot certification runtime is read-only"));
}
