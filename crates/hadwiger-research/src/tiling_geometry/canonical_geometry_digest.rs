use crate::domain_artifacts::digest_basis::HadwigerArtifactPayloadEntry;

use super::rectangular_regions::RectangularTileRegion;

pub(crate) fn cell_payload(
    cell_id: &str,
    tiles: &[RectangularTileRegion],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "WORTH.hadwiger.tiling_cell.v1"),
        HadwigerArtifactPayloadEntry::text("cell_id", cell_id),
        HadwigerArtifactPayloadEntry::unsigned("tile_count", tiles.len() as u128),
    ];
    for tile in tiles {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "tile",
            tile.stable_token(),
        ));
    }
    payload
}

pub(crate) fn report_payload(
    schema: &'static str,
    source_cell: &str,
    query_declaration_digest: Option<&str>,
    screening_evaluation_digest: Option<&str>,
    evidence: &str,
    counters: &super::contact_facts::TilingGeometryCounters,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", schema),
        HadwigerArtifactPayloadEntry::text("source_cell", source_cell),
        HadwigerArtifactPayloadEntry::text("evidence", evidence),
        HadwigerArtifactPayloadEntry::unsigned("tile_count", counters.tile_count() as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "boundary_rows_checked",
            counters.boundary_ownership_rows_checked() as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "contact_pairs_checked",
            counters.contact_pairs_checked() as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "query_declarations",
            counters.query_declarations_performed() as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "screening_evaluations",
            counters.screening_evaluations_performed() as u128,
        ),
    ];
    if let Some(digest) = query_declaration_digest {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "query_declaration_digest",
            digest,
        ));
    }
    if let Some(digest) = screening_evaluation_digest {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "screening_evaluation_digest",
            digest,
        ));
    }
    payload
}
