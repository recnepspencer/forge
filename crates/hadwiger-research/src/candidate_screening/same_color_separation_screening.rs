use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::SameColorSeparationScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::CandidateScreeningEvaluation;
use super::optimization::SameColorSeparationCertificate;
use super::rectangular_screening_support::{
    admitted_declaration_digest, rejected_evaluation, replay_error, require_catalog_family,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

pub fn evaluate_same_color_separation_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: SameColorSeparationCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::SameColorSeparationDistanceSet;
    require_catalog_family(catalog, family)?;
    let query_digest = admitted_declaration_digest(
        handle,
        SameColorSeparationScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        family,
        "query_same_color_separation_declaration_not_admitted",
    )?;
    if certificate
        .left()
        .difference_min_squared_distance(certificate.right())
        .cmp_integer(1)
        .is_gt()
        || certificate
            .left()
            .difference_max_squared_distance(certificate.right())
            .cmp_integer(1)
            .is_lt()
    {
        return Err(replay_error(family, "same_color_distance_set_misses_unit"));
    }
    rejected_evaluation(
        catalog,
        family,
        subject,
        query_digest,
        "same_color_separation_certificate",
        certificate.stable_token(),
    )
}
