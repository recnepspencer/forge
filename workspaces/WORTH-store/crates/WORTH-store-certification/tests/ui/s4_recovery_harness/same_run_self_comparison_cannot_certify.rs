use worth_store_certification::{
    RecoveryPhysicsShortcutAttempt, RecoveryPhysicsShortcutRejection,
};

fn main() {
    let _WORTHd =
        RecoveryPhysicsShortcutRejection::denied(RecoveryPhysicsShortcutAttempt::SameRunSelfComparison);
}
