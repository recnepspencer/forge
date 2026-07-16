use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::attachment_budget::DeliveryBackpressurePolicy;
use super::basis_request::QuerySubscriptionBasisBindingRequestKind;
use super::bridge_family::BridgeSubscriptionDeclarationFamilyKind;
use super::bridge_slice::BridgeSubscriptionSliceKind;
use super::delivery::QuerySubscriptionDeliveryIntent;
use super::delivery_cause::QuerySubscriptionDeliveryCauseKind;
use super::equivalence::QuerySubscriptionEquivalenceBasis;
use super::family::QuerySubscriptionFamily;
use super::future_selection::QuerySubscriptionFutureSelection;
use super::input::LiveQueryAdmissionArtifact;
use super::maintenance_delta::QuerySubscriptionMaintenanceDeltaKind;
use super::patch_group::QueryPatchGroupKind;
use super::posture::{
    QuerySubscriptionBasisPosture, QuerySubscriptionBridgePosture, QuerySubscriptionCostPosture,
};
use super::selection_live_graph_access::QuerySubscriptionLiveGraphAccessPosture;
use super::signal_strategy::QuerySubscriptionSignalStrategyRequestKind;
use super::slice::{QuerySubscriptionSliceIntent, QuerySubscriptionSlicePart};
use super::slice_budget::QuerySubscriptionSliceBudget;

const SUBSCRIPTION_IDENTITY_SCOPE: WorthQueryEvidenceScope =
    WorthQueryEvidenceScope::SubscriptionActivationReceipt;
pub(super) fn live_relevance_identity(
    live_family: &LiveQueryFamily,
    query_identity: &WorthQueryEvidenceIdentity,
    plan_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "live_relevance_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("live_family"),
            live_family.as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("query"), query_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("plan"), plan_identity)
        .seal()
}

pub(super) fn slice_intent_identity(
    parts: &[QuerySubscriptionSlicePart],
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_slice_intent_v1",
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("parts"),
            parts.iter().map(QuerySubscriptionSlicePart::canonical_part),
        )
        .seal()
}

pub(super) fn patch_group_identity(
    kind: QueryPatchGroupKind,
    source_identity: &WorthQueryEvidenceIdentity,
    width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_patch_group_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
        .field_usize(WorthQueryEvidenceTag::new("width"), width as usize)
        .seal()
}

pub(super) fn delivery_cause_evidence_label_identity(label: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_delivery_cause_evidence_label_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("label"), label)
        .seal()
}

pub(super) fn delivery_cause_identity(
    kind: QuerySubscriptionDeliveryCauseKind,
    evidence_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_delivery_cause_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(WorthQueryEvidenceTag::new("evidence"), evidence_identity)
        .seal()
}

pub(super) fn diagnostic_evidence_identity(
    stage: &str,
    outcome: &str,
    reason: &str,
    source_identity: &WorthQueryEvidenceIdentity,
    counter_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_evidence_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("stage"), stage)
        .field_shape(WorthQueryEvidenceTag::new("outcome"), outcome)
        .field_shape(WorthQueryEvidenceTag::new("reason"), reason)
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counter_identity)
        .seal()
}

pub(super) fn diagnostic_stage_trace_identity(
    stage: &str,
    outcome: &str,
    source_identity: &WorthQueryEvidenceIdentity,
    evidence_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_stage_trace_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("stage"), stage)
        .field_shape(WorthQueryEvidenceTag::new("outcome"), outcome)
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("evidence"), evidence_identity)
        .seal()
}

pub(super) fn diagnostic_trace_identity<'a>(
    terminal_stage: &str,
    counters_identity: &WorthQueryEvidenceIdentity,
    stage_traces: impl IntoIterator<Item = &'a WorthQueryEvidenceIdentity>,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_trace_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("terminal_stage"), terminal_stage)
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("stages"), stage_traces)
        .seal()
}

pub(super) fn diagnostic_selection_context_selected_identity(
    equivalence_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_selection_context_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), "selected")
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("equivalence"),
            equivalence_identity,
        )
        .seal()
}

pub(super) fn diagnostic_selection_context_denied_identity(
    source_identity: &WorthQueryEvidenceIdentity,
    query_family_label: &str,
    declaration_family_label: &str,
    basis_posture_label: &str,
    live_graph_access_posture_label: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_selection_context_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), "selection_denied")
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
        .field_shape(
            WorthQueryEvidenceTag::new("query_family"),
            query_family_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("declaration_family"),
            declaration_family_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("basis_posture"),
            basis_posture_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("live_graph_access_posture"),
            live_graph_access_posture_label,
        )
        .seal()
}

pub(super) fn preview_residue_report_identity(
    authoritative_routing_width: u64,
    authoritative_checkpoint_width: u64,
    authoritative_replay_width: u64,
    authoritative_diagnostics_width: u64,
    authoritative_writeback_width: u64,
    temporary_execution_width: u64,
    temporary_diagnostics_width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_subscription_residue_report_v1",
        )
        .field_usize(
            WorthQueryEvidenceTag::new("authoritative_routing"),
            authoritative_routing_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("authoritative_checkpoint"),
            authoritative_checkpoint_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("authoritative_replay"),
            authoritative_replay_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("authoritative_diagnostics"),
            authoritative_diagnostics_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("authoritative_writeback"),
            authoritative_writeback_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("temporary_execution"),
            temporary_execution_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("temporary_diagnostics"),
            temporary_diagnostics_width as usize,
        )
        .seal()
}

pub(super) fn subscription_family_capability_identity(
    family: &QuerySubscriptionFamily,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_family_capability_digest_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .seal()
}

pub(super) fn support_subject_identity(
    support_class: &str,
    family: &QuerySubscriptionFamily,
    future_selection_identity: &WorthQueryEvidenceIdentity,
    declaration_identity: &WorthQueryEvidenceIdentity,
    admission_identity: Option<&WorthQueryEvidenceIdentity>,
    source_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    let mut composer = WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_support_subject_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("support_class"), support_class)
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("declaration"),
            declaration_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity);
    if let Some(admission_identity) = admission_identity {
        composer = composer
            .field_evidence_identity(WorthQueryEvidenceTag::new("admission"), admission_identity);
    }
    composer.seal()
}

pub(super) fn support_matrix_row_identity(
    family: &QuerySubscriptionFamily,
    support_class: &str,
    posture: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_support_matrix_row_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_shape(WorthQueryEvidenceTag::new("support_class"), support_class)
        .field_shape(WorthQueryEvidenceTag::new("posture"), posture)
        .seal()
}

pub(super) fn support_matrix_identity<'a>(
    family: &QuerySubscriptionFamily,
    capability_identity: &WorthQueryEvidenceIdentity,
    row_identities: impl IntoIterator<Item = &'a WorthQueryEvidenceIdentity>,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_support_matrix_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("capability"),
            capability_identity,
        )
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("rows"), row_identities)
        .seal()
}

pub(super) fn support_lookup_receipt_identity(
    family: &QuerySubscriptionFamily,
    support_class: &str,
    resolution_posture: &str,
    consumed_lookup_width: usize,
    remaining_lookup_width: usize,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_support_lookup_receipt_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_shape(WorthQueryEvidenceTag::new("support_class"), support_class)
        .field_shape(
            WorthQueryEvidenceTag::new("resolution_posture"),
            resolution_posture,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("consumed_lookup_width"),
            consumed_lookup_width,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("remaining_lookup_width"),
            remaining_lookup_width,
        )
        .seal()
}

pub(super) fn support_counters_identity(
    support_report_request_count: u64,
    supported_family_count: u64,
    denied_family_count: u64,
    deferred_family_count: u64,
    uncertified_family_denial_count: u64,
    support_matrix_emission_count: u64,
    support_family_index_lookup_count: u64,
    support_matrix_scan_debt_count: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_support_counters_v1",
        )
        .field_usize(
            WorthQueryEvidenceTag::new("support_report_request"),
            support_report_request_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("supported_family"),
            supported_family_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("denied_family"),
            denied_family_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("deferred_family"),
            deferred_family_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("uncertified_family_denial"),
            uncertified_family_denial_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("support_matrix_emission"),
            support_matrix_emission_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("support_family_index_lookup"),
            support_family_index_lookup_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("support_matrix_scan_debt"),
            support_matrix_scan_debt_count as usize,
        )
        .seal()
}

pub(super) fn support_report_identity(
    subject_identity: &WorthQueryEvidenceIdentity,
    posture: &str,
    matrix_identity: &WorthQueryEvidenceIdentity,
    lookup_receipt_identity: &WorthQueryEvidenceIdentity,
    counters_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_support_report_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("subject"), subject_identity)
        .field_shape(WorthQueryEvidenceTag::new("posture"), posture)
        .field_evidence_identity(WorthQueryEvidenceTag::new("matrix"), matrix_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("lookup_receipt"),
            lookup_receipt_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn diagnostic_counters_identity(
    diagnostic_trace_emission_count: u64,
    diagnostic_bundle_emission_count: u64,
    denied_bundle_emission_count: u64,
    diagnostic_missing_stage_denial_count: u64,
    diagnostic_bundle_width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_counters_v1",
        )
        .field_usize(
            WorthQueryEvidenceTag::new("diagnostic_trace_emission"),
            diagnostic_trace_emission_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("diagnostic_bundle_emission"),
            diagnostic_bundle_emission_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("diagnostic_denied_bundle_emission"),
            denied_bundle_emission_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("diagnostic_missing_stage_denial"),
            diagnostic_missing_stage_denial_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("diagnostic_bundle_width"),
            diagnostic_bundle_width as usize,
        )
        .seal()
}

pub(super) fn diagnostic_bundle_width_identity(
    stage_evidence_count: usize,
    failure_evidence_count: usize,
    hostile_row_reference_count: usize,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_bundle_width_v1",
        )
        .field_usize(
            WorthQueryEvidenceTag::new("stage_evidence_count"),
            stage_evidence_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("failure_evidence_count"),
            failure_evidence_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("hostile_row_reference_count"),
            hostile_row_reference_count,
        )
        .seal()
}

pub(super) fn diagnostic_assembly_receipt_identity(
    bundle_assembly_posture: &str,
    stage_evidence_composition_count: usize,
    semantic_label_carry_forward_count: usize,
    stage_rederivation_count: usize,
    bundle_width_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_assembly_receipt_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("bundle_assembly_posture"),
            bundle_assembly_posture,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("stage_evidence_composition_count"),
            stage_evidence_composition_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("semantic_label_carry_forward_count"),
            semantic_label_carry_forward_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("stage_rederivation_count"),
            stage_rederivation_count,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("bundle_width"),
            bundle_width_identity,
        )
        .seal()
}

pub(super) fn diagnostic_semantic_labels_identity(
    query_family_label: &str,
    declaration_family_label: &str,
    bridge_family_label: &str,
    bridge_slice_labels: &[String],
    basis_posture_label: &str,
    signal_strategy_class_label: &str,
    live_graph_access_posture_label: &str,
    support_posture_label: &str,
    denial_or_coverage_class_label: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_semantic_labels_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("query_family"),
            query_family_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("declaration_family"),
            declaration_family_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("bridge_family"),
            bridge_family_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("basis_posture"),
            basis_posture_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("signal_strategy_class"),
            signal_strategy_class_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("live_graph_access_posture"),
            live_graph_access_posture_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("support_posture"),
            support_posture_label,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("denial_or_coverage_class"),
            denial_or_coverage_class_label,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("bridge_slices"),
            bridge_slice_labels.iter().map(String::as_str),
        )
        .seal()
}

pub(super) fn diagnostic_failure_identity(
    failure_kind: &str,
    diagnostic_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_failure_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("failure_kind"), failure_kind)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("diagnostic"),
            diagnostic_identity,
        )
        .seal()
}

pub(super) fn diagnostic_admitted_bundle_identity(
    trace_identity: &WorthQueryEvidenceIdentity,
    labels_identity: &WorthQueryEvidenceIdentity,
    support_report_identity: &WorthQueryEvidenceIdentity,
    lifecycle_identity: &WorthQueryEvidenceIdentity,
    receipt_identity: &WorthQueryEvidenceIdentity,
    counters_identity: &WorthQueryEvidenceIdentity,
    admission_identity: &WorthQueryEvidenceIdentity,
    continuation_identity: Option<&WorthQueryEvidenceIdentity>,
    preview_identity: Option<&WorthQueryEvidenceIdentity>,
    closeout_identity: Option<&WorthQueryEvidenceIdentity>,
) -> WorthQueryEvidenceIdentity {
    let mut composer = WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_admitted_diagnostic_bundle_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("trace"), trace_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("labels"), labels_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("support"),
            support_report_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("lifecycle_certification"),
            lifecycle_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("receipt"), receipt_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("admission"), admission_identity);
    if let Some(continuation_identity) = continuation_identity {
        composer = composer.field_evidence_identity(
            WorthQueryEvidenceTag::new("continuation"),
            continuation_identity,
        );
    }
    if let Some(preview_identity) = preview_identity {
        composer = composer
            .field_evidence_identity(WorthQueryEvidenceTag::new("preview"), preview_identity);
    }
    if let Some(closeout_identity) = closeout_identity {
        composer = composer
            .field_evidence_identity(WorthQueryEvidenceTag::new("closeout"), closeout_identity);
    }
    composer.seal()
}

pub(super) fn diagnostic_denied_bundle_identity(
    trace_identity: &WorthQueryEvidenceIdentity,
    labels_identity: &WorthQueryEvidenceIdentity,
    failure_identity: &WorthQueryEvidenceIdentity,
    receipt_identity: &WorthQueryEvidenceIdentity,
    counters_identity: &WorthQueryEvidenceIdentity,
    support_report_identity: Option<&WorthQueryEvidenceIdentity>,
) -> WorthQueryEvidenceIdentity {
    let mut composer = WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_denied_diagnostic_bundle_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("trace"), trace_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("labels"), labels_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("failure"), failure_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("receipt"), receipt_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity);
    if let Some(support_report_identity) = support_report_identity {
        composer = composer.field_evidence_identity(
            WorthQueryEvidenceTag::new("support"),
            support_report_identity,
        );
    }
    composer.seal()
}

pub(super) fn live_delivery_intent_projection_identity(
    live_family: &LiveQueryFamily,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "live_delivery_intent_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("live_family"),
            live_family.as_str(),
        )
        .seal()
}

pub(super) fn subscription_fanout_plan_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    affected_consumer_attachment_width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_fanout_plan_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_usize(
            WorthQueryEvidenceTag::new("affected_consumer_attachment_width"),
            affected_consumer_attachment_width as usize,
        )
        .seal()
}

pub(super) fn subscription_fanout_report_identity(
    plan_identity: &WorthQueryEvidenceIdentity,
    shared_lane_count: u64,
    fanout_width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_fanout_report_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("plan"), plan_identity)
        .field_usize(
            WorthQueryEvidenceTag::new("shared_lane_count"),
            shared_lane_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("fanout_width"),
            fanout_width as usize,
        )
        .seal()
}

pub(super) fn lifecycle_acknowledgement_frontier_identity(
    attachment_identity: &WorthQueryEvidenceIdentity,
    sequence: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_acknowledgement_frontier_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_usize(WorthQueryEvidenceTag::new("sequence"), sequence as usize)
        .seal()
}

pub(super) fn subscription_performance_receipt_source_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    consumer_identity: &WorthQueryEvidenceIdentity,
    backpressure_policy: &DeliveryBackpressurePolicy,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_performance_receipt_source_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("consumer"), consumer_identity)
        .field_shape(
            WorthQueryEvidenceTag::new("backpressure"),
            backpressure_policy.as_str(),
        )
        .seal()
}

#[cfg(test)]
pub(super) fn lifecycle_continuation_endpoint_identity(
    role: &str,
    source_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_continuation_endpoint_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
        .seal()
}

#[cfg(test)]
pub(super) fn lifecycle_continuation_ordinary_checkpoint_identity(
    active_lane_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_continuation_ordinary_checkpoint_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("active_lane"),
            active_lane_identity,
        )
        .seal()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn manual_bridge_witness_identity(
    query_family: &str,
    bridge_family: &str,
    basis_posture: &str,
    signal_strategy_class: &str,
    query_declaration_identity: &WorthQueryEvidenceIdentity,
    bridge_declaration_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    signal_strategy_identity: &WorthQueryEvidenceIdentity,
    activation_identity: &WorthQueryEvidenceIdentity,
    assembly_posture: &str,
    bridge_slices: &[BridgeSubscriptionSliceKind],
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_manual_bridge_witness_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("query_family"), query_family)
        .field_shape(WorthQueryEvidenceTag::new("bridge_family"), bridge_family)
        .field_shape(WorthQueryEvidenceTag::new("basis_posture"), basis_posture)
        .field_shape(
            WorthQueryEvidenceTag::new("signal_strategy_class"),
            signal_strategy_class,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("basis_binding"),
            basis_binding_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("activation"),
            activation_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("assembly_posture"),
            assembly_posture,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("bridge_slices"),
            bridge_slices
                .iter()
                .map(BridgeSubscriptionSliceKind::as_str),
        )
        .seal()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn query_subscription_declaration_identity(
    family: &QuerySubscriptionFamily,
    live_family: &LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    cost_posture: &QuerySubscriptionCostPosture,
    basis_posture: &QuerySubscriptionBasisPosture,
    bridge_posture: &QuerySubscriptionBridgePosture,
    live_graph_access_posture: &QuerySubscriptionLiveGraphAccessPosture,
    future_selection: &QuerySubscriptionFutureSelection,
    equivalence_identity: &WorthQueryEvidenceIdentity,
    slice_intent: &QuerySubscriptionSliceIntent,
    delivery_intent: &QuerySubscriptionDeliveryIntent,
    max_admitted_slice_count: usize,
    slice_budget: &QuerySubscriptionSliceBudget,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_declaration_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("live_family"),
            live_family.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("view_family"),
            view_family
                .as_ref()
                .map(LiveViewShapeFamily::as_str)
                .unwrap_or("none"),
        )
        .field_shape(WorthQueryEvidenceTag::new("cost"), cost_posture.as_str())
        .field_shape(WorthQueryEvidenceTag::new("basis"), basis_posture.as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("bridge"),
            bridge_posture.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("live_graph_access_posture"),
            live_graph_access_posture.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection.projection_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("equivalence"),
            equivalence_identity,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("slice_intent"),
            slice_intent
                .parts()
                .iter()
                .map(QuerySubscriptionSlicePart::canonical_part),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("delivery_intent"),
            delivery_intent.as_str(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("work_budget_max_slices"),
            max_admitted_slice_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("slice_budget_projection"),
            slice_budget.projected_slice_width_limit(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("slice_budget_ordering"),
            slice_budget.ordering_slice_width_limit(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("slice_budget_grouping"),
            slice_budget.grouping_slice_width_limit(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("slice_budget_relation"),
            slice_budget.relation_scope_slice_width_limit(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("slice_budget_metadata"),
            slice_budget.metadata_slice_width_limit(),
        )
        .seal()
}

pub(super) fn basis_binding_request_identity(
    request_kind: &QuerySubscriptionBasisBindingRequestKind,
    source_declaration_identity: &WorthQueryEvidenceIdentity,
    source_equivalence_identity: &WorthQueryEvidenceIdentity,
    scoped_declaration_basis_digest: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_basis_binding_request_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("request_kind"),
            request_kind.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("source_declaration"),
            source_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("source_equivalence"),
            source_equivalence_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("scoped_declaration_basis"),
            scoped_declaration_basis_digest,
        )
        .seal()
}

pub(super) fn signal_strategy_request_identity(
    request_kind: &QuerySubscriptionSignalStrategyRequestKind,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_signal_strategy_request_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("request_kind"),
            request_kind.as_str(),
        )
        .seal()
}

pub(super) fn bridge_lowering_plan_identity(
    query_declaration_identity: &WorthQueryEvidenceIdentity,
    bridge_family: &BridgeSubscriptionDeclarationFamilyKind,
    basis_request_identity: &WorthQueryEvidenceIdentity,
    signal_strategy_identity: &WorthQueryEvidenceIdentity,
    bridge_slices: &[BridgeSubscriptionSliceKind],
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_bridge_lowering_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("bridge_family"),
            bridge_family.as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_request_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("bridge_slices"),
            bridge_slices
                .iter()
                .map(BridgeSubscriptionSliceKind::as_str),
        )
        .seal()
}

pub(super) fn admission_artifact_identity(
    query_declaration_identity: &WorthQueryEvidenceIdentity,
    bridge_declaration_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    signal_strategy_identity: &WorthQueryEvidenceIdentity,
    diagnostics_identity: &WorthQueryEvidenceIdentity,
    support_identity: &WorthQueryEvidenceIdentity,
    declaration_width_limit: usize,
    bridge_width_limit: usize,
    basis_width_limit: usize,
    signal_width_limit: usize,
    activation_width_limit: usize,
    counters_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_admission_artifact_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("diagnostics"),
            diagnostics_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("support"), support_identity)
        .field_usize(
            WorthQueryEvidenceTag::new("budget_declaration"),
            declaration_width_limit,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("budget_bridge"),
            bridge_width_limit,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("budget_basis"),
            basis_width_limit,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("budget_signal"),
            signal_width_limit,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("budget_activation"),
            activation_width_limit,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn activation_checkpoint_identity(
    query_declaration_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    future_selection_projection_identity: &WorthQueryEvidenceIdentity,
    signal_strategy_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_active_checkpoint_identity_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_projection_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .seal()
}

pub(super) fn activation_input_identity(
    admission_identity: &WorthQueryEvidenceIdentity,
    query_declaration_identity: &WorthQueryEvidenceIdentity,
    bridge_declaration_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    checkpoint_identity: &WorthQueryEvidenceIdentity,
    signal_strategy_identity: &WorthQueryEvidenceIdentity,
    future_selection_projection_identity: &WorthQueryEvidenceIdentity,
    counters_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_activation_input_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_projection_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("checkpoint"),
            checkpoint_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn typed_identity_drift(
    left: &WorthQueryEvidenceIdentity,
    right: &WorthQueryEvidenceIdentity,
) -> bool {
    !matches!(left.eq_same_scheme(right), Ok(true))
}

pub(super) fn active_lane_identity(
    activation_identity: &WorthQueryEvidenceIdentity,
    admission_identity: &WorthQueryEvidenceIdentity,
    query_declaration_identity: &WorthQueryEvidenceIdentity,
    bridge_declaration_identity: &WorthQueryEvidenceIdentity,
    future_selection_projection_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    scoped_declaration_basis_digest: &str,
    scoped_activation_basis_digest: &str,
    checkpoint_identity: &WorthQueryEvidenceIdentity,
    signal_strategy_identity: &WorthQueryEvidenceIdentity,
    lifecycle_posture: &str,
    delivery_posture: &str,
    lookup_class: &str,
    allocation_policy: &str,
    registry_lookup_width: usize,
    fanout_width: usize,
    allocation_scope_width: usize,
    performance_receipt_identity: &WorthQueryEvidenceIdentity,
    counters_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "active_subscription_lane_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("activation"),
            activation_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_projection_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_shape(
            WorthQueryEvidenceTag::new("scoped_declaration_basis"),
            scoped_declaration_basis_digest,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("scoped_activation_basis"),
            scoped_activation_basis_digest,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("checkpoint"),
            checkpoint_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_shape(WorthQueryEvidenceTag::new("lifecycle"), lifecycle_posture)
        .field_shape(WorthQueryEvidenceTag::new("delivery"), delivery_posture)
        .field_shape(WorthQueryEvidenceTag::new("lookup"), lookup_class)
        .field_shape(WorthQueryEvidenceTag::new("allocation"), allocation_policy)
        .field_usize(
            WorthQueryEvidenceTag::new("budget_registry"),
            registry_lookup_width,
        )
        .field_usize(WorthQueryEvidenceTag::new("budget_fanout"), fanout_width)
        .field_usize(
            WorthQueryEvidenceTag::new("budget_allocation"),
            allocation_scope_width,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("performance"),
            performance_receipt_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn scale_counter_snapshot_identity(
    fixture_size: &str,
    fixture_row_count: u64,
    activation_identity: &WorthQueryEvidenceIdentity,
    admission_identity: &WorthQueryEvidenceIdentity,
    counter_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_scale_counter_snapshot_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("fixture_size"), fixture_size)
        .field_usize(
            WorthQueryEvidenceTag::new("fixture_row_count"),
            fixture_row_count as usize,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("activation"),
            activation_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("counter"), counter_identity)
        .seal()
}

pub(super) fn scale_slope_report_identity(
    activation_identity: &WorthQueryEvidenceIdentity,
    admission_identity: &WorthQueryEvidenceIdentity,
    small_snapshot_identity: &WorthQueryEvidenceIdentity,
    medium_snapshot_identity: &WorthQueryEvidenceIdentity,
    large_snapshot_identity: &WorthQueryEvidenceIdentity,
    small_row_count: u64,
    medium_row_count: u64,
    large_row_count: u64,
    structural_counter_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_scale_slope_report_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("activation"),
            activation_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("small_snapshot"),
            small_snapshot_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("medium_snapshot"),
            medium_snapshot_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("large_snapshot"),
            large_snapshot_identity,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("small_row_count"),
            small_row_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("medium_row_count"),
            medium_row_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("large_row_count"),
            large_row_count as usize,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("structural_counter"),
            structural_counter_identity,
        )
        .seal()
}

pub(super) fn certification_activation_bundle_identity(
    admission_identity: &WorthQueryEvidenceIdentity,
    activation_identity: &WorthQueryEvidenceIdentity,
    query_declaration_identity: &WorthQueryEvidenceIdentity,
    bridge_declaration_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    signal_strategy_identity: &WorthQueryEvidenceIdentity,
    diagnostics_identity: &WorthQueryEvidenceIdentity,
    support_identity: &WorthQueryEvidenceIdentity,
    admission_counters_identity: &WorthQueryEvidenceIdentity,
    activation_counters_identity: &WorthQueryEvidenceIdentity,
    scale_slope_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_certification_bundle_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("activation"),
            activation_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("diagnostics"),
            diagnostics_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("support"), support_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("admission_counters"),
            admission_counters_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("activation_counters"),
            activation_counters_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("scale_slope"),
            scale_slope_identity,
        )
        .seal()
}

pub(super) fn lifecycle_context_query_identity(
    live: &LiveQueryAdmissionArtifact,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_query_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("live_family"),
            live.live_family().as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            live.future_selection().projection_identity(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("basis"),
            live.basis_posture().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("view_family"),
            live.view_family()
                .map(|family| family.as_str())
                .unwrap_or("none"),
        )
        .seal()
}

pub(super) fn lifecycle_context_policy_identity(policy: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_policy_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("policy"), policy)
        .seal()
}

pub(super) fn lifecycle_context_tenant_basis_identity(
    tenant_basis: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_tenant_basis_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("tenant_basis"), tenant_basis)
        .seal()
}

pub(super) fn lifecycle_context_relationship_proof_identity(
    relationship_proof: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_relationship_proof_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("relationship_proof"),
            relationship_proof,
        )
        .seal()
}

pub(super) fn lifecycle_context_collection_absent_identity() -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_collection_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("collection"), "none")
        .seal()
}

pub(super) fn lifecycle_subscription_family_identity(
    family: &QuerySubscriptionFamily,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_family_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .seal()
}

pub(super) fn lifecycle_subscription_equivalence_identity(
    basis: &QuerySubscriptionEquivalenceBasis,
) -> WorthQueryEvidenceIdentity {
    basis.evidence_identity().clone()
}

pub(super) fn lifecycle_active_lane_handle_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    handle: &ActiveSubscriptionLaneHandle,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_active_lane_handle_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_usize(
            WorthQueryEvidenceTag::new("index"),
            handle.lane_index() as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("generation"),
            handle.registry_generation() as usize,
        )
        .seal()
}

#[cfg(test)]
pub(super) fn lifecycle_absent_work_packet_identity() -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "active_delivery_work_packet_absent_v1",
        )
        .seal()
}

pub(super) fn lifecycle_absent_performance_receipt_identity() -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_performance_receipt_absent_v1",
        )
        .seal()
}

pub(super) fn lifecycle_absent_preview_isolation_identity() -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_preview_isolation_absent_v1",
        )
        .seal()
}

pub(super) fn lifecycle_absent_preview_residue_identity() -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_preview_residue_absent_v1",
        )
        .seal()
}

pub(super) fn lifecycle_absent_continuation_identity() -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_continuation_absent_v1",
        )
        .seal()
}

pub(super) fn lifecycle_performance_sequence_identity<'a>(
    receipts: impl IntoIterator<Item = &'a WorthQueryEvidenceIdentity>,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_performance_receipt_v1",
        )
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("elements"), receipts)
        .seal()
}

pub(super) fn lifecycle_labeled_counter_identity(
    role: &str,
    counter_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_counter_element_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_evidence_identity(WorthQueryEvidenceTag::new("counter"), counter_identity)
        .seal()
}

pub(super) fn lifecycle_counter_sequence_identity<'a>(
    counters: impl IntoIterator<Item = &'a WorthQueryEvidenceIdentity>,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_counter_snapshot_v1",
        )
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("elements"), counters)
        .seal()
}

pub(super) fn lifecycle_support_matrix_identity<'a>(
    support_identities: impl IntoIterator<Item = &'a WorthQueryEvidenceIdentity>,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_support_matrix_v1",
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("elements"),
            support_identities,
        )
        .seal()
}

pub(super) fn lifecycle_delivery_window_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    attachment_identity: &WorthQueryEvidenceIdentity,
    sequence: u64,
    delivery_window_width: u64,
    patch_group_width: u64,
    allocation_scope_width: u64,
    allocation_posture: ActiveSubscriptionAllocationPosture,
    backpressure_policy: DeliveryBackpressurePolicy,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_delivery_window_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_usize(WorthQueryEvidenceTag::new("sequence"), sequence as usize)
        .field_usize(
            WorthQueryEvidenceTag::new("window_width"),
            delivery_window_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("patch_width"),
            patch_group_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("allocation_width"),
            allocation_scope_width as usize,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("allocation_posture"),
            allocation_posture.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("backpressure"),
            backpressure_policy.as_str(),
        )
        .seal()
}

pub(super) fn lifecycle_maintenance_delta_identity(
    kind: QuerySubscriptionMaintenanceDeltaKind,
    lane_identity: &WorthQueryEvidenceIdentity,
    scope_identity: &WorthQueryEvidenceIdentity,
    width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_maintenance_delta_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("scope"), scope_identity)
        .field_usize(WorthQueryEvidenceTag::new("width"), width as usize)
        .seal()
}

pub(super) fn lifecycle_maintenance_delta_scope_identity(
    scope_label: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_maintenance_delta_scope_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("scope"), scope_label)
        .seal()
}

pub(super) fn lifecycle_maintenance_delta_identity_typed(
    kind: QuerySubscriptionMaintenanceDeltaKind,
    lane_identity: &WorthQueryEvidenceIdentity,
    commit_identity: &WorthQueryEvidenceIdentity,
    collection_identity: &WorthQueryEvidenceIdentity,
    entity_identity: &WorthQueryEvidenceIdentity,
    width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_maintenance_delta_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("commit"), commit_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("collection"),
            collection_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("entity"), entity_identity)
        .field_usize(WorthQueryEvidenceTag::new("width"), width as usize)
        .seal()
}

pub(super) fn lifecycle_work_packet_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    attachment_identity: &WorthQueryEvidenceIdentity,
    maintenance_delta_identity: &WorthQueryEvidenceIdentity,
    lowering_report_identity: &WorthQueryEvidenceIdentity,
    density_posture: &str,
    affected_lane_width: u64,
    affected_attachment_width: u64,
    patch_group_width: u64,
    continuation_width: u64,
    preview_residue_width: u64,
    allocation_scope_width: u64,
    allocation_posture: ActiveSubscriptionAllocationPosture,
    performance_receipt_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "active_delivery_work_packet_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("maintenance_delta"),
            maintenance_delta_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("lowering_report"),
            lowering_report_identity,
        )
        .field_shape(WorthQueryEvidenceTag::new("density"), density_posture)
        .field_usize(
            WorthQueryEvidenceTag::new("affected_lane_width"),
            affected_lane_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("affected_attachment_width"),
            affected_attachment_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("patch_group_width"),
            patch_group_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("continuation_width"),
            continuation_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("preview_residue_width"),
            preview_residue_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("allocation_scope_width"),
            allocation_scope_width as usize,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("allocation_posture"),
            allocation_posture.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("performance"),
            performance_receipt_identity,
        )
        .seal()
}

pub(super) fn lifecycle_delivery_batch_receipt_identity(
    attachment_identity: &WorthQueryEvidenceIdentity,
    sequence: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_delivery_batch_receipt_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_usize(WorthQueryEvidenceTag::new("sequence"), sequence as usize)
        .seal()
}

pub(super) fn lifecycle_delivery_batch_identity(
    delivery_window_identity: &WorthQueryEvidenceIdentity,
    work_packet_identity: &WorthQueryEvidenceIdentity,
    delivery_cause_identity: &WorthQueryEvidenceIdentity,
    has_relational_patch: bool,
    patch_group_identity: &WorthQueryEvidenceIdentity,
    receipt_identity: &WorthQueryEvidenceIdentity,
    performance_receipt_identity: &WorthQueryEvidenceIdentity,
    delivery_posture: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_delivery_batch_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("delivery_window"),
            delivery_window_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("work_packet"),
            work_packet_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("delivery_cause"),
            delivery_cause_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("relational_patch"),
            if has_relational_patch {
                "true"
            } else {
                "false"
            },
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("patch_group"),
            patch_group_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("receipt"), receipt_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("performance"),
            performance_receipt_identity,
        )
        .field_shape(WorthQueryEvidenceTag::new("posture"), delivery_posture)
        .seal()
}

#[cfg(test)]
pub(super) fn lifecycle_continuation_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    continuation_class: &str,
    source_identity: &WorthQueryEvidenceIdentity,
    target_identity: &WorthQueryEvidenceIdentity,
    future_selection_identity: &WorthQueryEvidenceIdentity,
    basis_identity: &WorthQueryEvidenceIdentity,
    checkpoint_identity: &WorthQueryEvidenceIdentity,
    authority_identity: &WorthQueryEvidenceIdentity,
    remap_width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_continuation_evidence_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_shape(WorthQueryEvidenceTag::new("class"), continuation_class)
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("target"), target_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("checkpoint"),
            checkpoint_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("authority"), authority_identity)
        .field_usize(
            WorthQueryEvidenceTag::new("remap_width"),
            remap_width as usize,
        )
        .seal()
}

pub(super) fn lifecycle_closeout_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    attachment_identity: &WorthQueryEvidenceIdentity,
    future_selection_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    checkpoint_identity: &WorthQueryEvidenceIdentity,
    closeout_kind: &str,
    lane_terminal: bool,
    support_identity: &WorthQueryEvidenceIdentity,
    performance_receipt_identity: &WorthQueryEvidenceIdentity,
    counters_identity: &WorthQueryEvidenceIdentity,
    source_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_closeout_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("checkpoint"),
            checkpoint_identity,
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), closeout_kind)
        .field_shape(
            WorthQueryEvidenceTag::new("lane_terminal"),
            if lane_terminal { "true" } else { "false" },
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("support"), support_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("performance"),
            performance_receipt_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
        .seal()
}

pub(super) fn diagnostic_source_identity(
    live: &LiveQueryAdmissionArtifact,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_subscription_live_admission_source_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("live_family"),
            live.live_family().as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("query"), live.query_identity())
        .field_evidence_identity(WorthQueryEvidenceTag::new("plan"), live.plan_identity())
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("collection"),
            live.collection_identity(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("view_family"),
            live.view_family()
                .map(|family| family.as_str())
                .unwrap_or("none"),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("basis"),
            live.basis_posture().as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            live.future_selection().projection_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("policy"),
            live.policy_context_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("tenant"),
            live.tenant_context_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("relationship_proof"),
            live.relationship_proof_context_identity(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("relationship_proof_posture"),
            live.relationship_proof_posture().as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("relevance"),
            live.relevance_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("delivery"),
            live.delivery_intent_identity(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("projection_width"),
            live.authorized_projection_width(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("ordering_width"),
            live.ordering_width(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("grouping_width"),
            live.grouping_width(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("relation_scope_width"),
            live.relation_scope_width(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("metadata_width"),
            live.view_shape_metadata_width(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("source"),
            live.construction_source().as_str(),
        )
        .seal()
}

#[cfg(test)]
pub(super) fn preview_epoch_identity(epoch: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_subscription_epoch_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("epoch"), epoch)
        .seal()
}

#[allow(clippy::too_many_arguments)]
#[cfg(test)]
pub(super) fn preview_isolation_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    attachment_identity: &WorthQueryEvidenceIdentity,
    future_selection_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    checkpoint_identity: &WorthQueryEvidenceIdentity,
    preview_epoch_identity: &WorthQueryEvidenceIdentity,
    lifecycle_state: &str,
    preview_residue_budget_width: u64,
    counters_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_subscription_isolation_artifact_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("checkpoint"),
            checkpoint_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("epoch"), preview_epoch_identity)
        .field_shape(WorthQueryEvidenceTag::new("state"), lifecycle_state)
        .field_usize(
            WorthQueryEvidenceTag::new("residue_budget"),
            preview_residue_budget_width as usize,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn preview_authoritative_sharing_denial_identity(
    isolation_identity: &WorthQueryEvidenceIdentity,
    authoritative_lane_identity: &WorthQueryEvidenceIdentity,
    preview_basis_binding_identity: &WorthQueryEvidenceIdentity,
    authoritative_basis_binding_identity: &WorthQueryEvidenceIdentity,
    preview_checkpoint_identity: &WorthQueryEvidenceIdentity,
    authoritative_checkpoint_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_authoritative_sharing_denial_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("preview"), isolation_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authoritative"),
            authoritative_lane_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("preview_basis"),
            preview_basis_binding_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authoritative_basis"),
            authoritative_basis_binding_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("preview_checkpoint"),
            preview_checkpoint_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authoritative_checkpoint"),
            authoritative_checkpoint_identity,
        )
        .seal()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn preview_discard_closeout_identity(
    lane_identity: &WorthQueryEvidenceIdentity,
    attachment_identity: &WorthQueryEvidenceIdentity,
    future_selection_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    checkpoint_identity: &WorthQueryEvidenceIdentity,
    preview_epoch_identity: &WorthQueryEvidenceIdentity,
    isolation_identity: &WorthQueryEvidenceIdentity,
    residue_report_identity: &WorthQueryEvidenceIdentity,
    performance_receipt_identity: &WorthQueryEvidenceIdentity,
    lifecycle_state: &str,
    counters_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_subscription_discard_closeout_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("lane"), lane_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("checkpoint"),
            checkpoint_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("epoch"), preview_epoch_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("isolation"), isolation_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("residue_report"),
            residue_report_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("performance"),
            performance_receipt_identity,
        )
        .field_shape(WorthQueryEvidenceTag::new("state"), lifecycle_state)
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn preview_promotion_authority_identity(
    authority_label: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_subscription_promotion_authority_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("authority"), authority_label)
        .seal()
}

pub(super) fn preview_promotion_rebinding_identity(
    preview_basis_binding_identity: &WorthQueryEvidenceIdentity,
    authoritative_basis_binding_identity: &WorthQueryEvidenceIdentity,
    preview_checkpoint_identity: &WorthQueryEvidenceIdentity,
    authoritative_checkpoint_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_subscription_promotion_rebinding_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("preview_basis"),
            preview_basis_binding_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authoritative_basis"),
            authoritative_basis_binding_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("preview_checkpoint"),
            preview_checkpoint_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authoritative_checkpoint"),
            authoritative_checkpoint_identity,
        )
        .seal()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn preview_promotion_handoff_identity(
    preview_lane_identity: &WorthQueryEvidenceIdentity,
    authoritative_lane_identity: &WorthQueryEvidenceIdentity,
    attachment_identity: &WorthQueryEvidenceIdentity,
    future_selection_identity: &WorthQueryEvidenceIdentity,
    basis_binding_identity: &WorthQueryEvidenceIdentity,
    preview_checkpoint_identity: &WorthQueryEvidenceIdentity,
    authoritative_checkpoint_identity: &WorthQueryEvidenceIdentity,
    preview_epoch_identity: &WorthQueryEvidenceIdentity,
    isolation_identity: &WorthQueryEvidenceIdentity,
    residue_report_identity: &WorthQueryEvidenceIdentity,
    authority_identity: &WorthQueryEvidenceIdentity,
    rebinding_identity: &WorthQueryEvidenceIdentity,
    performance_receipt_identity: &WorthQueryEvidenceIdentity,
    lifecycle_state: &str,
    counters_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_subscription_promotion_handoff_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("preview_lane"),
            preview_lane_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authoritative_lane"),
            authoritative_lane_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("future_selection"),
            future_selection_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("preview_checkpoint"),
            preview_checkpoint_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authoritative_checkpoint"),
            authoritative_checkpoint_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("epoch"), preview_epoch_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("isolation"), isolation_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("residue_report"),
            residue_report_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("authority"), authority_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("rebinding"), rebinding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("performance"),
            performance_receipt_identity,
        )
        .field_shape(WorthQueryEvidenceTag::new("state"), lifecycle_state)
        .field_evidence_identity(WorthQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn lifecycle_certification_bundle_identity(
    base_bundle_identity: &WorthQueryEvidenceIdentity,
    admission_identity: &WorthQueryEvidenceIdentity,
    query_identity: &WorthQueryEvidenceIdentity,
    bridge_identity: &WorthQueryEvidenceIdentity,
    signal_strategy_identity: &WorthQueryEvidenceIdentity,
    query_scope_identity: &WorthQueryEvidenceIdentity,
    subscription_family_identity: &WorthQueryEvidenceIdentity,
    subscription_equivalence_identity: &WorthQueryEvidenceIdentity,
    policy_identity: &WorthQueryEvidenceIdentity,
    tenant_basis_identity: &WorthQueryEvidenceIdentity,
    relationship_proof_identity: &WorthQueryEvidenceIdentity,
    view_shape_identity: &WorthQueryEvidenceIdentity,
    basis_posture_identity: &WorthQueryEvidenceIdentity,
    active_lane_identity: &WorthQueryEvidenceIdentity,
    active_lane_handle_identity: &WorthQueryEvidenceIdentity,
    performance_sequence_identity: &WorthQueryEvidenceIdentity,
    attachment_identity: &WorthQueryEvidenceIdentity,
    delivery_window_identity: &WorthQueryEvidenceIdentity,
    maintenance_delta_identity: &WorthQueryEvidenceIdentity,
    work_packet_identity: &WorthQueryEvidenceIdentity,
    delivery_batch_identity: &WorthQueryEvidenceIdentity,
    delivery_receipt_identity: &WorthQueryEvidenceIdentity,
    continuation_identity: &WorthQueryEvidenceIdentity,
    closeout_identity: &WorthQueryEvidenceIdentity,
    support_matrix_identity: &WorthQueryEvidenceIdentity,
    counter_sequence_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_certification_bundle_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("base"), base_bundle_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("query_declaration"),
            query_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("bridge_declaration"),
            bridge_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("query"), query_scope_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("subscription_family"),
            subscription_family_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("subscription_equivalence"),
            subscription_equivalence_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("policy"), policy_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("tenant_basis"),
            tenant_basis_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("relationship_proof"),
            relationship_proof_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("view_shape"),
            view_shape_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_posture_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("active_lane"),
            active_lane_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("active_lane_handle"),
            active_lane_handle_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("performance"),
            performance_sequence_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("attachment"),
            attachment_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("delivery_window"),
            delivery_window_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("maintenance_delta"),
            maintenance_delta_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("work_packet"),
            work_packet_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("delivery_batch"),
            delivery_batch_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("delivery_receipt"),
            delivery_receipt_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("continuation"),
            continuation_identity,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("closeout"), closeout_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("support"),
            support_matrix_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("counters"),
            counter_sequence_identity,
        )
        .seal()
}

pub(super) fn lifecycle_active_lane_lookup_class_identity(
    lookup_class: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_active_lane_lookup_class_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("lookup_class"), lookup_class)
        .seal()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn lifecycle_subscription_budget_identity(
    registry_lookup_width: u64,
    fanout_width: u64,
    allocation_scope_width: u64,
    lookup_class: &str,
    allocation_posture: &str,
    durable_checkpoint_requested: bool,
    store_backed_restart_requested: bool,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "active_subscription_budget_v1",
        )
        .field_usize(
            WorthQueryEvidenceTag::new("lookup_width"),
            registry_lookup_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("fanout_width"),
            fanout_width as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("allocation_scope_width"),
            allocation_scope_width as usize,
        )
        .field_shape(WorthQueryEvidenceTag::new("lookup_class"), lookup_class)
        .field_shape(
            WorthQueryEvidenceTag::new("allocation_posture"),
            allocation_posture,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("durable_checkpoint_requested"),
            if durable_checkpoint_requested {
                "true"
            } else {
                "false"
            },
        )
        .field_shape(
            WorthQueryEvidenceTag::new("store_backed_restart_requested"),
            if store_backed_restart_requested {
                "true"
            } else {
                "false"
            },
        )
        .seal()
}

pub(super) fn lifecycle_allocation_posture_identity(
    posture: &str,
    allocation_scope_width: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_allocation_posture_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("posture"), posture)
        .field_usize(
            WorthQueryEvidenceTag::new("allocation_scope_width"),
            allocation_scope_width as usize,
        )
        .seal()
}

pub(super) fn lifecycle_active_delivery_density_posture_identity(
    posture: &str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_active_delivery_density_posture_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("posture"), posture)
        .seal()
}

pub(super) fn lifecycle_context_view_shape_identity(
    view_family: Option<&str>,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_view_shape_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("view"),
            view_family.unwrap_or("none"),
        )
        .seal()
}

pub(super) fn lifecycle_context_basis_posture_identity(basis: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_basis_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("basis"), basis)
        .seal()
}

pub(super) fn lifecycle_preview_promotion_residue_identity(
    residue_identity: &WorthQueryEvidenceIdentity,
    handoff_identity: &WorthQueryEvidenceIdentity,
    authoritative_lane_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_preview_residue_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("residue"), residue_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("handoff"), handoff_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authoritative_lane"),
            authoritative_lane_identity,
        )
        .seal()
}
