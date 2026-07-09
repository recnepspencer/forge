use worth_foundational::canonicalization_api::lower_lane::{
    basis::{
        prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
        CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
        CanonicalBasisReadyArtifact, CanonicalBasisValue, CanonicalIntegerWidth,
        CanonicalizationRuleVersion,
    },
    comparison::{
        compare_canonical_basis, prepare_canonical_comparison, CanonicalComparisonOutcome,
        CanonicalEquivalenceBasis,
    },
};
use worth_foundational::InternedString;
use worth_proof::TransitionOutcome;

use crate::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StorePhysicalSecurityMetadataCarrier, StoreTenantScope,
};

#[derive(Debug, Clone)]
pub struct StorePhysicalSecurityMetadataCanonicalBasis {
    ready: CanonicalBasisReadyArtifact,
}

impl StorePhysicalSecurityMetadataCanonicalBasis {
    pub fn from_metadata(
        metadata: StorePhysicalSecurityMetadataCarrier,
    ) -> TransitionOutcome<Self, CanonicalBasisConstructionDenial> {
        match prepare_canonical_basis_sequence(
            CanonicalizationRuleVersion::new("store.s5.1.physical-security-metadata")
                .expect("static canonicalization rule version"),
            CanonicalBasisDomain::BoundaryArtifact,
            physical_metadata_canonical_entries(metadata),
        ) {
            TransitionOutcome::Success(ready) => TransitionOutcome::success(Self { ready }),
            TransitionOutcome::Denied(denial) => TransitionOutcome::denied(denial),
            TransitionOutcome::Deferred(deferred) => TransitionOutcome::deferred(deferred),
        }
    }

    pub const fn ready(&self) -> &CanonicalBasisReadyArtifact {
        &self.ready
    }
}

pub fn compare_store_physical_security_metadata(
    left: StorePhysicalSecurityMetadataCarrier,
    right: StorePhysicalSecurityMetadataCarrier,
) -> TransitionOutcome<CanonicalComparisonOutcome, CanonicalBasisConstructionDenial> {
    let left = match StorePhysicalSecurityMetadataCanonicalBasis::from_metadata(left) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
        TransitionOutcome::Deferred(deferred) => return TransitionOutcome::deferred(deferred),
    };
    let right = match StorePhysicalSecurityMetadataCanonicalBasis::from_metadata(right) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
        TransitionOutcome::Deferred(deferred) => return TransitionOutcome::deferred(deferred),
    };
    let comparison = match prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        left.ready,
        right.ready,
    ) {
        TransitionOutcome::Success(comparison) => comparison,
        TransitionOutcome::Denied(denial) => match denial {},
        TransitionOutcome::Deferred(deferred) => match deferred {},
        TransitionOutcome::Stale(stale) => match stale {},
        TransitionOutcome::RebindRequired(rebind) => match rebind {},
        TransitionOutcome::Failed(failed) => match failed {},
    };
    TransitionOutcome::success(compare_canonical_basis(&comparison))
}

fn physical_metadata_canonical_entries(
    metadata: StorePhysicalSecurityMetadataCarrier,
) -> [CanonicalBasisEntry; 7] {
    [
        canonical_u64_entry("key_scope", key_scope_tag(metadata.key_scope())),
        canonical_u64_entry("tenant_scope", tenant_scope_tag(metadata.tenant_scope())),
        canonical_u64_entry(
            "authenticity_requirement",
            authenticity_requirement_tag(metadata.authenticity_requirement()),
        ),
        canonical_u64_entry(
            "authenticity_requirement_class",
            authenticity_requirement_class_tag(metadata.authenticity_requirement().class()),
        ),
        canonical_u64_entry(
            "custody_posture",
            custody_posture_tag(metadata.custody_posture()),
        ),
        canonical_u64_entry(
            "legacy_posture",
            legacy_posture_tag(metadata.legacy_posture()),
        ),
        canonical_u64_entry(
            "key_version_posture",
            key_version_posture_tag(metadata.key_version_posture()),
        ),
    ]
}

fn canonical_u64_entry(locus: &'static str, value: u64) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::BoundaryArtifact,
        CanonicalBasisLocus::Named(InternedString::from(locus)),
        CanonicalBasisEntryKind::BoundaryAttachment,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value as u128,
        },
    )
}

const fn key_scope_tag(scope: StoreKeyScope) -> u64 {
    match scope {
        StoreKeyScope::StoreManagedRoot => 1,
        StoreKeyScope::TenantEnvelope => 2,
        StoreKeyScope::ArtifactEnvelope => 3,
        StoreKeyScope::PageEnvelope => 4,
        StoreKeyScope::WalCheckpointEnvelope => 5,
        StoreKeyScope::BlobChunkEnvelope => 6,
        StoreKeyScope::BackupExportEnvelope => 7,
        StoreKeyScope::RepairScopeEnvelope => 8,
        StoreKeyScope::SecurityLifecycleFoundation => 9,
    }
}

const fn tenant_scope_tag(scope: StoreTenantScope) -> u64 {
    match scope {
        StoreTenantScope::StoreInternal => 1,
        StoreTenantScope::TenantPhysicalBoundary => 2,
        StoreTenantScope::MultiTenantPhysicalBoundary => 3,
        StoreTenantScope::BackupRestoreBoundary => 4,
        StoreTenantScope::RepairBlastRadius => 5,
        StoreTenantScope::ImportReadmissionBoundary => 6,
        StoreTenantScope::SecurityLifecycleFoundation => 7,
    }
}

const fn authenticity_requirement_tag(requirement: StoreAuthenticityRequirement) -> u64 {
    match requirement {
        StoreAuthenticityRequirement::NotRequired => 1,
        StoreAuthenticityRequirement::Required(_) => 2,
    }
}

const fn authenticity_requirement_class_tag(
    class: Option<StoreAuthenticityRequirementClass>,
) -> u64 {
    match class {
        None => 0,
        Some(StoreAuthenticityRequirementClass::AuthenticatedFrame) => 1,
        Some(StoreAuthenticityRequirementClass::AuthenticatedWalRecord) => 2,
        Some(StoreAuthenticityRequirementClass::AuthenticatedManifest) => 3,
        Some(StoreAuthenticityRequirementClass::AuthenticatedBlobChunk) => 4,
        Some(StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule) => 5,
        Some(StoreAuthenticityRequirementClass::AuthenticatedRepairRead) => 6,
    }
}

const fn custody_posture_tag(posture: StoreCustodyPosture) -> u64 {
    match posture {
        StoreCustodyPosture::InternalStoreCustody => 1,
        StoreCustodyPosture::ExportPrepared => 2,
        StoreCustodyPosture::ExportedOutOfCustody => 3,
        StoreCustodyPosture::ImportedUnreadmitted => 4,
        StoreCustodyPosture::Readmitted => 5,
        StoreCustodyPosture::CustodyUnavailable => 6,
        StoreCustodyPosture::CustodyDenied => 7,
        StoreCustodyPosture::CustodyUnsupported => 8,
    }
}

const fn legacy_posture_tag(posture: StoreLegacySecurityPosture) -> u64 {
    match posture {
        StoreLegacySecurityPosture::NativeScoped => 1,
        StoreLegacySecurityPosture::LegacyUnscoped => 2,
        StoreLegacySecurityPosture::ReadmissionRequired => 3,
        StoreLegacySecurityPosture::SecurityMetadataUnavailable => 4,
        StoreLegacySecurityPosture::UnsupportedLegacyArtifact => 5,
    }
}

const fn key_version_posture_tag(posture: StoreKeyVersionPosture) -> u64 {
    match posture {
        StoreKeyVersionPosture::Current => 1,
        StoreKeyVersionPosture::Stale => 2,
        StoreKeyVersionPosture::RebindRequired => 3,
        StoreKeyVersionPosture::Unsupported => 4,
        StoreKeyVersionPosture::Unavailable => 5,
        StoreKeyVersionPosture::Denied => 6,
    }
}
