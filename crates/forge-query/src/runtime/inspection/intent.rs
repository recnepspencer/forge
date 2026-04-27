use crate::identity::hash_parts;

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryBranchIntentReceipt, ForgeQueryEffectIntentReceipt,
    ForgeQueryEffectPolicy, ForgeQueryIntentDenialEvidence, ForgeQueryIntentExecutionKind,
    ForgeQueryIntentReceipt, ForgeQueryIntentSourceLane,
};
use super::feedback::ForgeQueryFeedbackPhaseGraphInspection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentInspectionDeliveryCounters {
    affected_live_view_count: usize,
    affected_derived_view_count: usize,
    considered_computed_view_count: usize,
    considered_effect_count: usize,
    delivered_effect_count: usize,
    pending_write_intent_count: usize,
    suppressed_effect_count: usize,
    meaningful_effect_suppression_count: usize,
    effect_expression_failure_count: usize,
    refresh_fallback: bool,
    counter_digest: String,
}

impl ForgeQueryIntentInspectionDeliveryCounters {
    pub(in crate::runtime) fn from_receipt(receipt: &ForgeQueryIntentReceipt) -> Self {
        let affected_live_view_count = receipt.affected_live_view_ids().len();
        let affected_derived_view_count = receipt.affected_derived_view_ids().len();
        let considered_computed_view_count = receipt.considered_computed_view_count();
        let considered_effect_count = receipt.considered_effect_count();
        let delivered_effect_count = receipt.delivered_effect_count();
        let pending_write_intent_count = receipt.pending_write_intent_count();
        let suppressed_effect_count = receipt.suppressed_effect_count();
        let meaningful_effect_suppression_count = receipt.meaningful_effect_suppression_count();
        let effect_expression_failure_count = receipt.effect_expression_failure_count();
        let refresh_fallback = receipt.refresh_fallback();
        let counter_digest = hash_parts(&[
            "forge_query_intent_inspection_delivery_counters_v1".to_string(),
            format!("live:{affected_live_view_count}"),
            format!("derived:{affected_derived_view_count}"),
            format!("computed-considered:{considered_computed_view_count}"),
            format!("effects-considered:{considered_effect_count}"),
            format!("effects-delivered:{delivered_effect_count}"),
            format!("pending-write-intents:{pending_write_intent_count}"),
            format!("effects-suppressed:{suppressed_effect_count}"),
            format!("meaningful-suppressions:{meaningful_effect_suppression_count}"),
            format!("effect-expression-failures:{effect_expression_failure_count}"),
            format!("refresh-fallback:{refresh_fallback}"),
        ]);
        Self {
            affected_live_view_count,
            affected_derived_view_count,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
            counter_digest,
        }
    }

    pub fn affected_live_view_count(&self) -> usize {
        self.affected_live_view_count
    }

    pub fn affected_derived_view_count(&self) -> usize {
        self.affected_derived_view_count
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }

    pub fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn suppressed_effect_count(&self) -> usize {
        self.suppressed_effect_count
    }

    pub fn meaningful_effect_suppression_count(&self) -> usize {
        self.meaningful_effect_suppression_count
    }

    pub fn effect_expression_failure_count(&self) -> usize {
        self.effect_expression_failure_count
    }

    pub fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentReceiptInspection {
    intent_name: String,
    execution_kind: ForgeQueryIntentExecutionKind,
    strategy_identity: String,
    strategy_version: String,
    strategy_descriptor_digest: String,
    canonical_input_digest: String,
    outcome_digest: String,
    produced_mutation_digest: Option<String>,
    invariant_evidence: Vec<String>,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    commit_identity: String,
    snapshot_token: String,
    receipt_digest: String,
    delivery_counters: ForgeQueryIntentInspectionDeliveryCounters,
    inspection_digest: String,
}

impl ForgeQueryIntentReceiptInspection {
    pub(in crate::runtime) fn from_receipt(receipt: &ForgeQueryIntentReceipt) -> Self {
        let delivery_counters = ForgeQueryIntentInspectionDeliveryCounters::from_receipt(receipt);
        let produced_mutation_digest = receipt.produced_mutation_digest().map(str::to_string);
        let invariant_evidence = receipt.invariant_evidence().to_vec();
        let invariant_evidence_digest_part = invariant_evidence.join("|");
        let inspection_digest = hash_parts(&[
            "forge_query_intent_receipt_inspection_v1".to_string(),
            format!("intent:{}", receipt.intent_name()),
            format!("execution-kind:{}", receipt.execution_kind().as_str()),
            format!("strategy:{}", receipt.strategy_identity()),
            format!("version:{}", receipt.strategy_version()),
            format!("descriptor:{}", receipt.strategy_descriptor_digest()),
            format!("input:{}", receipt.canonical_input_digest()),
            format!("outcome:{}", receipt.outcome_digest()),
            format!(
                "produced-mutation:{}",
                produced_mutation_digest.as_deref().unwrap_or("none")
            ),
            format!("invariants:{invariant_evidence_digest_part}"),
            format!("source:{}", receipt.source_lane().as_str()),
            format!("target:{}", receipt.target_lane()),
            format!("commit:{}", receipt.commit_identity()),
            format!("snapshot:{}", receipt.snapshot_token()),
            format!("receipt:{}", receipt.receipt_digest()),
            format!("counters:{}", delivery_counters.counter_digest()),
        ]);
        Self {
            intent_name: receipt.intent_name().to_string(),
            execution_kind: receipt.execution_kind(),
            strategy_identity: receipt.strategy_identity().to_string(),
            strategy_version: receipt.strategy_version().to_string(),
            strategy_descriptor_digest: receipt.strategy_descriptor_digest().to_string(),
            canonical_input_digest: receipt.canonical_input_digest().to_string(),
            outcome_digest: receipt.outcome_digest().to_string(),
            produced_mutation_digest,
            invariant_evidence,
            source_lane: receipt.source_lane(),
            target_lane: receipt.target_lane(),
            commit_identity: receipt.commit_identity().to_string(),
            snapshot_token: receipt.snapshot_token().to_string(),
            receipt_digest: receipt.receipt_digest().to_string(),
            delivery_counters,
            inspection_digest,
        }
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn execution_kind(&self) -> ForgeQueryIntentExecutionKind {
        self.execution_kind
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
    }

    pub fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub fn produced_mutation_digest(&self) -> Option<&str> {
        self.produced_mutation_digest.as_deref()
    }

    pub fn invariant_evidence(&self) -> &[String] {
        &self.invariant_evidence
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn delivery_counters(&self) -> &ForgeQueryIntentInspectionDeliveryCounters {
        &self.delivery_counters
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentDenialInspection {
    intent_name: String,
    stage: &'static str,
    message: String,
    strategy_identity: String,
    strategy_version: String,
    returned_strategy_identity: Option<String>,
    returned_strategy_version: Option<String>,
    returned_strategy_descriptor_digest: Option<String>,
    canonical_input_digest: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    execution_kind: Option<ForgeQueryIntentExecutionKind>,
    attempt_digest: Option<String>,
    invariant_evidence: Vec<String>,
    snapshot_token: Option<String>,
    denial_digest: String,
    inspection_digest: String,
}

impl ForgeQueryIntentDenialInspection {
    pub(in crate::runtime) fn from_evidence(evidence: &ForgeQueryIntentDenialEvidence) -> Self {
        let returned_strategy_identity = evidence.returned_strategy_identity().map(str::to_string);
        let returned_strategy_version = evidence.returned_strategy_version().map(str::to_string);
        let returned_strategy_descriptor_digest = evidence
            .returned_strategy_descriptor_digest()
            .map(str::to_string);
        let attempt_digest = evidence.attempt_digest().map(str::to_string);
        let invariant_evidence = evidence.invariant_evidence().to_vec();
        let snapshot_token = evidence.snapshot_token().map(str::to_string);
        let invariant_evidence_digest_part = invariant_evidence.join("|");
        let inspection_digest = hash_parts(&[
            "forge_query_intent_denial_inspection_v1".to_string(),
            format!("intent:{}", evidence.intent_name()),
            format!("stage:{}", evidence.stage()),
            format!("message:{}", evidence.message()),
            format!("strategy:{}", evidence.strategy_identity()),
            format!("version:{}", evidence.strategy_version()),
            format!(
                "returned-strategy:{}",
                returned_strategy_identity
                    .as_deref()
                    .unwrap_or("not-executed")
            ),
            format!(
                "returned-version:{}",
                returned_strategy_version
                    .as_deref()
                    .unwrap_or("not-executed")
            ),
            format!(
                "returned-descriptor:{}",
                returned_strategy_descriptor_digest
                    .as_deref()
                    .unwrap_or("not-executed")
            ),
            format!("input:{}", evidence.canonical_input_digest()),
            format!("source:{}", evidence.source_lane().as_str()),
            format!("target:{}", evidence.target_lane()),
            format!(
                "execution-kind:{}",
                evidence
                    .execution_kind()
                    .map(ForgeQueryIntentExecutionKind::as_str)
                    .unwrap_or("not-executed")
            ),
            format!("attempt:{}", attempt_digest.as_deref().unwrap_or("none")),
            format!("invariants:{invariant_evidence_digest_part}"),
            format!("snapshot:{}", snapshot_token.as_deref().unwrap_or("none")),
            format!("denial:{}", evidence.denial_digest()),
        ]);
        Self {
            intent_name: evidence.intent_name().to_string(),
            stage: evidence.stage(),
            message: evidence.message().to_string(),
            strategy_identity: evidence.strategy_identity().to_string(),
            strategy_version: evidence.strategy_version().to_string(),
            returned_strategy_identity,
            returned_strategy_version,
            returned_strategy_descriptor_digest,
            canonical_input_digest: evidence.canonical_input_digest().to_string(),
            source_lane: evidence.source_lane(),
            target_lane: evidence.target_lane(),
            execution_kind: evidence.execution_kind(),
            attempt_digest,
            invariant_evidence,
            snapshot_token,
            denial_digest: evidence.denial_digest().to_string(),
            inspection_digest,
        }
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn stage(&self) -> &'static str {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn returned_strategy_identity(&self) -> Option<&str> {
        self.returned_strategy_identity.as_deref()
    }

    pub fn returned_strategy_version(&self) -> Option<&str> {
        self.returned_strategy_version.as_deref()
    }

    pub fn returned_strategy_descriptor_digest(&self) -> Option<&str> {
        self.returned_strategy_descriptor_digest.as_deref()
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn execution_kind(&self) -> Option<ForgeQueryIntentExecutionKind> {
        self.execution_kind
    }

    pub fn attempt_digest(&self) -> Option<&str> {
        self.attempt_digest.as_deref()
    }

    pub fn invariant_evidence(&self) -> &[String] {
        &self.invariant_evidence
    }

    pub fn snapshot_token(&self) -> Option<&str> {
        self.snapshot_token.as_deref()
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBranchIntentReceiptInspection {
    intent_name: String,
    strategy_identity: String,
    strategy_version: String,
    canonical_input_digest: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    basis_digest: String,
    basis_snapshot_token: String,
    admission_digest: String,
    receipt_digest: String,
    inspection_digest: String,
}

impl ForgeQueryBranchIntentReceiptInspection {
    pub(in crate::runtime) fn from_receipt(receipt: &ForgeQueryBranchIntentReceipt) -> Self {
        let basis_evidence = receipt.basis_evidence().to_vec();
        let basis_digest = hash_parts(&[
            "forge_query_branch_intent_receipt_basis_v1".to_string(),
            format!("intent:{}", receipt.intent_name()),
            format!("snapshot:{}", receipt.basis_snapshot_token()),
            format!("basis:{}", basis_evidence.join("|")),
        ]);
        let inspection_digest = hash_parts(&[
            "forge_query_branch_intent_receipt_inspection_v1".to_string(),
            format!("intent:{}", receipt.intent_name()),
            format!("strategy:{}", receipt.strategy_identity()),
            format!("version:{}", receipt.strategy_version()),
            format!("input:{}", receipt.canonical_input_digest()),
            format!("source:{}", receipt.source_lane().as_str()),
            format!("target:{}", receipt.target_lane()),
            format!("policy:{}", receipt.effect_policy().as_str()),
            format!("basis:{basis_digest}"),
            format!("snapshot:{}", receipt.basis_snapshot_token()),
            format!("admission:{}", receipt.admission_digest()),
            format!("receipt:{}", receipt.receipt_digest()),
        ]);
        Self {
            intent_name: receipt.intent_name().to_string(),
            strategy_identity: receipt.strategy_identity().to_string(),
            strategy_version: receipt.strategy_version().to_string(),
            canonical_input_digest: receipt.canonical_input_digest().to_string(),
            source_lane: receipt.source_lane(),
            target_lane: receipt.target_lane(),
            effect_policy: receipt.effect_policy(),
            basis_evidence,
            basis_digest,
            basis_snapshot_token: receipt.basis_snapshot_token().to_string(),
            admission_digest: receipt.admission_digest().to_string(),
            receipt_digest: receipt.receipt_digest().to_string(),
            inspection_digest,
        }
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
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

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn basis_snapshot_token(&self) -> &str {
        &self.basis_snapshot_token
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectIntentReceiptInspection {
    effect_name: String,
    trigger_commit_identity: String,
    trigger_source_kind: super::super::ForgeQueryEffectTriggerSourceKind,
    pending_intent_target: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    phase_digest: String,
    intent_receipt_digest: String,
    receipt_digest: String,
    feedback_graph: ForgeQueryFeedbackPhaseGraphInspection,
    inspection_digest: String,
}

impl ForgeQueryEffectIntentReceiptInspection {
    pub(in crate::runtime) fn from_receipt(receipt: &ForgeQueryEffectIntentReceipt) -> Self {
        let feedback_graph =
            ForgeQueryFeedbackPhaseGraphInspection::from_effect_intent_receipt(receipt);
        let phase_digest = hash_parts(&[
            "forge_query_effect_intent_receipt_phase_v1".to_string(),
            format!(
                "phases:{}",
                receipt
                    .phase_evidence()
                    .phases()
                    .iter()
                    .map(|phase| phase.as_str())
                    .collect::<Vec<_>>()
                    .join(">")
            ),
            format!(
                "loop-prevention:{}",
                receipt.phase_evidence().loop_prevention().as_str()
            ),
            format!(
                "idempotence:{}",
                receipt.phase_evidence().idempotence().as_str()
            ),
        ]);
        let inspection_digest = hash_parts(&[
            "forge_query_effect_intent_receipt_inspection_v1".to_string(),
            format!("effect:{}", receipt.effect_name()),
            format!("trigger-commit:{}", receipt.trigger_commit_identity()),
            format!(
                "trigger-source-kind:{}",
                receipt.trigger_source_kind().as_str()
            ),
            format!("pending-target:{}", receipt.pending_intent_target()),
            format!("source:{}", receipt.source_lane().as_str()),
            format!("target:{}", receipt.target_lane()),
            format!("policy:{}", receipt.effect_policy().as_str()),
            format!("phase:{phase_digest}"),
            format!(
                "intent-receipt:{}",
                receipt.intent_receipt().receipt_digest()
            ),
            format!("receipt:{}", receipt.receipt_digest()),
            format!("feedback-graph:{}", feedback_graph.graph_digest()),
        ]);
        Self {
            effect_name: receipt.effect_name().to_string(),
            trigger_commit_identity: receipt.trigger_commit_identity().to_string(),
            trigger_source_kind: receipt.trigger_source_kind(),
            pending_intent_target: receipt.pending_intent_target().to_string(),
            source_lane: receipt.source_lane(),
            target_lane: receipt.target_lane(),
            effect_policy: receipt.effect_policy(),
            phase_digest,
            intent_receipt_digest: receipt.intent_receipt().receipt_digest().to_string(),
            receipt_digest: receipt.receipt_digest().to_string(),
            feedback_graph,
            inspection_digest,
        }
    }

    pub fn effect_name(&self) -> &str {
        &self.effect_name
    }
    pub fn trigger_commit_identity(&self) -> &str {
        &self.trigger_commit_identity
    }
    pub fn trigger_source_kind(&self) -> super::super::ForgeQueryEffectTriggerSourceKind {
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
    pub fn phase_digest(&self) -> &str {
        &self.phase_digest
    }
    pub fn intent_receipt_digest(&self) -> &str {
        &self.intent_receipt_digest
    }
    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
    pub fn feedback_graph(&self) -> &ForgeQueryFeedbackPhaseGraphInspection {
        &self.feedback_graph
    }
    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
