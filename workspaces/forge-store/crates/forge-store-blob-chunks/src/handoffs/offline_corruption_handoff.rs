use forge_store_contracts::CorruptionHandoffDamageCase;

use crate::{BlobCorruptionDenial, BlobDamageCase};

pub const fn map_handoff_damage_case_to_blob(case: CorruptionHandoffDamageCase) -> BlobDamageCase {
    match case {
        CorruptionHandoffDamageCase::ChecksumMismatch => BlobDamageCase::ChecksumMismatch,
        CorruptionHandoffDamageCase::AuthenticityFailure => BlobDamageCase::AuthenticityFailure,
        CorruptionHandoffDamageCase::MissingChunk => BlobDamageCase::MissingChunk,
        CorruptionHandoffDamageCase::StaleGeneration => BlobDamageCase::StaleGeneration,
        CorruptionHandoffDamageCase::CrossScopeImport => BlobDamageCase::CrossScopeImport,
    }
}

/// Offline corruption handoff preserves the classified case through blob rejection.
pub const fn reject_offline_handoff_as_blob_authority(
    handoff_case: CorruptionHandoffDamageCase,
) -> BlobCorruptionDenial {
    BlobCorruptionDenial::OfflineObservationRejected {
        damage_case: map_handoff_damage_case_to_blob(handoff_case),
    }
}
