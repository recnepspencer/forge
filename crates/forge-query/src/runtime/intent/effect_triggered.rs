use crate::identity::hash_parts;
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectDelivery, ForgeQueryEffectPhaseEvidence,
    ForgeQueryEffectPolicy, ForgeQueryEffectTriggerSourceKind,
};
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectIntentReceipt {
    effect_name: String,
    trigger_commit_identity: String,
    trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
    pending_intent_target: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    phase_evidence: ForgeQueryEffectPhaseEvidence,
    intent_receipt: ForgeQueryIntentReceipt,
    receipt_digest: String,
}

impl ForgeQueryEffectIntentReceipt {
    pub(in crate::runtime) fn new(
        delivery: &ForgeQueryEffectDelivery,
        intent_receipt: ForgeQueryIntentReceipt,
    ) -> Self {
        let receipt_digest = hash_parts(&[
            "forge_query_effect_intent_receipt_v1".to_string(),
            format!("effect:{}", delivery.effect_name()),
            format!("trigger_commit:{}", delivery.commit_identity()),
            format!(
                "trigger_source_kind:{}",
                delivery.trigger_source_kind().as_str()
            ),
            format!("pending_target:{}", delivery.target()),
            format!(
                "source:{}",
                ForgeQueryIntentSourceLane::EffectTriggered.as_str()
            ),
            format!("target:{}", intent_receipt.target_lane()),
            format!("policy:{}", delivery.effect_policy().as_str()),
            format!(
                "phases:{}",
                delivery
                    .phase_evidence()
                    .phases()
                    .iter()
                    .map(|phase| phase.as_str())
                    .collect::<Vec<_>>()
                    .join(">")
            ),
            format!(
                "loop_prevention:{}",
                delivery.phase_evidence().loop_prevention().as_str()
            ),
            format!("intent_receipt:{}", intent_receipt.receipt_digest()),
        ]);
        Self {
            effect_name: delivery.effect_name().to_string(),
            trigger_commit_identity: delivery.commit_identity().to_string(),
            trigger_source_kind: delivery.trigger_source_kind(),
            pending_intent_target: delivery.target().to_string(),
            source_lane: ForgeQueryIntentSourceLane::EffectTriggered,
            target_lane: intent_receipt.target_lane(),
            effect_policy: delivery.effect_policy(),
            phase_evidence: delivery.phase_evidence().clone(),
            intent_receipt,
            receipt_digest,
        }
    }

    pub fn effect_name(&self) -> &str {
        &self.effect_name
    }

    pub fn trigger_commit_identity(&self) -> &str {
        &self.trigger_commit_identity
    }

    pub fn trigger_source_kind(&self) -> ForgeQueryEffectTriggerSourceKind {
        self.trigger_source_kind
    }

    pub fn pending_intent_target(&self) -> &str {
        &self.pending_intent_target
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn phase_evidence(&self) -> &ForgeQueryEffectPhaseEvidence {
        &self.phase_evidence
    }

    pub fn intent_receipt(&self) -> &ForgeQueryIntentReceipt {
        &self.intent_receipt
    }

    pub fn admission_family(&self) -> &str {
        self.intent_receipt.admission_family()
    }

    pub fn covered_entrypoint_label(&self) -> &str {
        self.intent_receipt.covered_entrypoint_label()
    }

    pub fn execution_seam_label(&self) -> &str {
        self.intent_receipt.execution_seam_label()
    }

    pub fn admission_decision_digest(&self) -> &str {
        self.intent_receipt.admission_decision_digest()
    }

    pub fn execution_handoff_digest(&self) -> &str {
        self.intent_receipt.execution_handoff_digest()
    }

    pub fn execution_binding_digest(&self) -> &str {
        self.intent_receipt.execution_binding_digest()
    }

    pub fn execution_provenance_chain_digest(&self) -> &str {
        self.intent_receipt.execution_provenance_chain_digest()
    }

    pub fn execution_provenance(&self) -> &ForgeQueryIntentExecutionProvenance {
        self.intent_receipt.execution_provenance()
    }

    pub fn decision_trace_envelope(&self) -> &ForgeQueryIntentDecisionTraceEnvelope {
        self.intent_receipt.decision_trace_envelope()
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
