use crate::identity::hash_parts;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::delivery_density::ActiveDeliveryDensityPosture;
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
    basis_binding_digest: String,
    checkpoint_identity_digest: String,
    preview_epoch_digest: String,
    residue_report_digest: String,
    performance_receipt: SubscriptionPerformanceReceipt,
    counters: ActiveSubscriptionCounters,
    closeout_digest: String,
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
            isolation.isolation_digest(),
        );
        counters.subscription_performance_receipt_count = 1;
        counters.subscription_budget_consumption_width = performance_receipt.consumed_width();
        counters.subscription_budget_remaining_width = performance_receipt.remaining_width();
        let closeout_digest = hash_parts(&[
            "preview_subscription_discard_closeout_v1".to_string(),
            format!("lane:{}", isolation.active_lane_digest().as_str()),
            format!("attachment:{}", isolation.attachment_digest().as_str()),
            format!(
                "future_selection:{}",
                isolation.future_selection().projection_digest()
            ),
            format!("basis:{}", isolation.basis_binding_digest()),
            format!("checkpoint:{}", isolation.checkpoint_identity_digest()),
            format!("epoch:{}", isolation.preview_epoch_digest()),
            format!("isolation:{}", isolation.isolation_digest()),
            format!("residue_report:{}", residue_report.report_digest()),
            format!(
                "performance:{}",
                performance_receipt.performance_receipt_for_reporting()
            ),
            format!(
                "state:{}",
                PreviewSubscriptionLifecycleState::PreviewDiscarded.as_str()
            ),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            active_lane_digest: isolation.active_lane_digest().clone(),
            attachment_digest: isolation.attachment_digest().clone(),
            future_selection: isolation.future_selection().clone(),
            basis_binding_digest: isolation.basis_binding_digest().to_string(),
            checkpoint_identity_digest: isolation.checkpoint_identity_digest().to_string(),
            preview_epoch_digest: isolation.preview_epoch_digest().to_string(),
            residue_report_digest: residue_report.report_digest().to_string(),
            performance_receipt,
            counters,
            closeout_digest,
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

    pub fn basis_binding_digest(&self) -> &str {
        &self.basis_binding_digest
    }

    pub fn checkpoint_identity_digest(&self) -> &str {
        &self.checkpoint_identity_digest
    }

    pub fn preview_epoch_digest(&self) -> &str {
        &self.preview_epoch_digest
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

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionPromotionHandoff {
    preview_lane_digest: ActiveSubscriptionLaneDigest,
    authoritative_active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    future_selection: QuerySubscriptionFutureSelection,
    preview_basis_binding_digest: String,
    authoritative_basis_binding_digest: String,
    preview_checkpoint_identity_digest: String,
    authoritative_checkpoint_identity_digest: String,
    preview_epoch_digest: String,
    residue_report_digest: String,
    authority_digest: String,
    rebinding_digest: String,
    performance_receipt: SubscriptionPerformanceReceipt,
    counters: ActiveSubscriptionCounters,
    handoff_digest: String,
}

impl PreviewSubscriptionPromotionHandoff {
    pub(super) fn new(
        isolation: PreviewSubscriptionIsolationArtifact,
        residue_report: &PreviewSubscriptionResidueReport,
        authoritative_lane: &ActiveSubscriptionLaneHandle,
        authority_digest: impl Into<String>,
    ) -> Self {
        let authority_digest = hash_parts(&[
            "preview_subscription_promotion_authority_v1".to_string(),
            format!("authority:{}", authority_digest.into()),
        ]);
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_promotion_handoff_count = 1;
        let performance_receipt = SubscriptionPerformanceReceipt::new(
            1,
            1,
            ActiveDeliveryDensityPosture::SparseDelta,
            super::active_budget::ActiveSubscriptionAllocationPosture::LifecycleArena,
            isolation.isolation_digest(),
        );
        let rebinding_digest = hash_parts(&[
            "preview_subscription_promotion_rebinding_v1".to_string(),
            format!("preview_basis:{}", isolation.basis_binding_digest()),
            format!(
                "authoritative_basis:{}",
                authoritative_lane.basis_binding_digest()
            ),
            format!(
                "preview_checkpoint:{}",
                isolation.checkpoint_identity_digest()
            ),
            format!(
                "authoritative_checkpoint:{}",
                authoritative_lane.checkpoint_identity_digest()
            ),
        ]);
        counters.subscription_performance_receipt_count = 1;
        counters.subscription_budget_consumption_width = performance_receipt.consumed_width();
        counters.subscription_budget_remaining_width = performance_receipt.remaining_width();
        let handoff_digest = hash_parts(&[
            "preview_subscription_promotion_handoff_v1".to_string(),
            format!("preview_lane:{}", isolation.active_lane_digest().as_str()),
            format!(
                "authoritative_lane:{}",
                authoritative_lane.lane_digest().as_str()
            ),
            format!("attachment:{}", isolation.attachment_digest().as_str()),
            format!(
                "future_selection:{}",
                isolation.future_selection().projection_digest()
            ),
            format!("basis:{}", isolation.basis_binding_digest()),
            format!(
                "preview_checkpoint:{}",
                isolation.checkpoint_identity_digest()
            ),
            format!(
                "authoritative_checkpoint:{}",
                authoritative_lane.checkpoint_identity_digest()
            ),
            format!("epoch:{}", isolation.preview_epoch_digest()),
            format!("isolation:{}", isolation.isolation_digest()),
            format!("residue_report:{}", residue_report.report_digest()),
            format!("authority:{}", authority_digest),
            format!("rebinding:{}", rebinding_digest),
            format!(
                "performance:{}",
                performance_receipt.performance_receipt_for_reporting()
            ),
            format!(
                "state:{}",
                PreviewSubscriptionLifecycleState::PreviewPromoted.as_str()
            ),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            preview_lane_digest: isolation.active_lane_digest().clone(),
            authoritative_active_lane_digest: authoritative_lane.lane_digest().clone(),
            attachment_digest: isolation.attachment_digest().clone(),
            future_selection: isolation.future_selection().clone(),
            preview_basis_binding_digest: isolation.basis_binding_digest().to_string(),
            authoritative_basis_binding_digest: authoritative_lane
                .basis_binding_digest()
                .to_string(),
            preview_checkpoint_identity_digest: isolation.checkpoint_identity_digest().to_string(),
            authoritative_checkpoint_identity_digest: authoritative_lane
                .checkpoint_identity_digest()
                .to_string(),
            preview_epoch_digest: isolation.preview_epoch_digest().to_string(),
            residue_report_digest: residue_report.report_digest().to_string(),
            authority_digest,
            rebinding_digest,
            performance_receipt,
            counters,
            handoff_digest,
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

    pub fn preview_basis_binding_digest(&self) -> &str {
        &self.preview_basis_binding_digest
    }

    pub fn authoritative_basis_binding_digest(&self) -> &str {
        &self.authoritative_basis_binding_digest
    }

    pub fn preview_checkpoint_identity_digest(&self) -> &str {
        &self.preview_checkpoint_identity_digest
    }

    pub fn authoritative_checkpoint_identity_digest(&self) -> &str {
        &self.authoritative_checkpoint_identity_digest
    }

    pub fn preview_epoch_digest(&self) -> &str {
        &self.preview_epoch_digest
    }

    pub fn residue_report_digest(&self) -> &str {
        &self.residue_report_digest
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub fn rebinding_digest(&self) -> &str {
        &self.rebinding_digest
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
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
            isolation.isolation_digest(),
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
    authority_digest: impl Into<String>,
) -> Result<PreviewSubscriptionPromotionHandoff, PreviewSubscriptionIsolationError> {
    if isolation.lifecycle_state() != PreviewSubscriptionLifecycleState::PreviewActive {
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_promotion_handoff_count = 1;
        return Err(PreviewSubscriptionIsolationError::new(
            PreviewSubscriptionIsolationDenialKind::PreviewLifecycleStateMismatch,
            "preview promotion requires an active preview isolation artifact",
            isolation.isolation_digest(),
            counters,
        ));
    }

    Ok(PreviewSubscriptionPromotionHandoff::new(
        isolation,
        residue_report,
        authoritative_lane,
        authority_digest,
    ))
}
