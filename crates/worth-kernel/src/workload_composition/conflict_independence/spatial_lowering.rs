use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingPosture;

use super::common::ConflictIndependenceDisposition;
use super::spatial::{
    SpatialConflictIndependenceDenial, SpatialConflictIndependenceDenialKind,
    SpatialConflictIndependenceRequest,
};

pub(super) struct SpatialIndependenceContext {
    pub disposition: ConflictIndependenceDisposition,
    pub denial: Option<SpatialConflictIndependenceDenial>,
}

pub(super) fn classify(
    request: &SpatialConflictIndependenceRequest<'_>,
) -> SpatialIndependenceContext {
    if request.left().execution_admission().is_denied()
        || request.right().execution_admission().is_denied()
    {
        return denied(
            SpatialConflictIndependenceDenialKind::SelectedPlanDenied,
            "spatial independence requires both selected conflict plans to be admitted before proof lowering",
        );
    }
    if request.left().authority().digest().as_str() != request.right().authority().digest().as_str()
    {
        return SpatialIndependenceContext {
            disposition: ConflictIndependenceDisposition::Disjoint,
            denial: None,
        };
    }
    if routing_postures(request).contains(&ConflictRoutingPosture::Denied) {
        return denied(
            SpatialConflictIndependenceDenialKind::DeclaredDenied,
            "spatial independence is denied because a selected conflict family declaration marks the overlap as denied",
        );
    }
    if routing_postures(request).contains(&ConflictRoutingPosture::SerializableOnly) {
        return SpatialIndependenceContext {
            disposition: ConflictIndependenceDisposition::SerializableOnly,
            denial: None,
        };
    }
    let all_proven_independent = routing_postures(request)
        .iter()
        .all(|posture| *posture == ConflictRoutingPosture::ProvenIndependent);
    if all_proven_independent {
        return SpatialIndependenceContext {
            disposition: ConflictIndependenceDisposition::CompatibleAspectOverlap,
            denial: None,
        };
    }
    denied(
        SpatialConflictIndependenceDenialKind::MissingPositiveProof,
        "spatial independence requires either disjoint locality or an explicit compatible overlap declaration",
    )
}

fn denied(
    kind: SpatialConflictIndependenceDenialKind,
    detail: &'static str,
) -> SpatialIndependenceContext {
    SpatialIndependenceContext {
        disposition: ConflictIndependenceDisposition::Denied,
        denial: Some(SpatialConflictIndependenceDenial {
            kind,
            detail: detail.to_string(),
        }),
    }
}

fn routing_postures(
    request: &SpatialConflictIndependenceRequest<'_>,
) -> Vec<ConflictRoutingPosture> {
    request
        .left()
        .selected_families()
        .iter()
        .chain(request.right().selected_families().iter())
        .map(|row| row.routing_posture())
        .collect()
}
