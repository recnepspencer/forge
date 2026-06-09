use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::TileDiameterScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::CandidateScreeningEvaluation;
use super::optimization::TileDiameterCertificate;
use super::rectangular_screening_support::{
    admitted_declaration_digest, rejected_evaluation, replay_error, require_catalog_family,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

pub fn evaluate_tile_diameter_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: TileDiameterCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::TileDiameterSafety;
    require_catalog_family(catalog, family)?;
    let query_digest = admitted_declaration_digest(
        handle,
        TileDiameterScreeningDeclaration::new(subject.stable_token(), certificate.stable_token()),
        family,
        "query_tile_diameter_declaration_not_admitted",
    )?;
    if certificate.tile().diameter_squared().cmp_integer(1).is_lt() {
        return Err(replay_error(family, "tile_diameter_below_unit"));
    }
    rejected_evaluation(
        catalog,
        family,
        subject,
        query_digest,
        "tile_diameter_certificate",
        certificate.stable_token(),
    )
}
