use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};
use crate::domain_declarations::TranslationRotationClosureScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_index::ScreeningFiniteGraphIndex;
use super::graph_embedding_screening_support::{
    declare_screening_request, replay_error, require_catalog_family, screening_evaluation,
};
use super::optimization::TranslationRotationClosureCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_translation_rotation_closure_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: TranslationRotationClosureCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::TranslationRotationClosure;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        TranslationRotationClosureScreeningDeclaration::new(
            graph.reference().stable_token(),
            certificate.stable_token(),
        ),
        "query_translation_rotation_closure_screening_declaration_not_admitted",
    )?;
    let graph_index = ScreeningFiniteGraphIndex::from_graph_version(graph);
    if !graph_index.mapping_is_injective_to_target(&graph_index, certificate.vertex_mapping()) {
        return Err(replay_error(family, "closure_mapping_not_injective"));
    }
    let conflict = certificate
        .same_color_pairs()
        .iter()
        .find(|(left, right)| graph_index.is_adjacent_label(left, right));
    screening_evaluation(
        catalog,
        family,
        graph.reference(),
        if conflict.is_some() {
            CandidateScreeningVerdict::Rejected
        } else {
            CandidateScreeningVerdict::Passed
        },
        &query_digest,
        format!(
            "closure_mapping_size={};same_color_unit_conflict={}",
            certificate.vertex_mapping().len(),
            conflict
                .map(|(left, right)| format!("{left}-{right}"))
                .unwrap_or_else(|| "none".to_string())
        ),
    )
}
