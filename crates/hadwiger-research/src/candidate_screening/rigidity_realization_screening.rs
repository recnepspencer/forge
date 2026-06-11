use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};
use crate::domain_declarations::RigidityRealizationScreeningDeclaration;
use crate::mathematical_verification::ExactRational;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_index::ScreeningExactEmbeddingIndex;
use super::graph_embedding_screening_support::{
    declare_screening_request, replay_error, require_catalog_family, screening_evaluation,
};
use super::optimization::{RigidityRealizationCertificate, RigidityRealizationPosture};
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_rigidity_realization_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: RigidityRealizationCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::RigidityRealizationConsistency;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        RigidityRealizationScreeningDeclaration::new(
            graph.reference().stable_token(),
            certificate.stable_token(),
        ),
        "query_rigidity_realization_screening_declaration_not_admitted",
    )?;
    let embedding_index = ScreeningExactEmbeddingIndex::new(graph, certificate.embedding());
    if !embedding_index.all_graph_edges_are_unit(family)? {
        if certificate.expected_posture() != RigidityRealizationPosture::Impossible {
            return Err(replay_error(family, "rigidity_impossible_posture_required"));
        }
        return screening_evaluation(
            catalog,
            family,
            graph.reference(),
            CandidateScreeningVerdict::Rejected,
            &query_digest,
            "realization_consistency=impossible_non_unit_edge",
        );
    }
    let rank = rigidity_matrix_rank(&embedding_index, family)?;
    let target = local_rigidity_rank(graph.vertices().len());
    let replayed = if rank < target {
        RigidityRealizationPosture::Flexible
    } else {
        RigidityRealizationPosture::LocallyRigid
    };
    if certificate.expected_posture() != replayed
        && certificate.expected_posture() != RigidityRealizationPosture::GloballyRigidUnsupported
    {
        return Err(replay_error(family, "rigidity_posture_mismatch"));
    }
    let verdict = match replayed {
        RigidityRealizationPosture::Flexible => CandidateScreeningVerdict::Priority,
        RigidityRealizationPosture::LocallyRigid => CandidateScreeningVerdict::Passed,
        RigidityRealizationPosture::Impossible => CandidateScreeningVerdict::Rejected,
        RigidityRealizationPosture::GloballyRigidUnsupported => CandidateScreeningVerdict::Priority,
    };
    screening_evaluation(
        catalog,
        family,
        graph.reference(),
        verdict,
        &query_digest,
        format!(
            "rigidity_rank={rank};local_rigidity_target={target};posture={}",
            replayed.as_str()
        ),
    )
}

fn rigidity_matrix_rank(
    embedding_index: &ScreeningExactEmbeddingIndex<'_>,
    family: CandidateScreeningInvariantFamily,
) -> Result<usize, CandidateScreeningError> {
    let vertices = embedding_index.graph().vertices();
    let mut rows = Vec::new();
    for (left, right) in embedding_index.graph().edges() {
        let left_point = embedding_index.point(left, family)?;
        let right_point = embedding_index.point(right, family)?;
        let dx = left_point.x().sub(right_point.x());
        let dy = left_point.y().sub(right_point.y());
        let mut row = vec![ExactRational::zero(); vertices.len() * 2];
        let left_index = vertices
            .iter()
            .position(|vertex| vertex == left)
            .unwrap_or(0);
        let right_index = vertices
            .iter()
            .position(|vertex| vertex == right)
            .unwrap_or(0);
        row[left_index * 2] = dx.clone();
        row[left_index * 2 + 1] = dy.clone();
        row[right_index * 2] = ExactRational::zero().sub(&dx);
        row[right_index * 2 + 1] = ExactRational::zero().sub(&dy);
        rows.push(row);
    }
    Ok(row_rank(rows))
}

fn row_rank(mut matrix: Vec<Vec<ExactRational>>) -> usize {
    let mut rank = 0;
    let column_count = matrix.first().map(Vec::len).unwrap_or(0);
    for column in 0..column_count {
        let Some(pivot) = (rank..matrix.len()).find(|row| !matrix[*row][column].is_zero()) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let pivot_value = matrix[rank][column].clone();
        for row in 0..matrix.len() {
            if row == rank || matrix[row][column].is_zero() {
                continue;
            }
            let Some(factor) = matrix[row][column].div(&pivot_value) else {
                continue;
            };
            for col in column..column_count {
                matrix[row][col] = matrix[row][col].sub(&factor.mul(&matrix[rank][col]));
            }
        }
        rank += 1;
    }
    rank
}

fn local_rigidity_rank(vertex_count: usize) -> usize {
    vertex_count.saturating_mul(2).saturating_sub(3)
}
