use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::super::ForgeQueryFeedbackPhaseGraphInspection;
use super::declaration::ForgeQueryEffectExpressionFailurePosture;
use super::delivery::{ForgeQueryEffectDelivery, ForgeQueryEffectDeliveryFamily};
use super::phase::ForgeQueryEffectPhaseEvidence;
use super::registry::ForgeQueryEffectRuntime;

pub(super) struct EffectInspectionDigestSet {
    pub(super) trigger_digest: String,
    pub(super) condition_digest: String,
    pub(super) declaration_digest: String,
    pub(super) pending_delivery_digest: String,
    pub(super) latest_phase_digest: Option<String>,
    pub(super) inspection_digest: String,
}

pub(super) fn effect_inspection_digests(
    effect: &ForgeQueryEffectRuntime,
    condition_descriptor: &str,
    condition_inputs: &[String],
    condition_outputs: &[String],
    condition_failure_posture: Option<ForgeQueryEffectExpressionFailurePosture>,
    pending_delivery_count: usize,
    pending_delivered_count: usize,
    pending_suppressed_count: usize,
    pending_expression_failure_count: usize,
    pending_write_intent_count: usize,
    latest_delivery_family: Option<&ForgeQueryEffectDeliveryFamily>,
    latest_phase_evidence: Option<&ForgeQueryEffectPhaseEvidence>,
    feedback_graph: Option<&ForgeQueryFeedbackPhaseGraphInspection>,
) -> EffectInspectionDigestSet {
    let trigger_identity = trigger_inspection_identity(effect);
    let condition_identity = condition_inspection_identity(
        effect,
        condition_descriptor,
        condition_inputs,
        condition_outputs,
        condition_failure_posture,
    );
    let declaration_identity =
        declaration_inspection_identity(effect, &trigger_identity, &condition_identity);
    let pending_delivery_identity = pending_delivery_inspection_identity(
        effect,
        pending_delivery_count,
        pending_delivered_count,
        pending_suppressed_count,
        pending_expression_failure_count,
        pending_write_intent_count,
    );
    let latest_phase_identity = latest_phase_evidence.map(effect_phase_inspection_identity);
    let inspection_identity = inspection_identity(
        latest_delivery_family,
        feedback_graph,
        &declaration_identity,
        &pending_delivery_identity,
        latest_phase_identity.as_ref(),
    );

    EffectInspectionDigestSet {
        trigger_digest: trigger_identity.as_str().to_string(),
        condition_digest: condition_identity.as_str().to_string(),
        declaration_digest: declaration_identity.as_str().to_string(),
        pending_delivery_digest: pending_delivery_identity.as_str().to_string(),
        latest_phase_digest: latest_phase_identity
            .as_ref()
            .map(|identity| identity.as_str().to_string()),
        inspection_digest: inspection_identity.as_str().to_string(),
    }
}

fn trigger_inspection_identity(effect: &ForgeQueryEffectRuntime) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("artifact_kind"),
            "effect-trigger-inspection",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("name"),
            effect.declaration.name(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_kind"),
            effect.declaration.trigger().source_kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("source"),
            effect.declaration.trigger().source_name(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("write_adjacent_trigger"),
            effect.declaration.write_adjacent_trigger().identity(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspects"),
            effect
                .declaration
                .trigger()
                .aspects()
                .iter()
                .map(String::as_str),
        )
        .seal()
}

fn condition_inspection_identity(
    effect: &ForgeQueryEffectRuntime,
    condition_descriptor: &str,
    condition_inputs: &[String],
    condition_outputs: &[String],
    condition_failure_posture: Option<ForgeQueryEffectExpressionFailurePosture>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("artifact_kind"),
            "effect-condition-inspection",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("name"),
            effect.declaration.name(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("descriptor"),
            condition_descriptor,
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("inputs"),
            condition_inputs.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("outputs"),
            condition_outputs.iter().map(String::as_str),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("failure_posture"),
            effect_condition_failure_posture_label(condition_failure_posture),
        )
        .seal()
}

fn declaration_inspection_identity(
    effect: &ForgeQueryEffectRuntime,
    trigger_identity: &ForgeQueryEvidenceIdentity,
    condition_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("artifact_kind"),
            "effect-declaration-inspection",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("name"),
            effect.declaration.name(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("trigger"), trigger_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("condition"), condition_identity)
        .field_shape(
            ForgeQueryEvidenceTag::new("action"),
            effect.declaration.action().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            effect.declaration.target_lane().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("target"),
            effect.declaration.target(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("policy"),
            effect.declaration.effect_policy().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("suppression"),
            effect.declaration.suppression_policy().as_str(),
        )
        .seal()
}

fn pending_delivery_row_identity(
    delivery: &ForgeQueryEffectDelivery,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptPhase)
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_name"),
            delivery.effect_name(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trigger_commit"),
            delivery.trigger_commit_evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("trigger_source_kind"),
            delivery.trigger_source_kind().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            effect_delivery_family_label(delivery.family()),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_path"),
            delivery.aspect_paths().iter().map(String::as_str),
        )
        .seal()
}

fn pending_delivery_inspection_identity(
    effect: &ForgeQueryEffectRuntime,
    pending_delivery_count: usize,
    pending_delivered_count: usize,
    pending_suppressed_count: usize,
    pending_expression_failure_count: usize,
    pending_write_intent_count: usize,
) -> ForgeQueryEvidenceIdentity {
    let delivery_identities = effect
        .deliveries
        .iter()
        .map(pending_delivery_row_identity)
        .collect::<Vec<_>>();

    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptPhase)
        .field_shape(
            ForgeQueryEvidenceTag::new("artifact_kind"),
            "pending-delivery",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("name"),
            effect.declaration.name(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_delivery_count"),
            pending_delivery_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_delivered_count"),
            pending_delivered_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_suppressed_count"),
            pending_suppressed_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_expression_failure_count"),
            pending_expression_failure_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_write_intent_count"),
            pending_write_intent_count,
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("deliveries"),
            delivery_identities.iter(),
        )
        .seal()
}

fn inspection_identity(
    latest_delivery_family: Option<&ForgeQueryEffectDeliveryFamily>,
    feedback_graph: Option<&ForgeQueryFeedbackPhaseGraphInspection>,
    declaration_identity: &ForgeQueryEvidenceIdentity,
    pending_delivery_identity: &ForgeQueryEvidenceIdentity,
    latest_phase_identity: Option<&ForgeQueryEvidenceIdentity>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("artifact_kind"),
            "effect-inspection",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("declaration"),
            declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("pending_delivery"),
            pending_delivery_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("latest_family"),
            latest_delivery_family
                .map(effect_delivery_family_label)
                .unwrap_or("none"),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("latest_phase"),
            latest_phase_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("feedback_graph"),
            feedback_graph.map(|graph| graph.graph_identity()),
        )
        .seal()
}

fn effect_condition_failure_posture_label(
    posture: Option<ForgeQueryEffectExpressionFailurePosture>,
) -> &'static str {
    match posture {
        Some(ForgeQueryEffectExpressionFailurePosture::Admitted) => "admitted",
        Some(ForgeQueryEffectExpressionFailurePosture::DeterministicFailure) => {
            "deterministic-failure"
        }
        None => "none",
    }
}

fn effect_delivery_family_label(family: &ForgeQueryEffectDeliveryFamily) -> &'static str {
    match family {
        ForgeQueryEffectDeliveryFamily::Delivered => "delivered",
        ForgeQueryEffectDeliveryFamily::PendingWriteIntent => "pending-write-intent",
        ForgeQueryEffectDeliveryFamily::Suppressed => "suppressed",
        ForgeQueryEffectDeliveryFamily::ExpressionFailed => "expression-failed",
    }
}

fn effect_phase_inspection_identity(
    phase: &ForgeQueryEffectPhaseEvidence,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("artifact_kind"),
            "effect-phase-inspection",
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("phases"),
            phase.phases().iter().map(|entry| entry.as_str()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("loop_prevention"),
            phase.loop_prevention().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("idempotence"),
            phase.idempotence().as_str(),
        )
        .seal()
}
