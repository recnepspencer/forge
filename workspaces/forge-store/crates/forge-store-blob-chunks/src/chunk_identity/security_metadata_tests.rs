use forge_proof::TransitionOutcome;
use forge_store_security::{
    StoreApplicationOrgIdClaim, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreIamRoleClaim, StoreJwtSubjectClaim, StoreKeyScope,
    StoreKeyVersionPosture, StoreKmsKeyIdentifier, StoreOperatorIdentityClaim,
    StoreSecurityScopeAdmissionDenial, StoreTenantScope,
};

use crate::test_support::{
    blob_scope, blob_scope_from_parts, candidate_for_scope, deserialized_blob_scope_declaration,
    integrity_proof_for_scope, security_scope_admission_outcome,
};
use crate::{
    reject_application_org_claim_as_blob_chunk_security_scope,
    reject_deserialized_metadata_as_blob_chunk_security_scope,
    reject_iam_role_as_blob_chunk_security_scope, reject_jwt_claim_as_blob_chunk_security_scope,
    reject_kms_key_id_as_blob_chunk_security_scope,
    reject_operator_identity_as_blob_chunk_security_scope, BlobChunkCanonicalComparisonBasis,
    BlobChunkDedupeAdmission, BlobChunkDedupeAdmissionDenial, BlobChunkSecurityScopeDenial,
    BlobReachabilityProof, ScopedBlobChunk,
};

#[test]
fn admitted_chunk_proofs_preserve_security_metadata_witness() {
    let scope = blob_scope(
        "store.s7.phase2.metadata",
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let metadata = scope.metadata();
    let proof = integrity_proof_for_scope(scope, b"phase2-metadata");
    let scoped = ScopedBlobChunk::from_integrity_proof(proof);
    let reachability = BlobReachabilityProof::from_scoped_chunk(scoped);

    assert_eq!(metadata.key_scope(), StoreKeyScope::BlobChunkEnvelope);
    assert_eq!(
        metadata.key_version_posture(),
        StoreKeyVersionPosture::Current
    );
    assert_eq!(
        metadata.tenant_scope(),
        StoreTenantScope::TenantPhysicalBoundary
    );
    assert_eq!(
        metadata.authenticity_requirement(),
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk
        )
    );
    assert_eq!(
        metadata.custody_posture(),
        StoreCustodyPosture::InternalStoreCustody
    );
    assert_eq!(reachability.security_metadata(), metadata);
    assert_eq!(metadata.counters().key_scope_preservations(), 1);
    assert_eq!(metadata.counters().key_version_preservations(), 1);
    assert_eq!(metadata.counters().tenant_scope_preservations(), 1);
    assert_eq!(metadata.counters().authenticity_preservations(), 1);
    assert_eq!(metadata.counters().custody_preservations(), 1);
    assert_eq!(metadata.counters().metadata_witnesses_issued(), 1);
}

#[test]
fn dedupe_admission_preserves_security_metadata_witness() {
    let existing_scope = blob_scope(
        "store.s7.phase2.dedupe.same",
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let expected_metadata = existing_scope.metadata();
    let existing = candidate_for_scope(existing_scope);
    let candidate = candidate_for_scope(blob_scope(
        "store.s7.phase2.dedupe.same",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let basis = BlobChunkCanonicalComparisonBasis::from_candidates(&existing, &candidate)
        .expect("same metadata candidate comparison should prepare");
    let equivalence = basis
        .evaluate_foundational_equivalence()
        .expect("same metadata candidate comparison should evaluate");

    assert_eq!(existing.security_metadata(), expected_metadata);
    assert_eq!(candidate.security_metadata(), expected_metadata);

    let outcome = BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(equivalence)
        .admit();
    let share_claim = match outcome {
        TransitionOutcome::Success(claim) => claim,
        other => panic!("metadata-bound dedupe should admit: {other:?}"),
    };

    assert_eq!(share_claim.security_metadata(), expected_metadata);
    assert_eq!(
        share_claim.security_metadata().key_version_posture(),
        StoreKeyVersionPosture::Current
    );
}

#[test]
fn dedupe_denial_uses_metadata_witness_not_identity_summary() {
    let tenant_left = candidate_for_scope(blob_scope(
        "store.s7.phase2.dedupe.tenant.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let tenant_right = candidate_for_scope(blob_scope(
        "store.s7.phase2.dedupe.tenant.right",
        StoreTenantScope::MultiTenantPhysicalBoundary,
    ));

    assert!(matches!(
        BlobChunkCanonicalComparisonBasis::from_candidates(&tenant_left, &tenant_right),
        Err(BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch { counters })
            if counters.cross_scope_denials() == 1
    ));
    assert_ne!(
        tenant_left.security_metadata(),
        tenant_right.security_metadata()
    );
}

#[test]
fn stale_key_version_and_wrong_scope_fail_before_chunk_witness() {
    assert!(matches!(
        security_scope_admission_outcome(
            "store.s7.phase2.stale",
            StoreKeyScope::BlobChunkEnvelope,
            StoreKeyVersionPosture::Stale,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        TransitionOutcome::Stale(_)
    ));
    assert!(matches!(
        blob_scope_from_parts(
            "store.s7.phase2.wrong_key",
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        Err(BlobChunkSecurityScopeDenial::WrongKeyScope { .. })
    ));
    assert!(matches!(
        blob_scope_from_parts(
            "store.s7.phase2.wrong_tenant",
            StoreKeyScope::BlobChunkEnvelope,
            StoreTenantScope::BackupRestoreBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        Err(BlobChunkSecurityScopeDenial::WrongTenantScope { .. })
    ));
    assert!(matches!(
        security_scope_admission_outcome(
            "store.s7.phase2.unsupported_custody",
            StoreKeyScope::BlobChunkEnvelope,
            StoreKeyVersionPosture::Current,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
            ),
            StoreCustodyPosture::CustodyUnsupported,
        ),
        TransitionOutcome::Denied(StoreSecurityScopeAdmissionDenial::UnsupportedCustodyPosture)
    ));
}

#[test]
fn hostile_metadata_sources_are_rejected_before_chunk_witness() {
    assert_hostile_denial(reject_jwt_claim_as_blob_chunk_security_scope(
        StoreJwtSubjectClaim::raw("jwt-subject"),
    ));
    assert_hostile_denial(reject_application_org_claim_as_blob_chunk_security_scope(
        StoreApplicationOrgIdClaim::raw("org"),
    ));
    assert_hostile_denial(reject_kms_key_id_as_blob_chunk_security_scope(
        StoreKmsKeyIdentifier::raw("kms-key"),
    ));
    assert_hostile_denial(reject_iam_role_as_blob_chunk_security_scope(
        StoreIamRoleClaim::raw("iam-role"),
    ));
    assert_hostile_denial(reject_operator_identity_as_blob_chunk_security_scope(
        StoreOperatorIdentityClaim::raw("operator"),
    ));
    assert_hostile_denial(reject_deserialized_metadata_as_blob_chunk_security_scope(
        deserialized_blob_scope_declaration("store.s7.phase2.deserialize"),
    ));
}

fn assert_hostile_denial(denial: BlobChunkSecurityScopeDenial) {
    match denial {
        BlobChunkSecurityScopeDenial::IdentityProviderClaimRejected { counters }
        | BlobChunkSecurityScopeDenial::ApplicationOrgClaimRejected { counters }
        | BlobChunkSecurityScopeDenial::KmsKeyIdentifierRejected { counters }
        | BlobChunkSecurityScopeDenial::IamRoleClaimRejected { counters }
        | BlobChunkSecurityScopeDenial::OperatorIdentityRejected { counters }
        | BlobChunkSecurityScopeDenial::DeserializedMetadataRequiresReadmission { counters } => {
            assert_eq!(counters.denials(), 1);
            assert_eq!(counters.hostile_metadata_denials(), 1);
        }
        other => panic!("expected hostile metadata denial, got {other:?}"),
    }
}
