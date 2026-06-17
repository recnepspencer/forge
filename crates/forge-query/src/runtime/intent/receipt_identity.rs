use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;

use super::super::{ForgeQueryEffectDelivery, ForgeQueryEffectPhase};
use super::{
    ForgeQueryIntentDeclaration, ForgeQueryIntentExecution, ForgeQueryIntentExecutionProvenance,
    ForgeQueryIntentSourceLane, ForgeQueryWriteReceipt,
};

pub(super) fn authoritative_intent_receipt_identity(
    declaration: &ForgeQueryIntentDeclaration,
    execution: &ForgeQueryIntentExecution,
    write_receipt: &ForgeQueryWriteReceipt,
    execution_provenance: &ForgeQueryIntentExecutionProvenance,
    decision_trace_envelope: &ForgeQueryIntentDecisionTraceEnvelope,
) -> ForgeQueryEvidenceIdentity {
    let invariant_evidence_identities = receipt_value_identities(
        "authoritative-receipt-invariant-evidence",
        execution.invariant_evidence(),
    );
    let effect_trigger_identity = declaration.effect_trigger().map(|trigger| {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeIntentReceipt)
            .field_shape(
                ForgeQueryEvidenceTag::new("role"),
                "authoritative-receipt-effect-trigger",
            )
            .field_value(ForgeQueryEvidenceTag::new("digest"), trigger.digest())
            .seal()
    });
    let affected_live_view_identities = receipt_value_identities(
        "authoritative-receipt-affected-live-view",
        write_receipt.affected_live_view_ids(),
    );
    let affected_derived_view_identities = receipt_value_identities(
        "authoritative-receipt-affected-derived-view",
        write_receipt.affected_derived_view_ids(),
    );

    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeIntentReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            declaration.name(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_kind"),
            execution.execution_kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            execution.strategy_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_version"),
            execution.strategy_version(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_descriptor_digest"),
            execution.strategy_descriptor_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            execution.canonical_input_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("outcome_digest"),
            execution.outcome_digest(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("invariant_evidence"),
            invariant_evidence_identities.iter(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            declaration.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            declaration.target_lane().as_str(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("effect_trigger_digest"),
            effect_trigger_identity.as_ref(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit_evidence_identity"),
            write_receipt.commit_evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_evidence_identity"),
            write_receipt.snapshot_evidence_identity(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("affected_live_view_id"),
            affected_live_view_identities.iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("affected_derived_view_id"),
            affected_derived_view_identities.iter(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("considered_computed_view_count"),
            write_receipt.considered_computed_view_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("considered_effect_count"),
            write_receipt.considered_effect_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("delivered_effect_count"),
            write_receipt.delivered_effect_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_write_intent_count"),
            write_receipt.pending_write_intent_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("suppressed_effect_count"),
            write_receipt.suppressed_effect_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("meaningful_effect_suppression_count"),
            write_receipt.meaningful_effect_suppression_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("effect_expression_failure_count"),
            write_receipt.effect_expression_failure_count(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("refresh_fallback"),
            write_receipt.refresh_fallback(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("admission_family"),
            execution_provenance.family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("covered_entrypoint"),
            execution_provenance.entrypoint().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_seam"),
            execution_provenance.execution_seam().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("admission_decision_digest"),
            execution_provenance.admission_decision_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_handoff_digest"),
            execution_provenance.execution_handoff_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_binding_digest"),
            execution_provenance.execution_binding_digest(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("execution_provenance_chain_identity"),
            execution_provenance.execution_provenance_chain_identity(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("decision_trace_digest"),
            decision_trace_envelope.trace_digest(),
        )
        .seal()
}

fn receipt_value_identities(
    role: &'static str,
    values: &[String],
) -> Vec<ForgeQueryEvidenceIdentity> {
    values
        .iter()
        .map(|value| {
            forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeIntentReceipt)
                .field_shape(ForgeQueryEvidenceTag::new("role"), role)
                .field_value(ForgeQueryEvidenceTag::new("value"), value)
                .seal()
        })
        .collect()
}

pub(super) fn effect_intent_receipt_identity(
    delivery: &ForgeQueryEffectDelivery,
    intent_receipt: &super::ForgeQueryIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_name"),
            delivery.effect_name(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trigger_commit_evidence_identity"),
            delivery.trigger_commit_evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("trigger_source_kind"),
            delivery.trigger_source_kind().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("write_adjacent_trigger"),
            delivery.write_adjacent_trigger().identity(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pending_intent_target"),
            delivery.target(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            ForgeQueryIntentSourceLane::EffectTriggered.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            intent_receipt.target_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            delivery.effect_policy().as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("phase"),
            delivery
                .phase_evidence()
                .phases()
                .iter()
                .map(|phase: &ForgeQueryEffectPhase| phase.as_str()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("loop_prevention"),
            delivery.phase_evidence().loop_prevention().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("idempotence"),
            delivery.phase_evidence().idempotence().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("intent_receipt_identity"),
            intent_receipt.receipt_identity(),
        )
        .seal()
}
