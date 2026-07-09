use worth_store_operations::BackupExportCustodyMode;

use crate::{BlobChunkRootPublication, BlobExportAuthority, BlobExportPublishedBundle};

use super::backend::{current_authority, export_readiness};
use super::chunk_sequence::{chunk_window_for_ordinal, GeneratedBlobSequence};
use super::lifecycle_execution::ExecutedBlobLane;

#[derive(Debug, Clone)]
struct StreamedExportChunkMetadata {
    digest: String,
    manifest_row: crate::BlobExportChunkManifestRow,
    offline_declaration: crate::BlobExportOfflineChunkDeclaration,
}

pub(super) fn publish_export_bundle(
    case: &str,
    lane: &ExecutedBlobLane,
    publication: &BlobChunkRootPublication,
    generated: &GeneratedBlobSequence,
) -> BlobExportPublishedBundle {
    let authority = BlobExportAuthority::from_current_store_authority(current_authority(
        case,
        "blob-harness-export",
    ));
    publish_streamed_export_bundle(
        &authority,
        &export_readiness(case),
        &format!("{case}-export"),
        lane,
        publication,
        generated,
    )
    .expect("bundle")
}

fn publish_streamed_export_bundle(
    authority: &BlobExportAuthority,
    custody: &worth_store_operations::BackupExportCustodyReadiness,
    export_name: &str,
    lane: &ExecutedBlobLane,
    publication: &BlobChunkRootPublication,
    generated: &GeneratedBlobSequence,
) -> Result<BlobExportPublishedBundle, crate::BlobExportBundleDenial> {
    verify_streamed_export_common(export_name, custody, lane, publication)?;
    let mut streamed = generated
        .sequence
        .proof_frontier()
        .ordered_leaves()
        .iter()
        .map(|leaf| {
            let (offset, bytes) = chunk_window_for_ordinal(generated, leaf.ordinal().get());
            let chunk = authority.collect_exported_chunk_bytes(
                leaf,
                crate::BlobChunkByteWindow::borrowed(offset, &bytes).expect("window"),
            )?;
            if chunk.leaf().security_metadata() != lane.lifecycle.declaration().security_metadata()
                || !publication
                    .canonical_basis()
                    .contains_chunk_identity(chunk.leaf().identity())
            {
                return Err(crate::BlobExportBundleDenial::ChunkEvidenceMismatch {
                    counters: crate::BlobExportBundleCounters::start(),
                });
            }
            Ok(StreamedExportChunkMetadata {
                digest: chunk.leaf().identity().chunk_digest().as_str().to_owned(),
                manifest_row: crate::BlobExportChunkManifestRow::from_collected_chunk(&chunk),
                offline_declaration: crate::BlobExportOfflineChunkDeclaration::from_collected_chunk(
                    &chunk,
                ),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    streamed.sort_by(|left, right| {
        left.manifest_row
            .ordinal()
            .get()
            .cmp(&right.manifest_row.ordinal().get())
            .then_with(|| {
                left.manifest_row
                    .range()
                    .start()
                    .cmp(&right.manifest_row.range().start())
            })
            .then_with(|| left.digest.cmp(&right.digest))
    });
    verify_streamed_exported_chunks(publication, lane, &streamed)?;
    let manifest_rows = streamed
        .iter()
        .map(|chunk| chunk.manifest_row.clone())
        .collect::<Vec<_>>();
    let offline_declarations = streamed
        .iter()
        .map(|chunk| chunk.offline_declaration.clone())
        .collect::<Vec<_>>();
    let counts = crate::export_bundle::BlobExportEvidenceCounts::new(
        manifest_rows.len() as u64,
        offline_declarations
            .iter()
            .map(crate::BlobExportOfflineChunkDeclaration::bytes)
            .sum(),
        manifest_rows.len() as u64,
        0,
    );
    let (canonical_export, export_digest) =
        crate::export_bundle::prepare_export_artifact(export_name, publication.canonical_basis())?;
    let counters = crate::BlobExportBundleCounters::start().with_evidence(counts);
    Ok(BlobExportPublishedBundle::new(
        lane.lifecycle.declaration().object_id().clone(),
        lane.lifecycle.declaration().generation(),
        publication.chunk_tree_root().clone(),
        lane.lifecycle.declaration().security_metadata(),
        crate::BlobExportManifest::new(export_name.to_owned(), manifest_rows),
        crate::BlobExportCustodyEvidence::new(custody.identity(), custody.mode()),
        crate::BlobExportDigestEvidence::new(
            lane.lifecycle
                .declaration()
                .logical_content_digest()
                .clone(),
            export_digest,
            &offline_declarations,
        ),
        offline_declarations,
        canonical_export,
        counters,
    ))
}

fn verify_streamed_export_common(
    export_name: &str,
    custody: &worth_store_operations::BackupExportCustodyReadiness,
    lane: &ExecutedBlobLane,
    publication: &BlobChunkRootPublication,
) -> Result<(), crate::BlobExportBundleDenial> {
    if export_name.trim().is_empty() {
        return Err(crate::BlobExportBundleDenial::EmptyExportName {
            counters: crate::BlobExportBundleCounters::start(),
        });
    }
    if lane.lifecycle.reachability() != &lane.reachability
        || !lane
            .reachability
            .matches_lifecycle_declaration(lane.lifecycle.declaration())
    {
        return Err(crate::BlobExportBundleDenial::StaleReachability {
            counters: crate::BlobExportBundleCounters::start().record_stale_reachability_denial(),
        });
    }
    if !lane.placement.matches_reachability(&lane.reachability) {
        return Err(crate::BlobExportBundleDenial::PlacementLifecycleMismatch {
            counters: crate::BlobExportBundleCounters::start(),
        });
    }
    match custody.mode() {
        Some(BackupExportCustodyMode::Export) | None => {}
        Some(BackupExportCustodyMode::Backup | BackupExportCustodyMode::PointInTimeRecovery) => {
            return Err(crate::BlobExportBundleDenial::CustodyNotExportReady {
                counters: crate::BlobExportBundleCounters::start(),
            });
        }
    }
    if publication.canonical_basis().total_bytes() == 0 {
        return Err(crate::BlobExportBundleDenial::MissingChunk {
            counters: crate::BlobExportBundleCounters::start().record_missing_chunk_denial(),
        });
    }
    Ok(())
}

fn verify_streamed_exported_chunks(
    publication: &BlobChunkRootPublication,
    lane: &ExecutedBlobLane,
    streamed: &[StreamedExportChunkMetadata],
) -> Result<(), crate::BlobExportBundleDenial> {
    let mut reachable: Vec<_> = lane
        .reachability
        .reachable_chunks()
        .iter()
        .map(|chunk| chunk.chunk_digest().as_str().to_owned())
        .collect();
    reachable.sort();
    let mut exported = streamed
        .iter()
        .map(|chunk| chunk.digest.clone())
        .collect::<Vec<_>>();
    exported.sort();
    let exported_bytes: u64 = streamed
        .iter()
        .map(|chunk| chunk.offline_declaration.bytes())
        .sum();
    if reachable != exported || exported_bytes != publication.canonical_basis().total_bytes() {
        return Err(crate::BlobExportBundleDenial::MissingChunk {
            counters: crate::BlobExportBundleCounters::start().record_missing_chunk_denial(),
        });
    }
    Ok(())
}
