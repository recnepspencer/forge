use worth_proof::TransitionOutcome;
use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::{
    admit_store_security_scope, readmit_deserialized_security_scope_declaration,
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityAuthoritySource, StoreSecurityScopeAdmissionDeferred,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionFailure, StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionRequest, StoreSecurityScopeAdmissionStale,
    StoreSecurityScopeDeclarationProvenance, StoreSecurityScopeDenial, StoreSecurityScopeIdentity,
    StoreTenantScope,
};

pub type StoreRepairPhysicalRegionAdmissionOutcome = TransitionOutcome<
    StoreRepairPhysicalRegionWitness,
    StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionDeferred,
    StoreSecurityScopeAdmissionStale,
    StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionFailure,
>;

pub const fn repair_blast_radius_authenticity() -> StoreAuthenticityRequirement {
    StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedRepairRead,
    )
}

pub const fn repair_blast_radius_expectation(
    custody_posture: StoreCustodyPosture,
) -> StoreSecurityScopeAdmissionExpectation {
    StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::RepairScopeEnvelope,
        StoreTenantScope::RepairBlastRadius,
        repair_blast_radius_authenticity(),
        custody_posture,
    )
}

pub const fn reject_repair_authority_source(
    source: StoreSecurityAuthoritySource,
) -> StoreSecurityScopeDenial {
    crate::reject_non_store_security_scope_source(source)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreRepairPhysicalRegionDeclaration {
    region_id: String,
}

impl StoreRepairPhysicalRegionDeclaration {
    pub fn raw(region_id: impl Into<String>) -> Self {
        Self {
            region_id: region_id.into(),
        }
    }

    pub fn region_id(&self) -> &str {
        &self.region_id
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreRepairPhysicalRegionWitness {
    declaration: StoreRepairPhysicalRegionDeclaration,
    identity: StoreSecurityScopeIdentity,
    admitted_scope: StoreAdmittedSecurityScope,
}

impl StoreRepairPhysicalRegionWitness {
    pub fn admit_native(
        current_authority: &StoreCurrentAuthorityWitness,
        declaration: StoreRepairPhysicalRegionDeclaration,
        key_version_posture: StoreKeyVersionPosture,
        custody_posture: StoreCustodyPosture,
    ) -> StoreRepairPhysicalRegionAdmissionOutcome {
        let raw = StoreRawSecurityScopeDeclaration::native(
            current_authority.physical_witness(),
            StoreKeyScope::RepairScopeEnvelope,
            key_version_posture,
            StoreTenantScope::RepairBlastRadius,
            repair_blast_radius_authenticity(),
            custody_posture,
        );
        admit_from_raw(current_authority, declaration, raw, custody_posture)
    }

    pub fn admit_offline_report(
        current_authority: &StoreCurrentAuthorityWitness,
        declaration: StoreRepairPhysicalRegionDeclaration,
        raw: StoreRawSecurityScopeDeclaration,
    ) -> StoreRepairPhysicalRegionAdmissionOutcome {
        let custody_posture = raw
            .custody_posture()
            .unwrap_or(StoreCustodyPosture::CustodyUnavailable);
        let expectation = repair_blast_radius_expectation(custody_posture);
        let raw = match raw.provenance() {
            StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => {
                match readmit_deserialized_security_scope_declaration(
                    current_authority,
                    raw,
                    expectation,
                ) {
                    Ok(raw) => raw,
                    Err(source) => return TransitionOutcome::denied(source),
                }
            }
            _ => raw,
        };
        admit_from_raw(current_authority, declaration, raw, custody_posture)
    }

    pub fn region_id(&self) -> &str {
        self.declaration.region_id()
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn admitted_scope(&self) -> &StoreAdmittedSecurityScope {
        &self.admitted_scope
    }

    pub fn into_admitted_scope(self) -> StoreAdmittedSecurityScope {
        self.admitted_scope
    }
}

fn admit_from_raw(
    current_authority: &StoreCurrentAuthorityWitness,
    declaration: StoreRepairPhysicalRegionDeclaration,
    raw: StoreRawSecurityScopeDeclaration,
    custody_posture: StoreCustodyPosture,
) -> StoreRepairPhysicalRegionAdmissionOutcome {
    let request = StoreSecurityScopeAdmissionRequest::from_raw_declaration(
        current_authority,
        raw,
        repair_blast_radius_expectation(custody_posture),
    );
    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted_scope) => {
            let identity = admitted_scope.identity();
            TransitionOutcome::success(StoreRepairPhysicalRegionWitness {
                declaration,
                identity,
                admitted_scope,
            })
        }
        TransitionOutcome::Denied(source) => TransitionOutcome::denied(source),
        TransitionOutcome::Stale(stale) => TransitionOutcome::stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::rebind_required(rebind),
        TransitionOutcome::Deferred(deferred) => TransitionOutcome::deferred(deferred),
        TransitionOutcome::Failed(failed) => TransitionOutcome::failed(failed),
    }
}
