use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::NumericalMarginScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::CandidateScreeningEvaluation;
use super::optimization::NumericalMarginCertificate;
use super::rectangular_screening_support::{
    admitted_declaration_digest, rejected_evaluation, replay_error, require_catalog_family,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

pub fn evaluate_numerical_margin_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: NumericalMarginCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::NumericalMargin;
    require_catalog_family(catalog, family)?;
    let query_digest = admitted_declaration_digest(
        handle,
        NumericalMarginScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        family,
        "query_numerical_margin_declaration_not_admitted",
    )?;
    if !certificate
        .left()
        .unit_circle_intersects_difference(certificate.right())
    {
        return Err(replay_error(family, "distance_interval_has_clear_margin"));
    }
    rejected_evaluation(
        catalog,
        family,
        subject,
        query_digest,
        "numerical_margin_certificate",
        certificate.stable_token(),
    )
}
