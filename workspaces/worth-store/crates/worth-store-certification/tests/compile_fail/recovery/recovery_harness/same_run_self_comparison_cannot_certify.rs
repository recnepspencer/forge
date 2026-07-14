use worth_store_certification::{
    RecoveryPhysicsShortcutAttempt, RecoveryPhysicsShortcutRejection,
};

fn main() {
    let _forged =
        RecoveryPhysicsShortcutRejection::denied(RecoveryPhysicsShortcutAttempt::SameRunSelfComparison);
}
