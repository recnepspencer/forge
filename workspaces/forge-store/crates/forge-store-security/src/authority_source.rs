#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityAuthoritySource {
    RawString,
    SemanticId,
    TerminalJsonLabel,
    JwtSubjectClaim,
    ApplicationOrgId,
    KmsKeyId,
    IamRole,
    OperatorIdentity,
    AuditRecord,
    OfflineVerifierEvidence,
    FoundationalEvidence,
    ProofProgression,
    StoreCurrentAuthorityWitnessOnly,
}

pub const fn classify_raw_string_as_security_scope_source() -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::RawString
}

pub const fn classify_semantic_id_as_security_scope_source() -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::SemanticId
}

pub const fn classify_terminal_json_label_as_security_scope_source() -> StoreSecurityAuthoritySource
{
    StoreSecurityAuthoritySource::TerminalJsonLabel
}

pub const fn classify_identity_provider_claim_as_security_scope_source(
) -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::JwtSubjectClaim
}

pub const fn classify_app_org_id_as_security_scope_source() -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::ApplicationOrgId
}

pub const fn classify_kms_key_id_as_security_scope_source() -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::KmsKeyId
}

pub const fn classify_iam_role_as_security_scope_source() -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::IamRole
}

pub const fn classify_operator_identity_as_security_scope_source() -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::OperatorIdentity
}

pub const fn classify_audit_record_as_security_scope_source() -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::AuditRecord
}

pub const fn classify_offline_verifier_evidence_as_security_scope_source(
) -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::OfflineVerifierEvidence
}

pub const fn classify_foundational_evidence_as_security_scope_source(
) -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::FoundationalEvidence
}

pub const fn classify_proof_progression_as_security_scope_source() -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::ProofProgression
}

pub const fn classify_store_current_authority_as_security_scope_source(
) -> StoreSecurityAuthoritySource {
    StoreSecurityAuthoritySource::StoreCurrentAuthorityWitnessOnly
}
