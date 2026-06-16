use super::super::scope::{compose_sequence_digest, consumption_scope_encoder, seal};
use crate::ForgeQueryEvidenceTag;

use super::super::super::receipt_transitions::{
    ProjectionConsumptionDeferredNeighborFamily, ProjectionConsumptionTransitionKind,
    ProjectionConsumptionTransitionPosture,
};

pub(crate) fn compose_transition_rule_digest(
    kind: ProjectionConsumptionTransitionKind,
    posture: ProjectionConsumptionTransitionPosture,
    detail: &str,
    deferred_neighbor: Option<ProjectionConsumptionDeferredNeighborFamily>,
) -> String {
    let mut encoder = consumption_scope_encoder("projection_consumption_transition_rule_v1")
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("detail"), detail);
    if let Some(neighbor) = deferred_neighbor {
        encoder = encoder.field_shape(
            ForgeQueryEvidenceTag::new("deferred_neighbor"),
            neighbor.as_str(),
        );
    }
    seal(encoder)
}

pub(crate) fn compose_transition_rules_digest(rule_digests: &[String]) -> String {
    compose_sequence_digest(
        "projection_consumption_transition_rules_v1",
        "rule",
        rule_digests.iter().map(String::as_str),
    )
}
