use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, MinkowskiDifferenceScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::optimization::MinkowskiUnitIntersectionCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_minkowski_difference_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: MinkowskiUnitIntersectionCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::MinkowskiDifferenceGeometry;
    require_catalog_family(catalog, family)?;
    let declaration = declare_research_request_checked(
        handle,
        MinkowskiDifferenceScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family,
        reason: "query_minkowski_difference_screening_declaration_not_admitted",
    })?;
    if !certificate
        .left()
        .unit_circle_intersects_difference(certificate.right())
    {
        return Err(replay_error("minkowski_difference_misses_unit_circle"));
    }
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        CandidateScreeningVerdict::Rejected,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={};minkowski_unit_intersection_certificate={}",
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
        family: CandidateScreeningInvariantFamily::MinkowskiDifferenceGeometry,
        reason,
    }
}
