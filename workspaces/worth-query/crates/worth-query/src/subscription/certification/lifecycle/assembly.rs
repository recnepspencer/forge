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
use super::bundle::SubscriptionLifecycleCertificationBundle;
use super::context::SubscriptionLifecycleCertificationContext;
use super::error::SubscriptionLifecycleCertificationError;
use super::identities::assemble_lifecycle_bundle;
use super::inputs::LifecycleCertificationInputs;
use super::validation::validate_lifecycle_sources;
use super::vocabulary::SubscriptionLifecyclePreviewCertification;
use crate::evidence_identity::WorthQueryEvidenceIdentity;

#[allow(clippy::too_many_arguments)]
pub fn certify_subscription_lifecycle(
    context: SubscriptionLifecycleCertificationContext,
    admission: &QuerySubscriptionAdmissionArtifact,
    activation: &SubscriptionActivationInput,
    scale_report: &QuerySubscriptionScaleSlopeReport,
    active_admission: &ActiveSubscriptionLaneAdmission,
    active_lane_handle: &ActiveSubscriptionLaneHandle,
    attachment: &SubscriptionConsumerAttachment,
    delivery_window_identity: &WorthQueryEvidenceIdentity,
    maintenance_delta: &QuerySubscriptionMaintenanceDelta,
    lowering_report: &QueryMaintenanceDeltaLoweringReport,
    work_packet: &ActiveDeliveryWorkPacket,
    delivery_batch: &QueryDeliveryBatch,
    acknowledged_attachment: &SubscriptionConsumerAttachment,
    continuation: Option<&SubscriptionContinuationReport>,
    preview: SubscriptionLifecyclePreviewCertification<'_>,
    lifecycle_closeout: &SubscriptionLifecycleCloseout,
) -> Result<SubscriptionLifecycleCertificationBundle, SubscriptionLifecycleCertificationError> {
    let inputs = LifecycleCertificationInputs {
        context: &context,
        admission,
        activation,
        scale_report,
        active_admission,
        active_lane_handle,
        attachment,
        delivery_window_identity,
        maintenance_delta,
        lowering_report,
        work_packet,
        delivery_batch,
        acknowledged_attachment,
        continuation,
        preview,
        lifecycle_closeout,
    };
    let validated = validate_lifecycle_sources(&inputs)?;
    assemble_lifecycle_bundle(&inputs, validated)
}
