use super::*;
use crate::intent_admission::{
    ForgeQueryAdmittedIntentExecutionHandoff, ForgeQueryAuthoritativeIntentExecutionBinding,
    ForgeQueryEffectTriggeredIntentExecutionBinding, ForgeQueryIntentDecisionTraceEnvelope,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentReceipt {
    intent_name: String,
    execution_kind: ForgeQueryIntentExecutionKind,
    strategy_identity: String,
    strategy_version: String,
    strategy_descriptor_digest: String,
    canonical_input_digest: String,
    outcome_digest: String,
    invariant_evidence: Vec<String>,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    commit_identity: String,
    snapshot_token: String,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    considered_computed_view_count: usize,
    considered_effect_count: usize,
    delivered_effect_count: usize,
    pending_write_intent_count: usize,
    suppressed_effect_count: usize,
    meaningful_effect_suppression_count: usize,
    effect_expression_failure_count: usize,
    refresh_fallback: bool,
    execution_provenance: ForgeQueryIntentExecutionProvenance,
    decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
    receipt_digest: String,
}

impl ForgeQueryIntentReceipt {
    pub(in crate::runtime) fn from_authoritative_binding(
        binding: &ForgeQueryAuthoritativeIntentExecutionBinding,
        declaration: &ForgeQueryIntentDeclaration,
        execution: ForgeQueryIntentExecution,
        write_receipt: &ForgeQueryWriteReceipt,
    ) -> Self {
        let execution_provenance = ForgeQueryIntentExecutionProvenance::for_authoritative_binding(
            binding,
            execution.outcome_digest(),
            write_receipt.snapshot_token(),
        );
        let handoff = ForgeQueryAdmittedIntentExecutionHandoff::from(binding.handoff().clone());
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution(&handoff, &execution);
        Self::new(
            declaration,
            execution,
            write_receipt,
            execution_provenance,
            decision_trace_envelope,
        )
    }

    pub(in crate::runtime) fn from_effect_binding(
        binding: &ForgeQueryEffectTriggeredIntentExecutionBinding,
        declaration: &ForgeQueryIntentDeclaration,
        execution: ForgeQueryIntentExecution,
        write_receipt: &ForgeQueryWriteReceipt,
    ) -> Self {
        let execution_provenance = ForgeQueryIntentExecutionProvenance::for_effect_binding(
            binding,
            execution.outcome_digest(),
            write_receipt.snapshot_token(),
        );
        let handoff = ForgeQueryAdmittedIntentExecutionHandoff::from(binding.handoff().clone());
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution(&handoff, &execution);
        Self::new(
            declaration,
            execution,
            write_receipt,
            execution_provenance,
            decision_trace_envelope,
        )
    }

    fn new(
        declaration: &ForgeQueryIntentDeclaration,
        execution: ForgeQueryIntentExecution,
        write_receipt: &ForgeQueryWriteReceipt,
        execution_provenance: ForgeQueryIntentExecutionProvenance,
        decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
    ) -> Self {
        let affected_live_view_ids = write_receipt.affected_live_view_ids().to_vec();
        let affected_derived_view_ids = write_receipt.affected_derived_view_ids().to_vec();
        let commit_identity = write_receipt.commit_identity().to_string();
        let snapshot_token = write_receipt.snapshot_token().to_string();
        let considered_computed_view_count = write_receipt.considered_computed_view_count();
        let considered_effect_count = write_receipt.considered_effect_count();
        let delivered_effect_count = write_receipt.delivered_effect_count();
        let pending_write_intent_count = write_receipt.pending_write_intent_count();
        let suppressed_effect_count = write_receipt.suppressed_effect_count();
        let meaningful_effect_suppression_count =
            write_receipt.meaningful_effect_suppression_count();
        let effect_expression_failure_count = write_receipt.effect_expression_failure_count();
        let refresh_fallback = write_receipt.refresh_fallback();
        let invariant_evidence_digest_part = execution.invariant_evidence().join("|");
        let receipt_digest = hash_parts(&[
            "forge_query_intent_receipt_v1".to_string(),
            format!("intent:{}", declaration.name()),
            format!("execution-kind:{}", execution.execution_kind().as_str()),
            format!("strategy:{}", execution.strategy_identity()),
            format!("version:{}", execution.strategy_version()),
            format!("descriptor:{}", execution.strategy_descriptor_digest()),
            format!("input:{}", execution.canonical_input_digest()),
            format!("outcome:{}", execution.outcome_digest()),
            format!("invariants:{invariant_evidence_digest_part}"),
            format!("source:{}", declaration.source_lane().as_str()),
            format!("target:{}", declaration.target_lane()),
            format!("commit:{commit_identity}"),
            format!("snapshot:{snapshot_token}"),
            format!("live:{}", affected_live_view_ids.join("|")),
            format!("derived:{}", affected_derived_view_ids.join("|")),
            format!("computed-considered:{considered_computed_view_count}"),
            format!("effects-considered:{considered_effect_count}"),
            format!("effects-delivered:{delivered_effect_count}"),
            format!("pending-write-intents:{pending_write_intent_count}"),
            format!("effects-suppressed:{suppressed_effect_count}"),
            format!("meaningful-suppressions:{meaningful_effect_suppression_count}"),
            format!("effect-expression-failures:{effect_expression_failure_count}"),
            format!("refresh-fallback:{refresh_fallback}"),
            format!(
                "admission-family:{}",
                execution_provenance.family().as_str()
            ),
            format!(
                "covered-entrypoint:{}",
                execution_provenance.entrypoint().as_str()
            ),
            format!(
                "execution-seam:{}",
                execution_provenance.execution_seam().as_str()
            ),
            format!(
                "admission-decision:{}",
                execution_provenance.admission_decision_digest()
            ),
            format!(
                "execution-handoff:{}",
                execution_provenance.execution_handoff_digest()
            ),
            format!(
                "execution-binding:{}",
                execution_provenance.execution_binding_digest()
            ),
            format!(
                "execution-provenance:{}",
                execution_provenance.execution_provenance_chain_digest()
            ),
            format!("decision-trace:{}", decision_trace_envelope.trace_digest()),
        ]);
        Self {
            intent_name: declaration.name().to_string(),
            execution_kind: execution.execution_kind,
            strategy_identity: execution.strategy_identity,
            strategy_version: execution.strategy_version,
            strategy_descriptor_digest: execution.strategy_descriptor_digest,
            canonical_input_digest: execution.canonical_input_digest,
            outcome_digest: execution.outcome_digest,
            invariant_evidence: execution.invariant_evidence,
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
            commit_identity,
            snapshot_token,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
            execution_provenance,
            decision_trace_envelope,
            receipt_digest,
        }
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn execution_kind(&self) -> ForgeQueryIntentExecutionKind {
        self.execution_kind
    }

    pub fn is_idempotent_noop(&self) -> bool {
        self.execution_kind == ForgeQueryIntentExecutionKind::IdempotentNoop
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
        (self.execution_kind == ForgeQueryIntentExecutionKind::Mutating)
            .then_some(self.outcome_digest.as_str())
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

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
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

    pub fn admission_family(&self) -> &str {
        self.execution_provenance.family().as_str()
    }

    pub fn covered_entrypoint_label(&self) -> &str {
        self.execution_provenance.entrypoint().as_str()
    }

    pub fn execution_seam_label(&self) -> &str {
        self.execution_provenance.execution_seam().as_str()
    }

    pub fn admission_decision_digest(&self) -> &str {
        self.execution_provenance.admission_decision_digest()
    }

    pub fn execution_handoff_digest(&self) -> &str {
        self.execution_provenance.execution_handoff_digest()
    }

    pub fn execution_binding_digest(&self) -> &str {
        self.execution_provenance.execution_binding_digest()
    }

    pub fn execution_provenance_chain_digest(&self) -> &str {
        self.execution_provenance
            .execution_provenance_chain_digest()
    }

    pub fn execution_provenance(&self) -> &ForgeQueryIntentExecutionProvenance {
        &self.execution_provenance
    }

    pub fn decision_trace_envelope(&self) -> &ForgeQueryIntentDecisionTraceEnvelope {
        &self.decision_trace_envelope
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
