use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::evidence_identities::{
    preview_discard_closeout_identity, preview_promotion_authority_identity,
    preview_promotion_handoff_identity, preview_promotion_rebinding_identity,
};
use super::future_selection::QuerySubscriptionFutureSelection;
use super::performance_receipt::SubscriptionPerformanceReceipt;
use super::preview_isolation::{
    PreviewSubscriptionIsolationArtifact, PreviewSubscriptionLifecycleState,
};
use super::preview_isolation_error::{
    PreviewSubscriptionIsolationDenialKind, PreviewSubscriptionIsolationError,
};
use super::preview_residue::PreviewSubscriptionResidueReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionDiscardCloseout {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    future_selection: QuerySubscriptionFutureSelection,
    basis_binding_identity: ForgeQueryEvidenceIdentity,
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    preview_epoch_identity: ForgeQueryEvidenceIdentity,
    residue_report_digest: String,
    performance_receipt: SubscriptionPerformanceReceipt,
    counters: ActiveSubscriptionCounters,
    closeout_identity: ForgeQueryEvidenceIdentity,
}

impl PreviewSubscriptionDiscardCloseout {
    pub(super) fn new(
        isolation: PreviewSubscriptionIsolationArtifact,
        residue_report: PreviewSubscriptionResidueReport,
    ) -> Self {
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_discard_residue_check_count = 1;
        counters.preview_residue_width = residue_report.preview_residue_width();
        counters.preview_authoritative_residue_count = residue_report.authoritative_residue_width();
        let performance_receipt = SubscriptionPerformanceReceipt::new(
            residue_report.preview_residue_width(),
            isolation.preview_residue_budget_width(),
            ActiveDeliveryDensityPosture::SparseDelta,
            super::active_budget::ActiveSubscriptionAllocationPosture::LifecycleArena,
            isolation.attachment_digest().evidence_identity(),
        );
        counters.subscription_performance_receipt_count = 1;
        counters.subscription_budget_consumption_width = performance_receipt.consumed_width();
        counters.subscription_budget_remaining_width = performance_receipt.remaining_width();
        let closeout_identity = preview_discard_closeout_identity(
            isolation.active_lane_digest().evidence_identity(),
            isolation.attachment_digest().evidence_identity(),
            isolation.future_selection().projection_identity(),
            isolation.basis_binding_identity(),
            isolation.checkpoint_identity(),
            isolation.preview_epoch_identity(),
            isolation.isolation_identity(),
            residue_report.report_digest(),
            performance_receipt.performance_receipt_identity(),
            PreviewSubscriptionLifecycleState::PreviewDiscarded.as_str(),
            &counters.evidence_identity(),
        );
        Self {
            active_lane_digest: isolation.active_lane_digest().clone(),
            attachment_digest: isolation.attachment_digest().clone(),
            future_selection: isolation.future_selection().clone(),
            basis_binding_identity: isolation.basis_binding_identity().clone(),
            checkpoint_identity: isolation.checkpoint_identity().clone(),
            preview_epoch_identity: isolation.preview_epoch_identity().clone(),
            residue_report_digest: residue_report.report_digest().to_string(),
            performance_receipt,
            counters,
            closeout_identity,
        }
    }

    pub fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn basis_binding_for_reporting(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn checkpoint_for_reporting(&self) -> &str {
        self.checkpoint_identity.as_str()
    }

    pub fn preview_epoch_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.preview_epoch_identity
    }

    pub fn preview_epoch_for_reporting(&self) -> &str {
        self.preview_epoch_identity.as_str()
    }

    pub fn residue_report_digest(&self) -> &str {
        &self.residue_report_digest
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn closeout_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.closeout_identity
    }

    pub fn closeout_for_reporting(&self) -> &str {
        self.closeout_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionPromotionHandoff {
    preview_lane_digest: ActiveSubscriptionLaneDigest,
    authoritative_active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    future_selection: QuerySubscriptionFutureSelection,
    preview_basis_binding_identity: ForgeQueryEvidenceIdentity,
    authoritative_basis_binding_identity: ForgeQueryEvidenceIdentity,
    preview_checkpoint_identity: ForgeQueryEvidenceIdentity,
    authoritative_checkpoint_identity: ForgeQueryEvidenceIdentity,
    preview_epoch_identity: ForgeQueryEvidenceIdentity,
    residue_report_digest: String,
    authority_identity: ForgeQueryEvidenceIdentity,
    rebinding_identity: ForgeQueryEvidenceIdentity,
    performance_receipt: SubscriptionPerformanceReceipt,
    counters: ActiveSubscriptionCounters,
    handoff_identity: ForgeQueryEvidenceIdentity,
}

impl PreviewSubscriptionPromotionHandoff {
    pub(super) fn new(
        isolation: PreviewSubscriptionIsolationArtifact,
        residue_report: &PreviewSubscriptionResidueReport,
        authoritative_lane: &ActiveSubscriptionLaneHandle,
        authority_label: impl Into<String>,
    ) -> Self {
        let authority_identity = preview_promotion_authority_identity(&authority_label.into());
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_promotion_handoff_count = 1;
        let performance_receipt = SubscriptionPerformanceReceipt::new(
            1,
            1,
            ActiveDeliveryDensityPosture::SparseDelta,
            super::active_budget::ActiveSubscriptionAllocationPosture::LifecycleArena,
            isolation.attachment_digest().evidence_identity(),
        );
        let rebinding_identity = preview_promotion_rebinding_identity(
            isolation.basis_binding_identity(),
            authoritative_lane.basis_binding_identity(),
            isolation.checkpoint_identity(),
            authoritative_lane.checkpoint_identity(),
        );
        counters.subscription_performance_receipt_count = 1;
        counters.subscription_budget_consumption_width = performance_receipt.consumed_width();
        counters.subscription_budget_remaining_width = performance_receipt.remaining_width();
        let handoff_identity = preview_promotion_handoff_identity(
            isolation.active_lane_digest().evidence_identity(),
            authoritative_lane.lane_digest().evidence_identity(),
            isolation.attachment_digest().evidence_identity(),
            isolation.future_selection().projection_identity(),
            isolation.basis_binding_identity(),
            isolation.checkpoint_identity(),
            authoritative_lane.checkpoint_identity(),
            isolation.preview_epoch_identity(),
            isolation.isolation_identity(),
            residue_report.report_digest(),
            &authority_identity,
            &rebinding_identity,
            performance_receipt.performance_receipt_identity(),
            PreviewSubscriptionLifecycleState::PreviewPromoted.as_str(),
            &counters.evidence_identity(),
        );
        Self {
            preview_lane_digest: isolation.active_lane_digest().clone(),
            authoritative_active_lane_digest: authoritative_lane.lane_digest().clone(),
            attachment_digest: isolation.attachment_digest().clone(),
            future_selection: isolation.future_selection().clone(),
            preview_basis_binding_identity: isolation.basis_binding_identity().clone(),
            authoritative_basis_binding_identity: authoritative_lane.basis_binding_identity().clone(),
            preview_checkpoint_identity: isolation.checkpoint_identity().clone(),
            authoritative_checkpoint_identity: authoritative_lane.checkpoint_identity().clone(),
            preview_epoch_identity: isolation.preview_epoch_identity().clone(),
            residue_report_digest: residue_report.report_digest().to_string(),
            authority_identity,
            rebinding_identity,
            performance_receipt,
            counters,
            handoff_identity,
        }
    }

    pub fn preview_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.preview_lane_digest
    }

    pub fn authoritative_active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.authoritative_active_lane_digest
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn preview_basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.preview_basis_binding_identity
    }

    pub fn preview_basis_binding_for_reporting(&self) -> &str {
        self.preview_basis_binding_identity.as_str()
    }

    pub fn authoritative_basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.authoritative_basis_binding_identity
    }

    pub fn authoritative_basis_binding_for_reporting(&self) -> &str {
        self.authoritative_basis_binding_identity.as_str()
    }

    pub fn preview_checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.preview_checkpoint_identity
    }

    pub fn preview_checkpoint_for_reporting(&self) -> &str {
        self.preview_checkpoint_identity.as_str()
    }

    pub fn authoritative_checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.authoritative_checkpoint_identity
    }

    pub fn authoritative_checkpoint_for_reporting(&self) -> &str {
        self.authoritative_checkpoint_identity.as_str()
    }

    pub fn preview_epoch_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.preview_epoch_identity
    }

    pub fn preview_epoch_for_reporting(&self) -> &str {
        self.preview_epoch_identity.as_str()
    }

    pub fn residue_report_digest(&self) -> &str {
        &self.residue_report_digest
    }

    pub fn authority_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.authority_identity
    }

    pub fn authority_for_reporting(&self) -> &str {
        self.authority_identity.as_str()
    }

    pub fn rebinding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.rebinding_identity
    }

    pub fn rebinding_for_reporting(&self) -> &str {
        self.rebinding_identity.as_str()
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn handoff_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.handoff_identity
    }

    pub fn handoff_for_reporting(&self) -> &str {
        self.handoff_identity.as_str()
    }
}

pub fn discard_preview_subscription(
    isolation: PreviewSubscriptionIsolationArtifact,
    residue_report: PreviewSubscriptionResidueReport,
) -> Result<PreviewSubscriptionDiscardCloseout, PreviewSubscriptionIsolationError> {
    if isolation.lifecycle_state() != PreviewSubscriptionLifecycleState::PreviewActive {
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_discard_residue_check_count = 1;
        return Err(PreviewSubscriptionIsolationError::new(
            PreviewSubscriptionIsolationDenialKind::PreviewLifecycleStateMismatch,
            "preview discard requires an active preview isolation artifact",
            isolation.isolation_for_reporting(),
            counters,
        ));
    }
    if residue_report.authoritative_residue_width() > 0 {
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_discard_residue_check_count = 1;
        counters.preview_residue_width = residue_report.preview_residue_width();
        counters.preview_authoritative_residue_count = residue_report.authoritative_residue_width();
        return Err(PreviewSubscriptionIsolationError::new(
            PreviewSubscriptionIsolationDenialKind::PreviewDiscardResidueDenied,
            "preview discard cannot close while authoritative routing, checkpoint, replay, diagnostics, or writeback residue remains",
            residue_report.report_digest(),
            counters,
        ));
    }
    if residue_report.preview_residue_width() > isolation.preview_residue_budget_width() {
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_discard_residue_check_count = 1;
        counters.preview_residue_width = residue_report.preview_residue_width();
        return Err(PreviewSubscriptionIsolationError::new(
            PreviewSubscriptionIsolationDenialKind::PreviewDiscardResidueDenied,
            "preview discard cannot exceed the admitted preview residue budget",
            residue_report.report_digest(),
            counters,
        ));
    }

    Ok(PreviewSubscriptionDiscardCloseout::new(
        isolation,
        residue_report,
    ))
}

pub fn promote_preview_subscription(
    isolation: PreviewSubscriptionIsolationArtifact,
    residue_report: &PreviewSubscriptionResidueReport,
    authoritative_lane: &ActiveSubscriptionLaneHandle,
    authority_label: impl Into<String>,
) -> Result<PreviewSubscriptionPromotionHandoff, PreviewSubscriptionIsolationError> {
    if isolation.lifecycle_state() != PreviewSubscriptionLifecycleState::PreviewActive {
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_promotion_handoff_count = 1;
        return Err(PreviewSubscriptionIsolationError::new(
            PreviewSubscriptionIsolationDenialKind::PreviewLifecycleStateMismatch,
            "preview promotion requires an active preview isolation artifact",
            isolation.isolation_for_reporting(),
            counters,
        ));
    }

    Ok(PreviewSubscriptionPromotionHandoff::new(
        isolation,
        residue_report,
        authoritative_lane,
        authority_label,
    ))
}
