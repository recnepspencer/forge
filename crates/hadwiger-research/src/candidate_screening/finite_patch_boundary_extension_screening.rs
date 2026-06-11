use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::FinitePatchBoundaryExtensionScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_screening_support::{
    declare_screening_request, require_catalog_family, screening_evaluation,
};
use super::optimization::FinitePatchBoundaryExtensionCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_finite_patch_boundary_extension_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: FinitePatchBoundaryExtensionCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::FinitePatchBoundaryExtension;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        FinitePatchBoundaryExtensionScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        "query_finite_patch_boundary_extension_screening_declaration_not_admitted",
    )?;
    screening_evaluation(
        catalog,
        family,
        subject,
        if certificate.all_boundary_colorings_fail() {
            CandidateScreeningVerdict::Rejected
        } else {
            CandidateScreeningVerdict::Passed
        },
        &query_digest,
        format!(
            "all_boundary_colorings_fail={};certificate={}",
            certificate.all_boundary_colorings_fail(),
            certificate.stable_token()
        ),
    )
}
