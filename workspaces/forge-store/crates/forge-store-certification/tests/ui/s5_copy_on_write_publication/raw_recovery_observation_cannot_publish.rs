use forge_store_recovery_physics::{
    S5PublicationCrashStage, S5PublicationRecoveryObservation, S5RecoveredPublicationStructure,
};

fn main() {
    let _ = S5PublicationRecoveryObservation::new(
        S5PublicationCrashStage::DuringPublication,
        S5RecoveredPublicationStructure::MixedOldAndNewStructure {
            old_root_epoch: 1,
            old_manifest_epoch: 1,
            new_root_epoch: 2,
            new_manifest_epoch: 2,
        },
    );
}
