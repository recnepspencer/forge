use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectIdempotence, ForgeQueryEffectLoopPrevention,
    ForgeQueryEffectPolicy, ForgeQueryEffectTriggerSourceKind,
    ForgeQueryEffectWriteAdjacentTriggerClass,
};
use super::feedback::{ForgeQueryFeedbackPhaseNode, ForgeQueryFeedbackTermination};

pub(super) fn feedback_phase_graph_identity(
    parts: FeedbackPhaseGraphIdentityParts<'_>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::FeedbackPhaseGraph)
        .field_shape(ForgeQueryEvidenceTag::new("effect_name"), parts.effect_name)
        .field_shape(
            ForgeQueryEvidenceTag::new("trigger_source_kind"),
            parts.trigger_source_kind.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("write_adjacent_trigger_class"),
            parts.write_adjacent_trigger_class.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("write_adjacent_trigger_origin_identity"),
            parts.write_adjacent_trigger_origin_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trigger_commit_identity"),
            parts.trigger_commit_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            parts.source_lane.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("terminal_lane"),
            parts.terminal_lane.as_str(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            parts.effect_policy.map(ForgeQueryEffectPolicy::as_str),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("loop_prevention"),
            parts.loop_prevention.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("idempotence"),
            parts.idempotence.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("termination"),
            parts.termination.as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("phase_node"),
            parts.phase_nodes.iter().map(|node| node.as_str()),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("resubscribed_live_view_count"),
            parts.resubscribed_live_view_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("resubscribed_derived_view_count"),
            parts.resubscribed_derived_view_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_write_intent_count"),
            parts.pending_write_intent_count,
        )
        .seal()
}

pub(super) fn feedback_phase_graph_inspection_identity(
    graph_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::FeedbackPhaseGraphInspection)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("graph_digest"), graph_identity)
        .seal()
}

pub(super) struct FeedbackPhaseGraphIdentityParts<'a> {
    pub(super) effect_name: &'a str,
    pub(super) trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
    pub(super) write_adjacent_trigger_class: ForgeQueryEffectWriteAdjacentTriggerClass,
    pub(super) write_adjacent_trigger_origin_identity: &'a ForgeQueryEvidenceIdentity,
    pub(super) trigger_commit_identity: &'a ForgeQueryEvidenceIdentity,
    pub(super) source_lane: ForgeQueryAuthorityLane,
    pub(super) terminal_lane: ForgeQueryAuthorityLane,
    pub(super) effect_policy: Option<ForgeQueryEffectPolicy>,
    pub(super) loop_prevention: ForgeQueryEffectLoopPrevention,
    pub(super) idempotence: ForgeQueryEffectIdempotence,
    pub(super) termination: ForgeQueryFeedbackTermination,
    pub(super) phase_nodes: &'a [ForgeQueryFeedbackPhaseNode],
    pub(super) resubscribed_live_view_count: usize,
    pub(super) resubscribed_derived_view_count: usize,
    pub(super) pending_write_intent_count: usize,
}
