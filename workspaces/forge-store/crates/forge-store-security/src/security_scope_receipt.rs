use crate::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionCounterSnapshot, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeDeclarationProvenance, StoreSecurityScopeIdentity, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeProofProgressionIdentity {
    declared_scope_fingerprint: u64,
    expected_scope_fingerprint: u64,
    progression_fingerprint: u64,
}

impl StoreSecurityScopeProofProgressionIdentity {
    pub(crate) fn from_admission_inputs(
        declaration: StoreRawSecurityScopeDeclaration,
        expectation: StoreSecurityScopeAdmissionExpectation,
    ) -> Self {
        let declared_scope_fingerprint = declaration_fingerprint(declaration);
        let expected_scope_fingerprint = expectation_fingerprint(expectation);
        Self {
            declared_scope_fingerprint,
            expected_scope_fingerprint,
            progression_fingerprint: mix(declared_scope_fingerprint, expected_scope_fingerprint),
        }
    }

    pub const fn declared_scope_fingerprint(self) -> u64 {
        self.declared_scope_fingerprint
    }

    pub const fn expected_scope_fingerprint(self) -> u64 {
        self.expected_scope_fingerprint
    }

    pub const fn progression_fingerprint(self) -> u64 {
        self.progression_fingerprint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeAdmissionReceiptId {
    admission_sequence: u64,
    security_scope_fingerprint: u64,
    proof_progression_fingerprint: u64,
}

impl StoreSecurityScopeAdmissionReceiptId {
    pub const fn admission_sequence(self) -> u64 {
        self.admission_sequence
    }

    pub const fn security_scope_fingerprint(self) -> u64 {
        self.security_scope_fingerprint
    }

    pub const fn proof_progression_fingerprint(self) -> u64 {
        self.proof_progression_fingerprint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeAdmissionReceipt {
    receipt_id: StoreSecurityScopeAdmissionReceiptId,
    identity: StoreSecurityScopeIdentity,
    proof_progression_identity: StoreSecurityScopeProofProgressionIdentity,
    counters: StoreSecurityScopeAdmissionCounterSnapshot,
}

impl StoreSecurityScopeAdmissionReceipt {
    pub(crate) const fn new(
        identity: StoreSecurityScopeIdentity,
        proof_progression_identity: StoreSecurityScopeProofProgressionIdentity,
        counters: StoreSecurityScopeAdmissionCounterSnapshot,
    ) -> Self {
        Self {
            receipt_id: StoreSecurityScopeAdmissionReceiptId {
                admission_sequence: counters.requests(),
                security_scope_fingerprint: identity_fingerprint(identity),
                proof_progression_fingerprint: proof_progression_identity.progression_fingerprint(),
            },
            identity,
            proof_progression_identity,
            counters,
        }
    }

    pub const fn receipt_id(self) -> StoreSecurityScopeAdmissionReceiptId {
        self.receipt_id
    }

    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn proof_progression_identity(self) -> StoreSecurityScopeProofProgressionIdentity {
        self.proof_progression_identity
    }

    pub const fn counters(self) -> StoreSecurityScopeAdmissionCounterSnapshot {
        self.counters
    }
}

const fn identity_fingerprint(identity: StoreSecurityScopeIdentity) -> u64 {
    mix(
        mix(
            key_scope_tag(identity.key_scope()),
            tenant_scope_tag(identity.tenant_scope()),
        ),
        mix(
            mix(
                key_version_posture_tag(identity.key_version_posture()),
                authenticity_requirement_tag(identity.authenticity_requirement()),
            ),
            custody_posture_tag(identity.custody_posture()),
        ),
    )
}

fn declaration_fingerprint(declaration: StoreRawSecurityScopeDeclaration) -> u64 {
    mix(
        mix(
            key_scope_tag(declaration.key_scope()),
            tenant_scope_tag(declaration.tenant_scope()),
        ),
        mix(
            mix(
                key_version_posture_tag(declaration.key_version_posture()),
                option_authenticity_requirement_tag(declaration.authenticity_requirement()),
            ),
            mix(
                option_custody_posture_tag(declaration.custody_posture()),
                provenance_tag(declaration.provenance()),
            ),
        ),
    )
}

fn expectation_fingerprint(expectation: StoreSecurityScopeAdmissionExpectation) -> u64 {
    mix(
        mix(
            key_scope_tag(expectation.key_scope()),
            tenant_scope_tag(expectation.tenant_scope()),
        ),
        mix(
            authenticity_requirement_tag(expectation.authenticity_requirement()),
            custody_posture_tag(expectation.custody_posture()),
        ),
    )
}

const fn mix(left: u64, right: u64) -> u64 {
    left.wrapping_mul(1_099_511_628_211).wrapping_add(right)
}

const fn key_scope_tag(scope: StoreKeyScope) -> u64 {
    match scope {
        StoreKeyScope::StoreManagedRoot => 11,
        StoreKeyScope::TenantEnvelope => 12,
        StoreKeyScope::ArtifactEnvelope => 13,
        StoreKeyScope::PageEnvelope => 14,
        StoreKeyScope::WalCheckpointEnvelope => 15,
        StoreKeyScope::BlobChunkEnvelope => 16,
        StoreKeyScope::BackupExportEnvelope => 17,
        StoreKeyScope::RepairScopeEnvelope => 18,
        StoreKeyScope::SecurityLifecycleFoundation => 19,
    }
}

const fn key_version_posture_tag(posture: StoreKeyVersionPosture) -> u64 {
    match posture {
        StoreKeyVersionPosture::Current => 71,
        StoreKeyVersionPosture::Stale => 72,
        StoreKeyVersionPosture::RebindRequired => 73,
        StoreKeyVersionPosture::Unsupported => 74,
        StoreKeyVersionPosture::Unavailable => 75,
        StoreKeyVersionPosture::Denied => 76,
    }
}

const fn tenant_scope_tag(scope: StoreTenantScope) -> u64 {
    match scope {
        StoreTenantScope::StoreInternal => 21,
        StoreTenantScope::TenantPhysicalBoundary => 22,
        StoreTenantScope::MultiTenantPhysicalBoundary => 23,
        StoreTenantScope::BackupRestoreBoundary => 24,
        StoreTenantScope::RepairBlastRadius => 25,
        StoreTenantScope::ImportReadmissionBoundary => 26,
        StoreTenantScope::SecurityLifecycleFoundation => 27,
    }
}

const fn authenticity_requirement_tag(requirement: StoreAuthenticityRequirement) -> u64 {
    match requirement {
        StoreAuthenticityRequirement::NotRequired => 31,
        StoreAuthenticityRequirement::Required(class) => 40 + authenticity_class_tag(class),
    }
}

const fn option_authenticity_requirement_tag(
    requirement: Option<StoreAuthenticityRequirement>,
) -> u64 {
    match requirement {
        Some(requirement) => authenticity_requirement_tag(requirement),
        None => 30,
    }
}

const fn authenticity_class_tag(class: StoreAuthenticityRequirementClass) -> u64 {
    match class {
        StoreAuthenticityRequirementClass::AuthenticatedFrame => 1,
        StoreAuthenticityRequirementClass::AuthenticatedWalRecord => 2,
        StoreAuthenticityRequirementClass::AuthenticatedManifest => 3,
        StoreAuthenticityRequirementClass::AuthenticatedBlobChunk => 4,
        StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule => 5,
        StoreAuthenticityRequirementClass::AuthenticatedRepairRead => 6,
    }
}

const fn custody_posture_tag(posture: StoreCustodyPosture) -> u64 {
    match posture {
        StoreCustodyPosture::InternalStoreCustody => 51,
        StoreCustodyPosture::ExportPrepared => 52,
        StoreCustodyPosture::ExportedOutOfCustody => 53,
        StoreCustodyPosture::ImportedUnreadmitted => 54,
        StoreCustodyPosture::Readmitted => 55,
        StoreCustodyPosture::CustodyUnavailable => 56,
        StoreCustodyPosture::CustodyDenied => 57,
        StoreCustodyPosture::CustodyUnsupported => 58,
    }
}

const fn option_custody_posture_tag(posture: Option<StoreCustodyPosture>) -> u64 {
    match posture {
        Some(posture) => custody_posture_tag(posture),
        None => 50,
    }
}

const fn provenance_tag(provenance: StoreSecurityScopeDeclarationProvenance) -> u64 {
    match provenance {
        StoreSecurityScopeDeclarationProvenance::NativeStoreDeclaration => 61,
        StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => 62,
        StoreSecurityScopeDeclarationProvenance::StoreReadmitted => 63,
        StoreSecurityScopeDeclarationProvenance::ReplayedAdmissionEvidence => 64,
    }
}
