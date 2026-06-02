use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyQueryMutationFamilySupportStatus, TopologyQueryMutationLane,
    TopologyQueryMutationLaneSupportStatus, TopologyRuntimeAdapters,
    TopologyRuntimePostureCapability, TopologyRuntimePostureStatus,
};
use crate::test_support::schema_topology_authoring_boundary::seed_minimal_topology_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus};

#[test]
fn current_head_runtime_support_reports_authoritative_current_head_posture() {
    let runtime = build_milestone_one_runtime().expect(" runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let support = adapters.support();

    for capability in [
        TopologyRuntimePostureCapability::CurrentHeadLiveReads,
        TopologyRuntimePostureCapability::PostWriteMaterialization,
        TopologyRuntimePostureCapability::BranchPreviewBasis,
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
        TopologyRuntimePostureCapability::BranchLocalIntentStaging,
        TopologyRuntimePostureCapability::BranchLocalDeclarationExecution,
    ] {
        assert_eq!(
            support.runtime_posture_status(capability),
            TopologyRuntimePostureStatus::Denied
        );
    }
    assert!(support
        .query_mutation_lane_support_rows()
        .iter()
        .any(|row| row.status() == TopologyQueryMutationLaneSupportStatus::Admitted));
    assert_eq!(
        support.query_mutation_family_support_status(
            crate::topology_operators::TopologyMutationFamily::AttachBoundaryMembership
        ),
        TopologyQueryMutationFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert_eq!(
        support.query_mutation_family_support_status(
            crate::topology_operators::TopologyMutationFamily::AttachShellOrWireMembership
        ),
        TopologyQueryMutationFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert_eq!(
        support.query_mutation_family_support_status(
            crate::topology_operators::TopologyMutationFamily::RewireLoopSuccessor
        ),
        TopologyQueryMutationFamilySupportStatus::PartiallyAdmittedByLane
    );
    for lane in [
        TopologyQueryMutationLane::RelocateHalfEdgeBeforeSuccessor,
        TopologyQueryMutationLane::RelocateHalfEdgeSpanBeforeSuccessor,
        TopologyQueryMutationLane::CreateInnerLoopOnExistingFace,
        TopologyQueryMutationLane::RehomeAllOwnedHalfEdgesToNewWire,
        TopologyQueryMutationLane::SplitConnectedHalfEdgeSetIntoNewWire,
        TopologyQueryMutationLane::SplitSingleFaceFromTwoFaceShellToNewShell,
        TopologyQueryMutationLane::RehomeAllOwnedFacesToNewShell,
        TopologyQueryMutationLane::RewireLoopEndpoint,
        TopologyQueryMutationLane::SpliceRadialAdjacency,
    ] {
        assert_eq!(
            support.query_mutation_lane_support_status(lane),
            TopologyQueryMutationLaneSupportStatus::Admitted
        );
    }
    for family in [
        crate::topology_operators::TopologyMutationFamily::CreateTopologyEntity,
        crate::topology_operators::TopologyMutationFamily::DetachBoundaryMembership,
        crate::topology_operators::TopologyMutationFamily::DetachRadialAdjacency,
        crate::topology_operators::TopologyMutationFamily::DetachShellOrWireMembership,
        crate::topology_operators::TopologyMutationFamily::RetireTopologyEntity,
        crate::topology_operators::TopologyMutationFamily::RewireLoopEndpoint,
        crate::topology_operators::TopologyMutationFamily::SpliceRadialAdjacency,
    ] {
        assert_eq!(
            support.query_mutation_family_support_status(family),
            TopologyQueryMutationFamilySupportStatus::Admitted
        );
    }
}

#[test]
fn current_head_runtime_reads_seeded_topology_without_query_import() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_minimal_topology_through_schema_execution(&mut runtime, "query-runtime")
        .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    let entity_rows = workspace.read(surfaces.entities());
    let relation_rows = workspace.read(surfaces.relations());
    let persistent_name_rows = workspace.read(surfaces.persistent_names());

    assert!(!entity_rows.is_empty());
    assert!(!relation_rows.is_empty());
    assert!(!persistent_name_rows.is_empty());
    assert!(entity_rows.iter().any(|row| {
        row.external_row()
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
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    let receipt = workspace
        .insert("TopologyEntity", |builder| {
            builder
                .aspect("topology.kind", ".vertex")
                .aspect("topology.structure", "query-runtime-write.added_vertex")
                .aspect("naming.persistent_name", "query-runtime-write.added_vertex")
        })
        .expect("entity create should commit");

    let entity_rows = workspace.read(surfaces.entities());
    let materialized_rows = workspace.materialize(surfaces.materialized());
    let materialized: crate::facade::MaterializedTopologyView =
        serde_json::from_value(materialized_rows[0].clone()).expect("materialized topology row");
    assert!(entity_rows.iter().any(|row| {
        row.external_row()
            .get("naming")
            .and_then(|value| value.get("persistent_name"))
            .and_then(|value| value.as_str())
            .is_some_and(|name| name == "query-runtime-write.added_vertex")
    }));
    assert!(receipt
        .affected_derived_view_ids()
        .contains(&surfaces.materialized().name().to_string()));
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
        seed_minimal_topology_through_schema_execution(&mut runtime, "query-runtime-snapshot")
            .expect("seed topology");
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
    for capability in [
        TopologyRuntimePostureCapability::BranchPreviewBasis,
        TopologyRuntimePostureCapability::BranchLocalIntentStaging,
        TopologyRuntimePostureCapability::BranchLocalDeclarationExecution,
    ] {
        assert_eq!(
            support.runtime_posture_status(capability),
            TopologyRuntimePostureStatus::Denied
        );
    }
    assert!(support
        .query_mutation_lane_support_rows()
        .iter()
        .all(|row| row.status() == TopologyQueryMutationLaneSupportStatus::Denied));

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
fn current_head_runtime_support_profile_reports_branch_sessions_without_branch_intent_or_topology_declaration_execution(
) {
    let runtime = build_milestone_one_runtime().expect(" runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let support = adapters.support();
    let support_profile = support.support_profile();
    let preview_support = support_profile
        .support_for(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .expect("current-head preview support row should exist");

    assert_eq!(
        preview_support.status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert!(preview_support
        .evidence()
        .iter()
        .any(|entry| entry == "topology-current-head-preview-basis"));
    assert!(!preview_support
        .evidence()
        .iter()
        .any(|entry| entry == "topology-current-head-branch-intent-staging"));
    assert_eq!(
        support.runtime_posture_status(TopologyRuntimePostureCapability::BranchLocalIntentStaging),
        TopologyRuntimePostureStatus::Denied
    );
    assert_eq!(
        support.runtime_posture_status(
            TopologyRuntimePostureCapability::BranchLocalDeclarationExecution
        ),
        TopologyRuntimePostureStatus::Denied
    );
}

#[test]
fn snapshot_read_only_runtime_reads_seeded_topology_and_denies_writes() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology_through_schema_execution(&mut runtime, "query-runtime-snapshot")
            .expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let adapters = TopologyRuntimeAdapters::snapshot_read_only(read_view, seeded.snapshot);
    let mut workspace =
        topology_runtime(adapters, ".snapshot-read-only.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    assert!(!workspace.read(surfaces.entities()).is_empty());

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
