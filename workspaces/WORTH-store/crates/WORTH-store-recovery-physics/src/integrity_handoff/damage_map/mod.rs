mod classify;

use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_contracts::{CorruptionHandoffDamageCase, StableDigest};
use worth_store_physical_integrity::{
    PhysicalLocalityReport, QuarantineHandoffPosture, QuarantineRecord,
};

use crate::{
    admit_recovery_corruption_readmission, RecoveryBlockedByIntegrityDamage,
    RecoveryBlockingIntegritySource, RecoveryCorruptionReadmissionDenial,
    RecoveryCorruptionReadmissionHandoff, RecoveryIntegrityHandoffReceipt, S4IntegrityHandoffDenial,
    S4IntegrityHandoffDenialKind,
};
use crate::corruption_readmission::build_recovery_readmission_handoff;

pub use classify::classify_recovery_blocking_damage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineSummary {
    locality: PhysicalLocalityReport,
    handoff_posture: QuarantineHandoffPosture,
    receipt: RecoveryIntegrityHandoffReceipt,
    damage_case: CorruptionHandoffDamageCase,
}

impl QuarantineSummary {
    pub fn from_recovery_blocking_damage(
        record: &QuarantineRecord,
        receipt: RecoveryIntegrityHandoffReceipt,
        damage: &RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        crate::verify_quarantine_handoff_for_readmission(record, &receipt)?;
        Ok(Self {
            locality: record.locality(),
            handoff_posture: record.handoff_posture(),
            receipt,
            damage_case: classify_recovery_blocking_damage(damage.source(), damage),
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

    pub const fn damage_case(&self) -> CorruptionHandoffDamageCase {
        self.damage_case
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

impl IntegrityDamageMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_wal_damage(
        mut self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        require_source(&damage, RecoveryBlockingIntegritySource::WalFrame)?;
        self.record_recovery_blocking_damage(&damage);
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
        self.record_recovery_blocking_damage(&damage);
        self.checkpoint_damage.push(damage);
        Ok(self)
    }

    pub fn with_manifest_root_damage(
        mut self,
        damage: RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        require_source(&damage, RecoveryBlockingIntegritySource::ManifestRoot)?;
        self.record_recovery_blocking_damage(&damage);
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
        self.record_recovery_blocking_damage(&damage);
        self.unresolved_authority_damage.push(damage);
        Ok(self)
    }

    pub fn with_recovery_blocking_quarantine(
        mut self,
        record: &QuarantineRecord,
        receipt: RecoveryIntegrityHandoffReceipt,
        damage: &RecoveryBlockedByIntegrityDamage,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        let summary = QuarantineSummary::from_recovery_blocking_damage(record, receipt, damage)?;
        self.quarantine_summaries.push(summary);
        Ok(self)
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

    pub fn recovery_blocking_damage_cases(&self) -> Vec<CorruptionHandoffDamageCase> {
        self.recovery_blocking_findings
            .iter()
            .map(classify_recovery_blocking_case)
            .collect()
    }

    pub(crate) fn build_corruption_readmission_handoffs(
        &self,
    ) -> Vec<RecoveryCorruptionReadmissionHandoff> {
        self.quarantine_summaries
            .iter()
            .map(build_recovery_readmission_handoff)
            .collect()
    }

    pub fn admit_corruption_readmission(
        &self,
        summary: &QuarantineSummary,
        record: &QuarantineRecord,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<RecoveryCorruptionReadmissionHandoff, RecoveryCorruptionReadmissionDenial> {
        admit_recovery_corruption_readmission(summary, record, current_store_authority)
    }

    fn record_recovery_blocking_damage(&mut self, damage: &RecoveryBlockedByIntegrityDamage) {
        self.recovery_blocking_findings.push(damage.clone());
    }
}

fn classify_recovery_blocking_case(
    damage: &RecoveryBlockedByIntegrityDamage,
) -> CorruptionHandoffDamageCase {
    classify_recovery_blocking_damage(damage.source(), damage)
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
