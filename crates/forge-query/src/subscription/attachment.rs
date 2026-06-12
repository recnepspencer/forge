use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::acknowledgement::{
    QueryDeliveryBatchReceipt, QueryDeliverySequence, SubscriptionAcknowledgementFrontier,
};
use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::attachment_budget::{DeliveryBackpressurePolicy, SubscriptionConsumerAttachmentBudget};
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::attachment_error::{
    SubscriptionConsumerAttachmentDenialKind, SubscriptionConsumerAttachmentError,
};
use super::attachment_request::SubscriptionConsumerAttachmentRequest;
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::fanout::{SubscriptionFanoutPlan, SubscriptionFanoutReport};
use super::future_selection::QuerySubscriptionFutureSelection;
use super::performance_receipt::SubscriptionPerformanceReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionConsumerAttachment {
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    lane_digest: ActiveSubscriptionLaneDigest,
    future_selection: QuerySubscriptionFutureSelection,
    basis_binding_digest: String,
    checkpoint_identity_digest: String,
    consumer_identity: ForgeQueryEvidenceIdentity,
    delivery_cursor_identity: ForgeQueryEvidenceIdentity,
    attachment_index: u64,
    acknowledgement_frontier: SubscriptionAcknowledgementFrontier,
    next_delivery_sequence: QueryDeliverySequence,
    backpressure_policy: DeliveryBackpressurePolicy,
    fanout_report: SubscriptionFanoutReport,
    performance_receipt: SubscriptionPerformanceReceipt,
}

impl SubscriptionConsumerAttachment {
    pub(super) fn new(
        handle: &ActiveSubscriptionLaneHandle,
        request: SubscriptionConsumerAttachmentRequest,
        budget: SubscriptionConsumerAttachmentBudget,
        attachment_index: u64,
        affected_consumer_attachment_width: u64,
    ) -> Result<(Self, ActiveSubscriptionCounters), SubscriptionConsumerAttachmentError> {
        let mut counters = ActiveSubscriptionCounters::default();
        if budget.exceeds_phase_two_budget() {
            counters.consumer_attachment_denial_count = 1;
            return Err(SubscriptionConsumerAttachmentError::new(
                SubscriptionConsumerAttachmentDenialKind::AttachmentBudgetExceeded,
                "consumer attachment exceeds its explicit Phase 2 budget",
                handle.lane_digest().as_str(),
                counters,
            ));
        }
        if budget.backpressure_denial_requested() {
            counters.backpressure_denial_count = 1;
            return Err(SubscriptionConsumerAttachmentError::new(
                SubscriptionConsumerAttachmentDenialKind::BackpressureDenied,
                "consumer attachment requested an inadmissible backpressure posture",
                handle.lane_digest().as_str(),
                counters,
            ));
        }

        counters.consumer_attachment_count = 1;
        counters.fanout_width = budget.fanout_width();
        counters.affected_consumer_attachment_width = affected_consumer_attachment_width;
        counters.subscription_budget_consumption_width = 3;
        counters.subscription_budget_remaining_width = 3;
        if budget.backpressure_policy() == &DeliveryBackpressurePolicy::DropWithGapNotice {
            counters.delivery_gap_notice_count = 1;
        }
        if budget.allocation_scope_width() > 0
            && budget.backpressure_policy() == &DeliveryBackpressurePolicy::DebtExplicit
        {
            counters.heap_allocation_debt_count = 1;
        }

        let lane_digest = handle.lane_digest().clone();
        let fanout_plan =
            SubscriptionFanoutPlan::new(lane_digest.clone(), affected_consumer_attachment_width);
        let fanout_report = SubscriptionFanoutReport::new(fanout_plan, 1);
        let allocation_posture =
            if budget.backpressure_policy() == &DeliveryBackpressurePolicy::DebtExplicit {
                ActiveSubscriptionAllocationPosture::HeapAllocationDebtExplicit
            } else {
                ActiveSubscriptionAllocationPosture::LifecycleArena
            };
        let performance_receipt = SubscriptionPerformanceReceipt::new(
            3,
            budget.fanout_width()
                + budget.delivery_pacing_width()
                + budget.allocation_scope_width(),
            ActiveDeliveryDensityPosture::SparseDelta,
            allocation_posture,
            &format!(
                "{}:{}:{}",
                handle.lane_digest().as_str(),
                request.consumer_digest(),
                budget.backpressure_policy().as_str()
            ),
        );
        counters.subscription_performance_receipt_count = 1;
        counters.subscription_budget_consumption_width = performance_receipt.consumed_width();
        counters.subscription_budget_remaining_width = performance_receipt.remaining_width();
        let delivery_cursor_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "subscription_delivery_cursor_v1",
        )
        .field_identity(ForgeQueryEvidenceTag::new("lane"), lane_digest.as_str())
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("consumer"),
            request.consumer_identity(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("seed"),
            request.delivery_cursor_seed(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pacing_width"),
            budget.delivery_pacing_width().to_string(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("backpressure"),
            budget.backpressure_policy().as_str(),
        )
        .seal();
        let attachment_digest = SubscriptionConsumerAttachmentDigest::new(
            ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "subscription_consumer_attachment_v1",
            )
            .field_identity(ForgeQueryEvidenceTag::new("lane"), lane_digest.as_str())
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("consumer"),
                request.consumer_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("cursor"),
                &delivery_cursor_identity,
            )
            .field_value(
                ForgeQueryEvidenceTag::new("attachment_index"),
                attachment_index.to_string(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("handle_index"),
                handle.lane_index().to_string(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("handle_generation"),
                handle.registry_generation().to_string(),
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("fanout_report"),
                fanout_report.report_digest(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("allocation"),
                allocation_posture.as_str(),
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("performance"),
                performance_receipt.performance_receipt_digest(),
            )
            .seal(),
        );
        let acknowledgement_frontier =
            SubscriptionAcknowledgementFrontier::initial(attachment_digest.clone());

        Ok((
            Self {
                attachment_digest,
                lane_digest,
                future_selection: handle.future_selection().clone(),
                basis_binding_digest: handle.basis_binding_digest().to_string(),
                checkpoint_identity_digest: handle.checkpoint_identity_digest().to_string(),
                consumer_identity: request.consumer_identity().clone(),
                delivery_cursor_identity,
                attachment_index,
                acknowledgement_frontier,
                next_delivery_sequence: QueryDeliverySequence::initial().next(),
                backpressure_policy: *budget.backpressure_policy(),
                fanout_report,
                performance_receipt,
            },
            counters,
        ))
    }

    pub(super) fn advance_acknowledgement(
        mut self,
        receipt: QueryDeliveryBatchReceipt,
    ) -> Result<(Self, ActiveSubscriptionCounters), SubscriptionConsumerAttachmentError> {
        let mut counters = ActiveSubscriptionCounters::default();
        if receipt.attachment_digest() != &self.attachment_digest {
            counters.acknowledgement_receipt_mismatch_denial_count = 1;
            return Err(SubscriptionConsumerAttachmentError::new(
                SubscriptionConsumerAttachmentDenialKind::AcknowledgementReceiptMismatch,
                "acknowledgement receipt belongs to a different consumer attachment",
                receipt.receipt_digest(),
                counters,
            ));
        }
        if receipt.sequence() <= self.acknowledgement_frontier.acknowledged_sequence() {
            counters.acknowledgement_sequence_regression_denial_count = 1;
            return Err(SubscriptionConsumerAttachmentError::new(
                SubscriptionConsumerAttachmentDenialKind::AcknowledgementSequenceRegression,
                "acknowledgement receipt sequence does not advance the consumer frontier",
                receipt.receipt_digest(),
                counters,
            ));
        }

        self.acknowledgement_frontier = self.acknowledgement_frontier.advance(receipt.sequence());
        self.next_delivery_sequence = receipt.sequence().next();
        counters.acknowledgement_frontier_advance_count = 1;
        Ok((self, counters))
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_digest(&self) -> &str {
        &self.basis_binding_digest
    }

    pub fn checkpoint_identity_digest(&self) -> &str {
        &self.checkpoint_identity_digest
    }

    pub fn consumer_digest(&self) -> &str {
        self.consumer_identity.as_str()
    }

    pub fn consumer_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.consumer_identity
    }

    pub fn delivery_cursor_digest(&self) -> &str {
        self.delivery_cursor_identity.as_str()
    }

    pub fn delivery_cursor_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_cursor_identity
    }

    pub fn attachment_index(&self) -> u64 {
        self.attachment_index
    }

    pub fn acknowledgement_frontier(&self) -> &SubscriptionAcknowledgementFrontier {
        &self.acknowledgement_frontier
    }

    pub fn next_delivery_sequence(&self) -> QueryDeliverySequence {
        self.next_delivery_sequence
    }

    pub fn backpressure_policy(&self) -> &DeliveryBackpressurePolicy {
        &self.backpressure_policy
    }

    pub fn fanout_report(&self) -> &SubscriptionFanoutReport {
        &self.fanout_report
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }
}
