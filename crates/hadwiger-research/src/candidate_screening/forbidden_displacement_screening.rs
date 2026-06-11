use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, ForbiddenDisplacementScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::optimization::ForbiddenDisplacementCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_forbidden_displacement_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: ForbiddenDisplacementCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::ForbiddenDisplacementSet;
    require_catalog_family(catalog, family)?;
    let declaration = declare_research_request_checked(
        handle,
        ForbiddenDisplacementScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family,
        reason: "query_forbidden_displacement_screening_declaration_not_admitted",
    })?;
    if !certificate
        .tile()
        .forbidden_displacement_contains(certificate.dx(), certificate.dy())
    {
        return Err(replay_error("displacement_not_forbidden_for_rectangle"));
    }
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        CandidateScreeningVerdict::Rejected,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={};forbidden_displacement_certificate={}",
            canonical_digest_token(declaration.declaration_digest()),
            certificate.stable_token()
        ),
    )
    .map_err(Into::into)
}

fn require_catalog_family(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
) -> Result<(), CandidateScreeningError> {
    if catalog.has_family(family) {
        Ok(())
    } else {
        Err(CandidateScreeningError::MissingInvariantFamily(family))
    }
}

fn replay_error(reason: &'static str) -> CandidateScreeningError {
    CandidateScreeningError::CertificateReplayRejected {
        family: CandidateScreeningInvariantFamily::ForbiddenDisplacementSet,
        reason,
    }
}
