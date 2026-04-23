use crate::identity::hash_parts;

use super::acknowledgement::{QueryDeliveryBatchReceipt, QueryDeliverySequence};
use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::attachment::SubscriptionConsumerAttachment;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::continuation::SubscriptionContinuationReport;
use super::delivery_budget::QueryDeliveryWindowBudget;
use super::delivery_error::{QueryDeliveryDenialKind, QueryDeliveryError};
use super::delivery_work_packet::ActiveDeliveryWorkPacket;
use super::maintenance_delta::{
    QueryMaintenanceDeltaLoweringReport, QuerySubscriptionMaintenanceDelta,
};
use super::patch_group::{QueryPatchGroup, QueryPatchGroupKind};

#[derive(Debug, Eq, PartialEq)]
pub struct QueryDeliveryWindow {
    delivery_window_digest: String,
    active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    next_sequence: QueryDeliverySequence,
    delivery_window_width: u64,
    patch_group_width: u64,
    maintenance_delta_width: u64,
    allocation_scope_width: u64,
    allocation_posture: ActiveSubscriptionAllocationPosture,
}

impl QueryDeliveryWindow {
    pub(super) fn new(
        attachment: &SubscriptionConsumerAttachment,
        budget: QueryDeliveryWindowBudget,
    ) -> Result<(Self, ActiveSubscriptionCounters), QueryDeliveryError> {
        let mut counters = ActiveSubscriptionCounters::default();
        if budget.exceeds_phase_three_budget() {
            counters.delivery_window_overflow_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::DeliveryWindowBudgetExceeded,
                "delivery window exceeds its explicit Phase 3 budget",
                attachment.attachment_digest().as_str(),
                counters,
            ));
        }
        if budget.forbidden_allocation_posture() {
            counters.heap_allocation_denial_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::AllocationPostureForbidden,
                "delivery windows require delivery-window allocation posture",
                attachment.attachment_digest().as_str(),
                counters,
            ));
        }

        let delivery_window_digest = hash_parts(&[
            "query_delivery_window_v1".to_string(),
            format!("lane:{}", attachment.lane_digest().as_str()),
            format!("attachment:{}", attachment.attachment_digest().as_str()),
            format!("sequence:{}", attachment.next_delivery_sequence().get()),
            format!("window_width:{}", budget.delivery_window_width()),
            format!("patch_width:{}", budget.patch_group_width()),
            format!("allocation_width:{}", budget.allocation_scope_width()),
            format!(
                "allocation_posture:{}",
                budget.allocation_posture().as_str()
            ),
            format!("backpressure:{}", budget.backpressure_policy().as_str()),
        ]);
        counters.delivery_window_open_count = 1;
        counters.delivery_window_width = budget.delivery_window_width();
        counters.patch_group_width = budget.patch_group_width();
        if budget.allocation_posture().is_heap_debt() {
            counters.heap_allocation_debt_count = 1;
        }

        Ok((
            Self {
                delivery_window_digest,
                active_lane_digest: attachment.lane_digest().clone(),
                attachment_digest: attachment.attachment_digest().clone(),
                next_sequence: attachment.next_delivery_sequence(),
                delivery_window_width: budget.delivery_window_width(),
                patch_group_width: budget.patch_group_width(),
                maintenance_delta_width: budget.maintenance_delta_width(),
                allocation_scope_width: budget.allocation_scope_width(),
                allocation_posture: budget.allocation_posture(),
            },
            counters,
        ))
    }

    pub fn delivery_window_digest(&self) -> &str {
        &self.delivery_window_digest
    }

    pub fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn next_sequence(&self) -> QueryDeliverySequence {
        self.next_sequence
    }

    pub fn patch_group_width(&self) -> u64 {
        self.patch_group_width
    }

    pub fn maintenance_delta_width(&self) -> u64 {
        self.maintenance_delta_width
    }

    pub fn delivery_window_width(&self) -> u64 {
        self.delivery_window_width
    }

    pub fn allocation_scope_width(&self) -> u64 {
        self.allocation_scope_width
    }

    pub fn allocation_posture(&self) -> ActiveSubscriptionAllocationPosture {
        self.allocation_posture
    }

    pub(super) fn apply_continuation(self, report: &SubscriptionContinuationReport) -> Self {
        let delivery_window_digest = hash_parts(&[
            "query_delivery_window_continuation_v1".to_string(),
            format!("window:{}", self.delivery_window_digest),
            format!("continuation:{}", report.continuation_digest()),
            format!("report:{}", report.report_digest()),
            format!("remap_width:{}", report.remap_width()),
        ]);
        Self {
            delivery_window_digest,
            ..self
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueryDeliveryBatch {
    delivery_batch_digest: String,
    delivery_window_digest: String,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    sequence: QueryDeliverySequence,
    patch_group: QueryPatchGroup,
    receipt: QueryDeliveryBatchReceipt,
    counters: ActiveSubscriptionCounters,
}

impl QueryDeliveryBatch {
    pub(super) fn new(
        window: QueryDeliveryWindow,
        work_packet: ActiveDeliveryWorkPacket,
    ) -> Result<Self, QueryDeliveryError> {
        let mut counters = ActiveSubscriptionCounters::default();
        if window.active_lane_digest() != work_packet.active_lane_digest()
            || window.attachment_digest() != work_packet.attachment_digest()
        {
            counters.delivery_window_overflow_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::WorkPacketWindowMismatch,
                "delivery work packet must target the delivery window lane and attachment",
                work_packet.work_packet_digest(),
                counters,
            ));
        }
        if work_packet.patch_group_width() > window.patch_group_width()
            || work_packet.maintenance_delta().width() > window.maintenance_delta_width()
        {
            counters.delivery_window_overflow_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::DeliveryWindowBudgetExceeded,
                "delivery work packet exceeds the opened delivery window budget",
                work_packet.work_packet_digest(),
                counters,
            ));
        }

        let patch_group = QueryPatchGroup::new(
            QueryPatchGroupKind::from_delta_kind(work_packet.maintenance_delta().kind()),
            work_packet.maintenance_delta().maintenance_delta_digest(),
            work_packet.patch_group_width(),
        );
        let receipt = QueryDeliveryBatchReceipt::new(
            window.attachment_digest().clone(),
            window.next_sequence(),
        );
        let delivery_batch_digest = hash_parts(&[
            "query_delivery_batch_v1".to_string(),
            format!("window:{}", window.delivery_window_digest()),
            format!("work_packet:{}", work_packet.work_packet_digest()),
            format!("patch_group:{}", patch_group.patch_group_digest()),
            format!("receipt:{}", receipt.receipt_digest()),
            format!(
                "performance:{}",
                work_packet
                    .performance_receipt()
                    .performance_receipt_digest()
            ),
            format!(
                "posture:{}",
                work_packet.maintenance_delta().delivery_posture().as_str()
            ),
        ]);

        counters.delivery_batch_count = 1;
        counters.fanout_delivery_count = 1;
        counters.patch_group_count = 1;
        counters.patch_group_width = patch_group.width();
        match patch_group.kind() {
            QueryPatchGroupKind::DetailFieldPatchGroup => {
                counters.detail_field_patch_width = patch_group.width()
            }
            QueryPatchGroupKind::InspectorFocusedPatchGroup => {
                counters.focused_inspector_patch_width = patch_group.width()
            }
            QueryPatchGroupKind::CollectionMembershipPatchGroup => {
                counters.collection_membership_patch_width = patch_group.width()
            }
            QueryPatchGroupKind::CollectionOrderPatchGroup => {
                counters.collection_order_patch_width = patch_group.width()
            }
            QueryPatchGroupKind::GroupedMembershipPatchGroup => {
                counters.grouped_membership_patch_width = patch_group.width()
            }
            QueryPatchGroupKind::BoundedMaterializationScopePatchGroup => {
                counters.bounded_materialization_scope_patch_width = patch_group.width()
            }
            QueryPatchGroupKind::ContinuationPatchGroup => {
                counters.continuation_remap_count = patch_group.width()
            }
            QueryPatchGroupKind::DeliveryGapPatchGroup => {
                counters.delivery_gap_notice_count = 1;
            }
        }

        Ok(Self {
            delivery_batch_digest,
            delivery_window_digest: window.delivery_window_digest,
            attachment_digest: window.attachment_digest,
            sequence: window.next_sequence,
            patch_group,
            receipt,
            counters,
        })
    }

    pub fn delivery_batch_digest(&self) -> &str {
        &self.delivery_batch_digest
    }

    pub fn delivery_window_digest(&self) -> &str {
        &self.delivery_window_digest
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn sequence(&self) -> QueryDeliverySequence {
        self.sequence
    }

    pub fn patch_group(&self) -> &QueryPatchGroup {
        &self.patch_group
    }

    pub fn receipt(&self) -> &QueryDeliveryBatchReceipt {
        &self.receipt
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }
}

pub fn lower_query_subscription_maintenance_delta(
    delta: QuerySubscriptionMaintenanceDelta,
) -> Result<
    (
        QuerySubscriptionMaintenanceDelta,
        QueryMaintenanceDeltaLoweringReport,
        ActiveSubscriptionCounters,
    ),
    QueryDeliveryError,
> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.maintenance_delta_lowering_count = 1;
    counters.maintenance_delta_width = delta.width();
    let report = QueryMaintenanceDeltaLoweringReport::new(&delta);
    Ok((delta, report, counters))
}

pub fn deny_raw_cdc_delivery_fallback(
    source_digest: impl Into<String>,
) -> Result<QuerySubscriptionMaintenanceDelta, QueryDeliveryError> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.raw_cdc_delivery_denial_count = 1;
    Err(QueryDeliveryError::new(
        QueryDeliveryDenialKind::RawCdcFallbackDenied,
        "raw CDC cannot be consumed as active query delivery",
        source_digest,
        counters,
    ))
}

pub fn deny_raw_bridge_invalidation_delivery(
    source_digest: impl Into<String>,
) -> Result<QuerySubscriptionMaintenanceDelta, QueryDeliveryError> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.raw_bridge_invalidation_denial_count = 1;
    Err(QueryDeliveryError::new(
        QueryDeliveryDenialKind::RawBridgeInvalidationDenied,
        "raw bridge invalidation must lower into a query maintenance delta first",
        source_digest,
        counters,
    ))
}
