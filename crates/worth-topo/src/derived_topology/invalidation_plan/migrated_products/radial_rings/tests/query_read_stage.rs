use super::super::{
    close_radial_ring_migration_slice, RadialRingExecutionInput, RadialRingReadStageExecutor,
};
use super::support::{
    selected_radial_ring_touched_closure, selected_radial_rings_plan,
    selected_radial_rings_plan_with_query_read_digest,
};
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces;
use crate::projection::TopologyQueryRowLookup;
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
    TopologyCurrentHeadReadHandleExt,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::ForgeQueryApplicationFacade;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde_json::Value;

#[test]
fn read_stage_consumes_query_native_radial_neighborhood_view() {
    let mut runtime = build_milestone_one_runtime().expect("runtime should build");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "radial-ring.phase-13.query-read",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("primitive should seed");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "radial-ring.phase-13.query-read.runtime").unwrap();
    let surfaces = declare_topology_query_surfaces(&mut workspace).unwrap();
    let entity_rows = workspace.read::<Value>(surfaces.entities());
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let source_identity = TopologyQueryRowLookup::new(&entity_rows, &relation_rows)
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let mut reads = handle.topology_reads(&mut workspace);
    let radial = reads
        .radial_half_edge_neighborhood(
            &crate::projection::read_views::domain::TopologyReadAnchorIdentity::from_runtime_row_label(
                &source_identity,
            ),
        )
        .expect("radial neighborhood should execute through Query");
    let plan = selected_radial_rings_plan("radial-query-read");
    let touched_closure = selected_radial_ring_touched_closure("radial-query-read");

    let read_source = super::super::RadialRingReadSource::from_query_radial_neighborhood_views(
        &plan,
        &touched_closure,
        &[radial],
    )
    .unwrap();

    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(
        read_source.selected_rows()[0].source_half_edge_identity(),
        source_identity
    );
    assert_eq!(read_source.counters().half_edge_lookup_count(), 1);
    assert_eq!(read_source.counters().radial_relation_lookup_count(), 1);
    assert_eq!(read_source.counters().whole_view_fallback_count(), 0);
}

#[test]
fn query_native_radial_read_closes_full_migration_slice() {
    let mut runtime = build_milestone_one_runtime().expect("runtime should build");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "radial-ring.phase-13.query-closeout",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("primitive should seed");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "radial-ring.phase-13.query-closeout.runtime").unwrap();
    let surfaces = declare_topology_query_surfaces(&mut workspace).unwrap();
    let entity_rows = workspace.read::<Value>(surfaces.entities());
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let source_identity = TopologyQueryRowLookup::new(&entity_rows, &relation_rows)
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let mut reads = handle.topology_reads(&mut workspace);
    let radial = reads
        .radial_half_edge_neighborhood(
            &crate::projection::read_views::domain::TopologyReadAnchorIdentity::from_runtime_row_label(
                &source_identity,
            ),
        )
        .expect("radial neighborhood should execute through Query");
    let touched_closure = selected_radial_ring_touched_closure("radial-query-closeout");
    let provisional_plan = selected_radial_rings_plan("radial-query-closeout");
    let provisional_source =
        super::super::RadialRingReadSource::from_query_radial_neighborhood_views(
            &provisional_plan,
            &touched_closure,
            std::slice::from_ref(&radial),
        )
        .unwrap();
    let query_report_digest = provisional_source
        .query_report_digests()
        .first()
        .expect("Query radial view should produce a report digest")
        .to_string();
    let plan = selected_radial_rings_plan_with_query_read_digest(
        "radial-query-closeout",
        &query_report_digest,
    );
    let read_source = super::super::RadialRingReadSource::from_query_radial_neighborhood_views(
        &plan,
        &touched_closure,
        &[radial],
    )
    .unwrap();
    let read_receipt = RadialRingReadStageExecutor::execute(&plan, read_source).unwrap();
    assert_eq!(
        read_receipt.native_query_read_receipt_digest(),
        query_report_digest
    );
    let input =
        RadialRingExecutionInput::from_selected_plan_and_read_stage(&plan, read_receipt).unwrap();

    let closeout = close_radial_ring_migration_slice(&plan, input).unwrap();

    assert_eq!(closeout.counters().output_row_count(), 1);
    assert_eq!(closeout.counters().selected_source_row_count(), 1);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert_eq!(closeout.counters().read_stage_half_edge_lookup_count(), 1);
    assert_eq!(
        closeout
            .counters()
            .read_stage_radial_relation_lookup_count(),
        1
    );
}
