use crate::identity::hash_parts;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::attachment::SubscriptionConsumerAttachment;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::delivery_dimensions::PreviewResidueWidth;
use super::performance_receipt::SubscriptionPerformanceReceipt;
use super::preview_isolation_error::{
    PreviewSubscriptionIsolationDenialKind, PreviewSubscriptionIsolationError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewSubscriptionLifecycleState {
    PreviewActive,
    PreviewDiscarded,
    PreviewPromoted,
}

impl PreviewSubscriptionLifecycleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PreviewActive => "preview_active",
            Self::PreviewDiscarded => "preview_discarded",
            Self::PreviewPromoted => "preview_promoted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewSubscriptionResidueClass {
    AuthoritativeRouting,
    AuthoritativeCheckpoint,
    AuthoritativeReplay,
    AuthoritativeDiagnostics,
    AuthoritativeWriteback,
    TemporaryPreviewExecution,
    TemporaryPreviewDiagnostics,
}

impl PreviewSubscriptionResidueClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeRouting => "authoritative_routing",
            Self::AuthoritativeCheckpoint => "authoritative_checkpoint",
            Self::AuthoritativeReplay => "authoritative_replay",
            Self::AuthoritativeDiagnostics => "authoritative_diagnostics",
            Self::AuthoritativeWriteback => "authoritative_writeback",
            Self::TemporaryPreviewExecution => "temporary_preview_execution",
            Self::TemporaryPreviewDiagnostics => "temporary_preview_diagnostics",
        }
    }

    pub fn is_authoritative(&self) -> bool {
        matches!(
            self,
            Self::AuthoritativeRouting
                | Self::AuthoritativeCheckpoint
                | Self::AuthoritativeReplay
                | Self::AuthoritativeDiagnostics
                | Self::AuthoritativeWriteback
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionResidueReport {
    authoritative_routing_width: PreviewResidueWidth,
    authoritative_checkpoint_width: PreviewResidueWidth,
    authoritative_replay_width: PreviewResidueWidth,
    authoritative_diagnostics_width: PreviewResidueWidth,
    authoritative_writeback_width: PreviewResidueWidth,
    temporary_execution_width: PreviewResidueWidth,
    temporary_diagnostics_width: PreviewResidueWidth,
    report_digest: String,
}

impl PreviewSubscriptionResidueReport {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        authoritative_routing_width: PreviewResidueWidth,
        authoritative_checkpoint_width: PreviewResidueWidth,
        authoritative_replay_width: PreviewResidueWidth,
        authoritative_diagnostics_width: PreviewResidueWidth,
        authoritative_writeback_width: PreviewResidueWidth,
        temporary_execution_width: PreviewResidueWidth,
        temporary_diagnostics_width: PreviewResidueWidth,
    ) -> Self {
        let report_digest = hash_parts(&[
            "preview_subscription_residue_report_v1".to_string(),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeRouting.as_str(),
                authoritative_routing_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeCheckpoint.as_str(),
                authoritative_checkpoint_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeReplay.as_str(),
                authoritative_replay_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeDiagnostics.as_str(),
                authoritative_diagnostics_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeWriteback.as_str(),
                authoritative_writeback_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::TemporaryPreviewExecution.as_str(),
                temporary_execution_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::TemporaryPreviewDiagnostics.as_str(),
                temporary_diagnostics_width.get()
            ),
        ]);
        Self {
            authoritative_routing_width,
            authoritative_checkpoint_width,
            authoritative_replay_width,
            authoritative_diagnostics_width,
            authoritative_writeback_width,
            temporary_execution_width,
            temporary_diagnostics_width,
            report_digest,
        }
    }

    pub fn authoritative_residue_width(&self) -> u64 {
        self.authoritative_routing_width.get()
            + self.authoritative_checkpoint_width.get()
            + self.authoritative_replay_width.get()
            + self.authoritative_diagnostics_width.get()
            + self.authoritative_writeback_width.get()
    }

    pub fn temporary_residue_width(&self) -> u64 {
        self.temporary_execution_width.get() + self.temporary_diagnostics_width.get()
    }

    pub fn preview_residue_width(&self) -> u64 {
        self.authoritative_residue_width() + self.temporary_residue_width()
    }

    pub fn class_width(&self, residue_class: PreviewSubscriptionResidueClass) -> u64 {
        match residue_class {
            PreviewSubscriptionResidueClass::AuthoritativeRouting => {
                self.authoritative_routing_width.get()
            }
            PreviewSubscriptionResidueClass::AuthoritativeCheckpoint => {
                self.authoritative_checkpoint_width.get()
            }
            PreviewSubscriptionResidueClass::AuthoritativeReplay => {
                self.authoritative_replay_width.get()
            }
            PreviewSubscriptionResidueClass::AuthoritativeDiagnostics => {
                self.authoritative_diagnostics_width.get()
            }
            PreviewSubscriptionResidueClass::AuthoritativeWriteback => {
                self.authoritative_writeback_width.get()
            }
            PreviewSubscriptionResidueClass::TemporaryPreviewExecution => {
                self.temporary_execution_width.get()
            }
            PreviewSubscriptionResidueClass::TemporaryPreviewDiagnostics => {
                self.temporary_diagnostics_width.get()
            }
        }
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionIsolationArtifact {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    preview_epoch_digest: String,
    lifecycle_state: PreviewSubscriptionLifecycleState,
    preview_residue_budget_width: PreviewResidueWidth,
    counters: ActiveSubscriptionCounters,
    isolation_digest: String,
}

impl PreviewSubscriptionIsolationArtifact {
    pub(super) fn new(
        attachment: &SubscriptionConsumerAttachment,
        preview_epoch: impl Into<String>,
        preview_residue_budget_width: PreviewResidueWidth,
    ) -> Self {
        let preview_epoch_digest = hash_parts(&[
            "preview_subscription_epoch_v1".to_string(),
            format!("epoch:{}", preview_epoch.into()),
        ]);
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_active_lane_count = 1;
        counters.preview_residue_width = preview_residue_budget_width.get();
        let isolation_digest = hash_parts(&[
            "preview_subscription_isolation_artifact_v1".to_string(),
            format!("lane:{}", attachment.lane_digest().as_str()),
            format!("attachment:{}", attachment.attachment_digest().as_str()),
            format!("epoch:{}", preview_epoch_digest),
            format!(
                "state:{}",
                PreviewSubscriptionLifecycleState::PreviewActive.as_str()
            ),
            format!("residue_budget:{}", preview_residue_budget_width.get()),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            active_lane_digest: attachment.lane_digest().clone(),
            attachment_digest: attachment.attachment_digest().clone(),
            preview_epoch_digest,
            lifecycle_state: PreviewSubscriptionLifecycleState::PreviewActive,
            preview_residue_budget_width,
            counters,
            isolation_digest,
        }
    }

    pub fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn preview_epoch_digest(&self) -> &str {
        &self.preview_epoch_digest
    }

    pub fn lifecycle_state(&self) -> PreviewSubscriptionLifecycleState {
        self.lifecycle_state
    }

    pub fn preview_residue_budget_width(&self) -> u64 {
        self.preview_residue_budget_width.get()
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }

    pub fn isolation_digest(&self) -> &str {
        &self.isolation_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionDiscardCloseout {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
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
            format!("epoch:{}", isolation.preview_epoch_digest()),
            format!("isolation:{}", isolation.isolation_digest()),
            format!("residue_report:{}", residue_report.report_digest()),
            format!(
                "performance:{}",
                performance_receipt.performance_receipt_digest()
            ),
            format!(
                "state:{}",
                PreviewSubscriptionLifecycleState::PreviewDiscarded.as_str()
            ),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            active_lane_digest: isolation.active_lane_digest,
            attachment_digest: isolation.attachment_digest,
            preview_epoch_digest: isolation.preview_epoch_digest,
            residue_report_digest: residue_report.report_digest,
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
    preview_epoch_digest: String,
    authority_digest: String,
    performance_receipt: SubscriptionPerformanceReceipt,
    counters: ActiveSubscriptionCounters,
    handoff_digest: String,
}

impl PreviewSubscriptionPromotionHandoff {
    pub(super) fn new(
        isolation: PreviewSubscriptionIsolationArtifact,
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
            format!("epoch:{}", isolation.preview_epoch_digest()),
            format!("isolation:{}", isolation.isolation_digest()),
            format!("authority:{}", authority_digest),
            format!(
                "performance:{}",
                performance_receipt.performance_receipt_digest()
            ),
            format!(
                "state:{}",
                PreviewSubscriptionLifecycleState::PreviewPromoted.as_str()
            ),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            preview_lane_digest: isolation.active_lane_digest,
            authoritative_active_lane_digest: authoritative_lane.lane_digest().clone(),
            attachment_digest: isolation.attachment_digest,
            preview_epoch_digest: isolation.preview_epoch_digest,
            authority_digest,
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

    pub fn preview_epoch_digest(&self) -> &str {
        &self.preview_epoch_digest
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
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

#[allow(clippy::too_many_arguments)]
pub fn measure_preview_subscription_residue(
    authoritative_routing_width: PreviewResidueWidth,
    authoritative_checkpoint_width: PreviewResidueWidth,
    authoritative_replay_width: PreviewResidueWidth,
    authoritative_diagnostics_width: PreviewResidueWidth,
    authoritative_writeback_width: PreviewResidueWidth,
    temporary_execution_width: PreviewResidueWidth,
    temporary_diagnostics_width: PreviewResidueWidth,
) -> PreviewSubscriptionResidueReport {
    PreviewSubscriptionResidueReport::new(
        authoritative_routing_width,
        authoritative_checkpoint_width,
        authoritative_replay_width,
        authoritative_diagnostics_width,
        authoritative_writeback_width,
        temporary_execution_width,
        temporary_diagnostics_width,
    )
}

pub fn admit_preview_subscription_isolation(
    attachment: &SubscriptionConsumerAttachment,
    preview_epoch: impl Into<String>,
    preview_residue_budget_width: PreviewResidueWidth,
) -> Result<PreviewSubscriptionIsolationArtifact, PreviewSubscriptionIsolationError> {
    Ok(PreviewSubscriptionIsolationArtifact::new(
        attachment,
        preview_epoch,
        preview_residue_budget_width,
    ))
}

pub fn deny_preview_authoritative_sharing(
    isolation: &PreviewSubscriptionIsolationArtifact,
    authoritative_lane: &ActiveSubscriptionLaneHandle,
) -> Result<(), PreviewSubscriptionIsolationError> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.preview_authoritative_sharing_denial_count = 1;
    Err(PreviewSubscriptionIsolationError::new(
        PreviewSubscriptionIsolationDenialKind::PreviewAuthoritativeSharingDenied,
        "preview subscription isolation cannot share attachment or fanout state with an authoritative active lane",
        hash_parts(&[
            "preview_authoritative_sharing_denial_v1".to_string(),
            format!("preview:{}", isolation.isolation_digest()),
            format!("authoritative:{}", authoritative_lane.lane_digest().as_str()),
        ]),
        counters,
    ))
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
        authoritative_lane,
        authority_digest,
    ))
}
