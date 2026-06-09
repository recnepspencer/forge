use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::BoundaryOwnershipScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_screening_support::{
    declare_screening_request, require_catalog_family, screening_evaluation,
};
use super::optimization::BoundaryOwnershipCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_boundary_ownership_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: BoundaryOwnershipCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::BoundaryOwnership;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        BoundaryOwnershipScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        "query_boundary_ownership_screening_declaration_not_admitted",
    )?;
    let owned_boundaries = certificate
        .regions()
        .iter()
        .filter(|region| region.owns_boundary())
        .count();
    let verdict = if owned_boundaries != certificate.regions().len()
        || has_ambiguous_boundary_owner(&certificate)
        || has_same_color_unit_boundary_conflict(&certificate)
    {
        CandidateScreeningVerdict::Rejected
    } else {
        CandidateScreeningVerdict::Passed
    };
    screening_evaluation(
        catalog,
        family,
        subject,
        verdict,
        &query_digest,
        format!(
            "owned_boundaries={owned_boundaries};region_count={};certificate={}",
            certificate.regions().len(),
            certificate.stable_token()
        ),
    )
}

fn has_ambiguous_boundary_owner(certificate: &BoundaryOwnershipCertificate) -> bool {
    for left in 0..certificate.regions().len() {
        for right in (left + 1)..certificate.regions().len() {
            let left_region = &certificate.regions()[left];
            let right_region = &certificate.regions()[right];
            if left_region.owns_boundary()
                && right_region.owns_boundary()
                && left_region.region().stable_token() == right_region.region().stable_token()
            {
                return true;
            }
        }
    }
    false
}

fn has_same_color_unit_boundary_conflict(certificate: &BoundaryOwnershipCertificate) -> bool {
    for left in 0..certificate.regions().len() {
        for right in (left + 1)..certificate.regions().len() {
            let left_region = &certificate.regions()[left];
            let right_region = &certificate.regions()[right];
            if left_region.color_id() == right_region.color_id()
                && left_region
                    .region()
                    .unit_circle_intersects_difference(right_region.region())
            {
                return true;
            }
        }
    }
    false
}
