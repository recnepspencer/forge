use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::require_current_store_authority;
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_security::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use crate::{
    StoreCheckpointRecordIdentity, StoreWalRecordIdentity, WalSecurityMetadataCarrier,
    WalSecurityMetadataEnvelope,
};

#[test]
fn wal_and_checkpoint_metadata_preserve_record_identity() {
    let authority = require_current_store_authority(boundary_fact());
    let request = StoreSecurityScopeAdmissionRequest::platform_page_envelope(
        &authority,
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let admitted = match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("platform scope should admit: {outcome:?}"),
    };
    let witnesses = admitted.witnesses();
    let wal_metadata = WalSecurityMetadataCarrier::for_wal_record(
        witnesses,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let checkpoint_metadata = WalSecurityMetadataCarrier::for_checkpoint_record(
        witnesses,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let wal_identity = StoreWalRecordIdentity::new(42);
    let checkpoint_identity = StoreCheckpointRecordIdentity::new(7);

    let wal = WalSecurityMetadataEnvelope::wal_record(wal_identity, wal_metadata);
    let checkpoint =
        WalSecurityMetadataEnvelope::checkpoint_record(checkpoint_identity, checkpoint_metadata);

    assert_eq!(*wal.record(), wal_identity);
    assert_eq!(*checkpoint.record(), checkpoint_identity);
    assert_metadata_is_platform_scope(wal.security_metadata());
    assert_metadata_is_platform_scope(checkpoint.security_metadata());
}

fn assert_metadata_is_platform_scope(metadata: WalSecurityMetadataCarrier) {
    let physical_metadata = metadata.physical_metadata();
    assert_eq!(physical_metadata.key_scope(), StoreKeyScope::PageEnvelope);
    assert_eq!(
        physical_metadata.tenant_scope(),
        StoreTenantScope::TenantPhysicalBoundary
    );
    assert_eq!(
        physical_metadata.authenticity_requirement(),
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        )
    );
    assert_eq!(
        physical_metadata.custody_posture(),
        StoreCustodyPosture::InternalStoreCustody
    );
    assert_eq!(
        physical_metadata.legacy_posture(),
        StoreLegacySecurityPosture::NativeScoped
    );
    assert_eq!(
        physical_metadata.key_version_posture(),
        StoreKeyVersionPosture::Current
    );
}

fn boundary_fact() -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key("s51.phase3.wal").unwrap();
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract)])
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
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from("record")))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
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
