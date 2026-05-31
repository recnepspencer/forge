use crate::orchestration_inventory::ForgeQueryOrchestrationSurfaceInventory;
use crate::public_doc_coverage::{
    forge_query_public_doc_coverage_golden_transcripts, ForgeQueryPublicDocCoverageInventory,
    ForgeQueryPublicGoldenTranscriptKind,
};

#[test]
fn current_coverage_tracks_every_live_orchestration_surface() {
    let surfaces = ForgeQueryOrchestrationSurfaceInventory::current();
    let coverage = ForgeQueryPublicDocCoverageInventory::current();

    assert_eq!(
        coverage.source_inventory_digest(),
        surfaces.inventory_digest()
    );
    assert_eq!(coverage.rows().len(), surfaces.rows().len());

    for row in surfaces.rows() {
        let coverage_row = coverage
            .row_for_public_name(row.public_name())
            .expect("every live surface should have coverage");
        assert_eq!(
            coverage_row.canonical_base_name(),
            row.canonical_base_name()
        );
        assert_eq!(coverage_row.orchestration_family(), row.family());
        assert_eq!(coverage_row.visibility(), row.visibility());
        assert_eq!(coverage_row.surface_row_digest(), row.row_digest());
        assert!(coverage_row.has_golden_transcript());
        assert!(coverage_row.has_readme_discovery());
        assert!(coverage_row.has_journey_coverage());
    }
}

#[test]
fn golden_manifest_is_referenced_by_live_coverage_rows() {
    let coverage = ForgeQueryPublicDocCoverageInventory::current();
    let expected = forge_query_public_doc_coverage_golden_transcripts()
        .iter()
        .filter(|row| row.kind() == ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage)
        .map(|row| row.label())
        .collect::<std::collections::BTreeSet<_>>();
    let actual = coverage
        .rows()
        .iter()
        .filter_map(|row| row.golden_transcript().map(|golden| golden.label()))
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(actual, expected);
}
