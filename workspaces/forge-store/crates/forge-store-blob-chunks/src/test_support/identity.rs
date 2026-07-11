use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionOutcome,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use crate::BlobChunkSecurityScope;

pub(crate) fn blob_scope(
    identity_key: &str,
    tenant_scope: StoreTenantScope,
) -> BlobChunkSecurityScope {
    blob_scope_from_parts(
        identity_key,
        StoreKeyScope::BlobChunkEnvelope,
        tenant_scope,
        StoreAuthenticityRequirement::required(
            forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
    .expect("blob scope should admit")
}

pub(crate) fn admitted_blob_security_scope(
    identity_key: &str,
    tenant_scope: StoreTenantScope,
) -> StoreAdmittedSecurityScope {
    admitted_security_scope(
        identity_key,
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        StoreAuthenticityRequirement::required(
            forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(crate) fn blob_scope_from_parts(
    identity_key: &str,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity: StoreAuthenticityRequirement,
    custody: StoreCustodyPosture,
) -> Result<BlobChunkSecurityScope, crate::BlobChunkSecurityScopeDenial> {
    let admitted = admitted_security_scope(
        identity_key,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity,
        custody,
    );
    BlobChunkSecurityScope::from_admitted_security_scope(admitted)
}

pub(crate) fn security_scope_admission_outcome(
    identity_key: &str,
    key_scope: StoreKeyScope,
    key_version: StoreKeyVersionPosture,
    tenant_scope: StoreTenantScope,
    authenticity: StoreAuthenticityRequirement,
    custody: StoreCustodyPosture,
) -> StoreSecurityScopeAdmissionOutcome {
    let authority = current_authority(identity_key, "chunk-authority");
    let expectation =
        StoreSecurityScopeAdmissionExpectation::new(key_scope, tenant_scope, authenticity, custody);
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        key_scope,
        key_version,
        tenant_scope,
        authenticity,
        custody,
        expectation,
    );
    admit_store_security_scope(request)
}

pub(crate) fn deserialized_blob_scope_declaration(
    identity_key: &str,
) -> StoreRawSecurityScopeDeclaration {
    let authority = current_authority(identity_key, "chunk-authority");
    StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        Some(StoreAuthenticityRequirement::required(
            forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        )),
        Some(StoreCustodyPosture::InternalStoreCustody),
    )
}

pub(crate) fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

fn admitted_security_scope(
    identity_key: &str,
    key_scope: StoreKeyScope,
    key_version: StoreKeyVersionPosture,
    tenant_scope: StoreTenantScope,
    authenticity: StoreAuthenticityRequirement,
    custody: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, "chunk-authority");
    let expectation =
        StoreSecurityScopeAdmissionExpectation::new(key_scope, tenant_scope, authenticity, custody);
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        key_scope,
        key_version,
        tenant_scope,
        authenticity,
        custody,
        expectation,
    );

    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("security scope should admit before blob lane filtering: {outcome:?}"),
    }
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

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}
