use super::support::{current_lookup_rows, snapshot_basis_workspace};
use crate::facade::{
    topology_current_head_authoritative_context, topology_query_domain_entry, topology_runtime,
    topology_snapshot_read_only_context, TopologyCurrentHeadReadHandleExt,
    TopologyDomainQueryExecutionEngine, TopologyQueryAssembly, TopologyRuntimeAdapters,
    TopologySnapshotReadOnlyReadHandleExt,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::ForgeQueryApplicationFacade;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

#[test]
fn current_head_handle_bound_reads_execute_neighborhood_queries_and_accumulate_reports() {
    let (mut workspace, assembly) = seeded_workspace(
        "query.domain-query.handle-entry.current",
        MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    );
    let source_identity = current_lookup_rows(&mut workspace, &assembly)
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .expect("current-head context should validate")
        .admit()
        .expect("current-head context should admit");
    let mut reads = handle.topology_reads(&mut workspace);

    let shared_vertex = reads
        .shared_vertex_half_edge_neighborhood(&source_identity)
        .expect("shared-vertex read should execute through handle-bound entry");
    let radial = reads
        .radial_half_edge_neighborhood(&source_identity)
        .expect("radial read should execute through handle-bound entry");

    assert_eq!(
        reads.handle_identity_digest(),
        handle.handle_identity_digest()
    );
    assert_eq!(
        shared_vertex.request_report.execution_engine,
        TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(
        radial.request_report.execution_engine,
        TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent
    );
    assert_eq!(reads.aggregate_report().request_count, 2);
}

#[test]
fn snapshot_handle_bound_reads_preserve_historical_execution_posture() {
    let stem = "query.domain-query.handle-entry.snapshot";
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        stem,
        &MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
    )
    .expect("seed primitive");
    let (mut workspace, assembly) = snapshot_basis_workspace(
        &runtime,
        &format!("{stem}.workspace"),
        &verified.read_basis(),
    );
    let start_identity = current_lookup_rows(&mut workspace, &assembly)
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("wire should expose successor source");
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_snapshot_read_only_context())
        .validate()
        .expect("snapshot context should validate")
        .admit()
        .expect("snapshot context should admit");
    let snapshot_token = workspace.snapshot_token().to_string();
    let mut reads = handle.topology_reads(&mut workspace);

    let loop_cycle = reads
        .loop_cycle(&start_identity, 5)
        .expect("snapshot loop cycle should execute through handle-bound entry");

    assert_eq!(
        reads.operating_context_identity_digest(),
        handle.operating_context_identity_digest()
    );
    assert_eq!(
        loop_cycle.request_report.execution_engine,
        TopologyDomainQueryExecutionEngine::QueryRuntimeHistorical
    );
    assert_eq!(
        loop_cycle.request_report.executed_snapshot_token.as_deref(),
        Some(snapshot_token.as_str())
    );
    assert_eq!(reads.aggregate_report().request_count, 1);
}

fn seeded_workspace(
    stem: &str,
    primitive: MilestoneOnePrimitiveCase,
) -> (
    forge_query::facade::ForgeQueryWorkspace,
    TopologyQueryAssembly,
) {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    seed_milestone_one_primitive(&mut runtime, stem, &primitive).expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, stem).expect("workspace should build");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    (workspace, assembly)
}
