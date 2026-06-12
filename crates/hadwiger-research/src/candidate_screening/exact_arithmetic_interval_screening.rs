use crate::domain_artifacts::{GraphVersion, HadwigerArtifactReference};
use crate::domain_declarations::ExactArithmeticIntervalScreeningDeclaration;
use crate::mathematical_verification::ExactRational;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_index::ScreeningExactEmbeddingIndex;
use super::graph_embedding_screening_support::{
    declare_screening_request, replay_error, require_catalog_family, screening_evaluation,
};
use super::optimization::{ExactArithmeticIntervalCertificate, ExactArithmeticIntervalExpectation};
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_exact_arithmetic_interval_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    graph: Option<&GraphVersion>,
    certificate: ExactArithmeticIntervalCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::ExactArithmeticIntervalCertificate;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        ExactArithmeticIntervalScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        "query_exact_arithmetic_interval_screening_declaration_not_admitted",
    )?;
    let unit_contained = match &certificate {
        ExactArithmeticIntervalCertificate::PointPair {
            embedding,
            left_vertex,
            right_vertex,
            ..
        } => {
            let graph = graph.ok_or_else(|| replay_error(family, "point_pair_graph_required"))?;
            let embedding_index = ScreeningExactEmbeddingIndex::new(graph, embedding);
            embedding_index.squared_distance(left_vertex, right_vertex, family)?
                == ExactRational::integer(1)
        }
        ExactArithmeticIntervalCertificate::RectanglePair {
            left_region,
            right_region,
            ..
        } => left_region.unit_circle_intersects_difference(right_region),
    };
    let expectation = match &certificate {
        ExactArithmeticIntervalCertificate::PointPair { expectation, .. }
        | ExactArithmeticIntervalCertificate::RectanglePair { expectation, .. } => *expectation,
    };
    let replay_matches = matches!(
        (unit_contained, expectation),
        (true, ExactArithmeticIntervalExpectation::UnitContained)
            | (false, ExactArithmeticIntervalExpectation::UnitExcluded)
    );
    if !replay_matches {
        return Err(replay_error(family, "exact_interval_expectation_mismatch"));
    }
    let verdict = if unit_contained {
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
            "exact_interval_unit_contained={unit_contained};certificate={}",
            certificate.stable_token()
        ),
    )
}
