use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::basis_request::QuerySubscriptionBasisBindingRequestKind;
use super::bridge_family::BridgeSubscriptionDeclarationFamilyKind;
use super::bridge_slice::BridgeSubscriptionSliceKind;
use super::signal_strategy::QuerySubscriptionSignalStrategyRequestKind;

const SUBSCRIPTION_IDENTITY_SCOPE: ForgeQueryEvidenceScope =
    ForgeQueryEvidenceScope::SubscriptionActivationReceipt;

pub(super) fn subscription_source_identity(
    role: &str,
    source_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_source_identity_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_identity(ForgeQueryEvidenceTag::new("source_digest"), source_digest)
        .seal()
}

pub(super) fn basis_binding_request_identity(
    request_kind: &QuerySubscriptionBasisBindingRequestKind,
    source_declaration_identity: &ForgeQueryEvidenceIdentity,
    source_equivalence_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_basis_binding_request_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("request_kind"),
            request_kind.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("source_declaration"),
            source_declaration_identity,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("source_equivalence"),
            source_equivalence_digest,
        )
        .seal()
}

pub(super) fn signal_strategy_request_identity(
    request_kind: &QuerySubscriptionSignalStrategyRequestKind,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_signal_strategy_request_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("request_kind"),
            request_kind.as_str(),
        )
        .seal()
}

pub(super) fn bridge_lowering_query_declaration_identity(
    declaration_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    subscription_source_identity("query_declaration", declaration_digest)
}

pub(super) fn bridge_lowering_plan_identity(
    query_declaration_identity: &ForgeQueryEvidenceIdentity,
    bridge_family: &BridgeSubscriptionDeclarationFamilyKind,
    basis_request_identity: &ForgeQueryEvidenceIdentity,
    signal_strategy_identity: &ForgeQueryEvidenceIdentity,
    bridge_slices: &[BridgeSubscriptionSliceKind],
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_bridge_lowering_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("bridge_family"),
            bridge_family.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            basis_request_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("bridge_slices"),
            bridge_slices.iter().map(BridgeSubscriptionSliceKind::as_str),
        )
        .seal()
}

pub(super) fn admission_artifact_identity(
    query_declaration_identity: &ForgeQueryEvidenceIdentity,
    bridge_declaration_identity: &ForgeQueryEvidenceIdentity,
    basis_binding_identity: &ForgeQueryEvidenceIdentity,
    signal_strategy_identity: &ForgeQueryEvidenceIdentity,
    diagnostics_identity: &ForgeQueryEvidenceIdentity,
    support_identity: &ForgeQueryEvidenceIdentity,
    declaration_width_limit: usize,
    bridge_width_limit: usize,
    basis_width_limit: usize,
    signal_width_limit: usize,
    activation_width_limit: usize,
    counters_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_admission_artifact_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            basis_binding_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("diagnostics"),
            diagnostics_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("support"), support_identity)
        .field_usize(
            ForgeQueryEvidenceTag::new("budget_declaration"),
            declaration_width_limit,
        )
        .field_usize(ForgeQueryEvidenceTag::new("budget_bridge"), bridge_width_limit)
        .field_usize(ForgeQueryEvidenceTag::new("budget_basis"), basis_width_limit)
        .field_usize(ForgeQueryEvidenceTag::new("budget_signal"), signal_width_limit)
        .field_usize(
            ForgeQueryEvidenceTag::new("budget_activation"),
            activation_width_limit,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn activation_checkpoint_identity(
    query_declaration_identity: &ForgeQueryEvidenceIdentity,
    basis_binding_identity: &ForgeQueryEvidenceIdentity,
    future_selection_projection_identity: &ForgeQueryEvidenceIdentity,
    signal_strategy_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_active_checkpoint_identity_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            basis_binding_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("future_selection"),
            future_selection_projection_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .seal()
}

pub(super) fn activation_input_identity(
    admission_identity: &ForgeQueryEvidenceIdentity,
    query_declaration_identity: &ForgeQueryEvidenceIdentity,
    bridge_declaration_identity: &ForgeQueryEvidenceIdentity,
    basis_binding_identity: &ForgeQueryEvidenceIdentity,
    checkpoint_identity: &ForgeQueryEvidenceIdentity,
    signal_strategy_identity: &ForgeQueryEvidenceIdentity,
    future_selection_projection_identity: &ForgeQueryEvidenceIdentity,
    counters_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_activation_input_v1",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("future_selection"),
            future_selection_projection_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            basis_binding_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("checkpoint"), checkpoint_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn typed_identity_drift(
    left: &ForgeQueryEvidenceIdentity,
    right: &ForgeQueryEvidenceIdentity,
) -> bool {
    !matches!(left.eq_same_scheme(right), Ok(true))
}

pub(super) fn active_lane_identity(
    activation_identity: &ForgeQueryEvidenceIdentity,
    admission_identity: &ForgeQueryEvidenceIdentity,
    query_declaration_for_reporting: &str,
    bridge_declaration_identity: &ForgeQueryEvidenceIdentity,
    future_selection_projection_identity: &ForgeQueryEvidenceIdentity,
    basis_binding_identity: &ForgeQueryEvidenceIdentity,
    checkpoint_identity: &ForgeQueryEvidenceIdentity,
    signal_strategy_identity: &ForgeQueryEvidenceIdentity,
    lifecycle_posture: &str,
    delivery_posture: &str,
    lookup_class: &str,
    allocation_policy: &str,
    registry_lookup_width: usize,
    fanout_width: usize,
    allocation_scope_width: usize,
    performance_receipt_identity: &ForgeQueryEvidenceIdentity,
    counters_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "active_subscription_lane_v1",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("activation"), activation_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("admission"), admission_identity)
        .field_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            query_declaration_for_reporting,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("future_selection"),
            future_selection_projection_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            basis_binding_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("checkpoint"), checkpoint_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_shape(ForgeQueryEvidenceTag::new("lifecycle"), lifecycle_posture)
        .field_shape(ForgeQueryEvidenceTag::new("delivery"), delivery_posture)
        .field_shape(ForgeQueryEvidenceTag::new("lookup"), lookup_class)
        .field_shape(ForgeQueryEvidenceTag::new("allocation"), allocation_policy)
        .field_usize(ForgeQueryEvidenceTag::new("budget_registry"), registry_lookup_width)
        .field_usize(ForgeQueryEvidenceTag::new("budget_fanout"), fanout_width)
        .field_usize(
            ForgeQueryEvidenceTag::new("budget_allocation"),
            allocation_scope_width,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("performance"),
            performance_receipt_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("counters"), counters_identity)
        .seal()
}

pub(super) fn scale_counter_snapshot_identity(
    fixture_size: &str,
    fixture_row_count: u64,
    activation_identity: &ForgeQueryEvidenceIdentity,
    admission_identity: &ForgeQueryEvidenceIdentity,
    counter_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_scale_counter_snapshot_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("fixture_size"), fixture_size)
        .field_usize(ForgeQueryEvidenceTag::new("fixture_row_count"), fixture_row_count as usize)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("activation"), activation_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("counter"), counter_identity)
        .seal()
}

pub(super) fn certification_activation_bundle_identity(
    admission_identity: &ForgeQueryEvidenceIdentity,
    activation_identity: &ForgeQueryEvidenceIdentity,
    query_declaration_for_reporting: &str,
    bridge_declaration_identity: &ForgeQueryEvidenceIdentity,
    basis_binding_identity: &ForgeQueryEvidenceIdentity,
    signal_strategy_identity: &ForgeQueryEvidenceIdentity,
    diagnostics_identity: &ForgeQueryEvidenceIdentity,
    support_identity: &ForgeQueryEvidenceIdentity,
    admission_counters_identity: &ForgeQueryEvidenceIdentity,
    activation_counters_identity: &ForgeQueryEvidenceIdentity,
    scale_slope_identity: &ForgeQueryEvidenceIdentity,
    scale_activation_for_reporting: &str,
    scale_admission_for_reporting: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_certification_bundle_v1",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("activation"), activation_identity)
        .field_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            query_declaration_for_reporting,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            basis_binding_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("diagnostics"),
            diagnostics_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("support"), support_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_counters"),
            admission_counters_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("activation_counters"),
            activation_counters_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("scale_slope"),
            scale_slope_identity,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("scale_activation"),
            scale_activation_for_reporting,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("scale_admission"),
            scale_admission_for_reporting,
        )
        .seal()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn lifecycle_certification_bundle_identity(
    base_bundle_identity: &ForgeQueryEvidenceIdentity,
    admission_identity: &ForgeQueryEvidenceIdentity,
    query_identity: &ForgeQueryEvidenceIdentity,
    bridge_identity: &ForgeQueryEvidenceIdentity,
    signal_strategy_identity: &ForgeQueryEvidenceIdentity,
    query_digest: &str,
    subscription_family_digest: &str,
    subscription_equivalence_digest: &str,
    active_lane_digest: &str,
    active_lane_handle_digest: &str,
    performance_digest: &str,
    attachment_digest: &str,
    delivery_window_digest: &str,
    maintenance_delta_digest: &str,
    work_packet_digest: &str,
    delivery_batch_digest: &str,
    delivery_receipt_digest: &str,
    continuation_digest: &str,
    closeout_digest: &str,
    support_matrix_digest: &str,
    counter_snapshot: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "subscription_lifecycle_certification_bundle_v1",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("base"), base_bundle_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("admission"), admission_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            query_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            bridge_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            signal_strategy_identity,
        )
        .field_identity(ForgeQueryEvidenceTag::new("query"), query_digest)
        .field_identity(
            ForgeQueryEvidenceTag::new("subscription_family"),
            subscription_family_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("subscription_equivalence"),
            subscription_equivalence_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("active_lane"), active_lane_digest)
        .field_identity(
            ForgeQueryEvidenceTag::new("active_lane_handle"),
            active_lane_handle_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("performance"), performance_digest)
        .field_identity(ForgeQueryEvidenceTag::new("attachment"), attachment_digest)
        .field_identity(
            ForgeQueryEvidenceTag::new("delivery_window"),
            delivery_window_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("maintenance_delta"),
            maintenance_delta_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("work_packet"), work_packet_digest)
        .field_identity(ForgeQueryEvidenceTag::new("delivery_batch"), delivery_batch_digest)
        .field_identity(
            ForgeQueryEvidenceTag::new("delivery_receipt"),
            delivery_receipt_digest,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("continuation"),
            continuation_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("closeout"), closeout_digest)
        .field_identity(ForgeQueryEvidenceTag::new("support"), support_matrix_digest)
        .field_identity(ForgeQueryEvidenceTag::new("counters"), counter_snapshot)
        .seal()
}

pub(super) fn subscription_certification_projection(
    identity_family: &'static str,
    fields: impl IntoIterator<Item = (&'static str, String)>,
) -> String {
    let mut builder = ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE).field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        identity_family,
    );
    for (tag, value) in fields {
        builder = builder.field_identity(ForgeQueryEvidenceTag::new(tag), value.as_str());
    }
    builder.seal().as_str().to_string()
}

pub(super) fn subscription_certification_sequence_projection(
    identity_family: &'static str,
    element_family: &'static str,
    values: &[String],
) -> String {
    let mut identity = ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            identity_family,
        )
        .field_usize(ForgeQueryEvidenceTag::new("width"), values.len())
        .seal();
    for (index, value) in values.iter().enumerate() {
        let element = subscription_certification_projection(
            element_family,
            [
                ("index", index.to_string()),
                ("value", value.clone()),
            ],
        );
        identity = ForgeQueryEvidenceIdentity::compose(SUBSCRIPTION_IDENTITY_SCOPE)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                identity_family,
            )
            .field_evidence_identity(ForgeQueryEvidenceTag::new("prior"), &identity)
            .field_identity(ForgeQueryEvidenceTag::new("element"), element.as_str())
            .seal();
    }
    identity.as_str().to_string()
}
