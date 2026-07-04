use crate::certification::error::TopologyCertificationError;

use super::super::support::{closed_row, ensure, source_text};
use super::super::TopologyQueryBoundaryCleanupArea;

pub(crate) fn certify_snapshot_surfaces_row(
) -> Result<super::super::TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let declared_query_surfaces_mod =
        source_text("src/projection/runtime_boundary/declared_query_surfaces/mod.rs")?;
    let certification_snapshot_support =
        source_text("src/certification/support/historical_query_snapshot/mod.rs")?;
    let certification_snapshot_derived_support =
        source_text("src/certification/support/historical_query_snapshot/derived_snapshot.rs")?;
    let certification_snapshot_full_support =
        source_text("src/certification/support/historical_query_snapshot/full_snapshot.rs")?;
    let certification_read_basis_query_runtime =
        source_text("src/certification/support/read_basis_query_runtime.rs")?;
    let retained_artifacts = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/retained_artifacts.rs",
    )?;
    let current_head_materialized_support =
        source_text("src/certification/support/current_head_materialized_topology.rs")?;
    let declared_derived_surfaces = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/derived_surfaces/mod.rs",
    )?;
    let declared_query_diagnostics = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/query_diagnostics/mod.rs",
    )?;
    let retained_query_diagnostics = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/query_diagnostics/retained_diagnostics.rs",
    )?;
    let declared_equivalence_contract = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/query_diagnostics/equivalence_contract.rs",
    )?;
    let declared_truth_surfaces = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/truth_surfaces/mod.rs",
    )?;
    let declared_computed_views = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/derived_surfaces/computed_views.rs",
    )?;
    let projection_mod = source_text("src/projection/mod.rs")?;
    let read_views_domain_mod = source_text("src/projection/read_views/domain/mod.rs")?;
    let materialized_graph = source_text("src/derived_topology/materialized_graph/mod.rs")?;
    let persistent_naming = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/truth_surfaces/persistent_naming.rs",
    )?;
    let derived_read_basis =
        source_text("src/certification/derived_topology_closeout/read_basis.rs")?;
    let authority_read_basis =
        source_text("src/certification/authority_closeout/read_view/read_basis_trace.rs")?;

    ensure(
        !declared_query_surfaces_mod.contains("workspace.read(&self.entities)")
            && !declared_query_surfaces_mod.contains("workspace.materialize(&self.materialized)")
            && !declared_query_surfaces_mod.contains("naming_attachment_report_from_query"),
    )?;
    ensure(!declared_query_surfaces_mod.contains("use crate::projection::{"))?;
    ensure(!declared_query_surfaces_mod.contains("TopologyDerivedQuerySnapshot"))?;
    ensure(!declared_query_surfaces_mod.contains("snapshot_for_read_basis"))?;
    ensure(declared_query_surfaces_mod.contains("mod derived_surfaces;"))?;
    ensure(declared_query_surfaces_mod.contains("mod query_diagnostics;"))?;
    ensure(declared_query_surfaces_mod.contains("mod truth_surfaces;"))?;
    ensure(!declared_query_surfaces_mod.contains("historical_snapshot"))?;
    ensure(!declared_query_surfaces_mod.contains("mod historical_rows;"))?;
    ensure(!declared_query_surfaces_mod.contains("mod snapshot_decode;"))?;
    ensure(!projection_mod.contains("pub(crate) mod truth_surfaces;"))?;
    ensure(!projection_mod.contains("pub(crate) mod derived_surfaces;"))?;
    ensure(!projection_mod.contains("mod diagnostic_surfaces;"))?;
    ensure(read_views_domain_mod.contains("pub(crate) mod read_proof;"))?;
    ensure(declared_derived_surfaces.contains("declare_topology_interpreted_surface"))?;
    ensure(declared_query_diagnostics.contains("declare_topology_diagnostics_surface"))?;
    ensure(declared_truth_surfaces.contains("declare_topology_materialized_surface"))?;
    ensure(declared_computed_views.contains("decode_single_retained_payload_row("))?;
    ensure(declared_computed_views.contains("declared_retained_computed_row_sets(view)"))?;
    ensure(declared_computed_views.contains("interpreted_topology_from_upstreams("))?;
    ensure(declared_computed_views.contains("validation_report_from_upstreams("))?;
    ensure(declared_query_diagnostics.contains("mod retained_diagnostics;"))?;
    ensure(retained_query_diagnostics.contains("decode_single_retained_payload_row("))?;
    ensure(retained_query_diagnostics.contains("declared_retained_computed_row_sets(view)"))?;
    ensure(
        declared_equivalence_contract
            .contains("decode_single_retained_payload_row::<DerivedReadDiagnostics>"),
    )?;
    ensure(certification_snapshot_support.contains("historical_query_snapshot_for_read_basis"))?;
    ensure(
        certification_snapshot_support
            .contains("historical_derived_surface_snapshot_for_read_basis"),
    )?;
    ensure(certification_snapshot_full_support.contains("historical_naming_attachments"))?;
    ensure(certification_snapshot_full_support.contains("persistent_names"))?;
    ensure(certification_snapshot_full_support.contains("read_declared_query_surface_binding("))?;
    ensure(!certification_snapshot_full_support.contains("workspace.read(surfaces.entities())"))?;
    ensure(
        !certification_snapshot_full_support
            .contains("workspace.read(surfaces.persistent_names())"),
    )?;
    ensure(
        certification_snapshot_derived_support
            .contains("runtime.historical_derived_surface_snapshot()"),
    )?;
    ensure(
        certification_snapshot_derived_support
            .contains("runtime.historical_equivalence_read_basis_facts()"),
    )?;
    ensure(
        !certification_snapshot_derived_support.contains("materialize_derived_artifact_bundle("),
    )?;
    ensure(!certification_snapshot_derived_support.contains("build_derived_read_diagnostics("))?;
    ensure(
        !certification_snapshot_derived_support
            .contains("equivalence_contract_from_diagnostics_rows"),
    )?;
    ensure(retained_artifacts.contains("TopologyHistoricalTruthArtifact"))?;
    ensure(retained_artifacts.contains("TopologyHistoricalDerivedSurfaceSnapshot"))?;
    ensure(retained_artifacts.contains("materialize_topology_historical_truth_artifact("))?;
    ensure(
        retained_artifacts.contains("materialize_topology_historical_derived_surface_snapshot("),
    )?;
    ensure(retained_artifacts.contains("materialize_declared_query_surface_binding("))?;
    ensure(retained_artifacts.contains("decode_bundle_row("))?;
    ensure(retained_artifacts.contains("single_retained_row()"))?;
    ensure(retained_artifacts.contains("retained_payload::decode_retained_payload_row("))?;
    ensure(
        retained_artifacts
            .contains("diagnostics.equivalence_contract_report != equivalence_contract"),
    )?;
    ensure(retained_artifacts.contains("ForgeQueryDerivedArtifactBinding"))?;
    ensure(!retained_artifacts.contains("ForgeQueryDerivedMaterializationBundle"))?;
    ensure(retained_artifacts.contains("topology.historical.truth"))?;
    ensure(retained_artifacts.contains("topology.historical.derived_snapshot"))?;
    ensure(!retained_artifacts.contains("equivalence_contract_from_diagnostics_rows"))?;
    ensure(!retained_artifacts.contains("verify_scalar_alignment("))?;
    ensure(!retained_artifacts.contains("decode_row_pair("))?;
    ensure(!retained_artifacts.contains("decode_row_triple("))?;
    ensure(declared_query_surfaces_mod.contains("materialize_derived_artifact_binding("))?;
    ensure(declared_query_surfaces_mod.contains("read_live_artifact_binding("))?;
    ensure(declared_query_surfaces_mod.contains("materialize_intent(view)"))?;
    ensure(declared_query_surfaces_mod.contains(".single_retained_row()"))?;
    ensure(declared_query_surfaces_mod.contains("retained_payload::decode_retained_payload_row"))?;
    ensure(
        certification_read_basis_query_runtime.contains("struct HistoricalReadBasisQueryRuntime"),
    )?;
    ensure(
        certification_read_basis_query_runtime
            .contains("declare_topology_query_surfaces(&mut workspace)"),
    )?;
    ensure(
        certification_read_basis_query_runtime.contains("fn historical_derived_surface_snapshot("),
    )?;
    ensure(
        certification_read_basis_query_runtime
            .contains("fn historical_equivalence_read_basis_facts("),
    )?;
    ensure(certification_snapshot_support.contains("DerivedEquivalenceContractReport"))?;
    ensure(certification_snapshot_support.contains("equivalence.authority_snapshot_id"))?;
    ensure(certification_snapshot_support.contains("equivalence.authority_branch_id"))?;
    ensure(certification_snapshot_support.contains("equivalence.truth_basis_digest_hex"))?;
    ensure(certification_snapshot_support.contains("equivalence.touched_aspect_count"))?;
    ensure(!certification_snapshot_support.contains("ForgeQueryRetainedScalarFactSet"))?;
    ensure(!certification_snapshot_support.contains("ReadBasisEquivalenceField"))?;
    ensure(current_head_materialized_support.contains("current_head_materialized_topology"))?;
    ensure(!materialized_graph.contains("materialize_from_query_rows"))?;
    ensure(!persistent_naming.contains("naming_attachment_report_from_query_rows"))?;
    ensure(derived_read_basis.contains("historical_derived_surface_snapshot_for_read_basis("))?;
    ensure(!derived_read_basis.contains("historical_query_snapshot_for_read_basis("))?;
    ensure(!derived_read_basis.contains("stage_topology_read_from_view("))?;
    ensure(derived_read_basis.contains("HistoricalReadBasisQueryRuntime::open("))?;
    ensure(!authority_read_basis.contains("stage_topology_read_from_view("))?;
    ensure(authority_read_basis.contains("HistoricalReadBasisQueryRuntime::open("))?;
    ensure(
        !source_text("src/certification/projection_closeout/tests/materialization.rs")?
            .contains("historical_query_snapshot_for_read_basis("),
    )?;
    ensure(
        !source_text(
            "src/certification/topology_operator_closeout/scenario_programs/ambiguous_local_rewire.rs",
        )?
        .contains("historical_query_snapshot_for_read_basis("),
    )?;
    ensure(
        !source_text(
            "src/certification/topology_operator_closeout/scenario_programs/bowtie_adjacent.rs",
        )?
        .contains("historical_query_snapshot_for_read_basis("),
    )?;
    ensure(
        !source_text(
            "src/certification/topology_operator_closeout/scenario_programs/broken_radial_localization.rs",
        )?
        .contains("historical_query_snapshot_for_read_basis("),
    )?;
    ensure(
        !source_text(
            "src/certification/topology_operator_closeout/scenario_programs/cancellation_chain.rs",
        )?
        .contains("historical_query_snapshot_for_read_basis("),
    )?;
    ensure(
        !source_text(
            "src/certification/topology_operator_closeout/scenario_programs/split_collapse_churn.rs",
        )?
        .contains("historical_query_snapshot_for_read_basis("),
    )?;

    closed_row(
        TopologyQueryBoundaryCleanupArea::SnapshotSurfaces,
        "declared query surfaces only own live and computed declaration seams; certification support owns historical snapshot reconstruction only where true read-basis proof still needs it, certification read-basis callers cross one shared historical query-runtime seam instead of rebuilding staged query assembly inline, full historical naming attachments now cross one explicit Query-owned live-artifact binding instead of reopening direct live reads, the surviving local historical truth now crosses an explicit retained-artifact seam before derived or full snapshot orchestration, that retained-artifact seam decodes native retained payload rows through Query-owned binding helpers and compares semantic diagnostics/equivalence contract payloads instead of repeating local decode and comparison folklore, derived-topology closeout reads the narrower derived-surface snapshot lane instead of rebuilding naming attachments, current-head hostile baselines read the retained materialized surface directly, and certification historical truth now requires retained diagnostics and equivalence rows instead of rebuilding them locally once read-basis-ready declaration seeding is available",
        Some("src/certification/support/historical_query_snapshot/mod.rs"),
        [
            "src/projection/runtime_boundary/declared_query_surfaces/mod.rs",
            "src/projection/runtime_boundary/declared_query_surfaces/retained_artifacts.rs",
            "src/certification/support/historical_query_snapshot/mod.rs",
            "src/certification/support/historical_query_snapshot/derived_snapshot.rs",
            "src/certification/support/historical_query_snapshot/full_snapshot.rs",
            "src/certification/support/read_basis_query_runtime.rs",
            "src/certification/support/current_head_materialized_topology.rs",
            "src/projection/runtime_boundary/declared_query_surfaces/derived_surfaces/mod.rs",
            "src/projection/runtime_boundary/declared_query_surfaces/query_diagnostics/mod.rs",
            "src/projection/runtime_boundary/declared_query_surfaces/truth_surfaces/mod.rs",
            "src/projection/read_views/domain/mod.rs",
        ],
    )
}
