use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::MonodromyColorHolonomyScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_screening_support::{
    declare_screening_request, require_catalog_family, screening_evaluation,
};
use super::optimization::MonodromyColorHolonomyCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_monodromy_color_holonomy_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: MonodromyColorHolonomyCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::MonodromyColorHolonomy;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        MonodromyColorHolonomyScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        "query_monodromy_color_holonomy_screening_declaration_not_admitted",
    )?;
    let mut color = certificate.tracked_color().to_string();
    for permutation in certificate.loop_permutations() {
        color = permutation.apply(&color);
    }
    let compatible = color == certificate.tracked_color();
    screening_evaluation(
        catalog,
        family,
        subject,
        if compatible {
            CandidateScreeningVerdict::Passed
        } else {
            CandidateScreeningVerdict::Rejected
        },
        &query_digest,
        format!(
            "tracked_color={};returned_color={color};compatible={compatible}",
            certificate.tracked_color()
        ),
    )
}
