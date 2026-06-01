use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::*;
use crate::facade::certify_milestone_one_read_basis_traced;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::runtime_boundary::read_stage::{
    open_topology_read_view, stage_topology_read_from_view,
};
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
    report: &crate::facade::NamingAttachmentReport,
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
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-surfaces-sheet-disk",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let read_view =
        open_topology_read_view(&runtime, &verified.read_basis()).expect("read view should open");
    let (mut workspace, surfaces) =
        current_head_workspace(runtime, "topology-declared-query-surfaces");
    let snapshot = surfaces
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis())
        .expect("query snapshot should decode");
    let persistent_name_rows = workspace.read(surfaces.persistent_names());
    let staged = stage_topology_read_from_view(&read_view).expect("read stage should succeed");

    let mut certification_runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let _verified = seed_milestone_one_primitive(
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
        sorted_naming_attachments(&snapshot.naming_attachments),
        sorted_naming_attachments(&certified_runtime_report.naming_attachment_report)
    );
    assert!(persistent_name_rows.iter().all(|row| {
        row.external_row()
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .is_some()
    }));
    assert_eq!(
        snapshot.materialized.topology(),
        staged.materialized().topology()
    );
    assert_eq!(
        snapshot.materialized.report().breadth.topology_entity_count,
        staged.materialized().report().breadth.topology_entity_count
    );
    assert_eq!(
        snapshot
            .materialized
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
        snapshot.materialized.report().whole_view_materialization,
        staged.materialized().report().whole_view_materialization
    );
    assert_eq!(
        snapshot.materialized.report().fallback_class,
        staged.materialized().report().fallback_class
    );
    assert_eq!(
        snapshot.interpreted.interpretations(),
        staged.interpreted().interpretations()
    );
    assert_eq!(
        snapshot.interpreted.boundary_summaries(),
        staged.interpreted().boundary_summaries()
    );
    assert_eq!(
        snapshot.interpreted.radial_summaries(),
        staged.interpreted().radial_summaries()
    );
    assert_eq!(snapshot.interpreted.report(), staged.interpreted().report());
    assert_eq!(snapshot.validation, staged.validation().clone());
    assert_eq!(
        snapshot.diagnostics.invalidation_report,
        crate::projection::diagnostic_surfaces::build_derived_invalidation_report(
            &verified.read_basis()
        )
    );
    assert_eq!(
        snapshot.diagnostics.rebuild_report,
        crate::projection::diagnostic_surfaces::build_derived_rebuild_report(
            staged.materialized(),
            &snapshot.interpreted,
            staged.validation(),
        )
    );
    assert_eq!(
        snapshot.diagnostics.fallback_report,
        crate::projection::diagnostic_surfaces::build_derived_fallback_report(
            &verified.read_basis(),
            staged.materialized(),
        )
    );
    assert_eq!(
        snapshot.equivalence_contract.authority_snapshot_id,
        verified.read_basis().snapshot().snapshot_id.0
    );
    assert_eq!(
        snapshot.equivalence_contract.authority_branch_id,
        verified.read_basis().branch_id().0.as_str()
    );
    assert_eq!(
        snapshot.equivalence_contract.authoritative_mutation_origin,
        verified.read_basis().authoritative_mutation_origin()
    );
    assert_eq!(
        snapshot.equivalence_contract.derivation_origin,
        verified.read_basis().derivation_origin()
    );
    assert_eq!(
        snapshot.equivalence_contract.truth_basis_digest_hex,
        verified
            .read_basis()
            .authority
            .truth_basis_identity
            .mutation_digest_hex
    );
    assert_eq!(
        snapshot.equivalence_contract.touched_aspect_count,
        verified.read_basis().touched_aspects().len()
    );
    assert_eq!(
        snapshot.equivalence_contract.triggered_invalidation_targets,
        snapshot
            .diagnostics
            .invalidation_report
            .rows
            .iter()
            .filter(|row| row.triggered)
            .map(|row| row.target)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        snapshot.equivalence_contract.precision_fallback_count,
        verified.read_basis().precision_fallbacks.len()
    );
    assert_eq!(
        snapshot
            .equivalence_contract
            .precision_budget_fallback_count,
        verified.read_basis().precision_budget_fallbacks.len()
    );
}
