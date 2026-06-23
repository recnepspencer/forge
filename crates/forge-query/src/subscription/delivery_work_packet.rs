use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::delivery_dimensions::{
    ActiveDeliveryAffectedAttachmentWidth, ActiveDeliveryAffectedLaneWidth,
    ActiveDeliveryContinuationWidth, ActiveDeliveryPreviewResidueWidth, PatchGroupWidth,
};
use super::delivery_error::{QueryDeliveryDenialKind, QueryDeliveryError};
use super::evidence_identities::{lifecycle_work_packet_identity, typed_identity_drift};
use super::maintenance_delta::{
    QueryMaintenanceDeltaLoweringReport, QuerySubscriptionMaintenanceDelta,
};
use super::performance_receipt::SubscriptionPerformanceReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveDeliveryWorkPacket {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    maintenance_delta: QuerySubscriptionMaintenanceDelta,
    lowering_report: QueryMaintenanceDeltaLoweringReport,
    density_posture: ActiveDeliveryDensityPosture,
    affected_lane_width: u64,
    affected_attachment_width: u64,
    patch_group_width: u64,
    continuation_width: u64,
    preview_residue_width: u64,
    allocation_scope_width: u64,
    allocation_posture: ActiveSubscriptionAllocationPosture,
    performance_receipt: SubscriptionPerformanceReceipt,
    work_packet_identity: ForgeQueryEvidenceIdentity,
}

impl ActiveDeliveryWorkPacket {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        active_lane_digest: ActiveSubscriptionLaneDigest,
        attachment_digest: SubscriptionConsumerAttachmentDigest,
        maintenance_delta: QuerySubscriptionMaintenanceDelta,
        lowering_report: QueryMaintenanceDeltaLoweringReport,
        density_posture: ActiveDeliveryDensityPosture,
        affected_lane_width: ActiveDeliveryAffectedLaneWidth,
        affected_attachment_width: ActiveDeliveryAffectedAttachmentWidth,
        patch_group_width: PatchGroupWidth,
        continuation_width: ActiveDeliveryContinuationWidth,
        preview_residue_width: ActiveDeliveryPreviewResidueWidth,
        allocation_scope_width: super::active_dimensions::ActiveAllocationScopeWidth,
        allocation_posture: ActiveSubscriptionAllocationPosture,
    ) -> Result<(Self, ActiveSubscriptionCounters), QueryDeliveryError> {
        let mut counters = ActiveSubscriptionCounters::default();
        if allocation_posture.is_heap_denied() || !allocation_posture.admits_patch_scratch_phase() {
            counters.heap_allocation_denial_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::AllocationPostureForbidden,
                "active delivery work packets require patch-scratch allocation posture",
                maintenance_delta.evidence_identity().clone(),
                counters,
            ));
        }
        if density_posture == ActiveDeliveryDensityPosture::DenseRefreshDenied {
            counters.active_delivery_density_dense_denial_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::DenseRefreshDenied,
                "dense refresh delivery must be explicit debt or typed denial",
                maintenance_delta.evidence_identity().clone(),
                counters,
            ));
        }
        if typed_identity_drift(
            maintenance_delta.active_lane_digest().evidence_identity(),
            active_lane_digest.evidence_identity(),
        ) || typed_identity_drift(
            lowering_report.maintenance_delta_identity(),
            maintenance_delta.evidence_identity(),
        ) {
            counters.delivery_window_overflow_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::WorkPacketDeltaMismatch,
                "active delivery work packet must consume a lowered delta for the target lane",
                maintenance_delta.evidence_identity().clone(),
                counters,
            ));
        }

        let affected_lane_width = affected_lane_width.get();
        let affected_attachment_width = affected_attachment_width.get();
        let patch_group_width = patch_group_width.get();
        let continuation_width = continuation_width.get();
        let preview_residue_width = preview_residue_width.get();
        let allocation_scope_width = allocation_scope_width.get();
        let consumed_width = affected_lane_width
            + affected_attachment_width
            + patch_group_width
            + continuation_width
            + preview_residue_width
            + allocation_scope_width;
        let budgeted_width = consumed_width;
        let performance_receipt = SubscriptionPerformanceReceipt::new(
            consumed_width,
            budgeted_width,
            density_posture,
            allocation_posture,
            maintenance_delta.evidence_identity(),
        );
        let work_packet_identity = lifecycle_work_packet_identity(
            active_lane_digest.evidence_identity(),
            attachment_digest.evidence_identity(),
            maintenance_delta.evidence_identity(),
            lowering_report.evidence_identity(),
            density_posture.as_str(),
            affected_lane_width,
            affected_attachment_width,
            patch_group_width,
            continuation_width,
            preview_residue_width,
            allocation_scope_width,
            allocation_posture,
            performance_receipt.performance_receipt_identity(),
        );
        counters.active_delivery_work_packet_count = 1;
        counters.active_delivery_work_packet_width = consumed_width;
        match density_posture {
            ActiveDeliveryDensityPosture::SparseDelta => {
                counters.active_delivery_density_sparse_count = 1
            }
            ActiveDeliveryDensityPosture::BurstCoalesced => {
                counters.active_delivery_density_burst_coalesced_count = 1
            }
            ActiveDeliveryDensityPosture::DenseRefreshDebtExplicit => {
                counters.active_delivery_density_dense_debt_count = 1
            }
            ActiveDeliveryDensityPosture::DenseRefreshDenied => {}
        }
        counters.subscription_performance_receipt_count = 1;
        counters.subscription_budget_consumption_width = consumed_width;
        counters.subscription_budget_remaining_width = performance_receipt.remaining_width();
        if allocation_posture.is_heap_debt() {
            counters.heap_allocation_debt_count = 1;
        }

        Ok((
            Self {
                active_lane_digest,
                attachment_digest,
                maintenance_delta,
                lowering_report,
                density_posture,
                affected_lane_width,
                affected_attachment_width,
                patch_group_width,
                continuation_width,
                preview_residue_width,
                allocation_scope_width,
                allocation_posture,
                performance_receipt,
                work_packet_identity,
            },
            counters,
        ))
    }

    pub(crate) fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub(crate) fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn maintenance_delta(&self) -> &QuerySubscriptionMaintenanceDelta {
        &self.maintenance_delta
    }

    pub fn lowering_report(&self) -> &QueryMaintenanceDeltaLoweringReport {
        &self.lowering_report
    }

    pub fn density_posture(&self) -> ActiveDeliveryDensityPosture {
        self.density_posture
    }

    pub fn patch_group_width(&self) -> u64 {
        self.patch_group_width
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.work_packet_identity
    }

    pub fn affected_lane_width(&self) -> u64 {
        self.affected_lane_width
    }

    pub fn affected_attachment_width(&self) -> u64 {
        self.affected_attachment_width
    }

    pub fn continuation_width(&self) -> u64 {
        self.continuation_width
    }

    pub fn preview_residue_width(&self) -> u64 {
        self.preview_residue_width
    }

    pub fn allocation_scope_width(&self) -> u64 {
        self.allocation_scope_width
    }

    pub fn allocation_posture(&self) -> ActiveSubscriptionAllocationPosture {
        self.allocation_posture
    }
}
