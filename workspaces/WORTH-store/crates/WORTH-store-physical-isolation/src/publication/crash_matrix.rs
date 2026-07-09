use super::{PhysicalPublicationDenial, PhysicalPublicationReceipt};
use worth_store_recovery_physics::{
    ExecutedS5PublicationRecoveryReceipt, S5PublicationCrashStage, S5RecoveredPublicationStructure,
    S5RecoveredPublicationStructureKind,
};

#[derive(Debug, Clone, Copy)]
pub struct PublicationCrashRecoveryOutcome {
    recovery_receipt: ExecutedS5PublicationRecoveryReceipt,
    recovered: S5RecoveredPublicationStructure,
    mixed_tree: bool,
}

impl PublicationCrashRecoveryOutcome {
    pub fn admit_recovery_receipt(
        receipt: &PhysicalPublicationReceipt,
        recovery_receipt: ExecutedS5PublicationRecoveryReceipt,
    ) -> Result<Self, PhysicalPublicationDenial> {
        let recovered = bind_recovery_receipt_to_publication(receipt, recovery_receipt)?;
        if recovered.kind() == S5RecoveredPublicationStructureKind::MixedOldAndNewStructure {
            return Err(PhysicalPublicationDenial::MixedTreeAfterCrash);
        }
        Ok(Self {
            recovery_receipt,
            recovered,
            mixed_tree: false,
        })
    }

    pub const fn reject_mixed_tree_attempt() -> PhysicalPublicationDenial {
        PhysicalPublicationDenial::MixedTreeAfterCrash
    }

    pub const fn recovery_receipt(self) -> ExecutedS5PublicationRecoveryReceipt {
        self.recovery_receipt
    }

    pub const fn stage(self) -> S5PublicationCrashStage {
        self.recovery_receipt.stage()
    }

    pub const fn recovered(self) -> S5RecoveredPublicationStructure {
        self.recovered
    }

    pub const fn mixed_tree(self) -> bool {
        self.mixed_tree
    }
}

fn bind_recovery_receipt_to_publication(
    receipt: &PhysicalPublicationReceipt,
    recovery_receipt: ExecutedS5PublicationRecoveryReceipt,
) -> Result<S5RecoveredPublicationStructure, PhysicalPublicationDenial> {
    match recovery_receipt.recovered_kind() {
        S5RecoveredPublicationStructureKind::OldStableStructure => Ok(
            S5RecoveredPublicationStructure::old_stable_for_publication_admission(
                receipt.old_root().epoch().get(),
                receipt.old_root().manifest_epoch().get(),
            ),
        ),
        S5RecoveredPublicationStructureKind::NewStableStructure => Ok(
            S5RecoveredPublicationStructure::new_stable_for_publication_admission(
                receipt.new_root().epoch().get(),
                receipt.new_root().manifest_epoch().get(),
            ),
        ),
        S5RecoveredPublicationStructureKind::MixedOldAndNewStructure => {
            Err(PhysicalPublicationDenial::MixedTreeAfterCrash)
        }
    }
}
