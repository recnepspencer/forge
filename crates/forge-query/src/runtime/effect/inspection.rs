use crate::identity::hash_parts;

use super::super::ForgeQueryFeedbackPhaseGraphInspection;
use super::super::{ForgeQueryAuthorityLane, ForgeQueryEffectAction, ForgeQueryEffectPolicy};
use super::declaration::{
    ForgeQueryEffectCondition, ForgeQueryEffectExpressionFailurePosture,
    ForgeQueryEffectSuppressionPolicy, ForgeQueryEffectTriggerSourceKind,
};
use super::delivery::{ForgeQueryEffectCounters, ForgeQueryEffectDeliveryFamily};
use super::follow_on::{
    ForgeQueryEffectWriteAdjacentTrigger, ForgeQueryEffectWriteAdjacentTriggerClass,
};
use super::phase::ForgeQueryEffectPhaseEvidence;
use super::registry::ForgeQueryEffectRuntime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectInspectionEvidence {
    name: String,
    trigger_source: String,
    trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
    write_adjacent_trigger: ForgeQueryEffectWriteAdjacentTrigger,
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
    pending_delivered_count: usize,
    pending_suppressed_count: usize,
    pending_expression_failure_count: usize,
    pending_write_intent_count: usize,
    latest_delivery_family: Option<ForgeQueryEffectDeliveryFamily>,
    latest_phase_evidence: Option<ForgeQueryEffectPhaseEvidence>,
    feedback_graph: Option<ForgeQueryFeedbackPhaseGraphInspection>,
    trigger_digest: String,
    condition_digest: String,
    declaration_digest: String,
    pending_delivery_digest: String,
    latest_phase_digest: Option<String>,
    inspection_digest: String,
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
        let condition_failure_posture = match effect.declaration.condition() {
            ForgeQueryEffectCondition::Always => None,
            ForgeQueryEffectCondition::Expression(expression) => Some(expression.failure_posture()),
        };
        let pending_delivered_count = effect
            .deliveries
            .iter()
            .filter(|delivery| delivery.family() == &ForgeQueryEffectDeliveryFamily::Delivered)
            .count();
        let pending_suppressed_count = effect
            .deliveries
            .iter()
            .filter(|delivery| delivery.family() == &ForgeQueryEffectDeliveryFamily::Suppressed)
            .count();
        let pending_expression_failure_count = effect
            .deliveries
            .iter()
            .filter(|delivery| {
                delivery.family() == &ForgeQueryEffectDeliveryFamily::ExpressionFailed
            })
            .count();
        let pending_write_intent_count = effect
            .deliveries
            .iter()
            .filter(|delivery| {
                delivery.family() == &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
            })
            .count();
        let pending_delivery_count =
            pending_delivered_count + pending_suppressed_count + pending_expression_failure_count;
        let latest_delivery_family = effect
            .latest_delivery()
            .map(|delivery| delivery.family().clone());
        let latest_phase_evidence = effect
            .latest_delivery()
            .map(|delivery| delivery.phase_evidence().clone());
        let feedback_graph = ForgeQueryFeedbackPhaseGraphInspection::from_effect_runtime(effect);
        let trigger_digest = hash_parts(&[
            "forge_query_effect_trigger_inspection_v1".to_string(),
            format!("name:{}", effect.declaration.name()),
            format!(
                "source-kind:{}",
                effect.declaration.trigger().source_kind().as_str()
            ),
            format!("source:{}", effect.declaration.trigger().source_name()),
            format!(
                "write-adjacent-trigger:{}",
                effect.declaration.write_adjacent_trigger().digest()
            ),
            format!(
                "aspects:{}",
                effect.declaration.trigger().aspects().join("|")
            ),
        ]);
        let condition_digest = hash_parts(&[
            "forge_query_effect_condition_inspection_v1".to_string(),
            format!("name:{}", effect.declaration.name()),
            format!("descriptor:{condition_descriptor}"),
            format!("inputs:{}", condition_inputs.join("|")),
            format!("outputs:{}", condition_outputs.join("|")),
            format!(
                "failure-posture:{}",
                condition_failure_posture
                    .map(|posture| match posture {
                        ForgeQueryEffectExpressionFailurePosture::Admitted => "admitted",
                        ForgeQueryEffectExpressionFailurePosture::DeterministicFailure =>
                            "deterministic-failure",
                    })
                    .unwrap_or("none")
            ),
        ]);
        let declaration_digest = hash_parts(&[
            "forge_query_effect_declaration_inspection_v1".to_string(),
            format!("name:{}", effect.declaration.name()),
            format!("trigger:{trigger_digest}"),
            format!("condition:{condition_digest}"),
            format!("action:{:?}", effect.declaration.action()),
            format!("target-lane:{}", effect.declaration.target_lane()),
            format!("target:{}", effect.declaration.target()),
            format!("policy:{:?}", effect.declaration.effect_policy()),
            format!(
                "suppression:{}",
                effect.declaration.suppression_policy().as_str()
            ),
        ]);
        let pending_delivery_digest = hash_parts(
            &std::iter::once("forge_query_effect_pending_delivery_inspection_v1".to_string())
                .chain(std::iter::once(format!(
                    "name:{}",
                    effect.declaration.name()
                )))
                .chain(std::iter::once(format!(
                    "pending-delivery:{pending_delivery_count}"
                )))
                .chain(std::iter::once(format!(
                    "pending-delivered:{pending_delivered_count}"
                )))
                .chain(std::iter::once(format!(
                    "pending-suppressed:{pending_suppressed_count}"
                )))
                .chain(std::iter::once(format!(
                    "pending-expression-failure:{pending_expression_failure_count}"
                )))
                .chain(std::iter::once(format!(
                    "pending-write-intent:{pending_write_intent_count}"
                )))
                .chain(effect.deliveries.iter().map(|delivery| {
                    format!(
                        "{}:{}:{}:{}:{}",
                        delivery.effect_name(),
                        delivery.commit_identity(),
                        delivery.trigger_source_kind().as_str(),
                        match delivery.family() {
                            ForgeQueryEffectDeliveryFamily::Delivered => "delivered",
                            ForgeQueryEffectDeliveryFamily::PendingWriteIntent =>
                                "pending-write-intent",
                            ForgeQueryEffectDeliveryFamily::Suppressed => "suppressed",
                            ForgeQueryEffectDeliveryFamily::ExpressionFailed => "expression-failed",
                        },
                        delivery.aspect_paths().join("|")
                    )
                }))
                .collect::<Vec<_>>(),
        );
        let latest_phase_digest = latest_phase_evidence.as_ref().map(|phase| {
            hash_parts(&[
                "forge_query_effect_phase_inspection_v1".to_string(),
                format!(
                    "phases:{}",
                    phase
                        .phases()
                        .iter()
                        .map(|entry| entry.as_str())
                        .collect::<Vec<_>>()
                        .join("|")
                ),
                format!("loop-prevention:{}", phase.loop_prevention().as_str()),
                format!("idempotence:{}", phase.idempotence().as_str()),
            ])
        });
        let inspection_digest = hash_parts(&[
            "forge_query_effect_inspection_v1".to_string(),
            declaration_digest.clone(),
            pending_delivery_digest.clone(),
            format!(
                "latest-family:{}",
                latest_delivery_family
                    .as_ref()
                    .map(|family| match family {
                        ForgeQueryEffectDeliveryFamily::Delivered => "delivered",
                        ForgeQueryEffectDeliveryFamily::PendingWriteIntent =>
                            "pending-write-intent",
                        ForgeQueryEffectDeliveryFamily::Suppressed => "suppressed",
                        ForgeQueryEffectDeliveryFamily::ExpressionFailed => "expression-failed",
                    })
                    .unwrap_or("none")
            ),
            latest_phase_digest
                .clone()
                .unwrap_or_else(|| "no-phase-evidence".to_string()),
            format!(
                "feedback-graph:{}",
                feedback_graph
                    .as_ref()
                    .map(|graph| graph.graph_digest().to_string())
                    .unwrap_or_else(|| "none".to_string())
            ),
        ]);

        Self {
            name: effect.declaration.name().to_string(),
            trigger_source: effect.declaration.trigger().source_name().to_string(),
            trigger_source_kind: effect.declaration.trigger().source_kind(),
            write_adjacent_trigger: effect.declaration.write_adjacent_trigger().clone(),
            trigger_aspects: effect.declaration.trigger().aspects().to_vec(),
            condition_descriptor,
            condition_inputs,
            condition_outputs,
            condition_failure_posture,
            action: effect.declaration.action(),
            target_lane: effect.declaration.target_lane(),
            target: effect.declaration.target().to_string(),
            effect_policy: effect.declaration.effect_policy(),
            suppression_policy: effect.declaration.suppression_policy(),
            counters: effect.counters.clone(),
            pending_delivery_count,
            pending_delivered_count,
            pending_suppressed_count,
            pending_expression_failure_count,
            pending_write_intent_count,
            latest_delivery_family,
            latest_phase_evidence,
            feedback_graph,
            trigger_digest,
            condition_digest,
            declaration_digest,
            pending_delivery_digest,
            latest_phase_digest,
            inspection_digest,
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

    pub fn write_adjacent_trigger(&self) -> &ForgeQueryEffectWriteAdjacentTrigger {
        &self.write_adjacent_trigger
    }

    pub fn write_adjacent_trigger_class(&self) -> ForgeQueryEffectWriteAdjacentTriggerClass {
        self.write_adjacent_trigger.class()
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

    pub fn pending_delivered_count(&self) -> usize {
        self.pending_delivered_count
    }

    pub fn pending_suppressed_count(&self) -> usize {
        self.pending_suppressed_count
    }

    pub fn pending_expression_failure_count(&self) -> usize {
        self.pending_expression_failure_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn latest_delivery_family(&self) -> Option<&ForgeQueryEffectDeliveryFamily> {
        self.latest_delivery_family.as_ref()
    }

    pub fn latest_phase_evidence(&self) -> Option<&ForgeQueryEffectPhaseEvidence> {
        self.latest_phase_evidence.as_ref()
    }

    pub fn feedback_graph(&self) -> Option<&ForgeQueryFeedbackPhaseGraphInspection> {
        self.feedback_graph.as_ref()
    }

    pub fn trigger_digest(&self) -> &str {
        &self.trigger_digest
    }

    pub fn condition_digest(&self) -> &str {
        &self.condition_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn pending_delivery_digest(&self) -> &str {
        &self.pending_delivery_digest
    }

    pub fn latest_phase_digest(&self) -> Option<&str> {
        self.latest_phase_digest.as_deref()
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
