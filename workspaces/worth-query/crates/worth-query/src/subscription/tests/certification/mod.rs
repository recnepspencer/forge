use super::*;

struct LifecycleCertificationArtifacts {
    context: SubscriptionLifecycleCertificationContext,
    admission: QuerySubscriptionAdmissionArtifact,
    activation: SubscriptionActivationInput,
    scale_report: QuerySubscriptionScaleSlopeReport,
    active_admission: ActiveSubscriptionLaneAdmission,
    handle: ActiveSubscriptionLaneHandle,
    attachment: SubscriptionConsumerAttachment,
    delta: QuerySubscriptionMaintenanceDelta,
    lowering_report: QueryMaintenanceDeltaLoweringReport,
    work_packet: ActiveDeliveryWorkPacket,
    delivery_batch: QueryDeliveryBatch,
    acknowledged_attachment: SubscriptionConsumerAttachment,
    continuation_report: Option<SubscriptionContinuationReport>,
    preview: SubscriptionLifecyclePreviewCertificationArtifacts,
    closeout: SubscriptionLifecycleCloseout,
}

enum SubscriptionLifecyclePreviewCertificationArtifacts {
    None,
    Discard {
        isolation: PreviewSubscriptionIsolationArtifact,
        residue_report: PreviewSubscriptionResidueReport,
        discard_closeout: PreviewSubscriptionDiscardCloseout,
    },
    Promotion {
        isolation: PreviewSubscriptionIsolationArtifact,
        residue_report: PreviewSubscriptionResidueReport,
        promotion_handoff: PreviewSubscriptionPromotionHandoff,
    },
}

mod activation;
mod activation_world;
mod lifecycle;
mod lifecycle_world;
mod preview;
mod preview_certification_world;
mod preview_discard_world;
mod preview_promotion_world;
