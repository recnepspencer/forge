use super::acknowledgement::QueryDeliveryBatchReceipt;
use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::active_counters::ActiveSubscriptionCounters;
use super::active_dimensions::ActiveAllocationScopeWidth;
use super::active_error::ActiveSubscriptionLifecycleError;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::active_lane::ActiveSubscriptionLaneAdmission;
use super::active_posture::ActiveSubscriptionLifecyclePosture;
use super::active_registry::ActiveSubscriptionLaneRegistry;
use super::attachment::SubscriptionConsumerAttachment;
use super::attachment_budget::SubscriptionConsumerAttachmentBudget;
use super::attachment_error::{
    SubscriptionConsumerAttachmentDenialKind, SubscriptionConsumerAttachmentError,
};
use super::attachment_request::SubscriptionConsumerAttachmentRequest;
use super::closeout::{
    SubscriptionLifecycleCloseDenialKind, SubscriptionLifecycleCloseError,
    SubscriptionLifecycleCloseRequest, SubscriptionLifecycleCloseout,
};
use super::continuation::{
    apply_subscription_continuation, SubscriptionContinuationEvidence,
    SubscriptionContinuationReport,
};
use super::continuation_error::SubscriptionContinuationError;
use super::delivery_budget::QueryDeliveryWindowBudget;
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::delivery_dimensions::{
    ActiveDeliveryAffectedAttachmentWidth, ActiveDeliveryAffectedLaneWidth,
    ActiveDeliveryContinuationWidth, ActiveDeliveryPreviewResidueWidth, PatchGroupWidth,
};
use super::delivery_error::QueryDeliveryError;
use super::delivery_window::{QueryDeliveryBatch, QueryDeliveryWindow};
use super::delivery_work_packet::ActiveDeliveryWorkPacket;
use super::maintenance_delta::{
    QueryMaintenanceDeltaLoweringReport, QuerySubscriptionMaintenanceDelta,
};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ActiveSubscriptionRuntime {
    registry: ActiveSubscriptionLaneRegistry,
    counters: ActiveSubscriptionCounters,
    next_attachment_index: u64,
}

impl ActiveSubscriptionRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lane_count(&self) -> usize {
        self.registry.lane_count()
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }

    pub fn lane_lifecycle_posture(
        &self,
        handle: &ActiveSubscriptionLaneHandle,
    ) -> Option<&ActiveSubscriptionLifecyclePosture> {
        self.registry.lane_lifecycle_posture(handle)
    }

    fn attachment_width_after_attach(&self) -> u64 {
        self.next_attachment_index + 1
    }
}

pub fn open_active_subscription_lane(
    runtime: &mut ActiveSubscriptionRuntime,
    admission: ActiveSubscriptionLaneAdmission,
) -> Result<ActiveSubscriptionLaneHandle, ActiveSubscriptionLifecycleError> {
    let (handle, counters) = runtime.registry.open_lane(admission)?;
    runtime.counters = counters;
    Ok(handle)
}

pub fn join_active_subscription_lane(
    runtime: &mut ActiveSubscriptionRuntime,
    handle: &ActiveSubscriptionLaneHandle,
    admission: ActiveSubscriptionLaneAdmission,
) -> Result<ActiveSubscriptionLaneHandle, ActiveSubscriptionLifecycleError> {
    let (handle, counters) = runtime.registry.join_lane(handle, admission)?;
    runtime.counters = counters;
    Ok(handle)
}

pub fn attach_subscription_consumer(
    runtime: &mut ActiveSubscriptionRuntime,
    handle: &ActiveSubscriptionLaneHandle,
    request: SubscriptionConsumerAttachmentRequest,
    budget: SubscriptionConsumerAttachmentBudget,
) -> Result<SubscriptionConsumerAttachment, SubscriptionConsumerAttachmentError> {
    runtime.registry.validate_handle(handle).map_err(|error| {
        SubscriptionConsumerAttachmentError::new(
            SubscriptionConsumerAttachmentDenialKind::LaneHandleMismatch,
            error.message(),
            error.source_digest(),
            error.counters().clone(),
        )
    })?;
    let attachment_index = runtime.next_attachment_index;
    let (attachment, counters) = SubscriptionConsumerAttachment::new(
        handle,
        request,
        budget,
        attachment_index,
        runtime.attachment_width_after_attach(),
    )?;
    runtime
        .registry
        .register_attachment(handle, attachment.attachment_digest().as_str())
        .map_err(|error| {
            SubscriptionConsumerAttachmentError::new(
                SubscriptionConsumerAttachmentDenialKind::LaneHandleMismatch,
                error.message(),
                error.source_digest(),
                error.counters().clone(),
            )
        })?;
    runtime.next_attachment_index += 1;
    runtime.counters = counters;
    Ok(attachment)
}

pub fn open_query_delivery_window(
    runtime: &mut ActiveSubscriptionRuntime,
    attachment: &SubscriptionConsumerAttachment,
    budget: QueryDeliveryWindowBudget,
) -> Result<QueryDeliveryWindow, QueryDeliveryError> {
    let (window, counters) = QueryDeliveryWindow::new(attachment, budget)?;
    runtime.counters = counters;
    Ok(window)
}

#[allow(clippy::too_many_arguments)]
pub fn build_active_delivery_work_packet(
    runtime: &mut ActiveSubscriptionRuntime,
    attachment: &SubscriptionConsumerAttachment,
    delta: QuerySubscriptionMaintenanceDelta,
    lowering_report: QueryMaintenanceDeltaLoweringReport,
    density_posture: ActiveDeliveryDensityPosture,
    affected_lane_width: ActiveDeliveryAffectedLaneWidth,
    affected_attachment_width: ActiveDeliveryAffectedAttachmentWidth,
    patch_group_width: PatchGroupWidth,
    continuation_width: ActiveDeliveryContinuationWidth,
    preview_residue_width: ActiveDeliveryPreviewResidueWidth,
    allocation_scope_width: ActiveAllocationScopeWidth,
    allocation_posture: ActiveSubscriptionAllocationPosture,
) -> Result<ActiveDeliveryWorkPacket, QueryDeliveryError> {
    let (packet, counters) = ActiveDeliveryWorkPacket::new(
        attachment.lane_digest().clone(),
        attachment.attachment_digest().clone(),
        delta,
        lowering_report,
        density_posture,
        affected_lane_width,
        affected_attachment_width,
        patch_group_width,
        continuation_width,
        preview_residue_width,
        allocation_scope_width,
        allocation_posture,
    )?;
    runtime.counters = counters;
    Ok(packet)
}

pub fn emit_query_delivery_batch(
    runtime: &mut ActiveSubscriptionRuntime,
    window: QueryDeliveryWindow,
    work_packet: ActiveDeliveryWorkPacket,
) -> Result<QueryDeliveryBatch, QueryDeliveryError> {
    let batch = QueryDeliveryBatch::new(window, work_packet)?;
    runtime.counters = batch.counters().clone();
    Ok(batch)
}

pub fn apply_active_subscription_continuation(
    runtime: &mut ActiveSubscriptionRuntime,
    window: QueryDeliveryWindow,
    evidence: SubscriptionContinuationEvidence,
) -> Result<(QueryDeliveryWindow, SubscriptionContinuationReport), SubscriptionContinuationError> {
    let (window, report) = apply_subscription_continuation(window, evidence)?;
    let mut counters = ActiveSubscriptionCounters::default();
    counters.continuation_remap_width = report.remap_width();
    runtime.counters = counters;
    Ok((window, report))
}

pub fn advance_subscription_acknowledgement(
    runtime: &mut ActiveSubscriptionRuntime,
    attachment: SubscriptionConsumerAttachment,
    receipt: QueryDeliveryBatchReceipt,
) -> Result<SubscriptionConsumerAttachment, SubscriptionConsumerAttachmentError> {
    let (attachment, counters) = attachment.advance_acknowledgement(receipt)?;
    runtime.counters = counters;
    Ok(attachment)
}

pub fn close_subscription_lifecycle(
    runtime: &mut ActiveSubscriptionRuntime,
    handle: &ActiveSubscriptionLaneHandle,
    request: SubscriptionLifecycleCloseRequest,
) -> Result<SubscriptionLifecycleCloseout, SubscriptionLifecycleCloseError> {
    runtime.registry.validate_handle(handle).map_err(|error| {
        let mut counters = error.counters().clone();
        counters.subscription_lifecycle_closeout_denial_count = 1;
        SubscriptionLifecycleCloseError::new(
            SubscriptionLifecycleCloseDenialKind::LaneHandleMismatch,
            error.message(),
            error.source_digest(),
            counters,
        )
    })?;
    if request.lane_digest() != handle.lane_digest() {
        let mut counters = ActiveSubscriptionCounters::default();
        counters.subscription_lifecycle_closeout_denial_count = 1;
        return Err(SubscriptionLifecycleCloseError::new(
            SubscriptionLifecycleCloseDenialKind::AttachmentLaneMismatch,
            "subscription lifecycle closeout request must match the selected active lane handle",
            request.attachment_digest().as_str(),
            counters,
        ));
    }
    let lane_terminal = runtime
        .registry
        .close_attachment(handle, request.attachment_digest().as_str())
        .map_err(|error| {
            let denial_kind = if error.source_digest() == request.attachment_digest().as_str() {
                SubscriptionLifecycleCloseDenialKind::AttachmentNotActive
            } else {
                SubscriptionLifecycleCloseDenialKind::LaneHandleMismatch
            };
            let mut counters = error.counters().clone();
            counters.subscription_lifecycle_closeout_denial_count = 1;
            SubscriptionLifecycleCloseError::new(
                denial_kind,
                error.message(),
                error.source_digest(),
                counters,
            )
        })?;
    let closeout = SubscriptionLifecycleCloseout::new(request, lane_terminal);
    runtime.counters = closeout.counters().clone();
    Ok(closeout)
}
