use worth_foundational::canonicalization_api::lower_lane::comparison::CanonicalComparisonOutcome;
use worth_proof::TransitionOutcome;

use crate::{
    admit_store_security_scope, compare_store_physical_security_metadata,
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StorePhysicalSecurityMetadataCarrier, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use super::support::current_authority;

#[test]
fn equivalent_physical_metadata_uses_full_canonical_basis() {
    let left = metadata_with(
        "left-equivalent",
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
    let right = metadata_with(
        "right-equivalent",
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );

    assert_canonical_equivalent(left, right);
}

#[test]
fn physical_metadata_canonical_basis_distinguishes_every_metadata_family() {
    let native = metadata_with(
        "native",
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );

    for mismatched in [
        metadata_with(
            "key-scope",
            StoreKeyScope::WalCheckpointEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
            StoreLegacySecurityPosture::NativeScoped,
            StoreKeyVersionPosture::Current,
        ),
        metadata_with(
            "tenant-scope",
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::StoreInternal,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
            StoreLegacySecurityPosture::NativeScoped,
            StoreKeyVersionPosture::Current,
        ),
        metadata_with(
            "authenticity-requirement",
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
            StoreLegacySecurityPosture::NativeScoped,
            StoreKeyVersionPosture::Current,
        ),
        metadata_with(
            "authenticity-class",
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedManifest,
            ),
            StoreCustodyPosture::InternalStoreCustody,
            StoreLegacySecurityPosture::NativeScoped,
            StoreKeyVersionPosture::Current,
        ),
        metadata_with(
            "custody",
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::ExportPrepared,
            StoreLegacySecurityPosture::NativeScoped,
            StoreKeyVersionPosture::Current,
        ),
        metadata_with(
            "legacy",
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
            StoreLegacySecurityPosture::LegacyUnscoped,
            StoreKeyVersionPosture::Current,
        ),
        metadata_with(
            "key-version",
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
            StoreLegacySecurityPosture::NativeScoped,
            StoreKeyVersionPosture::Stale,
        ),
    ] {
        assert_canonical_mismatch(native, mismatched);
    }
}

fn metadata_with(
    label: &str,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
    legacy_posture: StoreLegacySecurityPosture,
    key_version_posture: StoreKeyVersionPosture,
) -> StorePhysicalSecurityMetadataCarrier {
    let authority = current_authority("s51.phase3.canonical", label);
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        key_scope,
        tenant_scope,
        authenticity_requirement,
        custody_posture,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity_requirement,
        custody_posture,
        expectation,
    );
    let admitted = match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("custom metadata scope should admit: {outcome:?}"),
    };
    let witnesses = admitted.into_witnesses_for_readiness_handoff();
    StorePhysicalSecurityMetadataCarrier::from_current_security_scope(
        &witnesses,
        key_version_posture,
        legacy_posture,
    )
}

fn assert_canonical_equivalent(
    left: StorePhysicalSecurityMetadataCarrier,
    right: StorePhysicalSecurityMetadataCarrier,
) {
    assert!(matches!(
        compare_store_physical_security_metadata(left, right),
        TransitionOutcome::Success(CanonicalComparisonOutcome::Equivalent(_))
    ));
}

fn assert_canonical_mismatch(
    left: StorePhysicalSecurityMetadataCarrier,
    right: StorePhysicalSecurityMetadataCarrier,
) {
    assert!(matches!(
        compare_store_physical_security_metadata(left, right),
        TransitionOutcome::Success(CanonicalComparisonOutcome::Mismatched(_))
    ));
}
