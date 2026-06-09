use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::ExactUnitDistanceConflictScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::CandidateScreeningEvaluation;
use super::optimization::ExactUnitDistanceConflictCertificate;
use super::rectangular_screening_support::{
    admitted_declaration_digest, rejected_evaluation, replay_error, require_catalog_family,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

pub fn evaluate_exact_unit_distance_conflict_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: ExactUnitDistanceConflictCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::ExactUnitDistanceConflict;
    require_catalog_family(catalog, family)?;
    let query_digest = admitted_declaration_digest(
        handle,
        ExactUnitDistanceConflictScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        family,
        "query_exact_unit_distance_conflict_declaration_not_admitted",
    )?;
    if !certificate
        .left()
        .unit_circle_intersects_difference(certificate.right())
    {
        return Err(replay_error(
            family,
            "exact_unit_distance_conflict_not_replayed",
        ));
    }
    rejected_evaluation(
        catalog,
        family,
        subject,
        query_digest,
        "exact_unit_distance_conflict_certificate",
        certificate.stable_token(),
    )
}
