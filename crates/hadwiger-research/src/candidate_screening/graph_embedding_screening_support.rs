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

pub(crate) fn declare_screening_request<I>(
    handle: &HadwigerResearchHandle,
    family: CandidateScreeningInvariantFamily,
    input: I,
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

pub(crate) fn screening_evaluation(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
    subject: HadwigerArtifactReference,
    verdict: CandidateScreeningVerdict,
    query_declaration_digest: &str,
    evidence: impl AsRef<str>,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        verdict,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={query_declaration_digest};{}",
            evidence.as_ref()
        ),
    )
    .map_err(Into::into)
}

pub(crate) fn replay_error(
    family: CandidateScreeningInvariantFamily,
    reason: &'static str,
) -> CandidateScreeningError {
    CandidateScreeningError::CertificateReplayRejected { family, reason }
}
