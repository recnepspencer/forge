use forge_store_contracts::CorruptionHandoffDamageCase;
use forge_store_physical_integrity::{
    PhysicalLocalityReport, QuarantineHandoffPosture, QuarantineRecord,
};

use crate::{
    IntegrityHandoffDenial, RecoveryBlockedByIntegrityDamage, RecoveryIntegrityHandoffReceipt,
};

use super::classify_recovery_blocking_damage;

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
    ) -> Result<Self, IntegrityHandoffDenial> {
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
