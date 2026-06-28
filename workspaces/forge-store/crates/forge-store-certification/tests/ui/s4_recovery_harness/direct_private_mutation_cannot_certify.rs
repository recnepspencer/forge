use forge_store_certification::{
    RecoveryPhysicsCounterExpectation, RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane,
    RecoveryPhysicsMutant, RecoveryPhysicsMutationFailureEvidence,
    RecoveryPhysicsMutationSuiteLaneEvidence,
};

fn main() {
    let _forged = RecoveryPhysicsMutationSuiteLaneEvidence {
        mutant: RecoveryPhysicsMutant::DirectPrivateMutationAccepted,
        lane: RecoveryPhysicsCrashLane::RenameDurability,
        failure_evidence: RecoveryPhysicsMutationFailureEvidence::CompileFailBoundary,
        counter: RecoveryPhysicsCounterExpectation::exact(
            RecoveryPhysicsCounterKind::MutationFailures,
            1,
        ),
    };
}
