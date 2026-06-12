use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::runtime::ForgeQueryIntentConsumerInspection;

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectDelivery, ForgeQueryEffectPhaseEvidence,
    ForgeQueryEffectPolicy, ForgeQueryEffectTriggerSourceKind,
    ForgeQueryEffectWriteAdjacentTrigger, ForgeQueryEffectWriteAdjacentTriggerClass,
};
use super::receipt_identity::effect_intent_receipt_identity;
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectIntentReceipt {
    effect_name: String,
    trigger_commit_evidence_identity: ForgeQueryEvidenceIdentity,
    trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
    write_adjacent_trigger: ForgeQueryEffectWriteAdjacentTrigger,
    pending_intent_target: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    phase_evidence: ForgeQueryEffectPhaseEvidence,
    intent_receipt: ForgeQueryIntentReceipt,
    receipt_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryEffectIntentReceipt {
    pub(in crate::runtime) fn new(
        delivery: &ForgeQueryEffectDelivery,
        intent_receipt: ForgeQueryIntentReceipt,
    ) -> Self {
        let receipt_identity = effect_intent_receipt_identity(delivery, &intent_receipt);
        Self {
            effect_name: delivery.effect_name().to_string(),
            trigger_commit_evidence_identity: delivery.trigger_commit_evidence_identity().clone(),
            trigger_source_kind: delivery.trigger_source_kind(),
            write_adjacent_trigger: delivery.write_adjacent_trigger().clone(),
            pending_intent_target: delivery.target().to_string(),
            source_lane: ForgeQueryIntentSourceLane::EffectTriggered,
            target_lane: intent_receipt.target_lane(),
            effect_policy: delivery.effect_policy(),
            phase_evidence: delivery.phase_evidence().clone(),
            intent_receipt,
            receipt_identity,
        }
    }

    pub fn effect_name(&self) -> &str {
        &self.effect_name
    }

    pub fn trigger_commit_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.trigger_commit_evidence_identity
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

    pub fn consumer_inspection(&self) -> ForgeQueryIntentConsumerInspection<'_> {
        self.intent_receipt.consumer_inspection()
    }

    pub fn receipt_digest(&self) -> &str {
        self.receipt_identity.as_str()
    }

    pub fn receipt_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.receipt_identity
    }
}
