use super::classification::BlobExportCanonicalClassification;
use forge_store_operations_vocabulary::BackupExportCustodyMode;

use super::counters::BlobExportBundleCounters;
use super::denial::BlobExportBundleDenial;
use super::intent::BlobExportIntent;

pub(crate) fn verify_export_transition<'a>(
    intent: &BlobExportIntent<'a>,
) -> Result<VerifiedBlobExportTransition<'a>, BlobExportBundleDenial> {
    if intent.export_name().trim().is_empty() {
        return Err(BlobExportBundleDenial::EmptyExportName {
            counters: BlobExportBundleCounters::start(),
        });
    }
    if intent.lifecycle().reachability() != intent.reachability()
        || !intent
            .reachability()
            .matches_lifecycle_declaration(intent.lifecycle().declaration())
    {
        return Err(BlobExportBundleDenial::StaleReachability {
            counters: BlobExportBundleCounters::start().record_stale_reachability_denial(),
        });
    }
    if !intent
        .placement()
        .matches_reachability(intent.reachability())
    {
        return Err(BlobExportBundleDenial::PlacementLifecycleMismatch {
            counters: BlobExportBundleCounters::start(),
        });
    }
    match intent.custody().mode() {
        Some(BackupExportCustodyMode::Export) | None => {}
        Some(BackupExportCustodyMode::Backup | BackupExportCustodyMode::PointInTimeRecovery) => {
            return Err(BlobExportBundleDenial::CustodyNotExportReady {
                counters: BlobExportBundleCounters::start(),
            });
        }
    }
    let classification =
        BlobExportCanonicalClassification::from_exported_chunks(intent.exported_chunks());
    verify_exported_chunks(intent, &classification)?;
    Ok(VerifiedBlobExportTransition { classification })
}

pub(crate) struct VerifiedBlobExportTransition<'a> {
    classification: BlobExportCanonicalClassification<'a>,
}

impl<'a> VerifiedBlobExportTransition<'a> {
    pub(crate) fn classification(&self) -> &BlobExportCanonicalClassification<'a> {
        &self.classification
    }
}

fn verify_exported_chunks(
    intent: &BlobExportIntent<'_>,
    classification: &BlobExportCanonicalClassification<'_>,
) -> Result<(), BlobExportBundleDenial> {
    let mut reachable: Vec<_> = intent
        .reachability()
        .reachable_chunks()
        .iter()
        .map(|chunk| chunk.chunk_digest().as_str().to_owned())
        .collect();
    reachable.sort();

    let mut exported = Vec::new();
    for chunk in classification.exported_chunks() {
        if chunk.leaf().security_metadata() != intent.lifecycle().declaration().security_metadata()
            || !intent
                .publication()
                .canonical_basis()
                .contains_chunk_identity(chunk.leaf().identity())
        {
            return Err(BlobExportBundleDenial::ChunkEvidenceMismatch {
                counters: BlobExportBundleCounters::start(),
            });
        }
        exported.push(chunk.leaf().identity().chunk_digest().as_str().to_owned());
    }
    exported.sort();
    if reachable != exported
        || classification.counts().exported_bytes()
            != intent.publication().canonical_basis().total_bytes()
    {
        return Err(BlobExportBundleDenial::MissingChunk {
            counters: BlobExportBundleCounters::start().record_missing_chunk_denial(),
        });
    }
    Ok(())
}
