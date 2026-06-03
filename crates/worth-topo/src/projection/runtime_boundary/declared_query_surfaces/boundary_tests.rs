#[test]
fn declared_query_surfaces_mod_does_not_inline_snapshot_row_fallback_logic() {
    let source = include_str!("mod.rs");
    let projection_mod = include_str!("../../mod.rs");
    let diagnostic_surfaces_mod = include_str!("../../diagnostic_surfaces/mod.rs");
    let certification_snapshot_support =
        include_str!("../../../certification/support/historical_query_snapshot/mod.rs");
    let certification_snapshot_derived_support = include_str!(
        "../../../certification/support/historical_query_snapshot/derived_snapshot.rs"
    );
    let certification_snapshot_full_support =
        include_str!("../../../certification/support/historical_query_snapshot/full_snapshot.rs");
    let certification_read_basis_query_runtime =
        include_str!("../../../certification/support/read_basis_query_runtime.rs");
    let declared_computed_views = include_str!("derived_surfaces/computed_views.rs");
    let declared_query_diagnostics = include_str!("query_diagnostics/mod.rs");
    let retained_artifacts = include_str!("retained_artifacts.rs");

    assert!(
        !source.contains("workspace.read(&self.entities)"),
        "query surfaces entry surface should not inline entity-row snapshot reads",
    );
    assert!(
        !source.contains("workspace.materialize(&self.materialized)"),
        "query surfaces entry surface should not inline materialized-row snapshot reads",
    );
    assert!(
        !source.contains("naming_attachment_report_from_query"),
        "query surfaces entry surface should not own naming attachment row decoding",
    );
    assert!(
        !source.contains("snapshot_for_read_basis"),
        "query surfaces entry surface should not expose historical snapshot reconstruction from production declared surfaces",
    );
    assert!(
        !source.contains("TopologyDerivedQuerySnapshot"),
        "query surfaces entry surface should not retain a production historical snapshot artifact type",
    );
    assert!(
        !source.contains("historical_snapshot"),
        "query surfaces entry surface should not keep a production historical snapshot child boundary",
    );
    assert!(
        !source.contains("use crate::projection::{"),
        "query surfaces entry surface should depend on explicit internal seams instead of a broad projection bucket",
    );
    for required in [
        "historical_query_snapshot_for_read_basis",
        "historical_derived_surface_snapshot_for_read_basis",
    ] {
        assert!(
            certification_snapshot_support.contains(required),
            "certification historical snapshot support should own retained-truth reconstruction seam `{required}`",
        );
    }
    for required in [
        "TopologyHistoricalTruthArtifact",
        "TopologyHistoricalDerivedSurfaceSnapshot",
        "materialize_topology_historical_truth_artifact(",
        "materialize_topology_historical_derived_surface_snapshot(",
        "materialize_declared_query_surface_binding(",
        "verify_scalar_alignment(",
        "decode_row_pair(",
        "decode_row_triple(",
        "ForgeQueryDerivedArtifactBinding",
        "topology.historical.truth",
        "topology.historical.derived_snapshot",
        ".decode_row_triple(",
        ".decode_row_pair(",
    ] {
        assert!(
            retained_artifacts.contains(required),
            "declared-query-surfaces retained-artifact seam should own `{required}`",
        );
    }
    for required in [
        "struct HistoricalReadBasisQueryRuntime",
        "fn query_surface_evidence(",
        "declare_topology_query_surfaces",
        "fn historical_derived_surface_snapshot(",
        "fn historical_equivalence_read_basis_facts(",
    ] {
        assert!(
            certification_read_basis_query_runtime.contains(required),
            "certification read-basis query runtime seam should own `{required}`",
        );
    }
    assert!(
        certification_snapshot_derived_support.contains("runtime.historical_derived_surface_snapshot()"),
        "certification derived-snapshot seam should consume the shared runtime-owned retained derived-surface snapshot boundary",
    );
    assert!(
        certification_snapshot_derived_support
            .contains("runtime.historical_equivalence_read_basis_facts()"),
        "certification derived-snapshot seam should consume Query-owned retained scalar read-basis evidence instead of decoding struct fields directly",
    );
    assert!(
        !certification_snapshot_derived_support.contains("materialize_derived_artifact_bundle("),
        "certification derived-snapshot seam should not reopen retained artifact materialization locally",
    );
    for required in ["historical_naming_attachments", "persistent_names"] {
        assert!(
            certification_snapshot_full_support.contains(required),
            "full historical snapshot seam should own `{required}`",
        );
    }
    assert!(
        certification_snapshot_full_support.contains("read_declared_query_surface_binding("),
        "full historical snapshot seam should consume one Query-owned live artifact binding for naming attachments",
    );
    assert!(
        !certification_snapshot_full_support.contains("workspace.read(surfaces.entities())"),
        "full historical snapshot seam should not reopen entity rows through direct workspace.read once Query owns the live artifact pack",
    );
    assert!(
        !certification_snapshot_full_support.contains("workspace.read(surfaces.persistent_names())"),
        "full historical snapshot seam should not reopen persistent-name rows through direct workspace.read once Query owns the live artifact pack",
    );
    assert!(
        source.contains("materialize_derived_artifact_binding("),
        "query surfaces entry surface should expose one explicit Query-owned retained-artifact binding lane for declared derived rows",
    );
    assert!(
        source.contains("read_live_artifact_binding("),
        "query surfaces entry surface should expose one explicit Query-owned live-artifact binding lane for declared live rows",
    );
    assert!(
        !retained_artifacts.contains("ForgeQueryDerivedMaterializationBundle"),
        "declared-query-surfaces retained-artifact seam should consume the stronger Query-owned artifact binding type instead of treating the naked bundle as the final artifact",
    );
    assert!(
        !retained_artifacts.contains("equivalence_contract_from_diagnostics_rows"),
        "declared-query-surfaces retained-artifact seam should compare diagnostics and equivalence rows through Query-owned retained scalar evidence instead of rebuilding diagnostics-carried equivalence locally",
    );
    assert!(
        source.contains("materialize_intent(view)"),
        "query surfaces entry surface should own one explicit materialization-intent lane for declared derived rows",
    );
    assert!(
        source.contains(".decode_single_row()"),
        "query surfaces entry surface should decode declared derived rows through the Query-owned materialization floor",
    );
    assert!(
        declared_computed_views.contains(
            "decode_single_computed_row(materialized_view_name)"
        ),
        "interpreted and validation maintainers should decode retained upstream rows through the Query-owned upstream-input seam",
    );
    assert!(
        declared_computed_views.contains("interpreted_topology_from_upstreams(")
            && declared_computed_views.contains("validation_report_from_upstreams("),
        "interpreted and validation maintainers should route through explicit retained-upstream helpers instead of rebuilding synthetic bags inline",
    );
    assert!(
        declared_query_diagnostics.contains("decode_single_computed_row(materialized_view_name)")
            && declared_query_diagnostics.contains("decode_single_computed_row(interpreted_view_name)")
            && declared_query_diagnostics.contains("decode_single_computed_row(validation_view_name)"),
        "diagnostics maintainers should decode retained upstream rows through the Query-owned upstream-input seam",
    );
    assert!(
        certification_snapshot_derived_support.contains("HistoricalReadBasisQueryRuntime"),
        "derived historical snapshot seam should consume the shared read-basis query runtime boundary",
    );
    assert!(
        certification_snapshot_derived_support.contains("runtime.historical_derived_surface_snapshot()"),
        "derived historical snapshot seam should consume one retained derived-surface snapshot from the shared query-runtime boundary",
    );
    assert!(
        certification_snapshot_full_support.contains("HistoricalReadBasisQueryRuntime"),
        "full historical snapshot seam should consume the shared read-basis query runtime boundary",
    );
    for forbidden in [
        "equivalence.authority_snapshot_id",
        "equivalence.authority_branch_id",
        "equivalence.truth_basis_digest_hex",
        "equivalence.touched_aspect_count",
    ] {
        assert!(
            !certification_snapshot_support.contains(forbidden),
            "historical snapshot read-basis proof should not spelunk decoded equivalence fields via `{forbidden}` once Query owns retained scalar evidence",
        );
    }
    for required in [
        "mod derived_surfaces;",
        "mod query_diagnostics;",
        "mod truth_surfaces;",
    ] {
        assert!(
            source.contains(required),
            "query surfaces entry surface should own internal declaration helper seam `{required}` directly",
        );
    }
    for forbidden in [
        "mod historical_snapshot;",
        "mod historical_rows;",
        "mod snapshot_decode;",
    ] {
        assert!(
            !source.contains(forbidden),
            "query surfaces entry surface should not keep historical reconstruction seam `{forbidden}` in production declared surfaces",
        );
    }
    for forbidden in [
        "pub(crate) mod truth_surfaces;",
        "pub(crate) mod derived_surfaces;",
    ] {
        assert!(
            !projection_mod.contains(forbidden),
            "projection root should not keep displaced internal declaration bucket `{forbidden}` after runtime-boundary rehome",
        );
    }
    assert!(
        !diagnostic_surfaces_mod.contains("pub(crate) mod query_diagnostics;"),
        "diagnostic surfaces should not keep the internal query-diagnostics declaration lane after runtime-boundary rehome",
    );
}
