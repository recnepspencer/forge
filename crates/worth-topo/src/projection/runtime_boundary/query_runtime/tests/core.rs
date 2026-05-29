use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyQueryEditFamilySupportStatus, TopologyQueryEditLane,
    TopologyQueryEditLaneSupportStatus, TopologyRuntimeAdapters, TopologyRuntimePostureCapability,
    TopologyRuntimePostureStatus,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus};
use schema::facade::topology_authoring::seed_minimal_topology;

#[test]
fn current_head_runtime_support_reports_authoritative_current_head_posture() {
    let runtime = build_milestone_one_runtime().expect(" runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let support = adapters.support();

    for capability in [
        TopologyRuntimePostureCapability::CurrentHeadLiveReads,
        TopologyRuntimePostureCapability::PostWriteMaterialization,
        TopologyRuntimePostureCapability::AuthoritativeWrites,
    ] {
        assert_eq!(
            support.runtime_posture_status(capability),
            TopologyRuntimePostureStatus::Admitted
        );
    }
    for capability in [
        TopologyRuntimePostureCapability::CurrentHeadMaterialization,
        TopologyRuntimePostureCapability::HistoricalBasis,
    ] {
        assert_eq!(
            support.runtime_posture_status(capability),
            TopologyRuntimePostureStatus::Denied
        );
    }
    assert!(support
        .query_edit_lane_support_rows()
        .iter()
        .any(|row| row.status() == TopologyQueryEditLaneSupportStatus::Admitted));
    assert_eq!(
        support.query_edit_family_support_status(
            crate::topology_operators::TopologyEditFamily::AttachBoundaryMembership
        ),
        TopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert_eq!(
        support.query_edit_family_support_status(
            crate::topology_operators::TopologyEditFamily::AttachShellOrWireMembership
        ),
        TopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert_eq!(
        support.query_edit_family_support_status(
            crate::topology_operators::TopologyEditFamily::RewireLoopSuccessor
        ),
        TopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
    );
    for lane in [
        TopologyQueryEditLane::RelocateHalfEdgeBeforeSuccessor,
        TopologyQueryEditLane::RelocateHalfEdgeSpanBeforeSuccessor,
        TopologyQueryEditLane::CreateInnerLoopOnExistingFace,
        TopologyQueryEditLane::RehomeAllOwnedHalfEdgesToNewWire,
        TopologyQueryEditLane::SplitConnectedHalfEdgeSetIntoNewWire,
        TopologyQueryEditLane::SplitSingleFaceFromTwoFaceShellToNewShell,
        TopologyQueryEditLane::RehomeAllOwnedFacesToNewShell,
        TopologyQueryEditLane::RewireLoopEndpoint,
        TopologyQueryEditLane::SpliceRadialAdjacency,
    ] {
        assert_eq!(
            support.query_edit_lane_support_status(lane),
            TopologyQueryEditLaneSupportStatus::Admitted
        );
    }
    for family in [
        crate::topology_operators::TopologyEditFamily::CreateTopologyEntity,
        crate::topology_operators::TopologyEditFamily::DetachBoundaryMembership,
        crate::topology_operators::TopologyEditFamily::DetachRadialAdjacency,
        crate::topology_operators::TopologyEditFamily::DetachShellOrWireMembership,
        crate::topology_operators::TopologyEditFamily::RetireTopologyEntity,
        crate::topology_operators::TopologyEditFamily::RewireLoopEndpoint,
        crate::topology_operators::TopologyEditFamily::SpliceRadialAdjacency,
    ] {
        assert_eq!(
            support.query_edit_family_support_status(family),
            TopologyQueryEditFamilySupportStatus::Admitted
        );
    }
}

#[test]
fn current_head_runtime_reads_seeded_topology_without_query_import() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_minimal_topology(&mut runtime, "query-runtime").expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.runtime").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

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
    let runtime = build_milestone_one_runtime().expect(" runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.authoritative").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

    let receipt = workspace
        .insert("TopologyEntity", |builder| {
            builder
                .aspect("topology.kind", ".vertex")
                .aspect("topology.structure", "query-runtime-write.added_vertex")
                .aspect("naming.persistent_name", "query-runtime-write.added_vertex")
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
            .is_some_and(|name| name == "query-runtime-write.added_vertex")
    }));
    assert!(receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert!(!materialized.topology().vertices.is_empty());
}

#[test]
fn current_head_runtime_denies_unsupported_insert_collections() {
    let runtime = build_milestone_one_runtime().expect(" runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.denials").expect("workspace");

    let error = workspace
        .insert("PersistentName", |builder| {
            builder
                .aspect("topology.kind", ".naming.persistent_name")
                .aspect("naming.persistent_name", ".current-head.denials.name")
        })
        .expect_err("unsupported insert collections must fail closed");

    assert!(error
        .to_string()
        .contains("does not admit insert collection `PersistentName`"));
}

#[test]
fn snapshot_read_only_runtime_support_reports_historical_read_only_posture() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-runtime-snapshot").expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let adapters = TopologyRuntimeAdapters::snapshot_read_only(read_view, seeded.snapshot);
    let support = adapters.support();

    for capability in [
        TopologyRuntimePostureCapability::CurrentHeadLiveReads,
        TopologyRuntimePostureCapability::CurrentHeadMaterialization,
        TopologyRuntimePostureCapability::PostWriteMaterialization,
        TopologyRuntimePostureCapability::AuthoritativeWrites,
    ] {
        assert_eq!(
            support.runtime_posture_status(capability),
            TopologyRuntimePostureStatus::Denied
        );
    }
    assert_eq!(
        support.runtime_posture_status(TopologyRuntimePostureCapability::HistoricalBasis),
        TopologyRuntimePostureStatus::Admitted
    );
    assert!(support
        .query_edit_lane_support_rows()
        .iter()
        .all(|row| row.status() == TopologyQueryEditLaneSupportStatus::Denied));

    let support_profile = support.support_profile();
    let live_support = support_profile
        .support_for(ForgeQueryRuntimeFacadeFamily::Live)
        .expect("snapshot runtime live support row should exist");
    assert_eq!(
        live_support.status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert!(live_support
        .evidence()
        .iter()
        .any(|entry| entry == "topology-snapshot-subscription-activation"));

    let preview_support = support_profile
        .support_for(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .expect("snapshot runtime preview support row should exist");
    assert_eq!(
        preview_support.status(),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert_eq!(
        preview_support.denial_reason(),
        Some(
            "topology snapshot runtime is bound to one historical basis and does not admit preview or branch-local sessions"
        )
    );
    assert!(preview_support
        .evidence()
        .iter()
        .any(|entry| entry == "topology-snapshot-historical-basis"));
}

#[test]
fn snapshot_read_only_runtime_reads_seeded_topology_and_denies_writes() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-runtime-snapshot").expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let adapters = TopologyRuntimeAdapters::snapshot_read_only(read_view, seeded.snapshot);
    let mut workspace =
        topology_runtime(adapters, ".snapshot-read-only.runtime").expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

    assert!(!workspace.read(assembly.entities()).is_empty());

    let error = workspace
        .insert("TopologyEntity", |builder| {
            builder
                .aspect("topology.kind", ".vertex")
                .aspect("topology.structure", ".snapshot-read-only.runtime.vertex")
                .aspect(
                    "naming.persistent_name",
                    ".snapshot-read-only.runtime.vertex",
                )
        })
        .expect_err("snapshot runtime must deny authoritative writes");

    assert!(error
        .to_string()
        .contains("snapshot certification runtime is read-only"));
}
