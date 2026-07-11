use crate::courtroom::harness::test_support::s4_integrity_handoff_test_support::intact_readiness;
use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationAdmission, AllocationByteBudget,
    AllocationEnvelopeDeclaration, BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest,
    BackgroundWorkBudgetSnapshot, FixedMetadataReservation,
};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_readiness::PhysicalAuthorityRecap;
use forge_store_recovery_physics::{
    RecoveryCheckpointRecordSecurityMetadataEnvelope, RecoveryEntryAdmission,
    RecoveryEntryAdmissionDecision, RecoveryMemoryEnvelope, RecoveryReplayEntryGate,
    RecoveryRootSecurityMetadataEnvelope, RecoverySecurityScopePropagation,
    RecoverySecurityScopePropagationInput, RecoveryWalRecordSecurityMetadataEnvelope,
};
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreCustodyPosture,
    StoreKeyVersionPosture, StoreLegacySecurityPosture, StoreSecurityScopeAdmissionRequest,
    StoreSecurityScopePropagationDenialKind,
};
use forge_store_wal::{
    CheckpointRecordSecurityMetadataEnvelope, StoreCheckpointRecordIdentity,
    StoreWalRecordIdentity, WalRecordSecurityMetadataEnvelope, WalSecurityMetadataCarrier,
};

#[test]
fn recovery_scope_propagation_uses_wal_checkpoint_carrier_identities() {
    let admission = admit_entry("wal-carrier");
    let security_scope = recovery_scope_from_wal_carriers(
        &admission,
        "wal-carrier",
        StoreKeyVersionPosture::Current,
        StoreKeyVersionPosture::Current,
        StoreKeyVersionPosture::Current,
    )
    .unwrap();

    let gate = match RecoveryReplayEntryGate::before_source_precedence(admission, security_scope) {
        TransitionOutcome::Success(gate) => gate,
        other => panic!("matching recovery entry and WAL-carried scope should gate: {other:?}"),
    };

    assert_eq!(gate.security_scope().wal_record_identity().sequence(), 42);
    assert_eq!(
        gate.security_scope()
            .checkpoint_record_identity()
            .checkpoint_epoch(),
        7
    );
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
}

#[test]
fn recovery_scope_denies_stale_wal_scope_before_replay_publication() {
    let admission = admit_entry("stale-wal");
    let denial = recovery_scope_from_wal_carriers(
        &admission,
        "stale-wal",
        StoreKeyVersionPosture::Stale,
        StoreKeyVersionPosture::Current,
        StoreKeyVersionPosture::Current,
    )
    .unwrap_err();

    assert_eq!(
        denial.store_denial().kind(),
        StoreSecurityScopePropagationDenialKind::StalePropagatedSecurityScope
    );
    assert_eq!(denial.store_denial().counters().stale(), 1);
}

#[test]
fn recovery_scope_denies_unsupported_wal_checkpoint_scope_before_replay_publication() {
    let admission = admit_entry("bad-ckpt");
    let denial = recovery_scope_from_wal_carriers(
        &admission,
        "bad-ckpt",
        StoreKeyVersionPosture::Unsupported,
        StoreKeyVersionPosture::Unsupported,
        StoreKeyVersionPosture::Current,
    )
    .unwrap_err();

    assert_eq!(
        denial.store_denial().kind(),
        StoreSecurityScopePropagationDenialKind::UnsupportedPropagatedSecurityScope
    );
    assert_eq!(denial.store_denial().counters().unsupported(), 1);
}

#[test]
fn recovery_scope_missing_root_denies_before_replay_publication() {
    let admission = admit_entry("missing-root");
    let admitted = platform_recovery_scope("missing-root");
    let wal = RecoveryWalRecordSecurityMetadataEnvelope::from_wal_record_envelope(&wal_record(
        &admitted,
        StoreKeyVersionPosture::Current,
    ));
    let checkpoint =
        RecoveryCheckpointRecordSecurityMetadataEnvelope::from_checkpoint_record_envelope(
            &checkpoint_record(&admitted, StoreKeyVersionPosture::Current),
        );

    let outcome = RecoverySecurityScopePropagation::admit_required(
        Some(&wal),
        Some(&checkpoint),
        None,
        &admission,
    );

    match outcome {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.store_denial().kind(),
                StoreSecurityScopePropagationDenialKind::MissingPropagatedSecurityScope
            );
            assert_eq!(denial.store_denial().counters().missing(), 1);
        }
        other => panic!("missing recovery root scope must deny before replay: {other:?}"),
    }
}

fn recovery_scope_from_wal_carriers(
    admission: &RecoveryEntryAdmission,
    identity: &str,
    wal_key_version: StoreKeyVersionPosture,
    checkpoint_key_version: StoreKeyVersionPosture,
    root_key_version: StoreKeyVersionPosture,
) -> Result<
    RecoverySecurityScopePropagation,
    forge_store_recovery_physics::RecoverySecurityScopePropagationDenial,
> {
    let admitted = platform_recovery_scope(identity);
    let wal = RecoveryWalRecordSecurityMetadataEnvelope::from_wal_record_envelope(&wal_record(
        &admitted,
        wal_key_version,
    ));
    let checkpoint =
        RecoveryCheckpointRecordSecurityMetadataEnvelope::from_checkpoint_record_envelope(
            &checkpoint_record(&admitted, checkpoint_key_version),
        );
    let root = RecoveryRootSecurityMetadataEnvelope::from_recovery_entry(
        admission,
        &admitted,
        root_key_version,
        StoreLegacySecurityPosture::NativeScoped,
    );
    match RecoverySecurityScopePropagation::admit(RecoverySecurityScopePropagationInput::new(
        &wal,
        &checkpoint,
        &root,
        admission,
    )) {
        TransitionOutcome::Success(scope) => Ok(scope),
        TransitionOutcome::Denied(denial) => Err(denial),
        TransitionOutcome::Deferred(deferred) => match deferred {},
        TransitionOutcome::Stale(stale) => match stale {},
        TransitionOutcome::RebindRequired(rebind) => match rebind {},
        TransitionOutcome::Failed(failed) => match failed {},
    }
}

fn wal_record(
    admitted: &StoreAdmittedSecurityScope,
    key_version: StoreKeyVersionPosture,
) -> WalRecordSecurityMetadataEnvelope {
    WalRecordSecurityMetadataEnvelope::wal_record(
        StoreWalRecordIdentity::new(42),
        WalSecurityMetadataCarrier::for_wal_record(
            admitted.witnesses(),
            key_version,
            StoreLegacySecurityPosture::NativeScoped,
        ),
    )
}

fn checkpoint_record(
    admitted: &StoreAdmittedSecurityScope,
    key_version: StoreKeyVersionPosture,
) -> CheckpointRecordSecurityMetadataEnvelope {
    CheckpointRecordSecurityMetadataEnvelope::checkpoint_record(
        StoreCheckpointRecordIdentity::new(7),
        WalSecurityMetadataCarrier::for_checkpoint_record(
            admitted.witnesses(),
            key_version,
            StoreLegacySecurityPosture::NativeScoped,
        ),
    )
}

fn admit_entry(identity: &str) -> RecoveryEntryAdmission {
    let decision = RecoveryEntryAdmission::admit(
        intact_readiness(identity),
        recovery_memory_envelope(),
        physical_authority(),
    );
    let RecoveryEntryAdmissionDecision::Admitted(admission) = decision else {
        panic!("intact typed S.3/S.2/S.1 evidence admits recovery entry");
    };
    admission
}

fn recovery_memory_envelope() -> RecoveryMemoryEnvelope {
    RecoveryMemoryEnvelope::from_admitted(admit_background()).unwrap()
}

fn admit_background() -> AdmittedBackgroundEnvelope {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = AllocationAdmission::from_declaration(
        AllocationEnvelopeDeclaration::declare()
            .foreground(bytes(512))
            .maintenance(bytes(512))
            .recovery(bytes(512))
            .scrub(bytes(512))
            .import_export(bytes(512))
            .streaming(bytes(512))
            .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
            .seal()
            .unwrap(),
    );
    admission
        .admit(
            BackgroundEnvelopeRequest::recovery_planning()
                .resident_frames(1)
                .resident_bytes(128)
                .pin_pages_for_bounded_step(1)
                .allocation_bytes(128)
                .finish(),
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16),
            &mut allocation,
        )
        .unwrap()
}

fn physical_authority() -> PhysicalAuthorityRecap {
    PhysicalAuthorityRecap::from_s1_authority(3, 2, 1).unwrap()
}

fn platform_recovery_scope(identity: &str) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity, "recovery-replay");
    match admit_store_security_scope(StoreSecurityScopeAdmissionRequest::platform_page_envelope(
        &authority,
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        other => panic!("platform recovery scope should admit: {other:?}"),
    }
}

fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .expect("Store boundary fact should admit matching identity")
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> forge_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}
