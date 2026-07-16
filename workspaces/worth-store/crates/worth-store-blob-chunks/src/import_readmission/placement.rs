use super::counters::BlobImportReadmissionCounters;
use super::denial::BlobImportReadmissionDenial;
use super::{BlobImportPlacementPlan, BlobImportPlacementSource};

pub(crate) fn plan_placement_admission(
    source: BlobImportPlacementSource,
    declared_chunks: u64,
    local_chunks: u64,
    counters: BlobImportReadmissionCounters,
) -> Result<BlobImportPlacementPlan, BlobImportReadmissionDenial> {
    if declared_chunks == 0 {
        return Err(BlobImportReadmissionDenial::PlacementOnlyEvidenceRejected {
            counters: counters.record_placement_only_denial(),
        });
    }
    Ok(match source {
        BlobImportPlacementSource::ScopeDenied => {
            BlobImportPlacementPlan::scope_denied(source, declared_chunks, local_chunks)
        }
        BlobImportPlacementSource::ColdUnavailable => {
            BlobImportPlacementPlan::requires_fetch(source, declared_chunks, local_chunks)
        }
        BlobImportPlacementSource::ExternalByReference if local_chunks == declared_chunks => {
            BlobImportPlacementPlan::deduped_locally(source, declared_chunks)
        }
        BlobImportPlacementSource::InlineInBundle if local_chunks == declared_chunks => {
            BlobImportPlacementPlan::already_present_locally(source, declared_chunks)
        }
        _ => BlobImportPlacementPlan::requires_fetch(source, declared_chunks, local_chunks),
    })
}
