use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};
use crate::domain_declarations::CandidateNoveltyScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_index::ScreeningFiniteGraphIndex;
use super::graph_embedding_screening_support::{
    declare_screening_request, replay_error, require_catalog_family, screening_evaluation,
};
use super::optimization::CandidateNoveltyCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_candidate_novelty_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: CandidateNoveltyCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::CandidateNoveltyNonIsomorphism;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        CandidateNoveltyScreeningDeclaration::new(
            graph.reference().stable_token(),
            certificate.stable_token(),
        ),
        "query_candidate_novelty_screening_declaration_not_admitted",
    )?;
    let graph_index = ScreeningFiniteGraphIndex::from_graph_version(graph);
    let fingerprint = graph_index.wl_fingerprint(certificate.wl_rounds());
    if fingerprint != certificate.known_fingerprint() {
        return Err(replay_error(
            family,
            "candidate_fingerprint_not_retained_duplicate",
        ));
    }
    screening_evaluation(
        catalog,
        family,
        graph.reference(),
        CandidateScreeningVerdict::Rejected,
        &query_digest,
        format!(
            "duplicate_fingerprint={fingerprint};wl_rounds={}",
            certificate.wl_rounds()
        ),
    )
}
