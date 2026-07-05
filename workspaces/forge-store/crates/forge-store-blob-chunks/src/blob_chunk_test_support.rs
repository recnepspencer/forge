use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_contracts::{
    StableDigest, StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionOutcome,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

pub(crate) use crate::blob_chunk_physical_test_support::physical_payload_for_bytes;

use crate::{
    BlobChunkCanonicalComparisonBasis, BlobChunkCanonicalEquivalence, BlobChunkDedupeCandidate,
    BlobChunkIntegrityProof, BlobChunkSecurityScope, BlobChunkSequenceAdmission, BlobChunkSize,
    BlobChunkStreamingResidencyProof, BlobChunkStreamingWindow, BlobChunkingRuleAdmission,
};

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
    let readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::blob_chunk(),
        admitted,
    );
    BlobChunkSecurityScope::from_s5_1_readiness(readiness)
}

pub(crate) fn non_blob_family_readiness(
    identity_key: &str,
) -> forge_store_readiness::S51AdmittedSecurityScopeReadiness {
    let admitted = admitted_security_scope(
        identity_key,
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::io_qos(),
        admitted,
    )
}

pub(crate) fn candidate_for_scope(scope: BlobChunkSecurityScope) -> BlobChunkDedupeCandidate {
    candidate_for_scope_with_digest(scope, "sha256:blob-s51-same-content")
}

pub(crate) fn candidate_for_bytes_and_scope(
    bytes: &[u8],
    scope: BlobChunkSecurityScope,
) -> BlobChunkDedupeCandidate {
    BlobChunkDedupeCandidate::from_integrity_proof(integrity_proof_for_scope(scope, bytes))
}

pub(crate) fn candidate_for_scope_with_digest(
    scope: BlobChunkSecurityScope,
    digest_raw: &str,
) -> BlobChunkDedupeCandidate {
    BlobChunkDedupeCandidate::from_integrity_proof(integrity_proof_for_scope(
        scope,
        digest_raw.as_bytes(),
    ))
}

pub(crate) fn integrity_proof_for_scope(
    scope: BlobChunkSecurityScope,
    bytes: &[u8],
) -> BlobChunkIntegrityProof {
    admitted_sequence_for_scope(scope, bytes)
        .first_chunk()
        .clone()
}

pub(crate) fn admitted_sequence_for_scope(
    scope: BlobChunkSecurityScope,
    bytes: &[u8],
) -> crate::AdmittedBlobChunkSequence {
    let rule = BlobChunkingRuleAdmission::fixed_size(
        BlobChunkSize::from_bytes(bytes.len() as u64).expect("nonempty chunk size"),
    )
    .expect("fixed-size rule should admit");
    BlobChunkSequenceAdmission::start(scope, rule, bytes.len() as u64)
        .expect("sequence should start")
        .push_payload(0, physical_payload_for_bytes(bytes))
        .expect("window should admit into sequence")
        .finish()
        .expect("sequence should finish")
}

pub(crate) fn admitted_multichunk_sequence_for_scope(
    scope: BlobChunkSecurityScope,
    bytes: &[u8],
    chunk_size: u64,
) -> crate::AdmittedBlobChunkSequence {
    let rule = BlobChunkingRuleAdmission::fixed_size(
        BlobChunkSize::from_bytes(chunk_size).expect("nonempty chunk size"),
    )
    .expect("fixed-size rule should admit");
    let mut admission = BlobChunkSequenceAdmission::start(scope, rule, bytes.len() as u64)
        .expect("sequence should start");
    let mut offset = 0_u64;
    for chunk in bytes.chunks(chunk_size as usize) {
        admission = admission
            .push_payload(offset, physical_payload_for_bytes(chunk))
            .expect("window should admit into sequence");
        offset += chunk.len() as u64;
    }
    admission.finish().expect("sequence should finish")
}

pub(crate) fn streaming_window() -> BlobChunkStreamingWindow {
    streaming_window_with_digest("sha256:blob-s51-same-content")
}

pub(crate) fn streaming_window_with_digest(raw: &str) -> BlobChunkStreamingWindow {
    let digest = StableDigest::new(raw).expect("digest");
    let residency =
        BlobChunkStreamingResidencyProof::bounded_window(4096, 1024).expect("bounded window");
    BlobChunkStreamingWindow::new(
        crate::BlobChunkIdentity::from_integrity_parts(digest.clone()),
        digest,
        residency,
    )
    .expect("bounded window should admit")
}

pub(crate) fn canonical_equivalence(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
) -> BlobChunkCanonicalEquivalence {
    BlobChunkCanonicalComparisonBasis::from_candidates(existing, candidate)
        .expect("candidate comparison basis should prepare")
        .evaluate_foundational_equivalence()
        .expect("candidate-derived equivalence should admit")
}

pub(crate) fn forced_collision_equivalence(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
) -> BlobChunkCanonicalEquivalence {
    BlobChunkCanonicalEquivalence::forced_digest_collision_fixture(existing, candidate)
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
