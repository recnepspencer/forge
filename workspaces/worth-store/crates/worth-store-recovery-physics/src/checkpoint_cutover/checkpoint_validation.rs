use crate::IntegrityDamageMap;

use super::{
    CheckpointId, CheckpointLocator, CheckpointManifest, CheckpointRecoveryCounterSnapshot,
    CheckpointValidationDenial, CheckpointValidationDenialKind, LocatedCheckpointCandidate,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointValidation {
    manifest: CheckpointManifest,
    locator: CheckpointLocator,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointValidation {
    pub fn validate_located_checkpoint(
        located: LocatedCheckpointCandidate,
        damage_map: &IntegrityDamageMap,
    ) -> Result<Self, CheckpointValidationDenial> {
        let counters = located
            .counters()
            .with_manifest_validation()
            .with_integrity_damage_check();
        if !damage_map.recovery_blocking_findings().is_empty() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::RecoveryBlockingIntegrityDamage,
                counters,
            ));
        }
        Ok(Self {
            manifest: located.candidate().manifest().clone(),
            locator: located.locator().clone(),
            counters,
        })
    }

    pub fn require_locator(
        candidate: super::CheckpointCandidate,
    ) -> Result<LocatedCheckpointCandidate, CheckpointValidationDenial> {
        Err(CheckpointValidationDenial::new(
            CheckpointValidationDenialKind::MissingCheckpointLocator,
            candidate.counters().with_locator_check(),
        ))
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        self.manifest.checkpoint_id()
    }

    pub fn manifest(&self) -> &CheckpointManifest {
        &self.manifest
    }

    pub fn locator(&self) -> &CheckpointLocator {
        &self.locator
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}
