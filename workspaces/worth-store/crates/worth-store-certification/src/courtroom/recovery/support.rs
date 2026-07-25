pub(super) use crate::courtroom::harness::test_support::integrity_handoff_test_support::{
    admit_recovery_handoff_payload, intact_integrity_model_input,
    recovery_blocking_quarantine_binding, unresolved_authority_record,
};
pub(super) use crate::courtroom::harness::test_support::recovery_blocking_damage_test_support::recovery_blocking_wal_damage_map;
pub(super) use crate::courtroom::harness::test_support::recovery_memory_allocation_test_support::recovery_memory_allocation;
use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::PhysicalAuthorityRecap;
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_recovery_physics::{
    AdmittedRecoveryIntegrityInput, IntegrityHandoffPayload,
    RecoveryCheckpointRecordSecurityMetadataEnvelope,
    RecoveryCheckpointRecordSecurityMetadataIdentity, RecoveryEntryAdmission,
    RecoveryEntryAdmissionDecision, RecoveryRootSecurityMetadataEnvelope,
    RecoverySecurityScopePropagation, RecoverySecurityScopePropagationInput,
    RecoveryWalRecordSecurityMetadataEnvelope, RecoveryWalRecordSecurityMetadataIdentity,
};
use worth_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreCustodyPosture,
    StoreKeyVersionPosture, StoreLegacySecurityPosture, StoreSecurityScopeAdmissionRequest,
};

pub(super) fn admit_entry(model_input: AdmittedRecoveryIntegrityInput) -> RecoveryEntryAdmission {
    let decision = RecoveryEntryAdmission::admit(
        model_input,
        recovery_memory_allocation(),
        physical_authority(),
    );
    let RecoveryEntryAdmissionDecision::Admitted(admission) = decision else {
        panic!("intact integrity model input admits the recovery algorithm entry");
    };
    *admission
}

pub(super) fn damaged_integrity_model_input() -> AdmittedRecoveryIntegrityInput {
    let intact = intact_integrity_model_input("blocked-entry");
    let (quarantine_record, quarantine_receipt, quarantine_damage) =
        recovery_blocking_quarantine_binding();
    let damage_map = recovery_blocking_wal_damage_map()
        .with_recovery_blocking_quarantine(
            &quarantine_record,
            quarantine_receipt,
            &quarantine_damage,
        )
        .unwrap();

    let payload = IntegrityHandoffPayload::declare()
        .root_manifest(intact.payload().root_manifest().clone())
        .segment_manifest(intact.payload().segment_manifest().clone())
        .page_frame(intact.payload().page_frames()[0].clone())
        .wal_frame(intact.payload().wal_frames()[0].clone())
        .checkpoint_record(intact.payload().checkpoint_records()[0].clone())
        .damage_map(damage_map)
        .inspection_envelope(intact.payload().inspection_envelope().clone())
        .seal()
        .unwrap();
    admit_recovery_handoff_payload(payload)
}

pub(super) fn physical_authority() -> PhysicalAuthorityRecap {
    PhysicalAuthorityRecap::from_physical_format_authority(3, 2, 1).unwrap()
}

pub(super) fn recovery_security_scope(
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

pub(super) fn platform_recovery_scope(identity: &str) -> StoreAdmittedSecurityScope {
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
) -> worth_foundational::ContractValidatedAspectArtifact {
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
