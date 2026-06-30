use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::*;
use crate::certification::certify_milestone_one_read_basis_traced;
use crate::certification::support::historical_query_snapshot::historical_query_snapshot_for_read_basis;
use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::runtime_boundary::read_stage::{
    open_topology_read_view, stage_topology_read_from_view,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::milestone_one_runtime_builder;

fn current_head_workspace(
    runtime: forge_relational::facade::runtime::RelationalRuntime,
    name: &str,
) -> (
    forge_query::facade::ForgeQueryWorkspace,
    TopologyDeclaredQuerySurfaces,
) {
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, name).expect("query workspace should build");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("query surfaces should declare");
    (workspace, surfaces)
}

fn sorted_naming_attachments(
    report: &crate::certification::NamingAttachmentReport,
) -> Vec<(String, String, Vec<String>)> {
    let mut rows = report
        .attachments
        .iter()
        .map(|row| {
            let mut attached_ids = row
                .attached_persistent_name_ids
                .iter()
                .map(|id| format!("{id:?}"))
                .collect::<Vec<_>>();
            attached_ids.sort();
            (
                format!("{:?}", row.topology_entity_id),
                row.topology_kind_name.clone(),
                attached_ids,
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

#[test]
fn query_native_assembly_reads_production_runtime_and_matches_staged_outputs() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query-native-surfaces-sheet-disk",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let read_view =
        open_topology_read_view(&runtime, &verified.read_basis()).expect("read view should open");
    let staged = stage_topology_read_from_view(&read_view).expect("read stage should succeed");
    let (mut workspace, surfaces) =
        current_head_workspace(runtime, "topology-declared-query-surfaces");
    let mut historical_runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let _historical_verified = seed_milestone_one_primitive_through_schema_execution(
        &mut historical_runtime,
        "query-native-surfaces-sheet-disk",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let mut query_runtime = HistoricalReadBasisQueryRuntime::open(
        &historical_runtime,
        verified.read_basis().clone(),
        "topology-declared-query-surfaces-historical-runtime",
    )
    .expect("historical read-basis query runtime should open");
    let snapshot = historical_query_snapshot_for_read_basis(&mut query_runtime)
        .expect("query snapshot should decode");
    let persistent_name_rows = workspace.read(surfaces.persistent_names());

    let mut certification_runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let _verified = seed_milestone_one_primitive_through_schema_execution(
        &mut certification_runtime,
        "query-native-surfaces-sheet-disk",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let certified_runtime_report = certify_milestone_one_read_basis_traced(
        &mut certification_runtime,
        verified.read_basis().clone(),
    )
    .expect("milestone one certification should succeed")
    .into_primary_result();

    assert_eq!(
        sorted_naming_attachments(snapshot.naming_attachments()),
        sorted_naming_attachments(&certified_runtime_report.naming_attachment_report)
    );
    assert!(persistent_name_rows.iter().all(|row| {
        row.scalar_value_at(
            &crate::query_native_runtime_boundary::native_field_path([
                "lineage",
                "provenance_partition",
            ])
            .expect("lineage provenance partition field path should build"),
        )
        .is_some()
    }));
    assert_eq!(
        snapshot.materialized().topology(),
        staged.materialized().topology()
    );
    assert_eq!(
        snapshot
            .materialized()
            .report()
            .breadth
            .topology_entity_count,
        staged.materialized().report().breadth.topology_entity_count
    );
    assert_eq!(
        snapshot
            .materialized()
            .report()
            .breadth
            .topology_relation_count,
        staged
            .materialized()
            .report()
            .breadth
            .topology_relation_count
    );
    assert_eq!(
        snapshot.materialized().report().whole_view_materialization,
        staged.materialized().report().whole_view_materialization
    );
    assert_eq!(
        snapshot.materialized().report().fallback_class,
        staged.materialized().report().fallback_class
    );
    assert_eq!(
        snapshot.interpreted().interpretations(),
        staged.interpreted().interpretations()
    );
    assert_eq!(
        snapshot.interpreted().boundary_summaries(),
        staged.interpreted().boundary_summaries()
    );
    assert_eq!(
        snapshot.interpreted().radial_summaries(),
        staged.interpreted().radial_summaries()
    );
    assert_eq!(
        snapshot.interpreted().report(),
        staged.interpreted().report()
    );
    assert_eq!(snapshot.validation(), staged.validation());
    assert_eq!(
        snapshot.diagnostics().invalidation_report,
        crate::projection::diagnostic_surfaces::derived_read_diagnostics::build_derived_invalidation_report(
            &verified.read_basis()
        )
    );
    assert_eq!(
        snapshot.diagnostics().rebuild_report,
        crate::projection::diagnostic_surfaces::derived_read_diagnostics::build_derived_rebuild_report(
            staged.materialized(),
            snapshot.interpreted(),
            staged.validation(),
        )
    );
    assert_eq!(
        snapshot.diagnostics().fallback_report,
        crate::projection::diagnostic_surfaces::derived_read_diagnostics::build_derived_fallback_report(
            &verified.read_basis(),
            staged.materialized(),
        )
    );
    assert_eq!(
        snapshot.equivalence_contract().authority_snapshot_id,
        verified.read_basis().snapshot().snapshot_id.0
    );
    assert_eq!(
        snapshot.equivalence_contract().authority_branch_id,
        verified.read_basis().branch_id().0.as_str()
    );
    assert_eq!(
        snapshot
            .equivalence_contract()
            .authoritative_mutation_origin,
        verified.read_basis().authoritative_mutation_origin()
    );
    assert_eq!(
        snapshot.equivalence_contract().derivation_origin,
        verified.read_basis().derivation_origin()
    );
    assert_eq!(
        snapshot.equivalence_contract().truth_basis_digest_hex,
        verified
            .read_basis()
            .authority
            .truth_basis_identity
            .mutation_digest_hex
    );
    assert_eq!(
        snapshot.equivalence_contract().touched_aspect_count,
        verified.read_basis().touched_aspects().len()
    );
    assert_eq!(
        snapshot
            .equivalence_contract()
            .triggered_invalidation_targets,
        snapshot
            .diagnostics()
            .invalidation_report
            .rows
            .iter()
            .filter(|row| row.triggered)
            .map(|row| row.target)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        snapshot.equivalence_contract().precision_fallback_count,
        verified.read_basis().precision_fallbacks.len()
    );
    assert_eq!(
        snapshot
            .equivalence_contract()
            .precision_budget_fallback_count,
        verified.read_basis().precision_budget_fallbacks.len()
    );
}

#[test]
fn diagnostics_surface_fails_closed_without_historical_read_basis_metadata() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query-native-surfaces-missing-historical-basis",
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
        "topology-declared-query-surfaces-missing-historical-basis",
    )
    .expect("query workspace should build");
    let surfaces =
        declare_topology_query_surfaces(&mut workspace).expect("query surfaces should declare");

    let diagnostics_row: serde_json::Value =
        materialize_declared_query_surface_row(&mut workspace, surfaces.diagnostics())
            .expect("diagnostics row should materialize as retained payload");
    let equivalence_row: serde_json::Value =
        materialize_declared_query_surface_row(&mut workspace, surfaces.equivalence_contract())
            .expect("equivalence row should materialize as retained payload");

    assert!(
        diagnostics_row["query_surface_error_kind"]
            .as_str()
            .is_some_and(|kind| kind == "missing_historical_read_basis_metadata"),
        "diagnostics surface should preserve the missing historical read-basis authority denial kind: {diagnostics_row:?}",
    );
    assert!(
        diagnostics_row["query_surface_error"]
            .as_str()
            .is_some_and(|message| message.contains(".topology.historical_read_basis")),
        "diagnostics surface should fail closed when runtime-owned historical read-basis metadata is absent instead of rebuilding authority from retained rows: {diagnostics_row:?}",
    );
    assert!(
        equivalence_row["query_surface_error_kind"]
            .as_str()
            .is_some_and(|kind| kind == "missing_historical_read_basis_metadata"),
        "equivalence surface should preserve the topology authority denial kind instead of flattening it into a generic payload decode failure: {equivalence_row:?}",
    );
    assert!(
        equivalence_row["query_surface_error"].as_str().is_some_and(|message| {
            message.contains("declared failure payload")
                && message.contains(".topology.historical_read_basis")
        }),
        "equivalence surface should refuse diagnostics-carried failure payloads as admitted topology input for the original authority reason: {equivalence_row:?}",
    );
}
