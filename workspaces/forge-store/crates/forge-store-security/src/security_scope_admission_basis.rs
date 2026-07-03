use crate::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreRawSecurityScopeDeclaration, StoreSecurityScopeProofProgressionIdentity, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeAdmissionExpectation {
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
}

impl StoreSecurityScopeAdmissionExpectation {
    pub const fn platform_page_envelope() -> Self {
        Self {
            key_scope: StoreKeyScope::PageEnvelope,
            tenant_scope: StoreTenantScope::TenantPhysicalBoundary,
            authenticity_requirement: StoreAuthenticityRequirement::required(
                crate::StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            custody_posture: StoreCustodyPosture::InternalStoreCustody,
        }
    }

    pub const fn new(
        key_scope: StoreKeyScope,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        custody_posture: StoreCustodyPosture,
    ) -> Self {
        Self {
            key_scope,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
        }
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }

    pub const fn custody_posture(self) -> StoreCustodyPosture {
        self.custody_posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeAdmissionBasis {
    declaration: StoreRawSecurityScopeDeclaration,
    expectation: StoreSecurityScopeAdmissionExpectation,
    proof_progression_identity: StoreSecurityScopeProofProgressionIdentity,
}

impl StoreSecurityScopeAdmissionBasis {
    pub fn new(
        declaration: StoreRawSecurityScopeDeclaration,
        expectation: StoreSecurityScopeAdmissionExpectation,
    ) -> Self {
        let proof_progression_identity =
            StoreSecurityScopeProofProgressionIdentity::from_admission_inputs(
                declaration,
                expectation,
            );
        Self {
            declaration,
            expectation,
            proof_progression_identity,
        }
    }

    pub const fn declaration(self) -> StoreRawSecurityScopeDeclaration {
        self.declaration
    }

    pub const fn expectation(self) -> StoreSecurityScopeAdmissionExpectation {
        self.expectation
    }

    pub const fn proof_progression_identity(self) -> StoreSecurityScopeProofProgressionIdentity {
        self.proof_progression_identity
    }
}
