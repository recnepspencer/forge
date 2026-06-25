use super::super::{
    close_shell_view_migration_slice, ShellViewExecutionInput, ShellViewReadStageExecutor,
};
use super::support::{
    selected_shell_view_touched_closure, selected_shell_views_plan,
    selected_shell_views_plan_with_query_read_digest,
};
use crate::derived_topology::invalidation_plan::migrated_products::shell_views::ShellViewMigrationError;
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::projection::read_views::TopologyShellBoundaryNeighborhoodView;
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
fn read_stage_consumes_query_native_shell_boundary_facts() {
    let mut runtime = build_milestone_one_runtime().expect("runtime should build");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "shell-view.phase-14.query-read",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("primitive should seed");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "shell-view.phase-14.query-read.runtime").unwrap();
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
    let shell_boundary = reads
        .shell_boundary_neighborhood(
            &crate::projection::read_views::domain::TopologyReadAnchorIdentity::from_runtime_row_label(
                &source_identity,
            ),
        )
        .expect("shell boundary neighborhood should execute through Query");
    let plan = selected_shell_views_plan("shell-query-read");
    let touched_closure = selected_shell_view_touched_closure("shell-query-read");

    let read_source = super::super::ShellViewReadSource::from_query_shell_boundary_views(
        &plan,
        &touched_closure,
        &[shell_boundary],
    )
    .unwrap();

    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(
        read_source.selected_rows()[0].source_half_edge_identity(),
        source_identity
    );
    assert_eq!(
        read_source.selected_rows()[0].touched_shell_identity(),
        expected_shell_identity_for_half_edge(&entity_rows, &relation_rows, &source_identity)
    );
    assert_eq!(read_source.counters().half_edge_lookup_count(), 1);
    assert_eq!(read_source.counters().radial_relation_lookup_count(), 1);
    assert_eq!(read_source.counters().whole_view_fallback_count(), 0);
}

#[test]
fn query_native_shell_boundary_read_closes_full_migration_slice() {
    let mut runtime = build_milestone_one_runtime().expect("runtime should build");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "shell-view.phase-14.query-closeout",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("primitive should seed");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "shell-view.phase-14.query-closeout.runtime").unwrap();
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
    let shell_boundary = reads
        .shell_boundary_neighborhood(
            &crate::projection::read_views::domain::TopologyReadAnchorIdentity::from_runtime_row_label(
                &source_identity,
            ),
        )
        .expect("shell boundary neighborhood should execute through Query");
    let touched_closure = selected_shell_view_touched_closure("shell-query-closeout");
    let provisional_plan = selected_shell_views_plan("shell-query-closeout");
    let provisional_source = super::super::ShellViewReadSource::from_query_shell_boundary_views(
        &provisional_plan,
        &touched_closure,
        std::slice::from_ref(&shell_boundary),
    )
    .unwrap();
    let query_report_digest = provisional_source
        .query_report_digests()
        .first()
        .expect("Query radial view should produce a report digest")
        .to_string();
    let plan = selected_shell_views_plan_with_query_read_digest(
        "shell-query-closeout",
        &query_report_digest,
    );
    let read_source = super::super::ShellViewReadSource::from_query_shell_boundary_views(
        &plan,
        &touched_closure,
        &[shell_boundary],
    )
    .unwrap();
    let read_receipt = ShellViewReadStageExecutor::execute(&plan, read_source).unwrap();
    assert_eq!(
        read_receipt.native_query_read_receipt_digest(),
        query_report_digest
    );
    let input =
        ShellViewExecutionInput::from_selected_plan_and_read_stage(&plan, read_receipt).unwrap();

    let closeout = close_shell_view_migration_slice(&plan, input).unwrap();

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

#[test]
fn radial_query_report_cannot_pose_as_shell_boundary_receipt() {
    let mut runtime = build_milestone_one_runtime().expect("runtime should build");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "shell-view.phase-14.radial-forgery",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("primitive should seed");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "shell-view.phase-14.radial-forgery.runtime").unwrap();
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
    let forged_shell_view = TopologyShellBoundaryNeighborhoodView {
        request_report: radial.request_report().clone(),
        touched_shell_identity: expected_shell_identity_for_half_edge(
            &entity_rows,
            &relation_rows,
            &source_identity,
        ),
        touched_face_identity: expected_face_identity_for_half_edge(
            &entity_rows,
            &relation_rows,
            &source_identity,
        ),
        source_half_edge_identity: radial.source_half_edge_identity().to_string(),
        source_edge_identity: radial.source_edge_identity().to_string(),
        current_target_half_edge_identity: radial.current_target_half_edge_identity().to_string(),
        current_target_edge_identity: radial.current_target_edge_identity().to_string(),
        source_radial_next_relation_identity: radial
            .source_radial_next_relation_identity()
            .to_string(),
        same_edge_half_edge_identities: radial.same_edge_half_edge_identities().to_vec(),
        different_edge_half_edge_identities: radial.different_edge_half_edge_identities().to_vec(),
        different_edge_half_edges: radial.different_edge_half_edges().to_vec(),
    };
    let plan = selected_shell_views_plan("shell-query-forgery");
    let touched_closure = selected_shell_view_touched_closure("shell-query-forgery");

    let error = super::super::ShellViewReadSource::from_query_shell_boundary_views(
        &plan,
        &touched_closure,
        &[forged_shell_view],
    )
    .unwrap_err();

    assert_eq!(error, ShellViewMigrationError::ReadStageQueryProofInvalid);
}

fn expected_shell_identity_for_half_edge(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    source_identity: &str,
) -> String {
    let face_identity =
        expected_face_identity_for_half_edge(entity_rows, relation_rows, source_identity);
    TopologyQueryRowLookup::new(entity_rows, relation_rows)
        .incoming_source_identity(&face_identity, TopologyRelationKind::ShellOwnsFace)
        .expect("face should have an owning shell")
}

fn expected_face_identity_for_half_edge(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    source_identity: &str,
) -> String {
    let lookup = TopologyQueryRowLookup::new(entity_rows, relation_rows);
    let loop_identity = lookup
        .incoming_source_identity(source_identity, TopologyRelationKind::LoopOwnsHalfEdge)
        .expect("half edge should be owned by a loop");
    lookup
        .incoming_source_identity(&loop_identity, TopologyRelationKind::FaceOuterLoop)
        .or_else(|_| {
            lookup.incoming_source_identity(&loop_identity, TopologyRelationKind::FaceInnerLoop)
        })
        .expect("loop should be owned by a face")
}
