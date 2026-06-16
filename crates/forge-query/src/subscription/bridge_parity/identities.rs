use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::support::{
    QuerySubscriptionBridgeParityClass, QuerySubscriptionBridgeParityCounters,
    QuerySubscriptionBridgeParityFailureKind,
};
use super::witness::BridgeWitnessAssemblyPosture;

pub(super) fn bridge_parity_counter_identity(
    counters: &QuerySubscriptionBridgeParityCounters,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_bridge_parity_counters_v1",
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("comparison_count"),
            counters.subscription_bridge_parity_comparison_count() as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("admitted_count"),
            counters.subscription_bridge_parity_admitted_count() as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("denial_count"),
            counters.subscription_bridge_parity_denial_count() as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("family_distinction_preservation_count"),
            counters.subscription_bridge_family_distinction_preservation_count() as usize,
        )
        .seal()
}

pub(super) fn bridge_parity_width_identity(
    compared_family_dimension_count: usize,
    compared_slice_dimension_count: usize,
    compared_basis_dimension_count: usize,
    compared_signal_dimension_count: usize,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_bridge_parity_width_v1",
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("family_dimensions"),
            compared_family_dimension_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("slice_dimensions"),
            compared_slice_dimension_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("basis_dimensions"),
            compared_basis_dimension_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("signal_dimensions"),
            compared_signal_dimension_count,
        )
        .seal()
}

pub(super) fn bridge_parity_receipt_identity(
    witness_assembly_posture: BridgeWitnessAssemblyPosture,
    parity_class: QuerySubscriptionBridgeParityClass,
    comparison_width_identity: &ForgeQueryEvidenceIdentity,
    semantic_rebuild_count: usize,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_bridge_parity_receipt_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("witness_assembly_posture"),
            witness_assembly_posture.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("parity_class"),
            parity_class.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("comparison_width"),
            comparison_width_identity,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("semantic_rebuild_count"),
            semantic_rebuild_count,
        )
        .seal()
}

pub(super) fn bridge_parity_comparison_identity(
    parity_class: QuerySubscriptionBridgeParityClass,
    query_declaration_identity: &ForgeQueryEvidenceIdentity,
    bridge_declaration_identity: &ForgeQueryEvidenceIdentity,
    witness_identity: &ForgeQueryEvidenceIdentity,
    activation_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_bridge_parity_comparison_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("parity_class"),
            parity_class.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            query_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            bridge_declaration_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("witness"), witness_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("activation"),
            activation_identity,
        )
        .seal()
}

pub(super) fn bridge_parity_explanation_identity(
    comparison_identity: &ForgeQueryEvidenceIdentity,
    witness_identity: &ForgeQueryEvidenceIdentity,
    receipt_identity: &ForgeQueryEvidenceIdentity,
    counter_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_bridge_parity_explanation_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("comparison"),
            comparison_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("witness"), witness_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("receipt"), receipt_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("counters"), counter_identity)
        .seal()
}

pub(super) fn bridge_parity_failure_identity(
    failure_kind: QuerySubscriptionBridgeParityFailureKind,
    parity_class: QuerySubscriptionBridgeParityClass,
    reason: &str,
    source_identity: &ForgeQueryEvidenceIdentity,
    evidence: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_bridge_parity_failure_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("failure_kind"),
            failure_kind.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("parity_class"),
            parity_class.as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("reason"), reason)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source"), source_identity)
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("evidence"),
            evidence.iter(),
        )
        .seal()
}
