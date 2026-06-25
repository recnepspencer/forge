use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::{
    historical_derived_surface_snapshot_for_read_basis, historical_query_snapshot_for_read_basis,
};
use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::projection::runtime_boundary::declared_query_surfaces::retained_artifacts::materialize_topology_historical_truth_artifact;
use crate::projection::runtime_boundary::declared_query_surfaces::{
    declare_topology_query_surfaces, equivalence_contract_from_diagnostics_rows,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::runtime_boundary::read_stage::{
    open_topology_read_view, stage_topology_read_from_view,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::milestone_one_runtime_builder;

#[test]
fn historical_query_snapshot_uses_retained_query_rows_after_runtime_declaration() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query-native-surfaces-historical-derived-rows",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let read_view =
        open_topology_read_view(&runtime, &verified.read_basis()).expect("read view should open");
    let adapters = TopologyRuntimeAdapters::snapshot_historical_basis(
        read_view,
        verified.read_basis().clone(),
    );
    let mut workspace = topology_runtime(
        adapters,
        "topology-declared-query-surfaces-historical-derived-rows",
    )
    .expect("query workspace should build");
    let surfaces =
        declare_topology_query_surfaces(&mut workspace).expect("query surfaces should declare");

    assert_eq!(
        materialized_row_count(&mut workspace, surfaces.materialized()),
        1
    );
    assert_eq!(
        materialized_row_count(&mut workspace, surfaces.interpreted()),
        1
    );
    assert_eq!(
        materialized_row_count(&mut workspace, surfaces.validation()),
        1
    );
    assert_eq!(
        materialized_row_count(&mut workspace, surfaces.diagnostics()),
        1
    );
    assert_eq!(
        materialized_row_count(&mut workspace, surfaces.equivalence_contract()),
        1
    );

    let mut query_runtime = HistoricalReadBasisQueryRuntime::open(
        &runtime,
        verified.read_basis().clone(),
        "historical-derived-rows-runtime",
    )
    .expect("read-basis query runtime should open");
    let snapshot = historical_query_snapshot_for_read_basis(&mut query_runtime)
        .expect("historical snapshot should decode");

    assert_eq!(
        equivalence_contract_from_diagnostics_rows(&[
            serde_json::to_value(snapshot.diagnostics()).expect("diagnostics should encode")
        ])
        .expect("diagnostics rows should decode equivalence"),
        snapshot.equivalence_contract().clone()
    );
}

#[test]
fn derived_topology_closeout_uses_narrow_historical_derived_surface_snapshot_lane() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/certification/derived_topology_closeout/read_basis.rs"),
    )
    .expect("derived topology closeout proof file should remain readable");

    assert!(
        source.contains("historical_derived_surface_snapshot_for_read_basis("),
        "derived topology closeout should use the narrower historical derived-surface helper",
    );
    assert!(
        !source.contains("historical_query_snapshot_for_read_basis("),
        "derived topology closeout should not rebuild naming attachments it does not read",
    );
    assert!(
        source.contains("HistoricalReadBasisQueryRuntime::open("),
        "derived topology closeout should open one shared read-basis query runtime seam instead of staging and declaring query surfaces inline",
    );
}

#[test]
fn historical_derived_surface_snapshot_matches_full_snapshot_except_naming_attachments() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query-native-surfaces-historical-derived-snapshot",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");

    let mut query_runtime = HistoricalReadBasisQueryRuntime::open(
        &runtime,
        verified.read_basis().clone(),
        "historical-derived-snapshot-runtime",
    )
    .expect("read-basis query runtime should open");
    let derived_snapshot = historical_derived_surface_snapshot_for_read_basis(&mut query_runtime)
        .expect("historical derived surface snapshot should decode");
    let full_snapshot = historical_query_snapshot_for_read_basis(&mut query_runtime)
        .expect("historical query snapshot should decode");

    assert_eq!(
        derived_snapshot.materialized(),
        full_snapshot.materialized()
    );
    assert_eq!(derived_snapshot.interpreted(), full_snapshot.interpreted());
    assert_eq!(derived_snapshot.validation(), full_snapshot.validation());
    assert_eq!(derived_snapshot.diagnostics(), full_snapshot.diagnostics());
    assert_eq!(
        derived_snapshot.equivalence_contract(),
        full_snapshot.equivalence_contract()
    );
}

#[test]
fn historical_derived_snapshot_reads_retained_diagnostics_and_equivalence_only() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/certification/support/historical_query_snapshot/derived_snapshot.rs"),
    )
    .expect("historical derived snapshot source should remain readable");

    assert!(
        source.contains("runtime.historical_derived_surface_snapshot()"),
        "historical derived snapshot should consume the shared runtime-owned retained derived-surface snapshot boundary",
    );
    assert!(
        source.contains("runtime.historical_equivalence_read_basis_facts()"),
        "historical derived snapshot should consume Query-owned retained payload read-basis evidence instead of reopening local rows",
    );
    assert!(
        !source.contains("materialize_derived_artifact_bundle("),
        "historical derived snapshot should not reopen retained artifact materialization locally once the runtime boundary owns it",
    );
}

#[test]
fn historical_snapshot_read_basis_proof_uses_retained_payload_report_evidence() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/certification/support/historical_query_snapshot/mod.rs"),
    )
    .expect("historical snapshot proof source should remain readable");

    assert!(
        source.contains("DerivedEquivalenceContractReport"),
        "historical snapshot read-basis proof should consume the decoded Query-owned retained payload report",
    );
    assert!(
        source.contains("equivalence.authority_snapshot_id")
            && source.contains("equivalence.touched_aspect_count"),
        "historical snapshot read-basis proof should compare semantic equivalence fields from the retained payload report",
    );
    assert!(
        !source.contains("ForgeQueryRetainedScalarFactSet")
            && !source.contains("ReadBasisEquivalenceField")
            && !source.contains("field_value_at(&path)"),
        "historical snapshot read-basis proof should not revive retained scalar fact plumbing",
    );
}

#[test]
fn authority_and_derived_read_basis_callers_use_shared_query_runtime_boundary() {
    let authority_source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/certification/authority_closeout/read_view/read_basis_trace.rs"),
    )
    .expect("authority closeout read-basis trace source should remain readable");
    let derived_source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/certification/derived_topology_closeout/read_basis.rs"),
    )
    .expect("derived topology closeout read-basis source should remain readable");

    for source in [&authority_source, &derived_source] {
        assert!(
            source.contains("HistoricalReadBasisQueryRuntime::open("),
            "historical read-basis certification callers should open the shared query-runtime seam",
        );
        assert!(
            !source.contains("stage_topology_read_from_view(&read_view)"),
            "historical read-basis certification callers should not stage read truth inline anymore",
        );
        assert!(
            !source.contains("declare_topology_query_surfaces("),
            "historical read-basis certification callers should not declare query surfaces inline anymore",
        );
    }
}

#[test]
fn historical_truth_artifact_decodes_retained_rows_without_staged_fallback() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query-native-surfaces-historical-truth-assembly",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let read_view =
        open_topology_read_view(&runtime, &verified.read_basis()).expect("read view should open");
    let adapters = TopologyRuntimeAdapters::snapshot_historical_basis(
        read_view,
        verified.read_basis().clone(),
    );
    let mut workspace = topology_runtime(
        adapters,
        "topology-declared-query-surfaces-historical-truth-assembly",
    )
    .expect("query workspace should build");
    let surfaces =
        declare_topology_query_surfaces(&mut workspace).expect("query surfaces should declare");

    let staged_truth = stage_topology_read_from_view(
        &open_topology_read_view(&runtime, &verified.read_basis()).expect("read view should open"),
    )
    .expect("read stage should succeed");
    let bundle = materialize_topology_historical_truth_artifact(&surfaces, &mut workspace)
        .expect("assembly should decode");
    assert_eq!(
        bundle.materialized().topology(),
        staged_truth.materialized().topology()
    );
    assert_eq!(
        bundle.materialized().report().breadth.topology_entity_count,
        staged_truth
            .materialized()
            .report()
            .breadth
            .topology_entity_count
    );
    assert_eq!(
        bundle
            .materialized()
            .report()
            .breadth
            .topology_relation_count,
        staged_truth
            .materialized()
            .report()
            .breadth
            .topology_relation_count
    );
    assert_eq!(
        bundle.interpreted().materialized().topology(),
        staged_truth.interpreted().materialized().topology()
    );
    assert_eq!(
        bundle.interpreted().interpretations(),
        staged_truth.interpreted().interpretations()
    );
    assert_eq!(
        bundle.interpreted().boundary_summaries(),
        staged_truth.interpreted().boundary_summaries()
    );
    assert_eq!(
        bundle.interpreted().radial_summaries(),
        staged_truth.interpreted().radial_summaries()
    );
    assert_eq!(
        bundle.interpreted().report(),
        staged_truth.interpreted().report()
    );
    assert_eq!(bundle.validation(), staged_truth.validation());
}

#[test]
fn historical_truth_artifact_uses_declared_surface_materialization_boundary() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/projection/runtime_boundary/declared_query_surfaces/retained_artifacts.rs"),
    )
    .expect("historical truth artifact source should remain readable");

    for required in [
        "materialize_declared_query_surface_binding(",
        "ForgeQueryDerivedArtifactBinding",
        "topology.historical.truth",
        "decode_bundle_row(",
        "retained_payload::decode_retained_payload_row(",
    ] {
        assert!(
            source.contains(required),
            "declared-query-surfaces retained artifact should use Query-owned retained-artifact boundary `{required}`",
        );
    }
    assert!(
        !source.contains("workspace.materialize(surfaces.materialized())"),
        "declared-query-surfaces retained artifact should not bypass the Query-owned materialization boundary for materialized rows",
    );
    assert!(
        !source.contains("ForgeQueryDerivedMaterializationBundle"),
        "declared-query-surfaces retained artifact should not treat the naked Query bundle as the final retained artifact boundary",
    );
}

fn materialized_row_count(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    view: &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
) -> usize {
    workspace
        .materialize_intent(view)
        .execute()
        .expect("declared surface should materialize")
        .row_count()
}

#[test]
fn historical_full_snapshot_uses_query_owned_live_artifact_binding_for_naming_rows() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/certification/support/historical_query_snapshot/full_snapshot.rs"),
    )
    .expect("historical full snapshot source should remain readable");

    assert!(
        source.contains("read_declared_query_surface_binding("),
        "historical full snapshot should consume one Query-owned live artifact binding for naming attachments",
    );
    assert!(
        !source.contains("workspace.read(surfaces.entities())"),
        "historical full snapshot should not reopen entity rows through direct workspace.read",
    );
    assert!(
        !source.contains("workspace.read(surfaces.persistent_names())"),
        "historical full snapshot should not reopen persistent-name rows through direct workspace.read",
    );
}
