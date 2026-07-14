use worth_store_recovery_physics::{
    PublicationCrashStage, RecoveredPublicationStructure, S5PublicationRecoveryObservation,
};

fn main() {
    let _ = S5PublicationRecoveryObservation::new(
        PublicationCrashStage::DuringPublication,
        RecoveredPublicationStructure::MixedOldAndNewStructure {
            old_root_epoch: 1,
            old_manifest_epoch: 1,
            new_root_epoch: 2,
            new_manifest_epoch: 2,
        },
    );
}
