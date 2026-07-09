use crate::{
    DamageClassification, PhysicalLocalityReport, QuarantineHandoffPosture,
    QuarantineLifecyclePosture, QuarantineReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineRecord {
    locality: PhysicalLocalityReport,
    damage_classification: DamageClassification,
    receipt: QuarantineReceipt,
    lifecycle_posture: QuarantineLifecyclePosture,
    handoff_posture: QuarantineHandoffPosture,
}

impl QuarantineRecord {
    pub(crate) const fn new(
        locality: PhysicalLocalityReport,
        damage_classification: DamageClassification,
        receipt: QuarantineReceipt,
        lifecycle_posture: QuarantineLifecyclePosture,
        handoff_posture: QuarantineHandoffPosture,
    ) -> Self {
        Self {
            locality,
            damage_classification,
            receipt,
            lifecycle_posture,
            handoff_posture,
        }
    }

    pub const fn locality(&self) -> PhysicalLocalityReport {
        self.locality
    }

    pub const fn damage_classification(&self) -> &DamageClassification {
        &self.damage_classification
    }

    pub const fn receipt(&self) -> &QuarantineReceipt {
        &self.receipt
    }

    pub const fn lifecycle_posture(&self) -> QuarantineLifecyclePosture {
        self.lifecycle_posture
    }

    pub const fn handoff_posture(&self) -> QuarantineHandoffPosture {
        self.handoff_posture
    }

    pub const fn proves_repair(&self) -> bool {
        false
    }

    pub const fn proves_recovery(&self) -> bool {
        false
    }
}
