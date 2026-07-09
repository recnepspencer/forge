mod builders;
mod tests;

use crate::harness::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, RejectionCertificationRow,
};

pub const MILESTONE_NINE_TWO_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "detail-active-lifecycle-delivery-ack",
    "equivalent-subscription-sharing-fanout",
    "grouped-membership-query-shaped-delivery",
    "identity-continuation-remap-delivery",
    "preview-discard-zero-authoritative-residue",
    "preview-promotion-boundary-handoff",
    "performance-receipt-posture-sensitive",
    "scale-slope-width-bounded-lifecycle",
];

pub const MILESTONE_NINE_TWO_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "masked-sharing-denies-before-join",
    "raw-cdc-delivery-denied-before-batch",
    "raw-bridge-invalidation-denied-before-batch",
    "preview-authoritative-sharing-denied",
    "preview-discard-authoritative-residue-denied",
    "dense-refresh-denied-before-work-packet",
    "store-backed-restart-denied-before-lane",
];

pub const MILESTONE_NINE_TWO_REQUIRED_COMPILE_FAIL_TARGETS: &[&str] = &[
    "active_subscription_lane_constructor_private.rs",
    "active_subscription_lane_handle_no_authority.rs",
    "active_subscription_raw_activation_forbidden.rs",
    "active_subscription_raw_bridge_declaration_forbidden.rs",
    "active_subscription_raw_cdc_delivery_forbidden.rs",
    "active_subscription_generic_handle_forbidden.rs",
    "active_subscription_shared_ack_frontier_forbidden.rs",
    "active_subscription_ack_without_receipt_forbidden.rs",
    "active_subscription_maintenance_delta_constructor_private.rs",
    "active_subscription_delivery_work_packet_required.rs",
    "active_subscription_delivery_batch_constructor_private.rs",
    "active_subscription_raw_fanout_width_forbidden.rs",
    "active_subscription_raw_delivery_window_width_forbidden.rs",
    "active_subscription_zero_delivery_window_width_forbidden.rs",
    "active_subscription_public_vec_patch_group_forbidden.rs",
    "active_subscription_dense_refresh_without_posture_forbidden.rs",
    "active_subscription_linear_scan_lookup_without_debt_forbidden.rs",
    "active_subscription_unbounded_heap_allocation_forbidden.rs",
    "active_subscription_preview_in_place_promotion_forbidden.rs",
    "active_subscription_preview_discard_without_closeout_forbidden.rs",
    "active_subscription_durable_checkpoint_forbidden.rs",
    "subscription_lifecycle_closeout_constructor_private.rs",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MilestoneNineTwoPerturbationClass {
    DetailLifecycleDelivery,
    EquivalentSharingFanout,
    GroupedMembershipDelivery,
    IdentityContinuationRemap,
    PreviewDiscardIsolation,
    PreviewPromotionBoundary,
    PerformanceReceiptPostureSensitive,
    ScaleSlopeWidthBounded,
    MaskedSharingDenied,
    RawCdcDeliveryDenied,
    RawBridgeInvalidationDenied,
    PreviewAuthoritativeSharingDenied,
    PreviewDiscardResidueDenied,
    DenseRefreshDenied,
    StoreBackedRestartDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineTwoFailureClass {
    ActiveLifecycleDenied,
    DeliveryDenied,
    PreviewIsolationDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCertificationBundle {
    pub query_digest: String,
    pub subscription_family_digest: String,
    pub subscription_declaration_digest: String,
    pub subscription_equivalence_digest: String,
    pub active_lane_digest: String,
    pub active_lane_handle_digest: String,
    pub active_lane_lookup_class_digest: String,
    pub subscription_budget_digest: String,
    pub subscription_performance_receipt_digest: String,
    pub consumer_attachment_digest: String,
    pub acknowledgement_frontier_digest: String,
    pub delivery_window_digest: String,
    pub maintenance_delta_digest: String,
    pub active_delivery_work_packet_digest: String,
    pub active_delivery_density_posture_digest: String,
    pub allocation_posture_digest: String,
    pub delivery_batch_digest: String,
    pub patch_group_digest: String,
    pub delivery_receipt_digest: String,
    pub continuation_digest: String,
    pub preview_isolation_digest: String,
    pub preview_residue_digest: String,
    pub policy_digest: String,
    pub tenant_basis_digest: String,
    pub relationship_proof_digest: String,
    pub view_shape_digest: String,
    pub basis_digest: String,
    pub bridge_declaration_digest: String,
    pub signal_strategy_digest: String,
    pub failure_digest: String,
    pub lifecycle_denial_digest: String,
    pub counter_snapshot: String,
    pub counter_evidence: Vec<String>,
    pub subscription_lifecycle_scale_slope_digest: String,
    pub compile_fail_boundary_digest: String,
    pub support_matrix_digest: String,
}

impl SubscriptionLifecycleCertificationBundle {
    pub(super) fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.subscription_family_digest.is_empty()
            && !self.subscription_declaration_digest.is_empty()
            && !self.subscription_equivalence_digest.is_empty()
            && !self.active_lane_digest.is_empty()
            && !self.active_lane_handle_digest.is_empty()
            && !self.active_lane_lookup_class_digest.is_empty()
            && !self.subscription_budget_digest.is_empty()
            && !self.subscription_performance_receipt_digest.is_empty()
            && !self.consumer_attachment_digest.is_empty()
            && !self.acknowledgement_frontier_digest.is_empty()
            && !self.delivery_window_digest.is_empty()
            && !self.maintenance_delta_digest.is_empty()
            && !self.active_delivery_work_packet_digest.is_empty()
            && !self.active_delivery_density_posture_digest.is_empty()
            && !self.allocation_posture_digest.is_empty()
            && !self.delivery_batch_digest.is_empty()
            && !self.patch_group_digest.is_empty()
            && !self.delivery_receipt_digest.is_empty()
            && !self.continuation_digest.is_empty()
            && !self.preview_isolation_digest.is_empty()
            && !self.preview_residue_digest.is_empty()
            && !self.policy_digest.is_empty()
            && !self.tenant_basis_digest.is_empty()
            && !self.relationship_proof_digest.is_empty()
            && !self.view_shape_digest.is_empty()
            && !self.basis_digest.is_empty()
            && !self.bridge_declaration_digest.is_empty()
            && !self.signal_strategy_digest.is_empty()
            && !self.counter_snapshot.is_empty()
            && !self.counter_evidence.is_empty()
            && !self.subscription_lifecycle_scale_slope_digest.is_empty()
            && !self.compile_fail_boundary_digest.is_empty()
            && !self.support_matrix_digest.is_empty()
    }

    pub(super) fn lifecycle_signature(&self) -> String {
        digest_parts(&[
            format!("subscription:{}", self.subscription_equivalence_digest),
            format!("lane:{}", self.active_lane_digest),
            format!("attachment:{}", self.consumer_attachment_digest),
            format!("window:{}", self.delivery_window_digest),
            format!("delta:{}", self.maintenance_delta_digest),
            format!("work_packet:{}", self.active_delivery_work_packet_digest),
            format!(
                "performance:{}",
                self.subscription_performance_receipt_digest
            ),
            format!("density:{}", self.active_delivery_density_posture_digest),
            format!("allocation:{}", self.allocation_posture_digest),
            format!("patch:{}", self.patch_group_digest),
            format!("continuation:{}", self.continuation_digest),
            format!("preview:{}", self.preview_isolation_digest),
            format!("residue:{}", self.preview_residue_digest),
            format!("policy:{}", self.policy_digest),
            format!("tenant:{}", self.tenant_basis_digest),
            format!("basis:{}", self.basis_digest),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineTwoRejectionBundle {
    pub failure_class: MilestoneNineTwoFailureClass,
    pub failure_kind: String,
    pub failure_digest: String,
    pub lifecycle_denial_digest: String,
    pub counter_snapshot: String,
}

pub type MilestoneNineTwoCertificationRow = CanonicalCertificationRow<
    MilestoneNineTwoPerturbationClass,
    SubscriptionLifecycleCertificationBundle,
>;
pub type MilestoneNineTwoRejectionRow = RejectionCertificationRow<
    MilestoneNineTwoPerturbationClass,
    SubscriptionLifecycleCertificationBundle,
    MilestoneNineTwoRejectionBundle,
>;
pub type MilestoneNineTwoCertificationMatrix = CertificationMatrix<
    MilestoneNineTwoPerturbationClass,
    SubscriptionLifecycleCertificationBundle,
    MilestoneNineTwoRejectionBundle,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineTwoCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub matrix: MilestoneNineTwoCertificationMatrix,
}

impl MilestoneNineTwoCertificationMatrix {
    pub fn into_milestone_nine_two_artifact(self) -> MilestoneNineTwoCertificationArtifact {
        MilestoneNineTwoCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest: digest_parts(&builders::bundle_digest_parts(&self)),
            coverage_matrix_digest: digest_parts(&builders::coverage_digest_parts(&self)),
            matrix: self,
        }
    }
}

pub struct MilestoneNineTwoCertificationAdapter;

impl MilestoneNineTwoCertificationAdapter {
    pub fn subscription_lifecycle_sharing_and_preview_certification_artifact(
    ) -> MilestoneNineTwoCertificationArtifact {
        Self::subscription_lifecycle_sharing_and_preview_parity_test()
            .into_milestone_nine_two_artifact()
    }

    pub fn subscription_lifecycle_sharing_and_preview_parity_test(
    ) -> MilestoneNineTwoCertificationMatrix {
        MilestoneNineTwoCertificationMatrix {
            suite_name: "Subscription Lifecycle Sharing And Preview Parity Test",
            rows: builders::canonical_rows(),
            rejection_rows: builders::rejection_rows(),
        }
    }
}
