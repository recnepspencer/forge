use forge_store_authority::StoreCurrentAuthorityWitness;

use crate::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreSecurityScopeDeclarationProvenance, StoreTenantScope,
    StoreTrustBoundaryReadmissionTrigger,
};
use forge_proof::TransitionOutcome;

#[derive(Debug, PartialEq, Eq)]
pub struct StoreReadmittedSecurityScope {
    admitted: StoreAdmittedSecurityScope,
    current_authority: StoreCurrentAuthorityWitness,
}

impl StoreReadmittedSecurityScope {
    pub const fn admitted(&self) -> &StoreAdmittedSecurityScope {
        &self.admitted
    }

    pub const fn current_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.current_authority
    }

    pub fn into_admitted(self) -> StoreAdmittedSecurityScope {
        self.admitted
    }
}

pub fn admit_readmitted_trust_boundary_security_scope(
    current_authority: &StoreCurrentAuthorityWitness,
    declaration: StoreRawSecurityScopeDeclaration,
    expected_key_version_posture: StoreKeyVersionPosture,
    expectation: StoreSecurityScopeAdmissionExpectation,
    trigger: StoreTrustBoundaryReadmissionTrigger,
) -> Result<StoreReadmittedSecurityScope, StoreSecurityScopeAdmissionDenial> {
    let declaration = readmit_trust_boundary_security_scope_declaration(
        current_authority,
        declaration,
        expected_key_version_posture,
        expectation,
        trigger,
    )?;
    let request = StoreSecurityScopeAdmissionRequest::from_raw_declaration(
        current_authority,
        declaration,
        expectation,
    );
    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => Ok(StoreReadmittedSecurityScope {
            admitted,
            current_authority: current_authority.clone(),
        }),
        TransitionOutcome::Denied(denial) => Err(denial),
        TransitionOutcome::Stale(_) | TransitionOutcome::RebindRequired(_) => {
            Err(StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture)
        }
        TransitionOutcome::Deferred(_) | TransitionOutcome::Failed(_) => {
            Err(StoreSecurityScopeAdmissionDenial::DeniedCustodyPosture)
        }
    }
}

pub fn readmit_trust_boundary_security_scope_declaration(
    current_authority: &StoreCurrentAuthorityWitness,
    declaration: StoreRawSecurityScopeDeclaration,
    expected_key_version_posture: StoreKeyVersionPosture,
    expectation: StoreSecurityScopeAdmissionExpectation,
    trigger: StoreTrustBoundaryReadmissionTrigger,
) -> Result<StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionDenial> {
    reject_non_import_boundary_provenance(declaration)?;
    reject_authenticity_drift(declaration, expectation.authenticity_requirement())?;
    reject_non_readmittable_custody(declaration)?;
    reject_portable_trust_boundary_trigger(current_authority, declaration, expectation, trigger)?;
    reject_physical_scope_drift(current_authority, declaration)?;
    reject_key_scope_drift(declaration, expectation.key_scope())?;
    reject_key_version_drift(declaration, expected_key_version_posture)?;
    reject_tenant_scope_drift(declaration, expectation.tenant_scope())?;

    Ok(declaration.trust_boundary_readmitted(expectation.custody_posture()))
}

fn reject_portable_trust_boundary_trigger(
    current_authority: &StoreCurrentAuthorityWitness,
    declaration: StoreRawSecurityScopeDeclaration,
    expectation: StoreSecurityScopeAdmissionExpectation,
    trigger: StoreTrustBoundaryReadmissionTrigger,
) -> Result<(), StoreSecurityScopeAdmissionDenial> {
    if !trigger.requires_security_scope_readmission() {
        return Err(StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence);
    }
    trigger
        .bind_to_readmission_candidate(declaration, current_authority, expectation)
        .map_err(|_| StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence)
}

fn reject_non_import_boundary_provenance(
    declaration: StoreRawSecurityScopeDeclaration,
) -> Result<(), StoreSecurityScopeAdmissionDenial> {
    match declaration.provenance() {
        StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => Ok(()),
        StoreSecurityScopeDeclarationProvenance::ReplayedAdmissionEvidence => {
            Err(StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence)
        }
        StoreSecurityScopeDeclarationProvenance::NativeStoreDeclaration
        | StoreSecurityScopeDeclarationProvenance::StoreReadmitted => {
            Err(StoreSecurityScopeAdmissionDenial::DeserializedSecurityScopeRequiresReadmission)
        }
    }
}

fn reject_physical_scope_drift(
    current_authority: &StoreCurrentAuthorityWitness,
    declaration: StoreRawSecurityScopeDeclaration,
) -> Result<(), StoreSecurityScopeAdmissionDenial> {
    if declaration.physical_witness() == current_authority.physical_witness() {
        Ok(())
    } else {
        Err(StoreSecurityScopeAdmissionDenial::WrongPhysicalSecurityScope)
    }
}

fn reject_key_scope_drift(
    declaration: StoreRawSecurityScopeDeclaration,
    expected: StoreKeyScope,
) -> Result<(), StoreSecurityScopeAdmissionDenial> {
    if declaration.key_scope() == expected {
        Ok(())
    } else {
        Err(StoreSecurityScopeAdmissionDenial::WrongKeyScope)
    }
}

fn reject_key_version_drift(
    declaration: StoreRawSecurityScopeDeclaration,
    expected: StoreKeyVersionPosture,
) -> Result<(), StoreSecurityScopeAdmissionDenial> {
    if declaration.key_version_posture() == expected && expected == StoreKeyVersionPosture::Current
    {
        Ok(())
    } else {
        Err(StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture)
    }
}

fn reject_tenant_scope_drift(
    declaration: StoreRawSecurityScopeDeclaration,
    expected: StoreTenantScope,
) -> Result<(), StoreSecurityScopeAdmissionDenial> {
    if declaration.tenant_scope() == expected {
        Ok(())
    } else {
        Err(StoreSecurityScopeAdmissionDenial::WrongTenantScope)
    }
}

fn reject_authenticity_drift(
    declaration: StoreRawSecurityScopeDeclaration,
    expected: StoreAuthenticityRequirement,
) -> Result<(), StoreSecurityScopeAdmissionDenial> {
    match declaration.authenticity_requirement() {
        Some(requirement) if requirement == expected => Ok(()),
        Some(_) => Err(StoreSecurityScopeAdmissionDenial::UnsupportedAuthenticityRequirement),
        None => Err(StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement),
    }
}

fn reject_non_readmittable_custody(
    declaration: StoreRawSecurityScopeDeclaration,
) -> Result<(), StoreSecurityScopeAdmissionDenial> {
    match declaration.custody_posture() {
        Some(
            StoreCustodyPosture::ExportedOutOfCustody | StoreCustodyPosture::ImportedUnreadmitted,
        ) => Ok(()),
        Some(_) => Err(StoreSecurityScopeAdmissionDenial::WrongCustodyPosture),
        None => Err(StoreSecurityScopeAdmissionDenial::MissingCustodyPosture),
    }
}
