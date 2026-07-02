use forge_store_recovery_physics::{S5PublicationCrashStage, S5PublicationRecoveryReplayInput};

fn main() {
    let replay =
        S5PublicationRecoveryReplayInput::from_crash_stage(S5PublicationCrashStage::BeforePublication);
    let _ = replay.execute();
}
