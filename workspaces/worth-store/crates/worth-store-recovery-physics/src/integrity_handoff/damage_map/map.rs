use worth_store_contracts::{CorruptionHandoffDamageCase, StableDigest};
use worth_store_physical_integrity::QuarantineRecord;

use crate::{
    IntegrityHandoffDenial, IntegrityHandoffDenialKind, RecoveryBlockedByIntegrityDamage,
    RecoveryBlockingIntegritySource, RecoveryIntegrityHandoffReceipt,
};

use super::{classify_recovery_blocking_damage, QuarantineSummary};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IntegrityDamageMap {
    wal_damage: Vec<RecoveryBlockedByIntegrityDamage>,
    checkpoint_damage: Vec<RecoveryBlockedByIntegrityDamage>,
    manifest_root_damage: Vec<RecoveryBlockedByIntegrityDamage>,
    unresolved_authority_damage: Vec<RecoveryBlockedByIntegrityDamage>,
    recovery_blocking_findings: Vec<RecoveryBlockedByIntegrityDamage>,
    quarantine_summaries: Vec<QuarantineSummary>,
}

impl IntegrityDamageMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_wal_damage(
        self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, IntegrityHandoffDenial> {
        self.with_damage(
            damage,
            RecoveryBlockingIntegritySource::WalFrame,
            DamageLane::Wal,
        )
    }

    pub fn with_checkpoint_damage(
        self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, IntegrityHandoffDenial> {
        self.with_damage(
            damage,
            RecoveryBlockingIntegritySource::CheckpointAdjacentRecord,
            DamageLane::Checkpoint,
        )
    }

    pub fn with_manifest_root_damage(
        self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, IntegrityHandoffDenial> {
        self.with_damage(
            damage,
            RecoveryBlockingIntegritySource::ManifestRoot,
            DamageLane::ManifestRoot,
        )
    }

    pub fn with_unresolved_authority_damage(
        self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, IntegrityHandoffDenial> {
        self.with_damage(
            damage,
            RecoveryBlockingIntegritySource::UnresolvedAuthorityDamage,
            DamageLane::UnresolvedAuthority,
        )
    }

    pub fn with_recovery_blocking_quarantine(
        mut self,
        record: &QuarantineRecord,
        receipt: RecoveryIntegrityHandoffReceipt,
        damage: &RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, IntegrityHandoffDenial> {
        self.quarantine_summaries
            .push(QuarantineSummary::from_recovery_blocking_damage(
                record, receipt, damage,
            )?);
        Ok(self)
    }

    pub fn basis(&self) -> StableDigest {
        StableDigest::new(format!(
            "recovery.integrity.damage-map:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
            self.wal_damage,
            self.checkpoint_damage,
            self.manifest_root_damage,
            self.unresolved_authority_damage,
            self.recovery_blocking_findings,
            self.quarantine_summaries
        ))
        .expect("damage map digest basis is non-empty")
    }

    pub fn wal_damage(&self) -> &[RecoveryBlockedByIntegrityDamage] {
        &self.wal_damage
    }
    pub fn checkpoint_damage(&self) -> &[RecoveryBlockedByIntegrityDamage] {
        &self.checkpoint_damage
    }
    pub fn manifest_root_damage(&self) -> &[RecoveryBlockedByIntegrityDamage] {
        &self.manifest_root_damage
    }
    pub fn unresolved_authority_damage(&self) -> &[RecoveryBlockedByIntegrityDamage] {
        &self.unresolved_authority_damage
    }
    pub fn recovery_blocking_findings(&self) -> &[RecoveryBlockedByIntegrityDamage] {
        &self.recovery_blocking_findings
    }
    pub fn quarantine_summaries(&self) -> &[QuarantineSummary] {
        &self.quarantine_summaries
    }

    pub fn recovery_blocking_damage_cases(&self) -> Vec<CorruptionHandoffDamageCase> {
        self.recovery_blocking_findings
            .iter()
            .map(|damage| classify_recovery_blocking_damage(damage.source(), damage))
            .collect()
    }

    fn with_damage(
        mut self,
        damage: RecoveryBlockedByIntegrityDamage,
        expected: RecoveryBlockingIntegritySource,
        lane: DamageLane,
    ) -> Result<Self, IntegrityHandoffDenial> {
        require_source(&damage, expected)?;
        self.recovery_blocking_findings.push(damage.clone());
        match lane {
            DamageLane::Wal => self.wal_damage.push(damage),
            DamageLane::Checkpoint => self.checkpoint_damage.push(damage),
            DamageLane::ManifestRoot => self.manifest_root_damage.push(damage),
            DamageLane::UnresolvedAuthority => self.unresolved_authority_damage.push(damage),
        }
        Ok(self)
    }
}

enum DamageLane {
    Wal,
    Checkpoint,
    ManifestRoot,
    UnresolvedAuthority,
}

fn require_source(
    damage: &RecoveryBlockedByIntegrityDamage,
    expected: RecoveryBlockingIntegritySource,
) -> Result<(), IntegrityHandoffDenial> {
    (damage.source() == expected).then_some(()).ok_or_else(|| {
        IntegrityHandoffDenial::new(IntegrityHandoffDenialKind::DamageMapSourceMismatch)
    })
}
