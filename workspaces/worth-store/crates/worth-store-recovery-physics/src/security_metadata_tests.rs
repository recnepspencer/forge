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
    admit_store_security_scope, StoreCustodyPosture, StoreKeyVersionPosture,
    StoreLegacySecurityPosture, StoreSecurityMetadata, StoreSecurityScopeAdmissionRequest,
    StoreSecurityScopeDeclarationProvenance,
};

use crate::RecoveryRootSecurityMetadataAdmission;

#[test]
fn recovery_root_security_metadata_lowers_to_raw_readmission_input() {
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
    let metadata = StoreSecurityMetadata::from_current_security_scope(
        admitted.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::ReadmissionRequired,
    );
    let admission = RecoveryRootSecurityMetadataAdmission::from_physical_metadata(metadata);

    let raw = admission.to_raw_security_scope_declaration(authority.physical_witness());

    assert_eq!(
        raw.provenance(),
        StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted
    );
    assert_eq!(raw.key_scope(), metadata.key_scope());
    assert_eq!(raw.tenant_scope(), metadata.tenant_scope());
    assert_eq!(
        raw.authenticity_requirement(),
        Some(metadata.authenticity_requirement())
    );
    assert_eq!(raw.custody_posture(), Some(metadata.custody_posture()));
    assert_eq!(raw.key_version_posture(), metadata.key_version_posture());
    assert_eq!(admission.metadata(), metadata);
}

fn boundary_fact() -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key("s51.phase3.recovery").unwrap();
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
        .value(AspectValue::String(InternedString::from("root")))
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
