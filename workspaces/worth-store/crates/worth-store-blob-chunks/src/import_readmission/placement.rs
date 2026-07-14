use worth_store_operations_vocabulary::{ImportPlacementPlan, ImportPlacementSource};

use super::counters::BlobImportReadmissionCounters;
use super::denial::BlobImportReadmissionDenial;

pub(crate) fn plan_placement_admission(
    source: ImportPlacementSource,
    declared_chunks: u64,
    local_chunks: u64,
    counters: BlobImportReadmissionCounters,
) -> Result<ImportPlacementPlan, BlobImportReadmissionDenial> {
    if declared_chunks == 0 {
        return Err(BlobImportReadmissionDenial::PlacementOnlyEvidenceRejected {
            counters: counters.record_placement_only_denial(),
        });
    }
    Ok(match source {
        ImportPlacementSource::ScopeDenied => {
            ImportPlacementPlan::scope_denied(source, declared_chunks, local_chunks)
        }
        ImportPlacementSource::ColdUnavailable => {
            ImportPlacementPlan::requires_fetch(source, declared_chunks, local_chunks)
        }
        ImportPlacementSource::ExternalByReference if local_chunks == declared_chunks => {
            ImportPlacementPlan::deduped_locally(source, declared_chunks)
        }
        ImportPlacementSource::InlineInBundle if local_chunks == declared_chunks => {
            ImportPlacementPlan::already_present_locally(source, declared_chunks)
        }
        _ => ImportPlacementPlan::requires_fetch(source, declared_chunks, local_chunks),
    })
}
