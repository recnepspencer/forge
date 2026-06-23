use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::proof::{
    PlanarBooleanFragmentConsumptionProof, PlanarBooleanFragmentConsumptionProofRow,
};
use super::row::PlanarBooleanClosedWalkCandidate;

pub(crate) fn closed_walk_candidate_identity(
    request_identity: &str,
    continuation_index_identity: &str,
    source_loop_identity: &str,
    fragment_identities: &[String],
    split_vertex_identities: &[String],
    continuation_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-closed-walk-candidate".to_string(),
        format!("request:{request_identity}"),
        format!("continuation-index:{continuation_index_identity}"),
        format!("source-loop:{source_loop_identity}"),
    ];
    parts.extend(
        fragment_identities
            .iter()
            .map(|identity| format!("fragment:{identity}")),
    );
    parts.extend(
        split_vertex_identities
            .iter()
            .map(|identity| format!("split-vertex:{identity}")),
    );
    parts.extend(
        continuation_identities
            .iter()
            .map(|identity| format!("continuation:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn closed_walk_candidate_set_identity(
    request_identity: &str,
    continuation_index_identity: &str,
    rows: &[PlanarBooleanClosedWalkCandidate],
) -> String {
    let mut parts = vec![
        "planar-boolean-closed-walk-candidate-set".to_string(),
        format!("request:{request_identity}"),
        format!("continuation-index:{continuation_index_identity}"),
    ];
    parts.extend(rows.iter().map(|row| {
        format!(
            "closed-walk-candidate:{}",
            row.closed_walk_candidate_identity()
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn fragment_consumption_proof_identity(
    request_identity: &str,
    continuation_index_identity: &str,
    rows: &[PlanarBooleanFragmentConsumptionProofRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-fragment-consumption-proof".to_string(),
        format!("request:{request_identity}"),
        format!("continuation-index:{continuation_index_identity}"),
    ];
    parts.extend(rows.iter().map(|row| {
        format!(
            "closed-walk-candidate:{}",
            row.closed_walk_candidate_identity()
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn consumption_proof_matches_candidate(
    candidate: &PlanarBooleanClosedWalkCandidate,
    proof: &PlanarBooleanFragmentConsumptionProof,
) -> bool {
    proof
        .proof_for_candidate_identity(candidate.closed_walk_candidate_identity())
        .map(|row| {
            row.fragment_identities() == candidate.fragment_identities()
                && row.split_vertex_identities() == candidate.split_vertex_identities()
                && row.continuation_identities()
                    == candidate
                        .continuations()
                        .iter()
                        .map(|continuation| continuation.continuation_identity().to_string())
                        .collect::<Vec<_>>()
        })
        .unwrap_or(false)
}
