use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryEffectAction, WorthQueryEffectCounters,
    WorthQueryEffectInspectionEvidence, WorthQueryEffectPolicy, WorthQueryEffectSuppressionPolicy,
    WorthQueryEffectTriggerSourceKind,
};

fn main() {
    let _worthd = WorthQueryEffectInspectionEvidence {
        name: "ui.Worthd".to_string(),
        trigger_source: "tasks.table".to_string(),
        trigger_source_kind: WorthQueryEffectTriggerSourceKind::LiveView,
        trigger_aspects: Vec::new(),
        condition_descriptor: "always".to_string(),
        condition_inputs: Vec::new(),
        condition_outputs: Vec::new(),
        condition_failure_posture: None,
        action: WorthQueryEffectAction::Deliver,
        target_lane: WorthQueryAuthorityLane::EffectDeliveryState,
        target: "ui.badges".to_string(),
        effect_policy: WorthQueryEffectPolicy::AuthoritativeAllowed,
        suppression_policy: WorthQueryEffectSuppressionPolicy::None,
        counters: WorthQueryEffectCounters::default(),
        pending_delivery_count: 0,
        pending_delivered_count: 0,
        pending_suppressed_count: 0,
        pending_expression_failure_count: 0,
        pending_write_intent_count: 0,
        latest_delivery_family: None,
        latest_phase_evidence: None,
        trigger_digest: String::new(),
        condition_digest: String::new(),
        declaration_digest: String::new(),
        pending_delivery_digest: String::new(),
        latest_phase_digest: None,
        inspection_digest: String::new(),
    };
}
