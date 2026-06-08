use crate::identity::hash_parts;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::attachment::SubscriptionConsumerAttachment;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::delivery_dimensions::PreviewResidueWidth;
use super::future_selection::QuerySubscriptionFutureSelection;
use super::preview_isolation_error::{
    PreviewSubscriptionIsolationDenialKind, PreviewSubscriptionIsolationError,
};

pub use super::preview_closeout::{
    discard_preview_subscription, promote_preview_subscription, PreviewSubscriptionDiscardCloseout,
    PreviewSubscriptionPromotionHandoff,
};
pub use super::preview_residue::{
    measure_preview_subscription_residue, PreviewSubscriptionResidueClass,
    PreviewSubscriptionResidueReport,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionIsolationArtifact {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    future_selection: QuerySubscriptionFutureSelection,
    basis_binding_digest: String,
    checkpoint_identity_digest: String,
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
            format!(
                "future_selection:{}",
                attachment.future_selection().projection_digest()
            ),
            format!("basis:{}", attachment.basis_binding_digest()),
            format!("checkpoint:{}", attachment.checkpoint_identity_digest()),
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
            future_selection: attachment.future_selection().clone(),
            basis_binding_digest: attachment.basis_binding_digest().to_string(),
            checkpoint_identity_digest: attachment.checkpoint_identity_digest().to_string(),
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
        ]),
        counters,
    ))
}
