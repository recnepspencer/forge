use forge_store_authority::StoreCurrentAuthorityWitness;

use crate::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionBasis,
    StoreSecurityScopeAdmissionExpectation, StoreTenantScope,
};

#[derive(Debug, Clone, Copy)]
pub struct StoreSecurityScopeAdmissionRequest<'a> {
    current_authority: &'a StoreCurrentAuthorityWitness,
    basis: StoreSecurityScopeAdmissionBasis,
}

impl<'a> StoreSecurityScopeAdmissionRequest<'a> {
    pub fn new(
        current_authority: &'a StoreCurrentAuthorityWitness,
        key_scope: StoreKeyScope,
        key_version_posture: StoreKeyVersionPosture,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        custody_posture: StoreCustodyPosture,
        expectation: StoreSecurityScopeAdmissionExpectation,
    ) -> Self {
        let declaration = StoreRawSecurityScopeDeclaration::native(
            current_authority.physical_witness(),
            key_scope,
            key_version_posture,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
        );
        Self::from_raw_declaration(current_authority, declaration, expectation)
    }

    pub fn platform_page_envelope(
        current_authority: &'a StoreCurrentAuthorityWitness,
        key_version_posture: StoreKeyVersionPosture,
        custody_posture: StoreCustodyPosture,
    ) -> Self {
        Self::new(
            current_authority,
            StoreKeyScope::PageEnvelope,
            key_version_posture,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                crate::StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            custody_posture,
            StoreSecurityScopeAdmissionExpectation::platform_page_envelope(),
        )
    }

    pub fn from_raw_declaration(
        current_authority: &'a StoreCurrentAuthorityWitness,
        declaration: StoreRawSecurityScopeDeclaration,
        expectation: StoreSecurityScopeAdmissionExpectation,
    ) -> Self {
        Self {
            current_authority,
            basis: StoreSecurityScopeAdmissionBasis::new(declaration, expectation),
        }
    }

    pub const fn current_authority(&self) -> &'a StoreCurrentAuthorityWitness {
        self.current_authority
    }

    pub const fn basis(&self) -> StoreSecurityScopeAdmissionBasis {
        self.basis
    }

    pub const fn declaration(&self) -> StoreRawSecurityScopeDeclaration {
        self.basis.declaration()
    }

    pub const fn key_version_posture(self) -> StoreKeyVersionPosture {
        self.basis.declaration().key_version_posture()
    }
}
