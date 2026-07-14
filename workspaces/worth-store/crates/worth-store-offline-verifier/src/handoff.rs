use worth_store_contracts::CorruptionHandoffDamageCase;

use crate::{
    classify_offline_damage_case, OfflineBlobCorruptionClassification,
    OfflineBlobCorruptionObservation, OfflineBlobDamageCaseHint,
};

/// Offline corruption classification does not mint blob read authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineBlobAuthorityRejection {
    ObservedCorruptionDoesNotMintBlobAuthority {
        handoff_damage_case: CorruptionHandoffDamageCase,
    },
}

pub const fn map_offline_damage_hint_to_handoff(
    hint: OfflineBlobDamageCaseHint,
) -> CorruptionHandoffDamageCase {
    match hint {
        OfflineBlobDamageCaseHint::ChecksumMismatch => {
            CorruptionHandoffDamageCase::ChecksumMismatch
        }
        OfflineBlobDamageCaseHint::AuthenticityFailure => {
            CorruptionHandoffDamageCase::AuthenticityFailure
        }
        OfflineBlobDamageCaseHint::MissingChunk => CorruptionHandoffDamageCase::MissingChunk,
        OfflineBlobDamageCaseHint::StaleGeneration => CorruptionHandoffDamageCase::StaleGeneration,
        OfflineBlobDamageCaseHint::CrossScopeImport => {
            CorruptionHandoffDamageCase::CrossScopeImport
        }
    }
}

pub fn reject_offline_classification_as_blob_authority(
    classification: &OfflineBlobCorruptionClassification,
) -> OfflineBlobAuthorityRejection {
    OfflineBlobAuthorityRejection::ObservedCorruptionDoesNotMintBlobAuthority {
        handoff_damage_case: map_offline_damage_hint_to_handoff(classification.damage_case_hint()),
    }
}

pub fn reject_offline_observation_as_blob_authority(
    observation: &OfflineBlobCorruptionObservation,
) -> OfflineBlobAuthorityRejection {
    let damage_case_hint = classify_offline_damage_case(
        observation.evidence_kind(),
        observation.raw_declaration().tenant_scope(),
        observation.raw_declaration().key_scope(),
    );
    OfflineBlobAuthorityRejection::ObservedCorruptionDoesNotMintBlobAuthority {
        handoff_damage_case: map_offline_damage_hint_to_handoff(damage_case_hint),
    }
}
