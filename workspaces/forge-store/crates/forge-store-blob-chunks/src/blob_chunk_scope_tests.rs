use forge_proof::TransitionOutcome;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeAdmissionDenial, StoreTenantScope,
};

use crate::blob_chunk_test_support::{
    blob_scope, blob_scope_from_parts, candidate_for_scope, candidate_for_scope_with_digest,
    canonical_equivalence, non_blob_family_readiness, security_scope_admission_outcome,
    streaming_window,
};
use crate::{
    BlobChunkCanonicalComparisonBasis, BlobChunkDedupeAdmission, BlobChunkDedupeAdmissionDenial,
    BlobChunkSecurityScope, BlobChunkSecurityScopeDenial, BlobChunkStreamingDenial,
    BlobChunkStreamingOperation, BlobChunkStreamingOperationKind, BlobChunkStreamingResidencyProof,
};

#[test]
fn blob_security_scope_consumes_admitted_key_tenant_authenticity_and_custody_scope() {
    let scope = blob_scope(
        "store.s51.blob.scope",
        StoreTenantScope::TenantPhysicalBoundary,
    );

    assert_eq!(scope.key_scope(), StoreKeyScope::BlobChunkEnvelope);
    assert_eq!(
        scope.authenticity_requirement(),
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk
        )
    );
    assert_eq!(
        scope.custody_posture(),
        StoreCustodyPosture::InternalStoreCustody
    );
    assert_eq!(scope.counters().readiness_inputs(), 1);
    assert_eq!(scope.counters().admitted_scope_consumed(), 1);
}

#[test]
fn blob_readiness_rejects_wrong_family_key_authenticity_tenant_and_custody() {
    assert_scope_denial(
        BlobChunkSecurityScope::from_s5_1_readiness(non_blob_family_readiness(
            "store.s51.blob.non_blob_family",
        )),
        |denial| {
            matches!(
                denial,
                BlobChunkSecurityScopeDenial::WrongReadinessFamily { .. }
            )
        },
    );
    assert_scope_denial(
        blob_scope_from_parts(
            "store.s51.blob.wrong_key",
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        |denial| matches!(denial, BlobChunkSecurityScopeDenial::WrongKeyScope { .. }),
    );
    assert_scope_denial(
        blob_scope_from_parts(
            "store.s51.blob.wrong_auth",
            StoreKeyScope::BlobChunkEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        |denial| {
            matches!(
                denial,
                BlobChunkSecurityScopeDenial::WrongAuthenticityRequirement { .. }
            )
        },
    );

    for tenant_scope in [
        StoreTenantScope::BackupRestoreBoundary,
        StoreTenantScope::RepairBlastRadius,
        StoreTenantScope::ImportReadmissionBoundary,
    ] {
        assert_scope_denial(
            blob_scope_from_parts(
                "store.s51.blob.cross_lane_tenant",
                StoreKeyScope::BlobChunkEnvelope,
                tenant_scope,
                StoreAuthenticityRequirement::required(
                    StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
                ),
                StoreCustodyPosture::InternalStoreCustody,
            ),
            |denial| {
                matches!(
                    denial,
                    BlobChunkSecurityScopeDenial::WrongTenantScope { .. }
                )
            },
        );
    }

    assert!(matches!(
        security_scope_admission_outcome(
            "store.s51.blob.bad_custody",
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
fn same_scope_dedupe_requires_candidate_derived_foundational_equivalence() {
    let existing = candidate_for_scope(blob_scope(
        "store.s51.blob.same_scope",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let candidate = candidate_for_scope(blob_scope(
        "store.s51.blob.same_scope",
        StoreTenantScope::TenantPhysicalBoundary,
    ));

    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(existing, candidate).admit(),
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::MissingFoundationalCanonicalEquivalence { counters }
        ) if counters.digest_comparisons() == 1
            && counters.digest_only_denials() == 1
    ));

    let existing = candidate_for_scope(blob_scope(
        "store.s51.blob.same_scope",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let candidate = candidate_for_scope(blob_scope(
        "store.s51.blob.same_scope",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let equivalence = canonical_equivalence(&existing, &candidate);
    let admitted = BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(equivalence)
        .admit();

    let share_claim = match admitted {
        TransitionOutcome::Success(claim) => claim,
        outcome => panic!("same-scope canonical equivalence should admit: {outcome:?}"),
    };
    assert_eq!(share_claim.counters().digest_comparisons(), 1);
    assert_eq!(
        share_claim
            .counters()
            .foundational_equivalence_comparisons(),
        1
    );
    assert_eq!(share_claim.counters().same_scope_admissions(), 1);
}

#[test]
fn dedupe_equivalence_cannot_be_reused_or_cross_scope_collapsed() {
    let existing = candidate_for_scope(blob_scope(
        "store.s51.blob.bound.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let candidate = candidate_for_scope(blob_scope(
        "store.s51.blob.bound.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let equivalence = canonical_equivalence(&existing, &candidate);

    let unrelated_existing = candidate_for_scope_with_digest(
        blob_scope(
            "store.s51.blob.bound.right",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        "sha256:blob-s51-other-content",
    );
    let unrelated_candidate = candidate_for_scope_with_digest(
        blob_scope(
            "store.s51.blob.bound.right",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        "sha256:blob-s51-other-content",
    );

    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(unrelated_existing, unrelated_candidate)
            .with_foundational_canonical_equivalence(equivalence)
            .admit(),
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::UnboundFoundationalEquivalence { counters }
        ) if counters.foundational_equivalence_comparisons() == 1
            && counters.cross_scope_denials() == 1
    ));

    let tenant_left = candidate_for_scope(blob_scope(
        "store.s51.blob.tenant.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let tenant_right = candidate_for_scope(blob_scope(
        "store.s51.blob.tenant.right",
        StoreTenantScope::MultiTenantPhysicalBoundary,
    ));
    assert!(matches!(
        BlobChunkCanonicalComparisonBasis::from_candidates(&tenant_left, &tenant_right),
        Err(BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch { counters })
            if counters.cross_scope_denials() == 1
    ));
    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(tenant_left, tenant_right).admit(),
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::CrossTenantScopeRequiresExplicitEquivalence {
                counters,
                ..
            }
        ) if counters.digest_comparisons() == 1
            && counters.cross_scope_denials() == 1
    ));
}

#[test]
fn streaming_operations_preserve_scope_with_bounded_residency() {
    let operations = [
        (
            BlobChunkStreamingOperationKind::Ingest,
            BlobChunkStreamingOperation::ingest
                as fn(BlobChunkSecurityScope) -> BlobChunkStreamingOperation,
        ),
        (
            BlobChunkStreamingOperationKind::Verification,
            BlobChunkStreamingOperation::verification
                as fn(BlobChunkSecurityScope) -> BlobChunkStreamingOperation,
        ),
        (
            BlobChunkStreamingOperationKind::ExportReadPreparation,
            BlobChunkStreamingOperation::export_read_preparation
                as fn(BlobChunkSecurityScope) -> BlobChunkStreamingOperation,
        ),
        (
            BlobChunkStreamingOperationKind::TierMovement,
            BlobChunkStreamingOperation::tier_movement
                as fn(BlobChunkSecurityScope) -> BlobChunkStreamingOperation,
        ),
        (
            BlobChunkStreamingOperationKind::ReclaimPreparation,
            BlobChunkStreamingOperation::reclaim_preparation
                as fn(BlobChunkSecurityScope) -> BlobChunkStreamingOperation,
        ),
    ];

    for (kind, operation) in operations {
        let scope = blob_scope(
            "store.s51.blob.streaming",
            StoreTenantScope::TenantPhysicalBoundary,
        );
        let original_identity = scope.identity();
        let observation = operation(scope)
            .observe_window(streaming_window())
            .expect("window should observe")
            .complete_without_whole_object_residency()
            .expect("single-window operation should complete");

        assert_eq!(observation.kind(), kind);
        assert_eq!(observation.scope().identity(), original_identity);
        assert_eq!(observation.window().residency().object_bytes(), 4096);
        assert_eq!(observation.window().residency().window_bytes(), 1024);
        assert_eq!(observation.counters().windows_observed(), 1);
        assert_eq!(observation.counters().max_resident_windows(), 1);
    }
}

#[test]
fn streaming_window_rejects_whole_object_and_oversized_residency() {
    assert!(matches!(
        BlobChunkStreamingResidencyProof::bounded_window(1024, 1024),
        Err(BlobChunkStreamingDenial::WholeObjectResidencyRequired)
    ));
    assert!(matches!(
        BlobChunkStreamingResidencyProof::bounded_window(1024, 2048),
        Err(BlobChunkStreamingDenial::WholeObjectResidencyRequired)
    ));
}

fn assert_scope_denial(
    outcome: Result<BlobChunkSecurityScope, BlobChunkSecurityScopeDenial>,
    matches_expected: impl FnOnce(&BlobChunkSecurityScopeDenial) -> bool,
) {
    let denial = outcome.expect_err("scope should deny");
    assert!(matches_expected(&denial));
    match denial {
        BlobChunkSecurityScopeDenial::WrongReadinessFamily { counters, .. }
        | BlobChunkSecurityScopeDenial::WrongKeyScope { counters, .. }
        | BlobChunkSecurityScopeDenial::WrongTenantScope { counters, .. }
        | BlobChunkSecurityScopeDenial::WrongAuthenticityRequirement { counters, .. }
        | BlobChunkSecurityScopeDenial::UnsupportedCustodyPosture { counters, .. }
        | BlobChunkSecurityScopeDenial::StaleKeyVersionPosture { counters, .. }
        | BlobChunkSecurityScopeDenial::IdentityProviderClaimRejected { counters }
        | BlobChunkSecurityScopeDenial::ApplicationOrgClaimRejected { counters }
        | BlobChunkSecurityScopeDenial::KmsKeyIdentifierRejected { counters }
        | BlobChunkSecurityScopeDenial::IamRoleClaimRejected { counters }
        | BlobChunkSecurityScopeDenial::OperatorIdentityRejected { counters }
        | BlobChunkSecurityScopeDenial::DeserializedMetadataRequiresReadmission { counters } => {
            assert_eq!(counters.readiness_inputs(), 1);
            assert_eq!(counters.denials(), 1);
        }
    }
}
