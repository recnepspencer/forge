use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::super::super::activation::SubscriptionActivationInput;
use super::super::super::active_handle::ActiveSubscriptionLaneHandle;
use super::super::super::active_lane::ActiveSubscriptionLaneAdmission;
use super::super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::super::attachment::SubscriptionConsumerAttachment;
use super::super::super::closeout::SubscriptionLifecycleCloseout;
use super::super::super::continuation::SubscriptionContinuationReport;
use super::super::super::delivery_window::QueryDeliveryBatch;
use super::super::super::delivery_work_packet::ActiveDeliveryWorkPacket;
use super::super::super::maintenance_delta::{
    QueryMaintenanceDeltaLoweringReport, QuerySubscriptionMaintenanceDelta,
};
use super::super::super::scale::QuerySubscriptionScaleSlopeReport;
use super::context::SubscriptionLifecycleCertificationContext;
use super::vocabulary::SubscriptionLifecyclePreviewCertification;

#[derive(Clone, Copy)]
pub(super) struct LifecycleCertificationInputs<'a> {
    pub(super) context: &'a SubscriptionLifecycleCertificationContext,
    pub(super) admission: &'a QuerySubscriptionAdmissionArtifact,
    pub(super) activation: &'a SubscriptionActivationInput,
    pub(super) scale_report: &'a QuerySubscriptionScaleSlopeReport,
    pub(super) active_admission: &'a ActiveSubscriptionLaneAdmission,
    pub(super) active_lane_handle: &'a ActiveSubscriptionLaneHandle,
    pub(super) attachment: &'a SubscriptionConsumerAttachment,
    pub(super) delivery_window_identity: &'a WorthQueryEvidenceIdentity,
    pub(super) maintenance_delta: &'a QuerySubscriptionMaintenanceDelta,
    pub(super) lowering_report: &'a QueryMaintenanceDeltaLoweringReport,
    pub(super) work_packet: &'a ActiveDeliveryWorkPacket,
    pub(super) delivery_batch: &'a QueryDeliveryBatch,
    pub(super) acknowledged_attachment: &'a SubscriptionConsumerAttachment,
    pub(super) continuation: Option<&'a SubscriptionContinuationReport>,
    pub(super) preview: SubscriptionLifecyclePreviewCertification<'a>,
    pub(super) lifecycle_closeout: &'a SubscriptionLifecycleCloseout,
}
