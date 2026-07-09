use crate::corruption::receipt_construction::construct_quarantine_receipt;
use crate::corruption::types::{BlobCorruptionDetectionSource, BlobQuarantineLifecycleState};
use crate::{BlobChunkQuarantine, BlobCorruptedChunkLocalization, BlobQuarantineAuthority};

pub fn seal_quarantine_from_localization(
    localization: BlobCorruptedChunkLocalization,
    authority: BlobQuarantineAuthority,
) -> BlobChunkQuarantine {
    let _current_authority = authority.into_current_authority();
    let state = classify_initial_quarantine_state(localization.source());
    construct_quarantine_receipt(localization, state)
}

pub fn seal(
    localization: BlobCorruptedChunkLocalization,
    authority: BlobQuarantineAuthority,
) -> BlobChunkQuarantine {
    seal_quarantine_from_localization(localization, authority)
}

const fn classify_initial_quarantine_state(
    source: BlobCorruptionDetectionSource,
) -> BlobQuarantineLifecycleState {
    match source {
        BlobCorruptionDetectionSource::ColdFetch => {
            BlobQuarantineLifecycleState::ColdUnavailableCorrupt
        }
        BlobCorruptionDetectionSource::ImportReadmission => {
            BlobQuarantineLifecycleState::ImportCorrupt
        }
        _ => BlobQuarantineLifecycleState::Quarantined,
    }
}
