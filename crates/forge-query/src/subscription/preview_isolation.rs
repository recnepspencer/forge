use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::attachment::SubscriptionConsumerAttachment;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::delivery_dimensions::PreviewResidueWidth;
use super::evidence_identities::{
    preview_authoritative_sharing_denial_identity, preview_epoch_identity,
    preview_isolation_identity,
};
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
    basis_binding_identity: ForgeQueryEvidenceIdentity,
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    preview_epoch_identity: ForgeQueryEvidenceIdentity,
    lifecycle_state: PreviewSubscriptionLifecycleState,
    preview_residue_budget_width: PreviewResidueWidth,
    counters: ActiveSubscriptionCounters,
    isolation_identity: ForgeQueryEvidenceIdentity,
}

impl PreviewSubscriptionIsolationArtifact {
    pub(super) fn new(
        attachment: &SubscriptionConsumerAttachment,
        preview_epoch: impl Into<String>,
        preview_residue_budget_width: PreviewResidueWidth,
    ) -> Self {
        let preview_epoch_identity = preview_epoch_identity(&preview_epoch.into());
        let mut counters = ActiveSubscriptionCounters::default();
        counters.preview_active_lane_count = 1;
        counters.preview_residue_width = preview_residue_budget_width.get();
        let isolation_identity = preview_isolation_identity(
            attachment.lane_digest().evidence_identity(),
            attachment.attachment_digest().evidence_identity(),
            attachment.future_selection().projection_identity(),
            attachment.basis_binding_identity(),
            attachment.checkpoint_identity(),
            &preview_epoch_identity,
            PreviewSubscriptionLifecycleState::PreviewActive.as_str(),
            preview_residue_budget_width.get(),
            &counters.evidence_identity(),
        );
        Self {
            active_lane_digest: attachment.lane_digest().clone(),
            attachment_digest: attachment.attachment_digest().clone(),
            future_selection: attachment.future_selection().clone(),
            basis_binding_identity: attachment.basis_binding_identity().clone(),
            checkpoint_identity: attachment.checkpoint_identity().clone(),
            preview_epoch_identity,
            lifecycle_state: PreviewSubscriptionLifecycleState::PreviewActive,
            preview_residue_budget_width,
            counters,
            isolation_identity,
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

    pub fn lifecycle_state(&self) -> PreviewSubscriptionLifecycleState {
        self.lifecycle_state
    }

    pub fn preview_residue_budget_width(&self) -> u64 {
        self.preview_residue_budget_width.get()
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }

    pub fn isolation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.isolation_identity
    }

    pub fn isolation_for_reporting(&self) -> &str {
        self.isolation_identity.as_str()
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
    let denial_identity = preview_authoritative_sharing_denial_identity(
        isolation.isolation_identity(),
        authoritative_lane.lane_digest().evidence_identity(),
        isolation.basis_binding_identity(),
        authoritative_lane.basis_binding_identity(),
        isolation.checkpoint_identity(),
        authoritative_lane.checkpoint_identity(),
    );
    Err(PreviewSubscriptionIsolationError::new(
        PreviewSubscriptionIsolationDenialKind::PreviewAuthoritativeSharingDenied,
        "preview subscription isolation cannot share attachment or fanout state with an authoritative active lane",
        denial_identity.as_str(),
        counters,
    ))
}
