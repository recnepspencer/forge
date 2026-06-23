use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::acknowledgement::{QueryDeliveryBatchReceipt, QueryDeliverySequence};
use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::attachment::SubscriptionConsumerAttachment;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::continuation::SubscriptionContinuationReport;
use super::delivery_budget::QueryDeliveryWindowBudget;
use super::delivery_cause::{QuerySubscriptionDeliveryCause, QuerySubscriptionDeliveryCauseKind};
use super::delivery_error::{QueryDeliveryDenialKind, QueryDeliveryError};
use super::delivery_work_packet::ActiveDeliveryWorkPacket;
use super::evidence_identities::{
    lifecycle_absent_performance_receipt_identity, lifecycle_absent_work_packet_identity,
    lifecycle_delivery_batch_identity, lifecycle_delivery_window_identity, typed_identity_drift,
};
use super::maintenance_delta::{
    QueryMaintenanceDeltaLoweringReport, QuerySubscriptionMaintenanceDelta,
};
use super::patch_group::{QueryPatchGroup, QueryPatchGroupKind};

#[derive(Debug, Eq, PartialEq)]
pub struct QueryDeliveryWindow {
    delivery_window_identity: ForgeQueryEvidenceIdentity,
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
                attachment.attachment_digest().evidence_identity().clone(),
                counters,
            ));
        }
        if budget.forbidden_allocation_posture() {
            counters.heap_allocation_denial_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::AllocationPostureForbidden,
                "delivery windows require delivery-window allocation posture",
                attachment.attachment_digest().evidence_identity().clone(),
                counters,
            ));
        }

        let delivery_window_identity = lifecycle_delivery_window_identity(
            attachment.lane_digest().evidence_identity(),
            attachment.attachment_digest().evidence_identity(),
            attachment.next_delivery_sequence().get(),
            budget.delivery_window_width(),
            budget.patch_group_width(),
            budget.allocation_scope_width(),
            budget.allocation_posture(),
            *budget.backpressure_policy(),
        );
        counters.delivery_window_open_count = 1;
        counters.delivery_window_width = budget.delivery_window_width();
        counters.patch_group_width = budget.patch_group_width();
        if budget.allocation_posture().is_heap_debt() {
            counters.heap_allocation_debt_count = 1;
        }

        Ok((
            Self {
                delivery_window_identity,
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

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_window_identity
    }

    pub(crate) fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub(crate) fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
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
        let delivery_window_identity = ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
            "query_delivery_window_continuation_v1",
        )
        .field_evidence_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("prior"),
            &self.delivery_window_identity,
        )
        .field_evidence_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("continuation"),
            report.evidence_identity(),
        )
        .field_usize(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("remap_width"),
            report.remap_width() as usize,
        )
        .seal();
        Self {
            delivery_window_identity,
            ..self
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueryDeliveryBatch {
    delivery_batch_identity: ForgeQueryEvidenceIdentity,
    delivery_window_identity: ForgeQueryEvidenceIdentity,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    sequence: QueryDeliverySequence,
    delivery_cause: QuerySubscriptionDeliveryCause,
    has_relational_patch: bool,
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
        if typed_identity_drift(
            window.active_lane_digest().evidence_identity(),
            work_packet.active_lane_digest().evidence_identity(),
        ) || typed_identity_drift(
            window.attachment_digest().evidence_identity(),
            work_packet.attachment_digest().evidence_identity(),
        ) {
            counters.delivery_window_overflow_count = 1;
            return Err(QueryDeliveryError::new(
                QueryDeliveryDenialKind::WorkPacketWindowMismatch,
                "delivery work packet must target the delivery window lane and attachment",
                work_packet.evidence_identity().clone(),
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
                work_packet.evidence_identity().clone(),
                counters,
            ));
        }

        let delivery_cause = QuerySubscriptionDeliveryCause::relational_patch(
            work_packet.maintenance_delta().evidence_identity(),
        );
        let patch_group = QueryPatchGroup::new(
            QueryPatchGroupKind::from_delta_kind(work_packet.maintenance_delta().kind()),
            work_packet.maintenance_delta().evidence_identity(),
            work_packet.patch_group_width(),
        );
        let receipt = QueryDeliveryBatchReceipt::new(
            window.attachment_digest().clone(),
            window.next_sequence(),
        );
        let delivery_batch_identity = lifecycle_delivery_batch_identity(
            window.evidence_identity(),
            work_packet.evidence_identity(),
            delivery_cause.delivery_cause_identity(),
            delivery_cause.has_relational_patch(),
            patch_group.patch_group_identity(),
            receipt.evidence_identity(),
            work_packet
                .performance_receipt()
                .performance_receipt_identity(),
            work_packet.maintenance_delta().delivery_posture().as_str(),
        );

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
            QueryPatchGroupKind::TimeOnlyDeliveryGroup
            | QueryPatchGroupKind::MixedCauseDeliveryGroup => {}
        }

        Ok(Self {
            delivery_batch_identity,
            delivery_window_identity: window.evidence_identity().clone(),
            attachment_digest: window.attachment_digest().clone(),
            sequence: window.next_sequence(),
            delivery_cause,
            has_relational_patch: true,
            patch_group,
            receipt,
            counters,
        })
    }

    #[allow(dead_code)]
    pub(super) fn new_time_only(
        window: QueryDeliveryWindow,
        delivery_cause: QuerySubscriptionDeliveryCause,
    ) -> Result<Self, QueryDeliveryError> {
        assert!(
            !delivery_cause.has_relational_patch(),
            "time-only delivery constructor must not consume relational patch causes"
        );
        let patch_group = QueryPatchGroup::new(
            QueryPatchGroupKind::TimeOnlyDeliveryGroup,
            delivery_cause.delivery_cause_identity(),
            0,
        );
        let receipt = QueryDeliveryBatchReceipt::new(
            window.attachment_digest().clone(),
            window.next_sequence(),
        );
        let absent_work_packet = lifecycle_absent_work_packet_identity();
        let absent_performance = lifecycle_absent_performance_receipt_identity();
        let delivery_batch_identity = lifecycle_delivery_batch_identity(
            window.evidence_identity(),
            &absent_work_packet,
            delivery_cause.delivery_cause_identity(),
            false,
            patch_group.patch_group_identity(),
            receipt.evidence_identity(),
            &absent_performance,
            "time_only",
        );
        let mut counters = ActiveSubscriptionCounters::default();
        counters.delivery_batch_count = 1;
        counters.fanout_delivery_count = 1;
        counters.patch_group_count = 1;

        Ok(Self {
            delivery_batch_identity,
            delivery_window_identity: window.evidence_identity().clone(),
            attachment_digest: window.attachment_digest().clone(),
            sequence: window.next_sequence(),
            delivery_cause,
            has_relational_patch: false,
            patch_group,
            receipt,
            counters,
        })
    }

    #[allow(dead_code)]
    pub(super) fn new_mixed_cause(
        window: QueryDeliveryWindow,
        delivery_cause: QuerySubscriptionDeliveryCause,
        has_relational_patch: bool,
        patch_group: QueryPatchGroup,
    ) -> Result<Self, QueryDeliveryError> {
        let receipt = QueryDeliveryBatchReceipt::new(
            window.attachment_digest().clone(),
            window.next_sequence(),
        );
        let absent_work_packet = lifecycle_absent_work_packet_identity();
        let absent_performance = lifecycle_absent_performance_receipt_identity();
        let delivery_batch_identity = lifecycle_delivery_batch_identity(
            window.evidence_identity(),
            &absent_work_packet,
            delivery_cause.delivery_cause_identity(),
            has_relational_patch,
            patch_group.patch_group_identity(),
            receipt.evidence_identity(),
            &absent_performance,
            "mixed_cause",
        );
        let mut counters = ActiveSubscriptionCounters::default();
        counters.delivery_batch_count = 1;
        counters.fanout_delivery_count = 1;
        counters.patch_group_count = 1;
        counters.patch_group_width = patch_group.width();

        Ok(Self {
            delivery_batch_identity,
            delivery_window_identity: window.evidence_identity().clone(),
            attachment_digest: window.attachment_digest().clone(),
            sequence: window.next_sequence(),
            delivery_cause,
            has_relational_patch,
            patch_group,
            receipt,
            counters,
        })
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_batch_identity
    }

    pub fn delivery_window_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_window_identity
    }

    pub(crate) fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn sequence(&self) -> QueryDeliverySequence {
        self.sequence
    }

    pub fn delivery_cause(&self) -> &QuerySubscriptionDeliveryCause {
        &self.delivery_cause
    }

    pub fn delivery_cause_kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        self.delivery_cause.kind()
    }

    pub fn has_relational_patch(&self) -> bool {
        self.has_relational_patch
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

type LoweredQuerySubscriptionMaintenanceDelta = (
    QuerySubscriptionMaintenanceDelta,
    QueryMaintenanceDeltaLoweringReport,
    ActiveSubscriptionCounters,
);

pub fn lower_query_subscription_maintenance_delta(
    delta: QuerySubscriptionMaintenanceDelta,
) -> Result<LoweredQuerySubscriptionMaintenanceDelta, QueryDeliveryError> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.maintenance_delta_lowering_count = 1;
    counters.maintenance_delta_width = delta.width();
    let report = QueryMaintenanceDeltaLoweringReport::new(&delta);
    Ok((delta, report, counters))
}
