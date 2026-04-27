use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryFeedbackPhaseGraphInspection,
    ForgeQueryFeedbackPhaseNode, ForgeQueryFeedbackTermination,
};

fn main() {
    let _ = ForgeQueryFeedbackPhaseGraphInspection {
        effect_name: String::new(),
        trigger_source_kind: forge_query::facade::ForgeQueryEffectTriggerSourceKind::LiveView,
        trigger_commit_identity: String::new(),
        source_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        terminal_lane: ForgeQueryAuthorityLane::EffectDeliveryState,
        effect_policy: Some(ForgeQueryEffectPolicy::AuthoritativeAllowed),
        loop_prevention: forge_query::facade::ForgeQueryEffectLoopPrevention::SingleCommitBoundary,
        idempotence: forge_query::facade::ForgeQueryEffectIdempotence::DeliveryReceiptIdentity,
        termination: ForgeQueryFeedbackTermination::Delivered,
        phase_nodes: vec![ForgeQueryFeedbackPhaseNode::TruthRead],
        resubscribed_live_view_count: 0,
        resubscribed_derived_view_count: 0,
        pending_write_intent_count: 0,
        graph_digest: String::new(),
        inspection_digest: String::new(),
    };
}
