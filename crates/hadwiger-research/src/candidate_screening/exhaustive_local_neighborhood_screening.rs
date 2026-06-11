use std::collections::BTreeSet;

use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};
use crate::domain_declarations::ExhaustiveLocalNeighborhoodScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_index::ScreeningFiniteGraphIndex;
use super::graph_embedding_screening_support::{
    declare_screening_request, replay_error, require_catalog_family, screening_evaluation,
};
use super::optimization::ExhaustiveLocalNeighborhoodCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_exhaustive_local_neighborhood_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: ExhaustiveLocalNeighborhoodCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::ExhaustiveLocalNeighborhood;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        ExhaustiveLocalNeighborhoodScreeningDeclaration::new(
            graph.reference().stable_token(),
            certificate.stable_token(),
        ),
        "query_exhaustive_local_neighborhood_screening_declaration_not_admitted",
    )?;
    let graph_index = ScreeningFiniteGraphIndex::from_graph_version(graph);
    let replayed =
        graph_index.neighborhood_within(certificate.root_vertex(), certificate.radius(), family)?;
    let expected = certificate
        .expected_vertices()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if replayed != expected {
        return Err(replay_error(family, "local_neighborhood_not_exhaustive"));
    }
    screening_evaluation(
        catalog,
        family,
        graph.reference(),
        CandidateScreeningVerdict::Passed,
        &query_digest,
        format!(
            "root={};radius={};neighborhood_size={}",
            certificate.root_vertex(),
            certificate.radius(),
            replayed.len()
        ),
    )
}
