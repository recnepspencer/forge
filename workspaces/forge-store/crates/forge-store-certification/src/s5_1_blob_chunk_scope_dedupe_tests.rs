use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_blob_chunks::{
    BlobChunkDedupeAdmission, BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCandidate,
    BlobChunkIdentity, BlobChunkSecurityScope, BlobChunkSecurityScopeDenial,
    BlobChunkStreamingDenial, BlobChunkStreamingOperation, BlobChunkStreamingResidencyProof,
    BlobChunkStreamingWindow, S7BlobChunkSecurityHandoff,
};
use forge_store_contracts::{
    StableDigest, StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

#[test]
fn s5_1_blob_chunk_scope_and_dedupe_readiness_public_api_courtroom() {
    let admitted_blob_scope = blob_scope(
        "cert.s51.blob.scope",
        StoreTenantScope::TenantPhysicalBoundary,
    );
    assert_eq!(
        admitted_blob_scope.key_scope(),
        StoreKeyScope::BlobChunkEnvelope
    );
    assert_eq!(admitted_blob_scope.counters().admitted_scope_consumed(), 1);

    let backup_readiness = blob_readiness_for(
        "cert.s51.blob.backup_tenant",
        StoreTenantScope::BackupRestoreBoundary,
    );
    assert!(matches!(
        S7BlobChunkSecurityHandoff::from_s5_1_readiness(backup_readiness),
        Err(BlobChunkSecurityScopeDenial::WrongTenantScope { counters, .. })
            if counters.denials() == 1
    ));

    let existing = candidate_for_scope(blob_scope(
        "cert.s51.blob.dedupe.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let candidate = candidate_for_scope(blob_scope(
        "cert.s51.blob.dedupe.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(existing, candidate).admit(),
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::MissingFoundationalCanonicalEquivalence { counters }
        ) if counters.digest_only_denials() == 1
    ));

    let tenant_left = candidate_for_scope(blob_scope(
        "cert.s51.blob.tenant.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let tenant_right = candidate_for_scope(blob_scope(
        "cert.s51.blob.tenant.right",
        StoreTenantScope::MultiTenantPhysicalBoundary,
    ));
    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(tenant_left, tenant_right).admit(),
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::CrossTenantScopeRequiresExplicitEquivalence {
                counters,
                ..
            }
        ) if counters.cross_scope_denials() == 1
    ));

    assert!(matches!(
        BlobChunkStreamingResidencyProof::bounded_window(2048, 2048),
        Err(BlobChunkStreamingDenial::WholeObjectResidencyRequired)
    ));
}

fn candidate_for_scope(scope: BlobChunkSecurityScope) -> BlobChunkDedupeCandidate {
    let digest = StableDigest::new("sha256:cert-s51-blob-content").expect("digest");
    let residency =
        BlobChunkStreamingResidencyProof::bounded_window(4096, 1024).expect("bounded residency");
    let window = BlobChunkStreamingWindow::new(
        BlobChunkIdentity::from_digest(digest.clone()),
        digest,
        residency,
    )
    .expect("window should admit");
    let observation = BlobChunkStreamingOperation::ingest(scope)
        .observe_window(window)
        .expect("window should observe")
        .complete_without_whole_object_residency()
        .expect("bounded observation should complete");
    BlobChunkDedupeCandidate::from_streaming_observation(observation)
}

fn blob_scope(identity_key: &str, tenant_scope: StoreTenantScope) -> BlobChunkSecurityScope {
    let handoff = S7BlobChunkSecurityHandoff::from_s5_1_readiness(blob_readiness_for(
        identity_key,
        tenant_scope,
    ))
    .expect("blob handoff should admit");
    BlobChunkSecurityScope::from_s7_handoff(handoff)
}

fn blob_readiness_for(
    identity_key: &str,
    tenant_scope: StoreTenantScope,
) -> forge_store_readiness::S51AdmittedSecurityScopeReadiness {
    accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::blob_chunk(),
        admitted_blob_security_scope(identity_key, tenant_scope),
    )
}

fn admitted_blob_security_scope(
    identity_key: &str,
    tenant_scope: StoreTenantScope,
) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, "chunk-authority");
    let authenticity = StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
    );
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BlobChunkEnvelope,
        tenant_scope,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    );

    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("blob security scope should admit: {outcome:?}"),
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
