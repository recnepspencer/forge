use crate::{
    CheckpointCoveredLsnRange, CheckpointCutoverReceipt, CheckpointId, CheckpointLocator,
    CheckpointRecoveryCounterSnapshot, CheckpointValidation, IntegrityDamageMap,
};

use super::{RecoveryLayoutAccessDenial, RecoveryLayoutAccessDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedRecoveryManifestLayoutRule {
    _private: (),
}

impl AdmittedRecoveryManifestLayoutRule {
    pub(crate) const fn internal_phase21() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase21-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase21() -> Self {
        Self::internal_phase21()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointCutoverLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointCutoverLayoutAdmission {
    _private: (),
}

impl CheckpointCutoverLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        self,
        _rule: &AdmittedRecoveryManifestLayoutRule,
    ) -> Result<CheckpointCutoverLayoutAdmission, RecoveryLayoutAccessDenial> {
        Ok(CheckpointCutoverLayoutAdmission { _private: () })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedCheckpointCutoverLayoutFamily {
    _admission: CheckpointCutoverLayoutAdmission,
}

impl AdmittedCheckpointCutoverLayoutFamily {
    pub(crate) const fn new(admission: CheckpointCutoverLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn validated_checkpoint(
        &self,
        validation: &CheckpointValidation,
    ) -> CheckpointRecoveryManifestLayoutReport {
        CheckpointRecoveryManifestLayoutReport::from_validation(validation)
    }

    pub fn published_cutover(
        &self,
        receipt: &CheckpointCutoverReceipt,
    ) -> CheckpointCutoverLayoutReport {
        CheckpointCutoverLayoutReport::from_receipt(receipt)
    }

    pub fn ensure_recovery_entry_allowed(
        &self,
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
        &self,
        _locator: &CheckpointLocator,
    ) -> Result<(), RecoveryLayoutAccessDenial> {
        Err(RecoveryLayoutAccessDenial::new(
            RecoveryLayoutAccessDenialKind::LocatorProjectionCannotStandInForCheckpointAuthority,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointRecoveryManifestLayoutReport {
    checkpoint_id: CheckpointId,
    covered_lsn_range: CheckpointCoveredLsnRange,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointRecoveryManifestLayoutReport {
    fn from_validation(validation: &CheckpointValidation) -> Self {
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
    fn from_receipt(receipt: &CheckpointCutoverReceipt) -> Self {
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
