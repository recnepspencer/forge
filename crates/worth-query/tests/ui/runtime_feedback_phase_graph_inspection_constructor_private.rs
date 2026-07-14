use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryEffectPolicy, WorthQueryFeedbackPhaseGraphInspection, WorthQueryFeedbackPhaseNode, WorthQueryFeedbackTermination};

fn main() {
    let _ = WorthQueryFeedbackPhaseGraphInspection {
        effect_name: String::new(),
        trigger_source_kind: worth_query::facade::runtime::WorthQueryEffectTriggerSourceKind::LiveView,
        trigger_commit_identity: String::new(),
        source_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        terminal_lane: WorthQueryAuthorityLane::EffectDeliveryState,
        effect_policy: Some(WorthQueryEffectPolicy::AuthoritativeAllowed),
        loop_prevention: worth_query::facade::runtime::WorthQueryEffectLoopPrevention::SingleCommitBoundary,
        idempotence: worth_query::facade::runtime::WorthQueryEffectIdempotence::DeliveryReceiptIdentity,
        termination: WorthQueryFeedbackTermination::Delivered,
        phase_nodes: vec![WorthQueryFeedbackPhaseNode::TruthRead],
        resubscribed_live_view_count: 0,
        resubscribed_derived_view_count: 0,
        pending_write_intent_count: 0,
        graph_digest: String::new(),
        inspection_digest: String::new(),
    };
}
