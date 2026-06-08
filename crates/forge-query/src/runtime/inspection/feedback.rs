use crate::identity::hash_parts;

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectDeliveryFamily, ForgeQueryEffectIdempotence,
    ForgeQueryEffectIntentReceipt, ForgeQueryEffectLoopPrevention, ForgeQueryEffectPolicy,
    ForgeQueryEffectRuntime, ForgeQueryEffectTriggerSourceKind,
    ForgeQueryEffectWriteAdjacentTriggerClass, ForgeQueryIntentExecutionKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryFeedbackPhaseNode {
    TruthRead,
    Derive,
    EffectDelivery,
    PendingWriteIntent,
    Suppressed,
    ExpressionFailure,
    Commit,
    BridgeRoute,
    Resubscribe,
}

impl ForgeQueryFeedbackPhaseNode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TruthRead => "truth-read",
            Self::Derive => "derive",
            Self::EffectDelivery => "effect-delivery",
            Self::PendingWriteIntent => "pending-write-intent",
            Self::Suppressed => "suppressed",
            Self::ExpressionFailure => "expression-failure",
            Self::Commit => "commit",
            Self::BridgeRoute => "bridge-route",
            Self::Resubscribe => "resubscribe",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryFeedbackTermination {
    Delivered,
    PendingIntentDeferred,
    Suppressed,
    ExpressionFailed,
    CommittedResubscribe,
    CoalescedNoMutation,
}

impl ForgeQueryFeedbackTermination {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Delivered => "delivered",
            Self::PendingIntentDeferred => "pending-intent-deferred",
            Self::Suppressed => "suppressed",
            Self::ExpressionFailed => "expression-failed",
            Self::CommittedResubscribe => "committed-resubscribe",
            Self::CoalescedNoMutation => "coalesced-no-mutation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryFeedbackPhaseGraphInspection {
    effect_name: String,
    trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
    write_adjacent_trigger_class: ForgeQueryEffectWriteAdjacentTriggerClass,
    write_adjacent_trigger_origin_identity: String,
    trigger_commit_identity: String,
    source_lane: ForgeQueryAuthorityLane,
    terminal_lane: ForgeQueryAuthorityLane,
    effect_policy: Option<ForgeQueryEffectPolicy>,
    loop_prevention: ForgeQueryEffectLoopPrevention,
    idempotence: ForgeQueryEffectIdempotence,
    termination: ForgeQueryFeedbackTermination,
    phase_nodes: Vec<ForgeQueryFeedbackPhaseNode>,
    resubscribed_live_view_count: usize,
    resubscribed_derived_view_count: usize,
    pending_write_intent_count: usize,
    graph_digest: String,
    inspection_digest: String,
}

impl ForgeQueryFeedbackPhaseGraphInspection {
    pub(in crate::runtime) fn from_effect_runtime(
        runtime: &ForgeQueryEffectRuntime,
    ) -> Option<Self> {
        let latest = runtime.latest_delivery()?;
        let phase_nodes = latest
            .phase_evidence()
            .phases()
            .iter()
            .map(|phase| match phase.as_str() {
                "truth-read" => ForgeQueryFeedbackPhaseNode::TruthRead,
                "derive" => ForgeQueryFeedbackPhaseNode::Derive,
                "effect-delivery" => ForgeQueryFeedbackPhaseNode::EffectDelivery,
                "pending-write-intent" => ForgeQueryFeedbackPhaseNode::PendingWriteIntent,
                "suppressed" => ForgeQueryFeedbackPhaseNode::Suppressed,
                "expression-failure" => ForgeQueryFeedbackPhaseNode::ExpressionFailure,
                _ => unreachable!("effect phase vocabulary is closed"),
            })
            .collect::<Vec<_>>();
        let termination = match latest.family() {
            ForgeQueryEffectDeliveryFamily::Delivered => ForgeQueryFeedbackTermination::Delivered,
            ForgeQueryEffectDeliveryFamily::PendingWriteIntent => {
                ForgeQueryFeedbackTermination::PendingIntentDeferred
            }
            ForgeQueryEffectDeliveryFamily::Suppressed => ForgeQueryFeedbackTermination::Suppressed,
            ForgeQueryEffectDeliveryFamily::ExpressionFailed => {
                ForgeQueryFeedbackTermination::ExpressionFailed
            }
        };
        Some(Self::new(
            runtime.name(),
            latest.trigger_source_kind(),
            latest.write_adjacent_trigger().class(),
            latest.write_adjacent_trigger().origin_identity(),
            latest.commit_identity(),
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            latest.authority_lane(),
            Some(runtime.effect_policy()),
            latest.phase_evidence().loop_prevention(),
            latest.phase_evidence().idempotence(),
            termination,
            phase_nodes,
            0,
            0,
            runtime.pending_write_intent_count(),
        ))
    }

    pub(in crate::runtime) fn from_effect_intent_receipt(
        receipt: &ForgeQueryEffectIntentReceipt,
    ) -> Self {
        let mut phase_nodes = receipt
            .phase_evidence()
            .phases()
            .iter()
            .map(|phase| match phase.as_str() {
                "truth-read" => ForgeQueryFeedbackPhaseNode::TruthRead,
                "derive" => ForgeQueryFeedbackPhaseNode::Derive,
                "effect-delivery" => ForgeQueryFeedbackPhaseNode::EffectDelivery,
                "pending-write-intent" => ForgeQueryFeedbackPhaseNode::PendingWriteIntent,
                "suppressed" => ForgeQueryFeedbackPhaseNode::Suppressed,
                "expression-failure" => ForgeQueryFeedbackPhaseNode::ExpressionFailure,
                _ => unreachable!("effect phase vocabulary is closed"),
            })
            .collect::<Vec<_>>();
        phase_nodes.push(ForgeQueryFeedbackPhaseNode::Commit);

        let (termination, resubscribed_live_view_count, resubscribed_derived_view_count) =
            if receipt.intent_receipt().execution_kind() == ForgeQueryIntentExecutionKind::Mutating
            {
                phase_nodes.push(ForgeQueryFeedbackPhaseNode::BridgeRoute);
                phase_nodes.push(ForgeQueryFeedbackPhaseNode::Resubscribe);
                (
                    ForgeQueryFeedbackTermination::CommittedResubscribe,
                    receipt.intent_receipt().affected_live_view_ids().len(),
                    receipt.intent_receipt().affected_derived_view_ids().len(),
                )
            } else {
                (ForgeQueryFeedbackTermination::CoalescedNoMutation, 0, 0)
            };

        Self::new(
            receipt.effect_name(),
            receipt.trigger_source_kind(),
            receipt.write_adjacent_trigger_class(),
            receipt.write_adjacent_trigger().origin_identity(),
            receipt.trigger_commit_identity(),
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            receipt.target_lane(),
            Some(receipt.effect_policy()),
            receipt.phase_evidence().loop_prevention(),
            receipt.phase_evidence().idempotence(),
            termination,
            phase_nodes,
            resubscribed_live_view_count,
            resubscribed_derived_view_count,
            receipt.intent_receipt().pending_write_intent_count(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        effect_name: &str,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        write_adjacent_trigger_class: ForgeQueryEffectWriteAdjacentTriggerClass,
        write_adjacent_trigger_origin_identity: &str,
        trigger_commit_identity: &str,
        source_lane: ForgeQueryAuthorityLane,
        terminal_lane: ForgeQueryAuthorityLane,
        effect_policy: Option<ForgeQueryEffectPolicy>,
        loop_prevention: ForgeQueryEffectLoopPrevention,
        idempotence: ForgeQueryEffectIdempotence,
        termination: ForgeQueryFeedbackTermination,
        phase_nodes: Vec<ForgeQueryFeedbackPhaseNode>,
        resubscribed_live_view_count: usize,
        resubscribed_derived_view_count: usize,
        pending_write_intent_count: usize,
    ) -> Self {
        let graph_digest = hash_parts(&[
            "forge_query_feedback_phase_graph_v1".to_string(),
            format!("effect:{effect_name}"),
            format!("trigger-source-kind:{}", trigger_source_kind.as_str()),
            format!(
                "write-adjacent-trigger-class:{}",
                write_adjacent_trigger_class.as_str()
            ),
            format!("write-adjacent-trigger-origin:{write_adjacent_trigger_origin_identity}"),
            format!("trigger-commit:{trigger_commit_identity}"),
            format!("source-lane:{source_lane}"),
            format!("terminal-lane:{terminal_lane}"),
            format!(
                "policy:{}",
                effect_policy
                    .map(ForgeQueryEffectPolicy::as_str)
                    .unwrap_or("none")
            ),
            format!("loop-prevention:{}", loop_prevention.as_str()),
            format!("idempotence:{}", idempotence.as_str()),
            format!("termination:{}", termination.as_str()),
            format!(
                "phases:{}",
                phase_nodes
                    .iter()
                    .map(|node| node.as_str())
                    .collect::<Vec<_>>()
                    .join(">")
            ),
            format!("resub-live:{resubscribed_live_view_count}"),
            format!("resub-derived:{resubscribed_derived_view_count}"),
            format!("pending-write-intents:{pending_write_intent_count}"),
        ]);
        let inspection_digest = hash_parts(&[
            "forge_query_feedback_phase_graph_inspection_v1".to_string(),
            graph_digest.clone(),
        ]);
        Self {
            effect_name: effect_name.to_string(),
            trigger_source_kind,
            write_adjacent_trigger_class,
            write_adjacent_trigger_origin_identity: write_adjacent_trigger_origin_identity
                .to_string(),
            trigger_commit_identity: trigger_commit_identity.to_string(),
            source_lane,
            terminal_lane,
            effect_policy,
            loop_prevention,
            idempotence,
            termination,
            phase_nodes,
            resubscribed_live_view_count,
            resubscribed_derived_view_count,
            pending_write_intent_count,
            graph_digest,
            inspection_digest,
        }
    }

    pub fn effect_name(&self) -> &str {
        &self.effect_name
    }

    pub fn trigger_source_kind(&self) -> ForgeQueryEffectTriggerSourceKind {
        self.trigger_source_kind
    }

    pub fn write_adjacent_trigger_class(&self) -> ForgeQueryEffectWriteAdjacentTriggerClass {
        self.write_adjacent_trigger_class
    }

    pub fn write_adjacent_trigger_origin_identity(&self) -> &str {
        &self.write_adjacent_trigger_origin_identity
    }

    pub fn trigger_commit_identity(&self) -> &str {
        &self.trigger_commit_identity
    }

    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }

    pub fn terminal_lane(&self) -> ForgeQueryAuthorityLane {
        self.terminal_lane
    }

    pub fn effect_policy(&self) -> Option<ForgeQueryEffectPolicy> {
        self.effect_policy
    }

    pub fn loop_prevention(&self) -> ForgeQueryEffectLoopPrevention {
        self.loop_prevention
    }

    pub fn idempotence(&self) -> ForgeQueryEffectIdempotence {
        self.idempotence
    }

    pub fn termination(&self) -> ForgeQueryFeedbackTermination {
        self.termination
    }

    pub fn phase_nodes(&self) -> &[ForgeQueryFeedbackPhaseNode] {
        &self.phase_nodes
    }

    pub fn resubscribed_live_view_count(&self) -> usize {
        self.resubscribed_live_view_count
    }

    pub fn resubscribed_derived_view_count(&self) -> usize {
        self.resubscribed_derived_view_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn graph_digest(&self) -> &str {
        &self.graph_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
