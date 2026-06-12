use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, HadwigerResearchDeclarationInput,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

pub(crate) fn admitted_declaration_digest<I>(
    handle: &HadwigerResearchHandle,
    input: I,
    family: CandidateScreeningInvariantFamily,
    reason: &'static str,
) -> Result<String, CandidateScreeningError>
where
    I: HadwigerResearchDeclarationInput,
{
    let declaration = declare_research_request_checked(handle, input)
        .admitted()
        .ok_or(CandidateScreeningError::SolverCandidateUnavailable { family, reason })?;
    Ok(canonical_digest_token(declaration.declaration_digest()))
}

pub(crate) fn rejected_evaluation(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
    subject: HadwigerArtifactReference,
    query_declaration_digest: String,
    evidence_label: &'static str,
    evidence_token: String,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        CandidateScreeningVerdict::Rejected,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={query_declaration_digest};{evidence_label}={evidence_token}"
        ),
    )
    .map_err(Into::into)
}

pub(crate) fn require_catalog_family(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
) -> Result<(), CandidateScreeningError> {
    if catalog.has_family(family) {
        Ok(())
    } else {
        Err(CandidateScreeningError::MissingInvariantFamily(family))
    }
}

pub(crate) fn replay_error(
    family: CandidateScreeningInvariantFamily,
    reason: &'static str,
) -> CandidateScreeningError {
    CandidateScreeningError::CertificateReplayRejected { family, reason }
}
