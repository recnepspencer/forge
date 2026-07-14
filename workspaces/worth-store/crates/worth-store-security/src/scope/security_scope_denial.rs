use crate::StoreSecurityAuthoritySource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeDenial {
    kind: StoreSecurityScopeDenialKind,
    source: StoreSecurityAuthoritySource,
}

impl StoreSecurityScopeDenial {
    pub const fn new(
        kind: StoreSecurityScopeDenialKind,
        source: StoreSecurityAuthoritySource,
    ) -> Self {
        Self { kind, source }
    }

    pub const fn kind(self) -> StoreSecurityScopeDenialKind {
        self.kind
    }

    pub const fn source(self) -> StoreSecurityAuthoritySource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopeDenialKind {
    RawStringIsNotSecurityScope,
    SemanticIdIsNotSecurityScope,
    TerminalJsonLabelIsNotSecurityScope,
    JwtSubjectIsNotTenantScope,
    ApplicationOrgIdIsNotTenantScope,
    KmsKeyIdIsNotKeyScope,
    IamRoleIsNotCustodyPosture,
    OperatorIdentityIsNotRepairAuthority,
    AuditRecordIsNotRepairAuthority,
    OfflineVerifierEvidenceIsNotRepairAuthority,
    FoundationalEvidenceIsNotStoreSecurityAuthority,
    ProofProgressionIsNotStoreSecurityAuthority,
    StoreCurrentAuthorityWitnessIsNotSecurityScopeAuthority,
}

pub const fn reject_non_store_security_scope_source(
    source: StoreSecurityAuthoritySource,
) -> StoreSecurityScopeDenial {
    let kind = match source {
        StoreSecurityAuthoritySource::RawString => {
            StoreSecurityScopeDenialKind::RawStringIsNotSecurityScope
        }
        StoreSecurityAuthoritySource::SemanticId => {
            StoreSecurityScopeDenialKind::SemanticIdIsNotSecurityScope
        }
        StoreSecurityAuthoritySource::TerminalJsonLabel => {
            StoreSecurityScopeDenialKind::TerminalJsonLabelIsNotSecurityScope
        }
        StoreSecurityAuthoritySource::JwtSubjectClaim => {
            StoreSecurityScopeDenialKind::JwtSubjectIsNotTenantScope
        }
        StoreSecurityAuthoritySource::ApplicationOrgId => {
            StoreSecurityScopeDenialKind::ApplicationOrgIdIsNotTenantScope
        }
        StoreSecurityAuthoritySource::KmsKeyId => {
            StoreSecurityScopeDenialKind::KmsKeyIdIsNotKeyScope
        }
        StoreSecurityAuthoritySource::IamRole => {
            StoreSecurityScopeDenialKind::IamRoleIsNotCustodyPosture
        }
        StoreSecurityAuthoritySource::OperatorIdentity => {
            StoreSecurityScopeDenialKind::OperatorIdentityIsNotRepairAuthority
        }
        StoreSecurityAuthoritySource::AuditRecord => {
            StoreSecurityScopeDenialKind::AuditRecordIsNotRepairAuthority
        }
        StoreSecurityAuthoritySource::OfflineVerifierEvidence => {
            StoreSecurityScopeDenialKind::OfflineVerifierEvidenceIsNotRepairAuthority
        }
        StoreSecurityAuthoritySource::FoundationalEvidence => {
            StoreSecurityScopeDenialKind::FoundationalEvidenceIsNotStoreSecurityAuthority
        }
        StoreSecurityAuthoritySource::ProofProgression => {
            StoreSecurityScopeDenialKind::ProofProgressionIsNotStoreSecurityAuthority
        }
        StoreSecurityAuthoritySource::StoreCurrentAuthorityWitnessOnly => {
            StoreSecurityScopeDenialKind::StoreCurrentAuthorityWitnessIsNotSecurityScopeAuthority
        }
    };
    StoreSecurityScopeDenial::new(kind, source)
}
