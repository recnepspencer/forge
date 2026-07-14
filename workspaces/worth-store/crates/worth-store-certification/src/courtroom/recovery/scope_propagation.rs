use super::support::{
    admit_entry, intact_readiness, platform_recovery_scope, recovery_security_scope,
};
use worth_proof::TransitionOutcome;
use worth_store_recovery_physics::{
    RecoveryCheckpointRecordSecurityMetadataEnvelope,
    RecoveryCheckpointRecordSecurityMetadataIdentity, RecoveryReplayEntryGate,
    RecoveryRootSecurityMetadataEnvelope, RecoverySecurityScopePropagation,
    RecoverySecurityScopePropagationInput, RecoveryWalRecordSecurityMetadataEnvelope,
    RecoveryWalRecordSecurityMetadataIdentity,
};
use worth_store_security::{
    StoreKeyVersionPosture, StoreLegacySecurityPosture, StoreSecurityScopePropagationDenialKind,
};

#[test]
fn replay_planning_requires_recovery_entry_admission() {
    let admission = admit_entry(intact_readiness("entry-required"));
    let identity = admission.entry_identity().clone();
    let security_scope = recovery_security_scope(&admission, "entry-required");

    let gate = match RecoveryReplayEntryGate::before_source_precedence(admission, security_scope) {
        TransitionOutcome::Success(gate) => gate,
        other => panic!("matching recovery entry and security scope should gate replay: {other:?}"),
    };

    assert_eq!(gate.entry_identity(), &identity);
    assert_eq!(gate.security_scope().entry_identity(), &identity);
    assert_eq!(
        gate.security_scope()
            .counters()
            .wal_checkpoint_store_counters()
            .preserved(),
        1
    );
    assert_eq!(
        gate.security_scope()
            .counters()
            .root_store_counters()
            .preserved(),
        1
    );
    assert!(!gate.replay_planning_started());
    assert!(!gate.source_precedence_chosen());
}

#[test]
fn replay_gate_denies_security_scope_bound_to_different_entry_identity() {
    let security_admission = admit_entry(intact_readiness("entry-security-scope"));
    let replay_admission = admit_entry(intact_readiness("entry-replay-mismatch"));
    let security_scope = recovery_security_scope(&security_admission, "entry-security-scope");

    let outcome =
        RecoveryReplayEntryGate::before_source_precedence(replay_admission, security_scope);

    match outcome {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.store_denial().kind(),
                StoreSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode
            );
            assert_eq!(denial.store_denial().counters().drifted(), 1);
        }
        other => panic!("mismatched replay scope must deny instead of panic: {other:?}"),
    }
}

#[test]
fn recovery_scope_propagation_denies_root_carrier_from_different_entry_admission() {
    let wal_admission = admit_entry(intact_readiness("entry-wal-checkpoint"));
    let root_admission = admit_entry(intact_readiness("entry-root-mismatch"));
    let admitted = platform_recovery_scope("entry-wal-checkpoint");
    let wal = RecoveryWalRecordSecurityMetadataEnvelope::from_admitted_scope(
        RecoveryWalRecordSecurityMetadataIdentity::new(1),
        &admitted,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let checkpoint = RecoveryCheckpointRecordSecurityMetadataEnvelope::from_admitted_scope(
        RecoveryCheckpointRecordSecurityMetadataIdentity::new(1),
        &admitted,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let root = RecoveryRootSecurityMetadataEnvelope::from_recovery_entry(
        &root_admission,
        &admitted,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );

    let input =
        RecoverySecurityScopePropagationInput::new(&wal, &checkpoint, &root, &wal_admission);

    match RecoverySecurityScopePropagation::admit(input) {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.store_denial().kind(),
                StoreSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode
            );
            assert_eq!(denial.store_denial().counters().drifted(), 1);
        }
        other => panic!("mismatched recovery root carrier must deny: {other:?}"),
    }
}
