use crate::{
    CheckpointCoveredLsnRange, CheckpointCutoverReceipt, CheckpointId, CheckpointLocator,
    CheckpointRecoveryCounterSnapshot, CheckpointValidation, IntegrityDamageMap,
};

use super::{RecoveryLayoutAccessDenial, RecoveryLayoutAccessDenialKind};

pub fn ensure_recovery_entry_allowed(
    damage_map: &IntegrityDamageMap,
) -> Result<(), RecoveryLayoutAccessDenial> {
    if damage_map.recovery_blocking_findings().is_empty() {
        Ok(())
    } else {
        Err(RecoveryLayoutAccessDenial::new(
            RecoveryLayoutAccessDenialKind::RecoveryBlockedByIntegrityDamage,
        ))
    }
}

pub fn reject_locator_projection(
    _locator: &CheckpointLocator,
) -> Result<(), RecoveryLayoutAccessDenial> {
    Err(RecoveryLayoutAccessDenial::new(
        RecoveryLayoutAccessDenialKind::LocatorProjectionCannotStandInForCheckpointAuthority,
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointRecoveryManifestLayoutReport {
    checkpoint_id: CheckpointId,
    covered_lsn_range: CheckpointCoveredLsnRange,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointRecoveryManifestLayoutReport {
    pub fn from_validation(validation: &CheckpointValidation) -> Self {
        Self {
            checkpoint_id: validation.checkpoint_id().clone(),
            covered_lsn_range: validation.manifest().covered_lsn_range(),
            counters: validation.counters(),
        }
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn covered_lsn_range(&self) -> CheckpointCoveredLsnRange {
        self.covered_lsn_range
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointCutoverLayoutReport {
    checkpoint_id: CheckpointId,
    covered_lsn_range: CheckpointCoveredLsnRange,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointCutoverLayoutReport {
    pub fn from_receipt(receipt: &CheckpointCutoverReceipt) -> Self {
        Self {
            checkpoint_id: receipt.checkpoint_id().clone(),
            covered_lsn_range: receipt.covered_lsn_range(),
            counters: receipt.counters(),
        }
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn covered_lsn_range(&self) -> CheckpointCoveredLsnRange {
        self.covered_lsn_range
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}
