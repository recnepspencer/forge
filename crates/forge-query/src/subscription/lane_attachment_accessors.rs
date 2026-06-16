use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::acknowledgement::{QueryDeliveryBatchReceipt, SubscriptionAcknowledgementFrontier};
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::active_lane::{ActiveSubscriptionLane, ActiveSubscriptionLaneAdmission};
use super::attachment::SubscriptionConsumerAttachment;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::closeout::SubscriptionLifecycleCloseout;
use super::continuation::{SubscriptionContinuationEvidence, SubscriptionContinuationReport};
use super::delivery_window::{QueryDeliveryBatch, QueryDeliveryWindow};
use super::delivery_work_packet::ActiveDeliveryWorkPacket;
use super::evidence_projection::subscription_evidence_projection;
use super::fanout::SubscriptionFanoutPlan;
use super::maintenance_delta::QuerySubscriptionMaintenanceDelta;
use super::preview_closeout::{
    PreviewSubscriptionDiscardCloseout, PreviewSubscriptionPromotionHandoff,
};
use super::preview_isolation::PreviewSubscriptionIsolationArtifact;

impl ActiveSubscriptionLaneDigest {
    pub fn lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.evidence_identity())
    }

    pub fn lane_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.evidence_identity()
    }
}

impl SubscriptionConsumerAttachmentDigest {
    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.evidence_identity())
    }

    pub fn attachment_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.evidence_identity()
    }
}

impl ActiveSubscriptionLaneHandle {
    pub fn lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.lane_digest().lane_projection()
    }

    pub fn lane_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.lane_digest().evidence_identity()
    }
}

impl SubscriptionConsumerAttachment {
    pub fn lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.lane_digest().lane_projection()
    }

    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }

    pub fn lane_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.lane_digest().evidence_identity()
    }

    pub fn attachment_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.attachment_digest().evidence_identity()
    }
}

impl ActiveSubscriptionLaneAdmission {
    pub fn lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.lane_digest().lane_projection()
    }

    pub fn lane_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.lane_digest().evidence_identity()
    }
}

impl ActiveSubscriptionLane {
    pub fn lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.lane_digest().lane_projection()
    }

    pub fn lane_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.lane_digest().evidence_identity()
    }
}

impl QueryDeliveryWindow {
    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.active_lane_digest().lane_projection()
    }

    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl QueryDeliveryBatch {
    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl QueryDeliveryBatchReceipt {
    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl ActiveDeliveryWorkPacket {
    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.active_lane_digest().lane_projection()
    }

    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl QuerySubscriptionMaintenanceDelta {
    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.active_lane_digest().lane_projection()
    }
}

impl SubscriptionLifecycleCloseout {
    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.active_lane_digest().lane_projection()
    }

    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl PreviewSubscriptionDiscardCloseout {
    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.active_lane_digest().lane_projection()
    }

    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl PreviewSubscriptionPromotionHandoff {
    pub fn preview_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.preview_lane_digest().lane_projection()
    }

    pub fn authoritative_active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.authoritative_active_lane_digest().lane_projection()
    }

    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl PreviewSubscriptionIsolationArtifact {
    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.active_lane_digest().lane_projection()
    }

    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl SubscriptionContinuationEvidence {
    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.active_lane_digest().lane_projection()
    }
}

impl SubscriptionContinuationReport {
    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.active_lane_digest().lane_projection()
    }
}

impl SubscriptionAcknowledgementFrontier {
    pub fn attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.attachment_digest().attachment_projection()
    }
}

impl SubscriptionFanoutPlan {
    pub fn lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        self.lane_digest().lane_projection()
    }
}
