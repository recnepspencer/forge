use super::support::*;
use crate::evidence_identity::forge_query_evidence_identity;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

pub(super) fn compose_authoritative_intent_receipt_identity(
    receipt: &ForgeQueryIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    compose_authoritative_intent_receipt_identity_with_effect_trigger(receipt, None)
}

pub(super) fn compose_authoritative_effect_triggered_intent_receipt_identity(
    receipt: &ForgeQueryEffectIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    compose_authoritative_intent_receipt_identity_with_effect_trigger(
        receipt.intent_receipt(),
        Some(receipt.write_adjacent_trigger().digest()),
    )
}

fn compose_authoritative_intent_receipt_identity_with_effect_trigger(
    receipt: &ForgeQueryIntentReceipt,
    effect_trigger_digest: Option<&str>,
) -> ForgeQueryEvidenceIdentity {
    let invariant_evidence_identities = receipt_value_identities(
        "authoritative-receipt-invariant-evidence",
        receipt.invariant_evidence(),
    );
    let effect_trigger_identity = effect_trigger_digest.map(|digest| {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeIntentReceipt)
            .field_shape(
                ForgeQueryEvidenceTag::new("role"),
                "authoritative-receipt-effect-trigger",
            )
            .field_value(ForgeQueryEvidenceTag::new("digest"), digest)
            .seal()
    });
    let affected_live_view_identities = receipt_value_identities(
        "authoritative-receipt-affected-live-view",
        &receipt.terminal_affected_live_view_ids_projection(),
    );
    let affected_derived_view_identities = receipt_value_identities(
        "authoritative-receipt-affected-derived-view",
        &receipt.terminal_affected_derived_view_ids_projection(),
    );

    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeIntentReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            receipt.intent_name(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_kind"),
            receipt.execution_kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            receipt.strategy_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_version"),
            receipt.strategy_version(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_descriptor_digest"),
            receipt.strategy_descriptor_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            receipt.canonical_input_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("outcome_digest"),
            receipt.outcome_digest(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("invariant_evidence"),
            invariant_evidence_identities.iter(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            receipt.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            receipt.target_lane().as_str(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("effect_trigger_digest"),
            effect_trigger_identity.as_ref(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit_evidence_identity"),
            receipt.commit_evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_evidence_identity"),
            receipt.snapshot_evidence_identity(),
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
            receipt.considered_computed_view_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("considered_effect_count"),
            receipt.considered_effect_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("delivered_effect_count"),
            receipt.delivered_effect_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_write_intent_count"),
            receipt.pending_write_intent_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("suppressed_effect_count"),
            receipt.suppressed_effect_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("meaningful_effect_suppression_count"),
            receipt.meaningful_effect_suppression_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("effect_expression_failure_count"),
            receipt.effect_expression_failure_count(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("refresh_fallback"),
            receipt.refresh_fallback(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("admission_family"),
            receipt.admission_family(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("covered_entrypoint"),
            receipt.covered_entrypoint_label(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_seam"),
            receipt.execution_seam_label(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("admission_decision_digest"),
            receipt.admission_decision_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_handoff_digest"),
            receipt.execution_handoff_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_binding_digest"),
            receipt.execution_binding_digest(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("execution_provenance_chain_identity"),
            receipt
                .execution_provenance()
                .execution_provenance_chain_identity(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("decision_trace_digest"),
            receipt.decision_trace_envelope().trace_digest(),
        )
        .seal()
}

pub(super) fn compose_intent_execution_provenance_chain_identity(
    receipt: &ForgeQueryIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    let provenance = receipt.execution_provenance();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::IntentExecutionProvenanceChain)
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            provenance.family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("entrypoint"),
            provenance.entrypoint().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("seam"),
            provenance.execution_seam().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("admission_decision_digest"),
            provenance.admission_decision_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_handoff_digest"),
            provenance.execution_handoff_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_binding_digest"),
            provenance.execution_binding_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_outcome_digest"),
            receipt.outcome_digest(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_token"),
            receipt.snapshot_evidence_identity(),
        )
        .seal()
}

pub(super) fn compose_effect_intent_receipt_identity(
    receipt: &ForgeQueryEffectIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
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
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("write_adjacent_trigger"),
            receipt.write_adjacent_trigger().identity(),
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
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("intent_receipt_identity"),
            receipt.intent_receipt().receipt_identity(),
        )
        .seal()
}

pub(super) fn compose_authoritative_intent_receipt_inspection_identity(
    inspection: &ForgeQueryIntentReceiptInspection,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::IntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            inspection.intent_name(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_kind"),
            inspection.execution_kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            inspection.strategy_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_version"),
            inspection.strategy_version(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_descriptor_digest"),
            inspection.strategy_descriptor_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            inspection.canonical_input_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("outcome_digest"),
            inspection.outcome_digest(),
        )
        .optional_identity(
            ForgeQueryEvidenceTag::new("produced_mutation_digest"),
            inspection.produced_mutation_digest(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("invariant_evidence"),
            inspection.invariant_evidence().iter().map(String::as_str),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            inspection.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            inspection.target_lane().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit_evidence_identity"),
            &inspection.commit_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_evidence_identity"),
            &inspection.snapshot_identity().evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_identity"),
            inspection.receipt_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("delivery_counter_identity"),
            inspection.delivery_counters().counter_identity(),
        )
        .seal()
}

pub(super) fn compose_effect_intent_receipt_inspection_identity(
    inspection: &ForgeQueryEffectIntentReceiptInspection,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_name"),
            inspection.effect_name(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trigger_commit_evidence_identity"),
            inspection.trigger_commit_evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("trigger_source_kind"),
            inspection.trigger_source_kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pending_intent_target"),
            inspection.pending_intent_target(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            inspection.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            inspection.target_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            inspection.effect_policy().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("phase_identity"),
            inspection.phase_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("intent_receipt_identity"),
            inspection.intent_receipt_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_identity"),
            inspection.receipt_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("feedback_graph_identity"),
            inspection.feedback_graph().graph_identity(),
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
