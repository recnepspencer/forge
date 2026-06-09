use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};
use crate::domain_declarations::UnitDistanceEmbeddabilityScreeningDeclaration;
use crate::mathematical_verification::ExactRational;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_index::ScreeningExactEmbeddingIndex;
use super::graph_embedding_screening_support::{
    declare_screening_request, replay_error, require_catalog_family, screening_evaluation,
};
use super::optimization::UnitDistanceEmbeddabilityCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_unit_distance_embeddability_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: UnitDistanceEmbeddabilityCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::UnitDistanceEmbeddability;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        UnitDistanceEmbeddabilityScreeningDeclaration::new(
            graph.reference().stable_token(),
            certificate.stable_token(),
        ),
        "query_unit_distance_embeddability_screening_declaration_not_admitted",
    )?;
    let embedding_index = ScreeningExactEmbeddingIndex::new(graph, certificate.embedding());
    if !embedding_index.all_graph_edges_are_unit(family)? {
        return screening_evaluation(
            catalog,
            family,
            graph.reference(),
            CandidateScreeningVerdict::Rejected,
            &query_digest,
            format!(
                "unit_distance_embeddability_failed;certificate={}",
                certificate.stable_token()
            ),
        );
    }
    for (left, right) in certificate.non_edge_exclusions() {
        if embedding_index.graph().is_adjacent_label(left, right) {
            return Err(replay_error(family, "non_edge_exclusion_names_graph_edge"));
        }
        if embedding_index.squared_distance(left, right, family)? == ExactRational::integer(1) {
            return screening_evaluation(
                catalog,
                family,
                graph.reference(),
                CandidateScreeningVerdict::Rejected,
                &query_digest,
                format!("optional_non_edge_unit_conflict={left}-{right}"),
            );
        }
    }
    screening_evaluation(
        catalog,
        family,
        graph.reference(),
        CandidateScreeningVerdict::Passed,
        &query_digest,
        format!(
            "all_graph_edges_unit=true;embedding={}",
            embedding_index.stable_token()
        ),
    )
}
