use crate::{
    RecoveryBlockedByIntegrityDamage, RecoveryBlockingIntegritySource,
    RecoveryIntegrityHandoffReceipt, S4IntegrityHandoffDenial, S4IntegrityHandoffDenialKind,
};
use forge_store_contracts::StableDigest;
use forge_store_physical_integrity::{
    PhysicalLocalityReport, QuarantineHandoffPosture, QuarantineRecord,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineSummary {
    locality: PhysicalLocalityReport,
    handoff_posture: QuarantineHandoffPosture,
    receipt: RecoveryIntegrityHandoffReceipt,
}

impl QuarantineSummary {
    pub fn from_quarantine_record(
        record: &QuarantineRecord,
        receipt: RecoveryIntegrityHandoffReceipt,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        receipt.require_quarantine_record_basis(record)?;
        Ok(Self {
            locality: record.locality(),
            handoff_posture: record.handoff_posture(),
            receipt,
        })
    }

    pub const fn locality(&self) -> PhysicalLocalityReport {
        self.locality
    }

    pub const fn handoff_posture(&self) -> QuarantineHandoffPosture {
        self.handoff_posture
    }

    pub const fn receipt(&self) -> &RecoveryIntegrityHandoffReceipt {
        &self.receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IntegrityDamageMap {
    wal_damage: Vec<RecoveryBlockedByIntegrityDamage>,
    checkpoint_damage: Vec<RecoveryBlockedByIntegrityDamage>,
    manifest_root_damage: Vec<RecoveryBlockedByIntegrityDamage>,
    unresolved_authority_damage: Vec<RecoveryBlockedByIntegrityDamage>,
    recovery_blocking_findings: Vec<RecoveryBlockedByIntegrityDamage>,
    quarantine_summaries: Vec<QuarantineSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryBlockingDamageCase {
    ChecksumMismatch,
    AuthenticityFailure,
    MissingChunk,
    StaleGeneration,
    CrossScopeImport,
}

pub fn classify_recovery_blocking_damage(
    source: RecoveryBlockingIntegritySource,
    damage: &RecoveryBlockedByIntegrityDamage,
) -> RecoveryBlockingDamageCase {
    match source {
        RecoveryBlockingIntegritySource::UnresolvedAuthorityDamage => {
            RecoveryBlockingDamageCase::CrossScopeImport
        }
        RecoveryBlockingIntegritySource::ManifestRoot => RecoveryBlockingDamageCase::StaleGeneration,
        RecoveryBlockingIntegritySource::CheckpointAdjacentRecord => {
            RecoveryBlockingDamageCase::AuthenticityFailure
        }
        RecoveryBlockingIntegritySource::WalFrame => match damage.wal_kind() {
            Some(forge_store_physical_integrity::WalFrameDamageDenialKind::ChecksumFailure) => {
                RecoveryBlockingDamageCase::ChecksumMismatch
            }
            Some(forge_store_physical_integrity::WalFrameDamageDenialKind::TornWalFrame)
            | Some(forge_store_physical_integrity::WalFrameDamageDenialKind::MismatchedLength) => {
                RecoveryBlockingDamageCase::MissingChunk
            }
            _ => RecoveryBlockingDamageCase::ChecksumMismatch,
        },
    }
}

impl IntegrityDamageMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_wal_damage(
        mut self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        require_source(&damage, RecoveryBlockingIntegritySource::WalFrame)?;
        self.recovery_blocking_findings.push(damage.clone());
        self.wal_damage.push(damage);
        Ok(self)
    }

    pub fn with_checkpoint_damage(
        mut self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        require_source(
            &damage,
            RecoveryBlockingIntegritySource::CheckpointAdjacentRecord,
        )?;
        self.recovery_blocking_findings.push(damage.clone());
        self.checkpoint_damage.push(damage);
        Ok(self)
    }

    pub fn with_manifest_root_damage(
        mut self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        require_source(&damage, RecoveryBlockingIntegritySource::ManifestRoot)?;
        self.recovery_blocking_findings.push(damage.clone());
        self.manifest_root_damage.push(damage);
        Ok(self)
    }

    pub fn with_unresolved_authority_damage(
        mut self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        require_source(
            &damage,
            RecoveryBlockingIntegritySource::UnresolvedAuthorityDamage,
        )?;
        self.recovery_blocking_findings.push(damage.clone());
        self.unresolved_authority_damage.push(damage);
        Ok(self)
    }

    pub fn with_quarantine_summary(mut self, summary: QuarantineSummary) -> Self {
        self.quarantine_summaries.push(summary);
        self
    }

    pub fn basis(&self) -> StableDigest {
        StableDigest::new(format!(
            "s4-damage-map:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
            self.wal_damage,
            self.checkpoint_damage,
            self.manifest_root_damage,
            self.unresolved_authority_damage,
            self.recovery_blocking_findings,
            self.quarantine_summaries
        ))
        .expect("S.4 damage map digest basis is non-empty")
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
}

fn require_source(
    damage: &RecoveryBlockedByIntegrityDamage,
    expected: RecoveryBlockingIntegritySource,
) -> Result<(), S4IntegrityHandoffDenial> {
    if damage.source() == expected {
        Ok(())
    } else {
        Err(S4IntegrityHandoffDenial::new(
            S4IntegrityHandoffDenialKind::DamageMapSourceMismatch,
        ))
    }
}
