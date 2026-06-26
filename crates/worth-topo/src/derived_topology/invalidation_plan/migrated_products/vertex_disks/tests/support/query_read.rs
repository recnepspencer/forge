use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::VertexDiskReadSource;
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::projection::read_views::domain::TopologyReadAnchorIdentity;
use crate::projection::read_views::TopologyHalfEdgeSharedVertexNeighborhoodView;
use crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces;
use crate::projection::TopologyQueryRowLookup;
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
    TopologyCurrentHeadReadHandleExt,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::ForgeQueryApplicationFacade;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde_json::Value;

use super::{selected_vertex_disk_touched_closure, selected_vertex_disks_plan};

pub(crate) struct QueryNativeSharedVertexFixture {
    pub(crate) source_identity: String,
    pub(crate) shared_vertex: TopologyHalfEdgeSharedVertexNeighborhoodView,
}

pub(crate) struct QueryNativeVertexDiskReadSourceFixture {
    pub(crate) source_identity: String,
    pub(crate) read_source: VertexDiskReadSource,
}

pub(crate) fn query_native_shared_vertex_view(case_name: &str) -> QueryNativeSharedVertexFixture {
    let mut workspace = seeded_query_runtime_workspace(case_name);
    let source_identity = first_shared_vertex_source_identity(&mut workspace);
    let shared_vertex = execute_shared_vertex_query_read(&mut workspace, &source_identity);
    QueryNativeSharedVertexFixture {
        source_identity,
        shared_vertex,
    }
}

pub(crate) fn query_native_vertex_disk_read_source(
    case_name: &str,
    operator_family: &'static str,
) -> QueryNativeVertexDiskReadSourceFixture {
    let fixture = query_native_shared_vertex_view(case_name);
    let plan = selected_vertex_disks_plan(operator_family);
    let touched_closure = selected_vertex_disk_touched_closure(operator_family);
    let read_source = VertexDiskReadSource::from_query_shared_vertex_neighborhood_views(
        &plan,
        &touched_closure,
        std::slice::from_ref(&fixture.shared_vertex),
    )
    .unwrap();
    QueryNativeVertexDiskReadSourceFixture {
        source_identity: fixture.source_identity,
        read_source,
    }
}

fn seeded_query_runtime_workspace(case_name: &str) -> forge_query::facade::ForgeQueryWorkspace {
    let mut runtime = build_milestone_one_runtime().expect("runtime should build");
    seed_nmt_edge_fan_primitive(&mut runtime, case_name);
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    topology_runtime(adapters, format!("{case_name}.runtime")).unwrap()
}

fn seed_nmt_edge_fan_primitive(runtime: &mut RelationalRuntime, case_name: &str) {
    seed_milestone_one_primitive_through_schema_execution(
        runtime,
        case_name,
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("primitive should seed");
}

fn first_shared_vertex_source_identity(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
) -> String {
    let surfaces = declare_topology_query_surfaces(workspace).unwrap();
    let entity_rows = workspace.read::<Value>(surfaces.entities());
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    TopologyQueryRowLookup::new(&entity_rows, &relation_rows)
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeStartsAtVertex)
        .expect("primitive should expose a half-edge vertex source")
}

fn execute_shared_vertex_query_read(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    source_identity: &str,
) -> TopologyHalfEdgeSharedVertexNeighborhoodView {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let mut reads = handle.topology_reads(workspace);
    let shared_vertex = reads
        .shared_vertex_half_edge_neighborhood(&TopologyReadAnchorIdentity::from_runtime_row_label(
            source_identity,
        ))
        .expect("shared vertex neighborhood should execute through Query");
    shared_vertex
}
