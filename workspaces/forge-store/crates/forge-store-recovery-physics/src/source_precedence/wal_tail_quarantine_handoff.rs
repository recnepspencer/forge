use crate::{
    RecoveryIntegrityHandoffReceipt, S4IntegrityHandoffDenial, S4IntegrityHandoffDenialKind,
};
use forge_store_physical_integrity::{
    QuarantineRecord, WalFrameDamageDenial, WalFrameIntegrityCounters,
    WalFrameIntegrityInputIdentity, WalTailIntegrityPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalTailIntegrityQuarantineHandoff {
    input_identity: WalFrameIntegrityInputIdentity,
    tail_posture: WalTailIntegrityPosture,
    counters: WalFrameIntegrityCounters,
    receipt: RecoveryIntegrityHandoffReceipt,
}

impl WalTailIntegrityQuarantineHandoff {
    pub fn from_wal_tail_damage_quarantine(
        denial: &WalFrameDamageDenial,
        record: &QuarantineRecord,
        receipt: RecoveryIntegrityHandoffReceipt,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        receipt.require_quarantine_record_basis(record)?;
        let basis = denial.basis().ok_or_else(|| {
            S4IntegrityHandoffDenial::new(S4IntegrityHandoffDenialKind::ReceiptBasisMismatch)
        })?;
        Ok(Self {
            input_identity: WalFrameIntegrityInputIdentity::from_wal_damage_basis(basis),
            tail_posture: denial.tail_posture(),
            counters: denial.counters(),
            receipt,
        })
    }

    pub const fn input_identity(&self) -> WalFrameIntegrityInputIdentity {
        self.input_identity
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }

    pub const fn counters(&self) -> WalFrameIntegrityCounters {
        self.counters
    }

    pub const fn receipt(&self) -> &RecoveryIntegrityHandoffReceipt {
        &self.receipt
    }
}
