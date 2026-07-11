use forge_store_recovery_physics::{PublicationCrashStage, PublicationRecoveryReplayInput};

fn main() {
    let replay =
        PublicationRecoveryReplayInput::from_crash_stage(PublicationCrashStage::BeforePublication);
    let _ = replay.execute();
}
