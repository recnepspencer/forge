use super::support::{
    intact_integrity_model_input, platform_recovery_scope, recovery_security_scope,
    with_admitted_entry,
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
    with_admitted_entry(
        intact_integrity_model_input("entry-required"),
        |admission| {
            let identity = admission.entry_identity().clone();
            let security_scope = recovery_security_scope(&admission, "entry-required");

            let gate = match RecoveryReplayEntryGate::before_source_precedence(
                admission,
                security_scope,
            ) {
                TransitionOutcome::Success(gate) => gate,
                other => {
                    panic!(
                        "matching recovery entry and security scope should gate replay: {other:?}"
                    )
                }
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
        },
    );
}

#[test]
fn replay_gate_denies_security_scope_bound_to_different_entry_identity() {
    with_admitted_entry(
        intact_integrity_model_input("entry-security-scope"),
        |security_admission| {
            with_admitted_entry(
                intact_integrity_model_input("entry-replay-mismatch"),
                |replay_admission| {
                    let security_scope =
                        recovery_security_scope(&security_admission, "entry-security-scope");

                    let outcome = RecoveryReplayEntryGate::before_source_precedence(
                        replay_admission,
                        security_scope,
                    );

                    match outcome {
                        TransitionOutcome::Denied(denial) => {
                            assert_eq!(
                                denial.store_denial().kind(),
                                StoreSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode
                            );
                            assert_eq!(denial.store_denial().counters().drifted(), 1);
                        }
                        other => {
                            panic!("mismatched replay scope must deny instead of panic: {other:?}")
                        }
                    }
                },
            );
        },
    );
}

#[test]
fn recovery_scope_propagation_denies_root_carrier_from_different_entry_admission() {
    with_admitted_entry(
        intact_integrity_model_input("entry-wal-checkpoint"),
        |wal_admission| {
            with_admitted_entry(
                intact_integrity_model_input("entry-root-mismatch"),
                |root_admission| {
                    let admitted = platform_recovery_scope("entry-wal-checkpoint");
                    let wal = RecoveryWalRecordSecurityMetadataEnvelope::from_admitted_scope(
                        RecoveryWalRecordSecurityMetadataIdentity::new(1),
                        &admitted,
                        StoreKeyVersionPosture::Current,
                        StoreLegacySecurityPosture::NativeScoped,
                    );
                    let checkpoint =
                        RecoveryCheckpointRecordSecurityMetadataEnvelope::from_admitted_scope(
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

                    let input = RecoverySecurityScopePropagationInput::new(
                        &wal,
                        &checkpoint,
                        &root,
                        &wal_admission,
                    );

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
                },
            );
        },
    );
}
