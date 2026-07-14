use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::lower_runtime_routing::{
    WorthQueryLowerRuntimeSupportPosture, WorthQueryLowerRuntimeSupportRow,
};
use crate::ordinary_outcome::WorthQueryOrdinaryRuntimePosture;
use crate::subscription::QuerySubscriptionDeliveryCauseKind;
use worth_runtime_bridge::facade::{
    BridgeDeniedMixedCause, BridgeMixedCauseDeliveryWindowPlan, BridgeMixedCauseOrdering,
    BridgeOrderedMixedCause, BridgeSuppressedMixedCause,
};

use super::{
    WorthQueryAuthorityLane, WorthQueryBatchWriteReceipt, WorthQueryRuntimeAsyncResultState,
    WorthQueryRuntimeBackendPosture, WorthQueryRuntimeDownstreamDeliveryClass,
    WorthQueryRuntimeDownstreamResumePostureKind, WorthQueryRuntimeDownstreamSupportPosture,
    WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeMixedCauseDelivery,
    WorthQueryRuntimeRemaskDispositionKind, WorthQueryRuntimeRemaskPosture,
    WorthQueryRuntimeRemaskReasonKind, WorthQueryWriteReceipt,
};

pub(in crate::runtime) fn lower_runtime_support_row_identity(
    row: &WorthQueryLowerRuntimeSupportRow,
) -> WorthQueryEvidenceIdentity {
    let mut identity =
        worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "lower_runtime_support_row_v1",
            )
            .field_shape(WorthQueryEvidenceTag::new("seam"), row.seam_key().as_str())
            .field_shape(
                WorthQueryEvidenceTag::new("capability"),
                row.capability_label(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("owner"),
                row.authority_owner().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("route_kind"),
                row.route_kind().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("artifact"),
                row.artifact_strength().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("posture"),
                row.posture().as_str(),
            );
    match row.detail() {
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSupportDetail::Crossing => {
            identity = identity.field_shape(WorthQueryEvidenceTag::new("detail"), "crossing");
        }
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSupportDetail::Closeout {
            closeout_target,
            required_closeout,
            certification_row,
        } => {
            identity = identity
                .field_shape(WorthQueryEvidenceTag::new("detail"), "closeout")
                .field_shape(
                    WorthQueryEvidenceTag::new("closeout_target"),
                    closeout_target,
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("required_closeout"),
                    required_closeout,
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("certification_row"),
                    certification_row,
                );
        }
    }
    identity.seal()
}

pub(in crate::runtime) fn lower_runtime_support_rows_aggregate_identity<'a>(
    rows: impl IntoIterator<Item = &'a WorthQueryLowerRuntimeSupportRow>,
) -> WorthQueryEvidenceIdentity {
    let row_identities = rows
        .into_iter()
        .map(lower_runtime_support_row_identity)
        .collect::<Vec<_>>();
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "runtime_downstream_durable_resume_support_v1",
        )
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("rows"), &row_identities)
        .seal()
}

pub(in crate::runtime) fn runtime_downstream_delivery_contract_identity(
    backend_posture: WorthQueryRuntimeBackendPosture,
    runtime_resume_support_status: WorthQueryLowerRuntimeSupportPosture,
    runtime_resume_support_identity: &WorthQueryEvidenceIdentity,
    durable_resume_support_status: WorthQueryLowerRuntimeSupportPosture,
    durable_resume_support_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_downstream_delivery_contract_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("posture"),
            backend_posture.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("runtime_resume"),
            runtime_resume_support_status.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("runtime_resume_support"),
            runtime_resume_support_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("durable_resume"),
            durable_resume_support_status.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("durable_resume_support"),
            durable_resume_support_identity,
        )
        .seal()
}

pub(in crate::runtime) struct RuntimeDownstreamDeliveryIdentityParts<'a> {
    pub view_name: &'a str,
    pub delivery_batch_identity: &'a WorthQueryEvidenceIdentity,
    pub delivery_class: WorthQueryRuntimeDownstreamDeliveryClass,
    pub delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    pub delivery_cause_identity: &'a WorthQueryEvidenceIdentity,
    pub sequence: u64,
    pub basis_identity: &'a WorthQueryEvidenceIdentity,
    pub support_posture: WorthQueryRuntimeDownstreamSupportPosture,
    pub support_identity: &'a WorthQueryEvidenceIdentity,
    pub mixed_cause_identity: Option<&'a WorthQueryEvidenceIdentity>,
    pub async_result_state_identity: Option<&'a WorthQueryEvidenceIdentity>,
    pub remask_identity: Option<&'a WorthQueryEvidenceIdentity>,
    pub runtime_resume_support_identity: &'a WorthQueryEvidenceIdentity,
    pub durable_resume_support_identity: &'a WorthQueryEvidenceIdentity,
}

pub(in crate::runtime) fn runtime_downstream_delivery_identity(
    parts: RuntimeDownstreamDeliveryIdentityParts<'_>,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_downstream_delivery_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("view"), parts.view_name)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("batch"),
            parts.delivery_batch_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("class"),
            parts.delivery_class.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("cause"),
            parts.delivery_cause_kind.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("cause_digest"),
            parts.delivery_cause_identity,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("sequence"),
            parts.sequence as usize,
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), parts.basis_identity)
        .field_shape(
            WorthQueryEvidenceTag::new("support_posture"),
            parts.support_posture.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("support"),
            parts.support_identity,
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("mixed_cause"),
            parts.mixed_cause_identity,
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("async_result_state"),
            parts.async_result_state_identity,
        )
        .optional_evidence_identity(WorthQueryEvidenceTag::new("remask"), parts.remask_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("runtime_resume"),
            parts.runtime_resume_support_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("durable_resume"),
            parts.durable_resume_support_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_atomic_identity(
    delivery_cause_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_mixed_cause_atomic_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("delivery_cause"),
            delivery_cause_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_ordering_identity(
    ordering: &BridgeMixedCauseOrdering,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_mixed_cause_ordering_v1",
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("ordering"),
            &ordering.ordering_identity().bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_delivery_window_identity(
    delivery_window: &BridgeMixedCauseDeliveryWindowPlan,
    ordering: &BridgeMixedCauseOrdering,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_mixed_cause_delivery_window_v1",
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("ordering"),
            &ordering.ordering_identity().bridge_admission_evidence(),
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("delivery_window"),
            &delivery_window
                .delivery_window_identity()
                .bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_ordered_cause_identity(
    cause: &BridgeOrderedMixedCause,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_mixed_cause_ordered_cause_v1",
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("ordered_cause"),
            &cause.ordered_cause_identity().bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_suppressed_cause_identity(
    cause: &BridgeSuppressedMixedCause,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_mixed_cause_suppressed_cause_v1",
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("suppressed_cause"),
            &cause
                .suppressed_cause_identity()
                .bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_denied_cause_identity(
    cause: &BridgeDeniedMixedCause,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_mixed_cause_denied_cause_v1",
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("denied_cause"),
            &cause.denied_cause_identity().bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_delivery_identity(
    delivery: &WorthQueryRuntimeMixedCauseDelivery,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_mixed_cause_delivery_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("ordering"),
            delivery.ordering_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("window"),
            delivery.delivery_window_identity(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("lane"),
            mixed_cause_lane_label(delivery.lane_kind()),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("coalescing"),
            delivery.coalescing_kind().as_public_str(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("ordered_kind"),
            delivery
                .ordered_member_kinds()
                .iter()
                .map(|member_kind| member_kind.as_public_str()),
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("ordered_cause"),
            delivery.ordered_cause_identities().iter(),
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("suppressed"),
            delivery.suppressed_cause_identities().iter(),
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("denied"),
            delivery.denied_cause_identities().iter(),
        )
        .seal()
}

fn mixed_cause_lane_label(lane: super::WorthQueryRuntimeMixedCauseLaneKind) -> &'static str {
    match lane {
        super::WorthQueryRuntimeMixedCauseLaneKind::Authoritative => "authoritative",
        super::WorthQueryRuntimeMixedCauseLaneKind::Preview => "preview",
    }
}

pub(in crate::runtime) fn runtime_remask_posture_identity(
    disposition_kind: WorthQueryRuntimeRemaskDispositionKind,
    reason_kind: WorthQueryRuntimeRemaskReasonKind,
    support_identity: &WorthQueryEvidenceIdentity,
    basis_identity: &WorthQueryEvidenceIdentity,
    policy_identity: &WorthQueryEvidenceIdentity,
    tenant_truth_identity: &WorthQueryEvidenceIdentity,
    tenant_schema_identity: &WorthQueryEvidenceIdentity,
    relationship_proof_identity: &WorthQueryEvidenceIdentity,
    schema_context_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_remask_posture_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("disposition"),
            disposition_kind.as_str(),
        )
        .field_shape(WorthQueryEvidenceTag::new("reason"), reason_kind.as_str())
        .field_evidence_identity(WorthQueryEvidenceTag::new("support"), support_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("policy"), policy_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("tenant_truth"),
            tenant_truth_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("tenant_schema"),
            tenant_schema_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("relationship_proof"),
            relationship_proof_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("schema_context"),
            schema_context_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_downstream_resume_posture_identity(
    kind: WorthQueryRuntimeDownstreamResumePostureKind,
    required_basis_identity: Option<&WorthQueryEvidenceIdentity>,
    support_posture: WorthQueryLowerRuntimeSupportPosture,
    support_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_downstream_resume_posture_v2",
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), kind.as_str())
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("required_basis"),
            required_basis_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("support_posture"),
            support_posture.as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("support"), support_identity)
        .seal()
}

pub(in crate::runtime) struct RuntimeLiveViewInspectionIdentityParts<'a> {
    pub view_name: &'a str,
    pub authority_lane: WorthQueryAuthorityLane,
    pub query_identity: &'a WorthQueryEvidenceIdentity,
    pub view_shape_identity: &'a WorthQueryEvidenceIdentity,
    pub subscription_family: &'a str,
    pub subscription_family_identity: &'a WorthQueryEvidenceIdentity,
    pub subscription_declaration_identity: &'a WorthQueryEvidenceIdentity,
    pub bridge_declaration_identity: &'a WorthQueryEvidenceIdentity,
    pub admission_identity: &'a WorthQueryEvidenceIdentity,
    pub activation_identity: &'a WorthQueryEvidenceIdentity,
    pub basis_binding_identity: &'a WorthQueryEvidenceIdentity,
    pub signal_strategy_identity: &'a WorthQueryEvidenceIdentity,
    pub active_lane_identity: &'a WorthQueryEvidenceIdentity,
    pub consumer_attachment_identity: &'a WorthQueryEvidenceIdentity,
    pub consumer_identity: &'a WorthQueryEvidenceIdentity,
    pub delivery_cursor_identity: &'a WorthQueryEvidenceIdentity,
    pub subscription_budget_policy: &'a str,
    pub active_lifecycle_budget_policy: &'a str,
    pub consumer_attachment_budget_policy: &'a str,
    pub runtime_budget_identity: &'a WorthQueryEvidenceIdentity,
    pub support_identity: &'a WorthQueryEvidenceIdentity,
    pub last_delivery_cause_kind: Option<QuerySubscriptionDeliveryCauseKind>,
    pub last_delivery_cause_identity: Option<&'a WorthQueryEvidenceIdentity>,
    pub last_delivery_had_relational_patch: bool,
    pub mixed_cause_identity: Option<&'a WorthQueryEvidenceIdentity>,
    pub ordinary_runtime_posture: Option<&'a WorthQueryOrdinaryRuntimePosture>,
    pub async_result_state: Option<&'a WorthQueryRuntimeAsyncResultState>,
    pub remask_posture: Option<&'a WorthQueryRuntimeRemaskPosture>,
    pub installation_identity: &'a WorthQueryEvidenceIdentity,
    pub counter_inspection_identity: &'a WorthQueryEvidenceIdentity,
}

pub(in crate::runtime) fn runtime_live_view_inspection_identity(
    parts: RuntimeLiveViewInspectionIdentityParts<'_>,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_live_view_inspection_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("view"), parts.view_name)
        .field_shape(
            WorthQueryEvidenceTag::new("authority_lane"),
            parts.authority_lane.as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("query"), parts.query_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("view_shape"),
            parts.view_shape_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            parts.subscription_family,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("subscription_family"),
            parts.subscription_family_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("subscription_declaration"),
            parts.subscription_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("bridge_declaration"),
            parts.bridge_declaration_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("admission"),
            parts.admission_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("activation"),
            parts.activation_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("basis_binding"),
            parts.basis_binding_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("signal_strategy"),
            parts.signal_strategy_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("active_lane"),
            parts.active_lane_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("consumer_attachment"),
            parts.consumer_attachment_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("consumer"),
            parts.consumer_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("delivery_cursor"),
            parts.delivery_cursor_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("subscription_budget_policy"),
            parts.subscription_budget_policy,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("active_lifecycle_budget_policy"),
            parts.active_lifecycle_budget_policy,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("consumer_attachment_budget_policy"),
            parts.consumer_attachment_budget_policy,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("runtime_budget"),
            parts.runtime_budget_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("support"),
            parts.support_identity,
        )
        .optional_shape(
            WorthQueryEvidenceTag::new("last_delivery_cause"),
            parts
                .last_delivery_cause_kind
                .map(QuerySubscriptionDeliveryCauseKind::as_str),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("last_delivery_digest"),
            parts.last_delivery_cause_identity,
        )
        .field_bool(
            WorthQueryEvidenceTag::new("last_delivery_relational"),
            parts.last_delivery_had_relational_patch,
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("mixed_cause"),
            parts.mixed_cause_identity,
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("ordinary_runtime_posture"),
            parts
                .ordinary_runtime_posture
                .map(WorthQueryOrdinaryRuntimePosture::evidence_identity),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("async_result_state"),
            parts
                .async_result_state
                .map(WorthQueryRuntimeAsyncResultState::result_state_identity),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("remask"),
            parts
                .remask_posture
                .map(WorthQueryRuntimeRemaskPosture::remask_identity),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("installation"),
            parts.installation_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("counters"),
            parts.counter_inspection_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_live_subscription_counter_inspection_identity(
    declaration_counter_identity: &WorthQueryEvidenceIdentity,
    active_lane_counter_identity: &WorthQueryEvidenceIdentity,
    consumer_attachment_counter_identity: &WorthQueryEvidenceIdentity,
    family_selection_count: u64,
    declaration_count: u64,
    bridge_lowering_count: u64,
    admission_count: u64,
    activation_input_count: u64,
    active_lane_admission_count: u64,
    active_lane_creation_count: u64,
    active_lane_join_count: u64,
    active_lane_handle_issue_count: u64,
    consumer_attachment_count: u64,
    consumer_attachment_denial_count: u64,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_live_subscription_inspection_counters_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("declaration_counters"),
            declaration_counter_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("active_lane_counters"),
            active_lane_counter_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("consumer_attachment_counters"),
            consumer_attachment_counter_identity,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("family_selection"),
            family_selection_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("declaration"),
            declaration_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("bridge_lowering"),
            bridge_lowering_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("admission"),
            admission_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("activation_input"),
            activation_input_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("active_lane_admission"),
            active_lane_admission_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("active_lane_creation"),
            active_lane_creation_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("active_lane_join"),
            active_lane_join_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("active_lane_handle_issue"),
            active_lane_handle_issue_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("consumer_attachment"),
            consumer_attachment_count as usize,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("consumer_attachment_denial"),
            consumer_attachment_denial_count as usize,
        )
        .seal()
}

pub(crate) fn runtime_state_snapshot_basis_label_identity(
    basis_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_state_snapshot_basis_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_identity)
        .seal()
}

pub(crate) fn runtime_state_snapshot_result_shape_label_identity(
    result_shape_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_state_snapshot_result_shape_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("result_shape"),
            result_shape_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_state_snapshot_result_shape_facade_family_identity(
    facade_family: WorthQueryRuntimeFacadeFamily,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_state_snapshot_result_shape_facade_family_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("facade_family"),
            facade_family.as_str(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_state_snapshot_result_shape_write_receipt_identity(
    receipt: &WorthQueryWriteReceipt,
) -> WorthQueryEvidenceIdentity {
    let declared_entity_identity = receipt
        .declared_entity_identity()
        .map(|identity| identity.evidence_identity());
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_state_snapshot_result_shape_write_receipt_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("mutation_family"),
            receipt.mutation_family().as_str(),
        )
        .optional_shape(
            WorthQueryEvidenceTag::new("declared_collection"),
            receipt.terminal_declared_collection_projection(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("commit_evidence_identity"),
            receipt.commit_evidence_identity(),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("declared_entity_identity"),
            declared_entity_identity.as_ref(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_state_snapshot_result_shape_batch_write_receipt_identity(
    receipt: &WorthQueryBatchWriteReceipt,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_runtime_state_snapshot_result_shape_batch_write_receipt_v1",
        )
        .field_usize(
            WorthQueryEvidenceTag::new("write_count"),
            receipt.write_count(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_live_view_consumer_attachment_identity(
    view_name: &str,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "runtime_live_view_consumer_attachment_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("view"), view_name)
        .seal()
}

pub(in crate::runtime) fn shared_read_unpublished_causality_identity(
    view_name: &str,
    snapshot_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "shared_read_unpublished_causality_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("view"), view_name)
        .field_evidence_identity(WorthQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .seal()
}

pub(in crate::runtime) fn shared_read_republishing_causality_identity(
    view_name: &str,
    kind: super::WorthQueryRuntimeAsyncResultStateKind,
    snapshot_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "shared_read_republishing_causality_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("view"), view_name)
        .field_shape(WorthQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(WorthQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .seal()
}

pub(in crate::runtime) fn shared_read_bind_retained_artifact_label_identity(
    view_name: &str,
    snapshot_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::SharedReadGeneration)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "shared_read_bind_retained_artifact_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("view"), view_name)
        .field_evidence_identity(WorthQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .seal()
}

#[cfg(test)]
pub(in crate::runtime) fn runtime_state_snapshot_test_subject_identity(
    label: &str,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(WorthQueryEvidenceTag::new("test_subject"), label)
        .seal()
}
