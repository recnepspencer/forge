use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::identity_authority::{
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCertificationBundle {
    pub(super) certification_bundle_authority: QuerySubscriptionAuthorityIdentity<
        WorthQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    >,
    pub(in crate::subscription) query_scope_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_family_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_declaration_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_equivalence_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) admission_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) active_lane_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) active_lane_handle_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) active_lane_lookup_class_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_budget_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_performance_receipt_identity:
        WorthQueryEvidenceIdentity,
    pub(in crate::subscription) consumer_attachment_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) acknowledgement_frontier_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) delivery_window_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) maintenance_delta_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) active_delivery_work_packet_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) active_delivery_density_posture_identity:
        WorthQueryEvidenceIdentity,
    pub(in crate::subscription) allocation_posture_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) delivery_batch_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) patch_group_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) delivery_receipt_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) continuation_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) preview_isolation_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) preview_residue_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) policy_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) tenant_basis_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) relationship_proof_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) view_shape_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) basis_posture_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) bridge_declaration_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) signal_strategy_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) counter_sequence_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_lifecycle_scale_slope_identity:
        WorthQueryEvidenceIdentity,
    pub(in crate::subscription) support_matrix_identity: WorthQueryEvidenceIdentity,
}

impl SubscriptionLifecycleCertificationBundle {
    pub fn certification_bundle_identity(&self) -> &WorthQueryEvidenceIdentity {
        self.certification_bundle_authority.value()
    }

    pub fn certification_bundle_authority(
        &self,
    ) -> &QuerySubscriptionAuthorityIdentity<
        WorthQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    > {
        &self.certification_bundle_authority
    }

    pub fn query_scope_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_scope_identity
    }

    pub fn subscription_family_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subscription_family_identity
    }

    pub fn subscription_declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subscription_declaration_identity
    }

    pub fn admission_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn active_lane_handle_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.active_lane_handle_identity
    }

    pub fn active_lane_lookup_class_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.active_lane_lookup_class_identity
    }

    pub fn subscription_budget_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subscription_budget_identity
    }

    pub fn subscription_performance_receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subscription_performance_receipt_identity
    }

    pub fn active_delivery_density_posture_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.active_delivery_density_posture_identity
    }

    pub fn allocation_posture_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.allocation_posture_identity
    }

    pub fn continuation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.continuation_identity
    }

    pub fn policy_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.policy_identity
    }

    pub fn tenant_basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.tenant_basis_identity
    }

    pub fn relationship_proof_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.relationship_proof_identity
    }

    pub fn view_shape_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.view_shape_identity
    }

    pub fn basis_posture_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_posture_identity
    }

    pub fn bridge_declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn signal_strategy_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn counter_sequence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.counter_sequence_identity
    }

    pub fn support_matrix_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.support_matrix_identity
    }
}
