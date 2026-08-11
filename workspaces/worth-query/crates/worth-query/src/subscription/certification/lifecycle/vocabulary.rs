use super::super::super::preview_isolation::{
    PreviewSubscriptionDiscardCloseout, PreviewSubscriptionIsolationArtifact,
    PreviewSubscriptionPromotionHandoff, PreviewSubscriptionResidueReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubscriptionLifecycleCertificationDenialKind {
    ActivationAdmissionMismatch,
    ActiveLaneSourceMismatch,
    AttachmentSourceMismatch,
    DeliveryWindowSourceMismatch,
    MaintenanceDeltaSourceMismatch,
    WorkPacketSourceMismatch,
    DeliveryBatchSourceMismatch,
    ContinuationSourceMismatch,
    PreviewEvidenceMissing,
    PreviewSourceMismatch,
    CloseoutSourceMismatch,
}

impl SubscriptionLifecycleCertificationDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActivationAdmissionMismatch => "activation_admission_mismatch",
            Self::ActiveLaneSourceMismatch => "active_lane_source_mismatch",
            Self::AttachmentSourceMismatch => "attachment_source_mismatch",
            Self::DeliveryWindowSourceMismatch => "delivery_window_source_mismatch",
            Self::MaintenanceDeltaSourceMismatch => "maintenance_delta_source_mismatch",
            Self::WorkPacketSourceMismatch => "work_packet_source_mismatch",
            Self::DeliveryBatchSourceMismatch => "delivery_batch_source_mismatch",
            Self::ContinuationSourceMismatch => "continuation_source_mismatch",
            Self::PreviewEvidenceMissing => "preview_evidence_missing",
            Self::PreviewSourceMismatch => "preview_source_mismatch",
            Self::CloseoutSourceMismatch => "closeout_source_mismatch",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SubscriptionLifecyclePreviewCertification<'a> {
    None,
    Discard {
        isolation: &'a PreviewSubscriptionIsolationArtifact,
        residue_report: &'a PreviewSubscriptionResidueReport,
        discard_closeout: &'a PreviewSubscriptionDiscardCloseout,
    },
    Promotion {
        isolation: &'a PreviewSubscriptionIsolationArtifact,
        residue_report: &'a PreviewSubscriptionResidueReport,
        promotion_handoff: &'a PreviewSubscriptionPromotionHandoff,
    },
}
