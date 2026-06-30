use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};

use super::common::ConflictIndependenceDisposition;
use super::topology::{
    TopologyConflictIndependenceDenial, TopologyConflictIndependenceDenialKind,
    TopologyConflictIndependenceRequest,
};

pub(super) struct TopologyIndependenceContext {
    pub disposition: ConflictIndependenceDisposition,
    pub denial: Option<TopologyConflictIndependenceDenial>,
}

pub(super) fn classify(
    request: &TopologyConflictIndependenceRequest<'_>,
) -> TopologyIndependenceContext {
    if request.left().execution_admission().is_denied()
        || request.right().execution_admission().is_denied()
    {
        return denied(
            TopologyConflictIndependenceDenialKind::SelectedPlanDenied,
            "topology independence requires both selected conflict plans to be admitted before proof lowering",
        );
    }
    if request.left().touched_closure().closure_digest()
        != request.right().touched_closure().closure_digest()
    {
        return TopologyIndependenceContext {
            disposition: ConflictIndependenceDisposition::Disjoint,
            denial: None,
        };
    }
    if routing_postures(request).contains(&ConflictRoutingPosture::Denied) {
        return denied(
            TopologyConflictIndependenceDenialKind::DeclaredDenied,
            "topology independence is denied because a selected conflict family declaration marks the overlap as denied",
        );
    }
    if routing_postures(request).contains(&ConflictRoutingPosture::SerializableOnly) {
        return TopologyIndependenceContext {
            disposition: ConflictIndependenceDisposition::SerializableOnly,
            denial: None,
        };
    }
    let aspect_pair = request.left().overlap_category() == ConflictOverlapCategory::Aspect
        && request.right().overlap_category() == ConflictOverlapCategory::Aspect;
    let all_proven_independent = routing_postures(request)
        .iter()
        .all(|posture| *posture == ConflictRoutingPosture::ProvenIndependent);
    if aspect_pair && all_proven_independent {
        return TopologyIndependenceContext {
            disposition: ConflictIndependenceDisposition::CompatibleAspectOverlap,
            denial: None,
        };
    }
    denied(
        TopologyConflictIndependenceDenialKind::MissingPositiveProof,
        "topology independence requires either disjoint locality or an explicit compatible aspect-overlap declaration",
    )
}

fn denied(
    kind: TopologyConflictIndependenceDenialKind,
    detail: &'static str,
) -> TopologyIndependenceContext {
    TopologyIndependenceContext {
        disposition: ConflictIndependenceDisposition::Denied,
        denial: Some(TopologyConflictIndependenceDenial {
            kind,
            detail: detail.to_string(),
        }),
    }
}

fn routing_postures(
    request: &TopologyConflictIndependenceRequest<'_>,
) -> Vec<ConflictRoutingPosture> {
    request
        .left()
        .selected_families()
        .iter()
        .chain(request.right().selected_families().iter())
        .map(|row| row.routing_posture())
        .collect()
}
