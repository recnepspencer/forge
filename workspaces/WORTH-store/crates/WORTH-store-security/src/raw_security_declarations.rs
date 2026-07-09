use worth_store_aspect_native::StorePhysicalBoundaryWitness;
use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionCounterSnapshot, StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionExpectation, StoreTenantScope,
};

use crate::security_scope_counters::StoreSecurityScopeAdmissionCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopeDeclarationProvenance {
    NativeStoreDeclaration,
    DeserializedUnadmitted,
    StoreReadmitted,
    ReplayedAdmissionEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreRawSecurityScopeDeclaration {
    physical_witness: StorePhysicalBoundaryWitness,
    key_scope: StoreKeyScope,
    key_version_posture: StoreKeyVersionPosture,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: Option<StoreAuthenticityRequirement>,
    custody_posture: Option<StoreCustodyPosture>,
    provenance: StoreSecurityScopeDeclarationProvenance,
}

impl StoreRawSecurityScopeDeclaration {
    pub const fn native(
        physical_witness: StorePhysicalBoundaryWitness,
        key_scope: StoreKeyScope,
        key_version_posture: StoreKeyVersionPosture,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        custody_posture: StoreCustodyPosture,
    ) -> Self {
        Self {
            physical_witness,
            key_scope,
            key_version_posture,
            tenant_scope,
            authenticity_requirement: Some(authenticity_requirement),
            custody_posture: Some(custody_posture),
            provenance: StoreSecurityScopeDeclarationProvenance::NativeStoreDeclaration,
        }
    }

    pub const fn deserialized_unadmitted(
        physical_witness: StorePhysicalBoundaryWitness,
        key_scope: StoreKeyScope,
        key_version_posture: StoreKeyVersionPosture,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: Option<StoreAuthenticityRequirement>,
        custody_posture: Option<StoreCustodyPosture>,
    ) -> Self {
        Self {
            physical_witness,
            key_scope,
            key_version_posture,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
            provenance: StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted,
        }
    }

    pub const fn replayed_admission_evidence(
        physical_witness: StorePhysicalBoundaryWitness,
        key_scope: StoreKeyScope,
        key_version_posture: StoreKeyVersionPosture,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: Option<StoreAuthenticityRequirement>,
        custody_posture: Option<StoreCustodyPosture>,
    ) -> Self {
        Self {
            physical_witness,
            key_scope,
            key_version_posture,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
            provenance: StoreSecurityScopeDeclarationProvenance::ReplayedAdmissionEvidence,
        }
    }

    const fn store_readmitted(self) -> Self {
        Self {
            provenance: StoreSecurityScopeDeclarationProvenance::StoreReadmitted,
            ..self
        }
    }

    pub(crate) const fn trust_boundary_readmitted(
        self,
        custody_posture: StoreCustodyPosture,
    ) -> Self {
        Self {
            custody_posture: Some(custody_posture),
            provenance: StoreSecurityScopeDeclarationProvenance::StoreReadmitted,
            ..self
        }
    }

    pub const fn physical_witness(self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn key_version_posture(self) -> StoreKeyVersionPosture {
        self.key_version_posture
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn authenticity_requirement(self) -> Option<StoreAuthenticityRequirement> {
        self.authenticity_requirement
    }

    pub const fn custody_posture(self) -> Option<StoreCustodyPosture> {
        self.custody_posture
    }

    pub const fn provenance(self) -> StoreSecurityScopeDeclarationProvenance {
        self.provenance
    }
}

pub fn readmit_deserialized_security_scope_declaration(
    current_authority: &StoreCurrentAuthorityWitness,
    declaration: StoreRawSecurityScopeDeclaration,
    expectation: StoreSecurityScopeAdmissionExpectation,
) -> Result<StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionDenial> {
    evaluate_deserialized_security_scope_readmission(current_authority, declaration, expectation)
        .into_result()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeReadmissionEvaluation {
    result: Result<StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionDenial>,
    counters: StoreSecurityScopeAdmissionCounterSnapshot,
}

pub fn evaluate_deserialized_security_scope_readmission(
    current_authority: &StoreCurrentAuthorityWitness,
    declaration: StoreRawSecurityScopeDeclaration,
    expectation: StoreSecurityScopeAdmissionExpectation,
) -> StoreSecurityScopeReadmissionEvaluation {
    let mut counters = StoreSecurityScopeAdmissionCounters::start_request();

    match declaration.provenance() {
        StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => {}
        StoreSecurityScopeDeclarationProvenance::ReplayedAdmissionEvidence => {
            counters.record_denial();
            counters.record_replayed_admission_evidence();
            return StoreSecurityScopeReadmissionEvaluation::new(
                Err(StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence),
                counters.snapshot(),
            );
        }
        StoreSecurityScopeDeclarationProvenance::NativeStoreDeclaration
        | StoreSecurityScopeDeclarationProvenance::StoreReadmitted => {}
    }

    counters.check_physical_binding();
    if declaration.physical_witness() != current_authority.physical_witness() {
        counters.record_denial();
        counters.record_wrong_physical_scope();
        return StoreSecurityScopeReadmissionEvaluation::new(
            Err(StoreSecurityScopeAdmissionDenial::WrongPhysicalSecurityScope),
            counters.snapshot(),
        );
    }
    counters.check_key_scope();
    if declaration.key_scope() != expectation.key_scope() {
        counters.record_denial();
        counters.record_wrong_key_scope();
        return StoreSecurityScopeReadmissionEvaluation::new(
            Err(StoreSecurityScopeAdmissionDenial::WrongKeyScope),
            counters.snapshot(),
        );
    }
    counters.check_key_version();
    if declaration.key_version_posture() != StoreKeyVersionPosture::Current {
        counters.record_denial();
        return StoreSecurityScopeReadmissionEvaluation::new(
            Err(StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture),
            counters.snapshot(),
        );
    }
    counters.check_tenant_scope();
    if declaration.tenant_scope() != expectation.tenant_scope() {
        counters.record_denial();
        counters.record_wrong_tenant_scope();
        return StoreSecurityScopeReadmissionEvaluation::new(
            Err(StoreSecurityScopeAdmissionDenial::WrongTenantScope),
            counters.snapshot(),
        );
    }
    counters.check_authenticity_requirement();
    match declaration.authenticity_requirement() {
        Some(requirement) if requirement == expectation.authenticity_requirement() => {}
        Some(_) => {
            counters.record_denial();
            counters.record_unsupported_authenticity_requirement();
            counters.record_unsupported_posture();
            return StoreSecurityScopeReadmissionEvaluation::new(
                Err(StoreSecurityScopeAdmissionDenial::UnsupportedAuthenticityRequirement),
                counters.snapshot(),
            );
        }
        None => {
            counters.record_denial();
            counters.record_missing_authenticity_requirement();
            return StoreSecurityScopeReadmissionEvaluation::new(
                Err(StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement),
                counters.snapshot(),
            );
        }
    }
    counters.check_custody_posture();
    match declaration.custody_posture() {
        Some(posture) if posture == expectation.custody_posture() => {}
        Some(_) => {
            counters.record_denial();
            return StoreSecurityScopeReadmissionEvaluation::new(
                Err(StoreSecurityScopeAdmissionDenial::WrongCustodyPosture),
                counters.snapshot(),
            );
        }
        None => {
            counters.record_denial();
            counters.record_missing_custody_posture();
            return StoreSecurityScopeReadmissionEvaluation::new(
                Err(StoreSecurityScopeAdmissionDenial::MissingCustodyPosture),
                counters.snapshot(),
            );
        }
    }

    counters.record_witnesses_issued();
    StoreSecurityScopeReadmissionEvaluation::new(
        Ok(declaration.store_readmitted()),
        counters.snapshot(),
    )
}

impl StoreSecurityScopeReadmissionEvaluation {
    const fn new(
        result: Result<StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionDenial>,
        counters: StoreSecurityScopeAdmissionCounterSnapshot,
    ) -> Self {
        Self { result, counters }
    }

    pub const fn counters(self) -> StoreSecurityScopeAdmissionCounterSnapshot {
        self.counters
    }

    pub fn into_result(
        self,
    ) -> Result<StoreRawSecurityScopeDeclaration, StoreSecurityScopeAdmissionDenial> {
        self.result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreJwtSubjectClaim {
    subject_claim_text: String,
}

impl StoreJwtSubjectClaim {
    pub fn raw(subject_claim_text: impl Into<String>) -> Self {
        Self {
            subject_claim_text: subject_claim_text.into(),
        }
    }

    pub fn subject_claim_text(&self) -> &str {
        &self.subject_claim_text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreApplicationOrgIdClaim {
    application_org_id_text: String,
}

impl StoreApplicationOrgIdClaim {
    pub fn raw(application_org_id_text: impl Into<String>) -> Self {
        Self {
            application_org_id_text: application_org_id_text.into(),
        }
    }

    pub fn application_org_id_text(&self) -> &str {
        &self.application_org_id_text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreKmsKeyIdentifier {
    kms_key_identifier_text: String,
}

impl StoreKmsKeyIdentifier {
    pub fn raw(kms_key_identifier_text: impl Into<String>) -> Self {
        Self {
            kms_key_identifier_text: kms_key_identifier_text.into(),
        }
    }

    pub fn kms_key_identifier_text(&self) -> &str {
        &self.kms_key_identifier_text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreIamRoleClaim {
    iam_role_text: String,
}

impl StoreIamRoleClaim {
    pub fn raw(iam_role_text: impl Into<String>) -> Self {
        Self {
            iam_role_text: iam_role_text.into(),
        }
    }

    pub fn iam_role_text(&self) -> &str {
        &self.iam_role_text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreOperatorIdentityClaim {
    operator_identity_text: String,
}

impl StoreOperatorIdentityClaim {
    pub fn raw(operator_identity_text: impl Into<String>) -> Self {
        Self {
            operator_identity_text: operator_identity_text.into(),
        }
    }

    pub fn operator_identity_text(&self) -> &str {
        &self.operator_identity_text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreRepairAuditRecordClaim {
    audit_record_text: String,
}

impl StoreRepairAuditRecordClaim {
    pub fn raw(audit_record_text: impl Into<String>) -> Self {
        Self {
            audit_record_text: audit_record_text.into(),
        }
    }

    pub fn audit_record_text(&self) -> &str {
        &self.audit_record_text
    }
}
