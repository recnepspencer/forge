use super::super::{ForgeQueryAuthorityLane, ForgeQueryEffectAction, ForgeQueryEffectPolicy};
use super::declaration::{
    ForgeQueryEffectCondition, ForgeQueryEffectExpressionFailurePosture,
    ForgeQueryEffectSuppressionPolicy, ForgeQueryEffectTriggerSourceKind,
};
use super::delivery::ForgeQueryEffectCounters;
use super::phase::ForgeQueryEffectPhaseEvidence;
use super::registry::ForgeQueryEffectRuntime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectInspectionEvidence {
    name: String,
    trigger_source: String,
    trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
    trigger_aspects: Vec<String>,
    condition_descriptor: String,
    condition_inputs: Vec<String>,
    condition_outputs: Vec<String>,
    condition_failure_posture: Option<ForgeQueryEffectExpressionFailurePosture>,
    action: ForgeQueryEffectAction,
    target_lane: ForgeQueryAuthorityLane,
    target: String,
    effect_policy: ForgeQueryEffectPolicy,
    suppression_policy: ForgeQueryEffectSuppressionPolicy,
    counters: ForgeQueryEffectCounters,
    pending_delivery_count: usize,
    pending_write_intent_count: usize,
    latest_phase_evidence: Option<ForgeQueryEffectPhaseEvidence>,
}
impl ForgeQueryEffectInspectionEvidence {
    pub(in crate::runtime) fn from_runtime(effect: &ForgeQueryEffectRuntime) -> Self {
        let (condition_descriptor, condition_inputs, condition_outputs) =
            match effect.declaration.condition() {
                ForgeQueryEffectCondition::Always => ("always".to_string(), Vec::new(), Vec::new()),
                ForgeQueryEffectCondition::Expression(expression) => (
                    expression.descriptor().to_string(),
                    expression.input_aspects().to_vec(),
                    expression.output_aspects().to_vec(),
                ),
            };
        Self {
            name: effect.declaration.name().to_string(),
            trigger_source: effect.declaration.trigger().source_name().to_string(),
            trigger_source_kind: effect.declaration.trigger().source_kind(),
            trigger_aspects: effect.declaration.trigger().aspects().to_vec(),
            condition_descriptor,
            condition_inputs,
            condition_outputs,
            condition_failure_posture: match effect.declaration.condition() {
                ForgeQueryEffectCondition::Always => None,
                ForgeQueryEffectCondition::Expression(expression) => {
                    Some(expression.failure_posture())
                }
            },
            action: effect.declaration.action(),
            target_lane: effect.declaration.target_lane(),
            target: effect.declaration.target().to_string(),
            effect_policy: effect.declaration.effect_policy(),
            suppression_policy: effect.declaration.suppression_policy(),
            counters: effect.counters.clone(),
            pending_delivery_count: effect
                .deliveries
                .iter()
                .filter(|delivery| {
                    delivery.family()
                        != &super::delivery::ForgeQueryEffectDeliveryFamily::PendingWriteIntent
                })
                .count(),
            pending_write_intent_count: effect
                .deliveries
                .iter()
                .filter(|delivery| {
                    delivery.family()
                        == &super::delivery::ForgeQueryEffectDeliveryFamily::PendingWriteIntent
                })
                .count(),
            latest_phase_evidence: effect
                .deliveries
                .last()
                .map(|delivery| delivery.phase_evidence().clone()),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn trigger_source(&self) -> &str {
        &self.trigger_source
    }
    pub fn trigger_source_kind(&self) -> ForgeQueryEffectTriggerSourceKind {
        self.trigger_source_kind
    }
    pub fn trigger_aspects(&self) -> &[String] {
        &self.trigger_aspects
    }
    pub fn condition_descriptor(&self) -> &str {
        &self.condition_descriptor
    }
    pub fn condition_inputs(&self) -> &[String] {
        &self.condition_inputs
    }
    pub fn condition_outputs(&self) -> &[String] {
        &self.condition_outputs
    }
    pub fn condition_failure_posture(&self) -> Option<ForgeQueryEffectExpressionFailurePosture> {
        self.condition_failure_posture
    }
    pub fn action(&self) -> ForgeQueryEffectAction {
        self.action
    }
    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }
    pub fn target(&self) -> &str {
        &self.target
    }
    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }
    pub fn suppression_policy(&self) -> ForgeQueryEffectSuppressionPolicy {
        self.suppression_policy
    }
    pub fn counters(&self) -> &ForgeQueryEffectCounters {
        &self.counters
    }
    pub fn pending_delivery_count(&self) -> usize {
        self.pending_delivery_count
    }
    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }
    pub fn latest_phase_evidence(&self) -> Option<&ForgeQueryEffectPhaseEvidence> {
        self.latest_phase_evidence.as_ref()
    }
}
