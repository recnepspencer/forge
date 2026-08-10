use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::super::super::active_counters::ActiveSubscriptionCounters;
use super::super::super::active_digest::ActiveSubscriptionLaneDigest;
use super::super::super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::super::super::evidence_projection::subscription_evidence_projection;
use super::super::super::future_selection::QuerySubscriptionFutureSelection;
use super::super::super::performance_receipt::SubscriptionPerformanceReceipt;
use super::PreviewSubscriptionDiscardCloseout;

impl PreviewSubscriptionDiscardCloseout {
    pub(crate) fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub(crate) fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.basis_binding_identity)
    }

    pub fn basis_binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn checkpoint_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.checkpoint_identity)
    }

    pub fn checkpoint_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn preview_epoch_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.preview_epoch_identity)
    }

    pub fn preview_epoch_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.preview_epoch_identity
    }

    pub fn residue_report_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.residue_report_identity)
    }

    pub fn residue_report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.residue_report_identity
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn closeout_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.closeout_identity)
    }

    pub fn closeout_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.closeout_identity
    }
}
