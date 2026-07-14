use worth_store_contracts::CorruptionHandoffDamageCase;
use worth_store_physical_integrity::WalFrameDamageDenialKind;

use crate::{RecoveryBlockedByIntegrityDamage, RecoveryBlockingIntegritySource};

pub fn classify_recovery_blocking_damage(
    source: RecoveryBlockingIntegritySource,
    damage: &RecoveryBlockedByIntegrityDamage,
) -> CorruptionHandoffDamageCase {
    match source {
        RecoveryBlockingIntegritySource::UnresolvedAuthorityDamage => {
            CorruptionHandoffDamageCase::CrossScopeImport
        }
        RecoveryBlockingIntegritySource::ManifestRoot => {
            CorruptionHandoffDamageCase::StaleGeneration
        }
        RecoveryBlockingIntegritySource::CheckpointAdjacentRecord => {
            CorruptionHandoffDamageCase::AuthenticityFailure
        }
        RecoveryBlockingIntegritySource::WalFrame => match damage.wal_kind() {
            Some(WalFrameDamageDenialKind::ChecksumFailure) => {
                CorruptionHandoffDamageCase::ChecksumMismatch
            }
            Some(WalFrameDamageDenialKind::TornWalFrame)
            | Some(WalFrameDamageDenialKind::MismatchedLength) => {
                CorruptionHandoffDamageCase::MissingChunk
            }
            _ => CorruptionHandoffDamageCase::ChecksumMismatch,
        },
    }
}
