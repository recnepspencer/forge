use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::ExactConflictGraphScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::CandidateScreeningEvaluation;
use super::optimization::ExactConflictGraphEdgeCertificate;
use super::rectangular_screening_support::{
    admitted_declaration_digest, rejected_evaluation, replay_error, require_catalog_family,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

pub fn evaluate_exact_conflict_graph_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: ExactConflictGraphEdgeCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::ExactConflictGraphConstruction;
    require_catalog_family(catalog, family)?;
    let query_digest = admitted_declaration_digest(
        handle,
        ExactConflictGraphScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        family,
        "query_exact_conflict_graph_declaration_not_admitted",
    )?;
    if !certificate
        .left()
        .unit_circle_intersects_difference(certificate.right())
    {
        return Err(replay_error(family, "conflict_graph_edge_not_certified"));
    }
    rejected_evaluation(
        catalog,
        family,
        subject,
        query_digest,
        "exact_conflict_graph_edge_certificate",
        certificate.stable_token(),
    )
}
