use crate::{
    courtroom::harness::test_support::s4_integrity_handoff_test_support::{admit_s4_handoff_payload, intact_readiness},
    courtroom::harness::test_support::s4_recovery_blocking_damage_test_support::recovery_blocking_wal_damage_map,
};
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
    BackgroundWorkBudgetSnapshot, BackgroundWorkClass, FixedMetadataReservation,
};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_physical_format::PhysicalSecurityScopePropagationDenialKind;
use forge_store_readiness::PhysicalAuthorityRecap;
use forge_store_recovery_physics::{
    RecoveryCheckpointRecordSecurityMetadataEnvelope,
    RecoveryCheckpointRecordSecurityMetadataIdentity, RecoveryEntryAdmission,
    RecoveryEntryAdmissionDecision, RecoveryMemoryEnvelope, RecoveryReplayEntryGate,
    RecoveryRootSecurityMetadataEnvelope, RecoverySecurityScopePropagation,
    RecoverySecurityScopePropagationInput, RecoveryWalRecordSecurityMetadataEnvelope,
    RecoveryWalRecordSecurityMetadataIdentity, S4IntegrityHandoffPayload,
};
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreCustodyPosture,
    StoreKeyVersionPosture, StoreLegacySecurityPosture, StoreSecurityScopeAdmissionRequest,
};

#[test]
fn intact_s3_handoff_and_recovery_envelope_produce_stable_entry_identity() {
    let first = admit_entry(intact_readiness("entry-stability"));
    let second = admit_entry(intact_readiness("entry-stability"));

    assert_eq!(first.entry_identity(), second.entry_identity());
    assert_eq!(first.recovery_basis(), second.recovery_basis());
    assert_eq!(
        first.recovery_basis().integrity_handoff_identity(),
        second.recovery_basis().integrity_handoff_identity()
    );
    assert_eq!(first.counters().vetted_record_count(), 5);
    assert_eq!(first.counters().memory_envelope_admissions(), 1);
    assert_eq!(first.counters().replay_plans_started(), 0);
    assert_eq!(first.counters().source_precedence_choices(), 0);
    assert!(!first.claims_replay_plan());
    assert!(!first.source_precedence_chosen());
}

#[test]
fn recovery_entry_rejects_wrong_or_unbounded_recovery_envelopes_before_admission() {
    let compaction = admit_background(class_request(BackgroundWorkClass::CompactionPlanning));
    let denial = RecoveryMemoryEnvelope::from_admitted(compaction).unwrap_err();

    assert_eq!(
        denial,
        forge_store_recovery_physics::RecoveryMemoryEnvelopeDenial::WrongBackgroundEnvelopeClass {
            expected: BackgroundWorkClass::RecoveryPlanning,
            actual: BackgroundWorkClass::CompactionPlanning,
        }
    );

    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    let unbounded = admission
        .admit(
            BackgroundEnvelopeRequest::recovery_planning()
                .resident_frames(1)
                .resident_bytes(128)
                .whole_object_memory_bytes(4096)
                .allocation_bytes(128)
                .finish(),
            permissive_budget(),
            &mut allocation,
        )
        .unwrap_err();

    assert_eq!(
        unbounded.work_class(),
        BackgroundWorkClass::RecoveryPlanning
    );
    assert_eq!(unbounded.counters().admitted(), 0);
}

#[test]
fn recovery_blocking_damage_blocks_before_replay_or_source_precedence() {
    let readiness = damaged_readiness();
    let decision =
        RecoveryEntryAdmission::admit(readiness, recovery_memory_envelope(), physical_authority());

    let RecoveryEntryAdmissionDecision::Blocked(blocked) = decision else {
        panic!("recovery-blocking S.3 damage must block entry");
    };

    assert_eq!(blocked.blocker_count(), 1);
    assert!(!blocked.replay_planning_started());
    assert!(!blocked.source_precedence_chosen());
}

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
                PhysicalSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode
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
                PhysicalSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode
            );
            assert_eq!(denial.store_denial().counters().drifted(), 1);
        }
        other => panic!("mismatched recovery root carrier must deny: {other:?}"),
    }
}

fn admit_entry(
    readiness: forge_store_recovery_physics::S4RecoveryPhysicsIntegrityReadiness,
) -> forge_store_recovery_physics::RecoveryEntryAdmission {
    let decision =
        RecoveryEntryAdmission::admit(readiness, recovery_memory_envelope(), physical_authority());
    let RecoveryEntryAdmissionDecision::Admitted(admission) = decision else {
        panic!("intact typed S.3/S.2/S.1 evidence admits recovery entry");
    };
    admission
}

fn damaged_readiness() -> forge_store_recovery_physics::S4RecoveryPhysicsIntegrityReadiness {
    let intact = intact_readiness("blocked-entry");
    let blocker = intact
        .payload()
        .damage_map()
        .quarantine_summaries()
        .first()
        .expect("fixture carries quarantine summary");
    let damage_map = recovery_blocking_wal_damage_map().with_quarantine_summary(blocker.clone());

    let payload = S4IntegrityHandoffPayload::declare()
        .root_manifest(intact.payload().root_manifest().clone())
        .segment_manifest(intact.payload().segment_manifest().clone())
        .page_frame(intact.payload().page_frames()[0].clone())
        .wal_frame(intact.payload().wal_frames()[0].clone())
        .checkpoint_record(intact.payload().checkpoint_records()[0].clone())
        .damage_map(damage_map)
        .inspection_envelope(intact.payload().inspection_envelope().clone())
        .seal()
        .unwrap();
    admit_s4_handoff_payload(payload)
}

fn recovery_memory_envelope() -> RecoveryMemoryEnvelope {
    RecoveryMemoryEnvelope::from_admitted(admit_background(class_request(
        BackgroundWorkClass::RecoveryPlanning,
    )))
    .expect("recovery envelope admits only recovery planning class")
}

fn admit_background(request: BackgroundEnvelopeRequest) -> AdmittedBackgroundEnvelope {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    admission
        .admit(request, permissive_budget(), &mut allocation)
        .expect("background envelope admits")
}

fn class_request(work_class: BackgroundWorkClass) -> BackgroundEnvelopeRequest {
    let builder = match work_class {
        BackgroundWorkClass::RecoveryPlanning => BackgroundEnvelopeRequest::recovery_planning(),
        BackgroundWorkClass::CompactionPlanning => BackgroundEnvelopeRequest::compaction_planning(),
        BackgroundWorkClass::ScrubPlanning => BackgroundEnvelopeRequest::scrub_planning(),
        BackgroundWorkClass::ImportExport => BackgroundEnvelopeRequest::import_export(),
        BackgroundWorkClass::LargeRecordStreaming => {
            BackgroundEnvelopeRequest::large_record_streaming()
        }
    };
    builder
        .resident_frames(1)
        .resident_bytes(128)
        .pin_pages_for_bounded_step(1)
        .allocation_bytes(128)
        .finish()
}

fn physical_authority() -> PhysicalAuthorityRecap {
    PhysicalAuthorityRecap::from_s1_authority(3, 2, 1).unwrap()
}

fn permissive_budget() -> BackgroundWorkBudgetSnapshot {
    BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16)
}

fn allocation_admission() -> AllocationAdmission {
    AllocationAdmission::from_declaration(
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
    )
}

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}

fn recovery_security_scope(
    admission: &RecoveryEntryAdmission,
    identity: &str,
) -> RecoverySecurityScopePropagation {
    let admitted = platform_recovery_scope(identity);
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
        admission,
        &admitted,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let input = RecoverySecurityScopePropagationInput::new(&wal, &checkpoint, &root, admission);
    match RecoverySecurityScopePropagation::admit(input) {
        TransitionOutcome::Success(propagation) => propagation,
        other => panic!("recovery security scope should admit: {other:?}"),
    }
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
        StoreAspectAuthorityInput::new(admitted_state, store_physical_witness()),
    )
    .expect("Store boundary fact should admit matching identity")
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

fn store_physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}
