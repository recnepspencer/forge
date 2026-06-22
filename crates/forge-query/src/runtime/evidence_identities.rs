use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeSupportPosture, ForgeQueryLowerRuntimeSupportRow,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryRuntimePosture;
use crate::subscription::QuerySubscriptionDeliveryCauseKind;
use forge_runtime_bridge::facade::{
    BridgeDeniedMixedCause, BridgeMixedCauseDeliveryWindowPlan, BridgeMixedCauseOrdering,
    BridgeOrderedMixedCause, BridgeSuppressedMixedCause,
};

use super::{
    ForgeQueryAuthorityLane, ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeAsyncResultState,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeDownstreamDeliveryClass,
    ForgeQueryRuntimeDownstreamResumePostureKind, ForgeQueryRuntimeDownstreamSupportPosture,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeMixedCauseDelivery,
    ForgeQueryRuntimeRemaskDispositionKind, ForgeQueryRuntimeRemaskPosture,
    ForgeQueryRuntimeRemaskReasonKind, ForgeQueryWriteReceipt,
};

pub(in crate::runtime) fn lower_runtime_support_row_identity(
    row: &ForgeQueryLowerRuntimeSupportRow,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "lower_runtime_support_row_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("seam"), row.seam_key().as_str())
            .field_shape(
                ForgeQueryEvidenceTag::new("capability"),
                row.capability_label(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("owner"),
                row.authority_owner().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("route_kind"),
                row.route_kind().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("artifact"),
                row.artifact_strength().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("posture"),
                row.posture().as_str(),
            );
    match row.detail() {
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSupportDetail::Crossing => {
            identity = identity.field_shape(ForgeQueryEvidenceTag::new("detail"), "crossing");
        }
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSupportDetail::Closeout {
            closeout_target,
            required_closeout,
            certification_row,
        } => {
            identity = identity
                .field_shape(ForgeQueryEvidenceTag::new("detail"), "closeout")
                .field_shape(
                    ForgeQueryEvidenceTag::new("closeout_target"),
                    closeout_target,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("required_closeout"),
                    required_closeout,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("certification_row"),
                    certification_row,
                );
        }
    }
    identity.seal()
}

pub(in crate::runtime) fn lower_runtime_support_rows_aggregate_identity<'a>(
    rows: impl IntoIterator<Item = &'a ForgeQueryLowerRuntimeSupportRow>,
) -> ForgeQueryEvidenceIdentity {
    let row_identities = rows
        .into_iter()
        .map(lower_runtime_support_row_identity)
        .collect::<Vec<_>>();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_downstream_durable_resume_support_v1",
        )
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("rows"), &row_identities)
        .seal()
}

pub(in crate::runtime) fn runtime_downstream_delivery_contract_identity(
    backend_posture: ForgeQueryRuntimeBackendPosture,
    runtime_resume_support_status: ForgeQueryLowerRuntimeSupportPosture,
    runtime_resume_support_identity: &ForgeQueryEvidenceIdentity,
    durable_resume_support_status: ForgeQueryLowerRuntimeSupportPosture,
    durable_resume_support_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_downstream_delivery_contract_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("posture"),
            backend_posture.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("runtime_resume"),
            runtime_resume_support_status.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime_resume_support"),
            runtime_resume_support_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("durable_resume"),
            durable_resume_support_status.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("durable_resume_support"),
            durable_resume_support_identity,
        )
        .seal()
}

pub(in crate::runtime) struct RuntimeDownstreamDeliveryIdentityParts<'a> {
    pub view_name: &'a str,
    pub delivery_batch_identity: &'a ForgeQueryEvidenceIdentity,
    pub delivery_class: ForgeQueryRuntimeDownstreamDeliveryClass,
    pub delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    pub delivery_cause_identity: &'a ForgeQueryEvidenceIdentity,
    pub sequence: u64,
    pub basis_identity: &'a ForgeQueryEvidenceIdentity,
    pub support_posture: ForgeQueryRuntimeDownstreamSupportPosture,
    pub support_identity: &'a ForgeQueryEvidenceIdentity,
    pub mixed_cause_identity: Option<&'a ForgeQueryEvidenceIdentity>,
    pub async_result_state_identity: Option<&'a ForgeQueryEvidenceIdentity>,
    pub remask_identity: Option<&'a ForgeQueryEvidenceIdentity>,
    pub runtime_resume_support_identity: &'a ForgeQueryEvidenceIdentity,
    pub durable_resume_support_identity: &'a ForgeQueryEvidenceIdentity,
}

pub(in crate::runtime) fn runtime_downstream_delivery_identity(
    parts: RuntimeDownstreamDeliveryIdentityParts<'_>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_downstream_delivery_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("view"), parts.view_name)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("batch"),
            parts.delivery_batch_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("class"),
            parts.delivery_class.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("cause"),
            parts.delivery_cause_kind.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("cause_digest"),
            parts.delivery_cause_identity,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("sequence"),
            parts.sequence as usize,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("basis"), parts.basis_identity)
        .field_shape(
            ForgeQueryEvidenceTag::new("support_posture"),
            parts.support_posture.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("support"),
            parts.support_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("mixed_cause"),
            parts.mixed_cause_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("async_result_state"),
            parts.async_result_state_identity,
        )
        .optional_evidence_identity(ForgeQueryEvidenceTag::new("remask"), parts.remask_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime_resume"),
            parts.runtime_resume_support_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("durable_resume"),
            parts.durable_resume_support_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_atomic_identity(
    delivery_cause_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_mixed_cause_atomic_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("delivery_cause"),
            delivery_cause_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_ordering_identity(
    ordering: &BridgeMixedCauseOrdering,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_mixed_cause_ordering_v1",
        )
        .field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("ordering"),
            &ordering.ordering_identity().bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_delivery_window_identity(
    delivery_window: &BridgeMixedCauseDeliveryWindowPlan,
    ordering: &BridgeMixedCauseOrdering,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_mixed_cause_delivery_window_v1",
        )
        .field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("ordering"),
            &ordering.ordering_identity().bridge_admission_evidence(),
        )
        .field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("delivery_window"),
            &delivery_window
                .delivery_window_identity()
                .bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_ordered_cause_identity(
    cause: &BridgeOrderedMixedCause,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_mixed_cause_ordered_cause_v1",
        )
        .field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("ordered_cause"),
            &cause.ordered_cause_identity().bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_suppressed_cause_identity(
    cause: &BridgeSuppressedMixedCause,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_mixed_cause_suppressed_cause_v1",
        )
        .field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("suppressed_cause"),
            &cause
                .suppressed_cause_identity()
                .bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_denied_cause_identity(
    cause: &BridgeDeniedMixedCause,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_mixed_cause_denied_cause_v1",
        )
        .field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("denied_cause"),
            &cause.denied_cause_identity().bridge_admission_evidence(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_mixed_cause_delivery_identity(
    delivery: &ForgeQueryRuntimeMixedCauseDelivery,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_mixed_cause_delivery_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("ordering"),
            delivery.ordering_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("window"),
            delivery.delivery_window_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lane"),
            mixed_cause_lane_label(delivery.lane_kind()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("coalescing"),
            delivery.coalescing_kind().as_public_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("ordered_kind"),
            delivery
                .ordered_member_kinds()
                .iter()
                .map(|member_kind| member_kind.as_public_str()),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("ordered_cause"),
            delivery.ordered_cause_identities().iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("suppressed"),
            delivery.suppressed_cause_identities().iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("denied"),
            delivery.denied_cause_identities().iter(),
        )
        .seal()
}

fn mixed_cause_lane_label(lane: super::ForgeQueryRuntimeMixedCauseLaneKind) -> &'static str {
    match lane {
        super::ForgeQueryRuntimeMixedCauseLaneKind::Authoritative => "authoritative",
        super::ForgeQueryRuntimeMixedCauseLaneKind::Preview => "preview",
    }
}

pub(in crate::runtime) fn runtime_remask_posture_identity(
    disposition_kind: ForgeQueryRuntimeRemaskDispositionKind,
    reason_kind: ForgeQueryRuntimeRemaskReasonKind,
    support_identity: &ForgeQueryEvidenceIdentity,
    basis_identity: &ForgeQueryEvidenceIdentity,
    policy_identity: &ForgeQueryEvidenceIdentity,
    tenant_truth_identity: &ForgeQueryEvidenceIdentity,
    tenant_schema_identity: &ForgeQueryEvidenceIdentity,
    relationship_proof_identity: &ForgeQueryEvidenceIdentity,
    schema_context_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_remask_posture_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("disposition"),
            disposition_kind.as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("reason"), reason_kind.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("support"), support_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("basis"), basis_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("policy"), policy_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("tenant_truth"),
            tenant_truth_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("tenant_schema"),
            tenant_schema_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("relationship_proof"),
            relationship_proof_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("schema_context"),
            schema_context_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_downstream_resume_posture_identity(
    kind: ForgeQueryRuntimeDownstreamResumePostureKind,
    required_basis_identity: Option<&ForgeQueryEvidenceIdentity>,
    support_posture: ForgeQueryLowerRuntimeSupportPosture,
    support_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_downstream_resume_posture_v2",
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("required_basis"),
            required_basis_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("support_posture"),
            support_posture.as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("support"), support_identity)
        .seal()
}

pub(in crate::runtime) struct RuntimeLiveViewInspectionIdentityParts<'a> {
    pub view_name: &'a str,
    pub authority_lane: ForgeQueryAuthorityLane,
    pub query_identity: &'a ForgeQueryEvidenceIdentity,
    pub view_shape_identity: &'a ForgeQueryEvidenceIdentity,
    pub subscription_family: &'a str,
    pub subscription_family_identity: &'a ForgeQueryEvidenceIdentity,
    pub subscription_declaration_identity: &'a ForgeQueryEvidenceIdentity,
    pub bridge_declaration_identity: &'a ForgeQueryEvidenceIdentity,
    pub admission_identity: &'a ForgeQueryEvidenceIdentity,
    pub activation_identity: &'a ForgeQueryEvidenceIdentity,
    pub basis_binding_identity: &'a ForgeQueryEvidenceIdentity,
    pub signal_strategy_identity: &'a ForgeQueryEvidenceIdentity,
    pub active_lane_identity: &'a ForgeQueryEvidenceIdentity,
    pub consumer_attachment_identity: &'a ForgeQueryEvidenceIdentity,
    pub consumer_identity: &'a ForgeQueryEvidenceIdentity,
    pub delivery_cursor_identity: &'a ForgeQueryEvidenceIdentity,
    pub subscription_budget_policy: &'a str,
    pub active_lifecycle_budget_policy: &'a str,
    pub consumer_attachment_budget_policy: &'a str,
    pub runtime_budget_identity: &'a ForgeQueryEvidenceIdentity,
    pub support_identity: &'a ForgeQueryEvidenceIdentity,
    pub last_delivery_cause_kind: Option<QuerySubscriptionDeliveryCauseKind>,
    pub last_delivery_cause_identity: Option<&'a ForgeQueryEvidenceIdentity>,
    pub last_delivery_had_relational_patch: bool,
    pub mixed_cause_identity: Option<&'a ForgeQueryEvidenceIdentity>,
    pub ordinary_runtime_posture: Option<&'a ForgeQueryOrdinaryRuntimePosture>,
    pub async_result_state: Option<&'a ForgeQueryRuntimeAsyncResultState>,
    pub remask_posture: Option<&'a ForgeQueryRuntimeRemaskPosture>,
    pub installation_identity: &'a ForgeQueryEvidenceIdentity,
    pub counter_inspection_identity: &'a ForgeQueryEvidenceIdentity,
}

pub(in crate::runtime) fn runtime_live_view_inspection_identity(
    parts: RuntimeLiveViewInspectionIdentityParts<'_>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_live_view_inspection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("view"), parts.view_name)
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_lane"),
            parts.authority_lane.as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("query"), parts.query_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("view_shape"),
            parts.view_shape_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            parts.subscription_family,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subscription_family"),
            parts.subscription_family_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subscription_declaration"),
            parts.subscription_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            parts.bridge_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission"),
            parts.admission_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("activation"),
            parts.activation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_binding"),
            parts.basis_binding_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            parts.signal_strategy_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("active_lane"),
            parts.active_lane_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("consumer_attachment"),
            parts.consumer_attachment_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("consumer"),
            parts.consumer_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("delivery_cursor"),
            parts.delivery_cursor_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("subscription_budget_policy"),
            parts.subscription_budget_policy,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("active_lifecycle_budget_policy"),
            parts.active_lifecycle_budget_policy,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("consumer_attachment_budget_policy"),
            parts.consumer_attachment_budget_policy,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime_budget"),
            parts.runtime_budget_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("support"),
            parts.support_identity,
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("last_delivery_cause"),
            parts
                .last_delivery_cause_kind
                .map(QuerySubscriptionDeliveryCauseKind::as_str),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("last_delivery_digest"),
            parts.last_delivery_cause_identity,
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("last_delivery_relational"),
            parts.last_delivery_had_relational_patch,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("mixed_cause"),
            parts.mixed_cause_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("ordinary_runtime_posture"),
            parts
                .ordinary_runtime_posture
                .map(ForgeQueryOrdinaryRuntimePosture::evidence_identity),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("async_result_state"),
            parts
                .async_result_state
                .map(ForgeQueryRuntimeAsyncResultState::result_state_identity),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("remask"),
            parts
                .remask_posture
                .map(ForgeQueryRuntimeRemaskPosture::remask_identity),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("installation"),
            parts.installation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("counters"),
            parts.counter_inspection_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_live_subscription_counter_inspection_identity(
    declaration_counter_identity: &ForgeQueryEvidenceIdentity,
    active_lane_counter_identity: &ForgeQueryEvidenceIdentity,
    consumer_attachment_counter_identity: &ForgeQueryEvidenceIdentity,
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
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_live_subscription_inspection_counters_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("declaration_counters"),
            declaration_counter_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("active_lane_counters"),
            active_lane_counter_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("consumer_attachment_counters"),
            consumer_attachment_counter_identity,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("family_selection"),
            family_selection_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("declaration"),
            declaration_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("bridge_lowering"),
            bridge_lowering_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("admission"),
            admission_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("activation_input"),
            activation_input_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("active_lane_admission"),
            active_lane_admission_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("active_lane_creation"),
            active_lane_creation_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("active_lane_join"),
            active_lane_join_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("active_lane_handle_issue"),
            active_lane_handle_issue_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("consumer_attachment"),
            consumer_attachment_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("consumer_attachment_denial"),
            consumer_attachment_denial_count as usize,
        )
        .seal()
}

pub(crate) fn runtime_state_snapshot_basis_label_identity(
    basis_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_state_snapshot_basis_v1",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("basis"), basis_identity)
        .seal()
}

pub(crate) fn runtime_state_snapshot_result_shape_label_identity(
    result_shape_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_state_snapshot_result_shape_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("result_shape"),
            result_shape_identity,
        )
        .seal()
}

pub(in crate::runtime) fn runtime_state_snapshot_result_shape_facade_family_identity(
    facade_family: ForgeQueryRuntimeFacadeFamily,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_state_snapshot_result_shape_facade_family_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("facade_family"),
            facade_family.as_str(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_state_snapshot_result_shape_write_receipt_identity(
    receipt: &ForgeQueryWriteReceipt,
) -> ForgeQueryEvidenceIdentity {
    let declared_entity_identity = receipt
        .declared_entity_identity()
        .map(|identity| identity.evidence_identity());
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_state_snapshot_result_shape_write_receipt_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("mutation_family"),
            receipt.mutation_family().as_str(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("declared_collection"),
            receipt.declared_collection(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit_evidence_identity"),
            receipt.commit_evidence_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("declared_entity_identity"),
            declared_entity_identity.as_ref(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_state_snapshot_result_shape_batch_write_receipt_identity(
    receipt: &ForgeQueryBatchWriteReceipt,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_runtime_state_snapshot_result_shape_batch_write_receipt_v1",
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("write_count"),
            receipt.write_count(),
        )
        .seal()
}

pub(in crate::runtime) fn runtime_live_view_consumer_attachment_identity(
    view_name: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_view_consumer_attachment_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("view"), view_name)
        .seal()
}

pub(in crate::runtime) fn shared_read_unpublished_causality_identity(
    view_name: &str,
    snapshot_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "shared_read_unpublished_causality_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("view"), view_name)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .seal()
}

pub(in crate::runtime) fn shared_read_republishing_causality_identity(
    view_name: &str,
    kind: super::ForgeQueryRuntimeAsyncResultStateKind,
    snapshot_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "shared_read_republishing_causality_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("view"), view_name)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .seal()
}

pub(in crate::runtime) fn shared_read_bind_retained_artifact_label_identity(
    view_name: &str,
    snapshot_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::SharedReadGeneration)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "shared_read_bind_retained_artifact_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("view"), view_name)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .seal()
}

#[cfg(test)]
pub(in crate::runtime) fn runtime_state_snapshot_test_subject_identity(
    label: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(ForgeQueryEvidenceTag::new("test_subject"), label)
        .seal()
}
