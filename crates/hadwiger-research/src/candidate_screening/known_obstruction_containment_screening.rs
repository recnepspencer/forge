use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};
use crate::domain_declarations::KnownObstructionContainmentScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_index::ScreeningFiniteGraphIndex;
use super::graph_embedding_screening_support::{
    declare_screening_request, replay_error, require_catalog_family, screening_evaluation,
};
use super::optimization::KnownObstructionContainmentCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_known_obstruction_containment_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: KnownObstructionContainmentCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::KnownObstructionContainment;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        KnownObstructionContainmentScreeningDeclaration::new(
            graph.reference().stable_token(),
            certificate.stable_token(),
        ),
        "query_known_obstruction_containment_screening_declaration_not_admitted",
    )?;
    let obstruction =
        ScreeningFiniteGraphIndex::from_graph_version(certificate.obstruction_graph());
    let candidate = ScreeningFiniteGraphIndex::from_graph_version(graph);
    if !obstruction.mapping_is_injective_to_target(&candidate, certificate.vertex_mapping()) {
        return Err(replay_error(family, "obstruction_mapping_not_injective"));
    }
    if !obstruction.preserves_edges(&candidate, certificate.vertex_mapping()) {
        return Err(replay_error(family, "obstruction_edges_not_preserved"));
    }
    screening_evaluation(
        catalog,
        family,
        graph.reference(),
        CandidateScreeningVerdict::Rejected,
        &query_digest,
        format!(
            "known_obstruction_vertices={};known_obstruction_edges={};mapping_size={}",
            obstruction.vertex_count(),
            obstruction.edge_count(),
            certificate.vertex_mapping().len()
        ),
    )
}
