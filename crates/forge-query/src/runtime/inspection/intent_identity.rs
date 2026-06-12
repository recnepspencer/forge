use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::super::{
    ForgeQueryBranchIntentReceipt, ForgeQueryEffectIntentReceipt, ForgeQueryIntentReceipt,
};
use super::feedback::ForgeQueryFeedbackPhaseGraphInspection;
use super::intent_delivery_counters::ForgeQueryIntentInspectionDeliveryCounters;

pub(super) fn intent_inspection_delivery_counter_identity(
    counters: IntentInspectionDeliveryCounterIdentityParts,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentInspectionDeliveryCounters)
        .field_usize(
            ForgeQueryEvidenceTag::new("affected_live_view_count"),
            counters.affected_live_view_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("affected_derived_view_count"),
            counters.affected_derived_view_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("considered_computed_view_count"),
            counters.considered_computed_view_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("considered_effect_count"),
            counters.considered_effect_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("delivered_effect_count"),
            counters.delivered_effect_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_write_intent_count"),
            counters.pending_write_intent_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("suppressed_effect_count"),
            counters.suppressed_effect_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("meaningful_effect_suppression_count"),
            counters.meaningful_effect_suppression_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("effect_expression_failure_count"),
            counters.effect_expression_failure_count,
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("refresh_fallback"),
            counters.refresh_fallback,
        )
        .seal()
}

pub(super) fn intent_receipt_inspection_identity(
    receipt: &ForgeQueryIntentReceipt,
    produced_mutation_digest: Option<&str>,
    delivery_counters: &ForgeQueryIntentInspectionDeliveryCounters,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            receipt.intent_name(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_kind"),
            receipt.execution_kind().as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            receipt.strategy_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_version"),
            receipt.strategy_version(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("strategy_descriptor_digest"),
            receipt.strategy_descriptor_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            receipt.canonical_input_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("outcome_digest"),
            receipt.outcome_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("produced_mutation_digest"),
            produced_mutation_digest,
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("invariant_evidence"),
            receipt.invariant_evidence().iter().map(String::as_str),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            receipt.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            receipt.target_lane().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit_evidence_identity"),
            receipt.commit_evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_evidence_identity"),
            receipt.snapshot_evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_identity"),
            receipt.receipt_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("delivery_counter_identity"),
            delivery_counters.counter_identity(),
        )
        .seal()
}

pub(super) fn branch_intent_receipt_inspection_basis_identity(
    receipt: &ForgeQueryBranchIntentReceipt,
    basis_evidence: &[String],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::BranchIntentReceiptInspectionBasis)
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            receipt.intent_name(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_snapshot_identity"),
            &receipt.basis_snapshot_identity().evidence_identity(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("basis_evidence"),
            basis_evidence.iter().map(String::as_str),
        )
        .seal()
}

pub(super) fn branch_intent_receipt_inspection_identity(
    receipt: &ForgeQueryBranchIntentReceipt,
    basis_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::BranchIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            receipt.intent_name(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            receipt.strategy_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_version"),
            receipt.strategy_version(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            receipt.canonical_input_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            receipt.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            receipt.target_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            receipt.effect_policy().as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("basis_identity"), basis_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_snapshot_identity"),
            &receipt.basis_snapshot_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_identity"),
            receipt.admission_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_identity"),
            receipt.receipt_identity(),
        )
        .seal()
}

pub(super) fn effect_intent_receipt_phase_identity(
    receipt: &ForgeQueryEffectIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptPhase)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("phase"),
            receipt
                .phase_evidence()
                .phases()
                .iter()
                .map(|phase| phase.as_str()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("loop_prevention"),
            receipt.phase_evidence().loop_prevention().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("idempotence"),
            receipt.phase_evidence().idempotence().as_str(),
        )
        .seal()
}

pub(super) fn effect_intent_receipt_inspection_identity(
    receipt: &ForgeQueryEffectIntentReceipt,
    phase_identity: &ForgeQueryEvidenceIdentity,
    feedback_graph: &ForgeQueryFeedbackPhaseGraphInspection,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_name"),
            receipt.effect_name(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trigger_commit_evidence_identity"),
            receipt.trigger_commit_evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("trigger_source_kind"),
            receipt.trigger_source_kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pending_intent_target"),
            receipt.pending_intent_target(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            receipt.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            receipt.target_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            receipt.effect_policy().as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("phase_identity"), phase_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("intent_receipt_identity"),
            receipt.intent_receipt().receipt_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_identity"),
            receipt.receipt_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("feedback_graph_identity"),
            feedback_graph.graph_identity(),
        )
        .seal()
}

pub(super) struct IntentInspectionDeliveryCounterIdentityParts {
    pub(super) affected_live_view_count: usize,
    pub(super) affected_derived_view_count: usize,
    pub(super) considered_computed_view_count: usize,
    pub(super) considered_effect_count: usize,
    pub(super) delivered_effect_count: usize,
    pub(super) pending_write_intent_count: usize,
    pub(super) suppressed_effect_count: usize,
    pub(super) meaningful_effect_suppression_count: usize,
    pub(super) effect_expression_failure_count: usize,
    pub(super) refresh_fallback: bool,
}
