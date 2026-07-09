use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::policy::BridgePolicyRejection;
use crate::source::{
    BridgeAsyncClassifiedDeniedCompletion, BridgeAsyncCompletionRejection,
    BridgeAsyncForwardCausalityRejection, BridgeAsyncRequestIdentityRejection,
    BridgeAsyncWritebackCommitReport, BridgeAsyncWritebackRejection,
};
use crate::subscription::{
    BridgeDeniedMixedCause, BridgeHistoricalTemporalReplayRejection,
    BridgeSubscriptionPreviewLifecycleDiscardRejection,
    BridgeSubscriptionPreviewLifecyclePromotionRejection, BridgeSubscriptionResumeBasisRejection,
    BridgeSuppressedMixedCause, BridgeTemporalWakeRoutingRejection,
};
use crate::temporal::BridgeTemporalBasisDenial;

use super::mapping_async::{
    localize_async_completion_rejection, localize_async_forward_causality_rejection,
    localize_async_request_identity, localize_async_writeback,
    localize_async_writeback_commit_report, localize_classified_completion,
};
use super::mapping_subscription::{
    localize_historical_temporal_replay_rejection, localize_mixed_cause_denial,
    localize_mixed_cause_suppression, localize_policy_rejection, localize_preview_discard,
    localize_preview_promotion, localize_resume_basis, localize_temporal_basis_denial,
    localize_temporal_wake_routing_rejection,
};
use super::{
    BridgeFailureEvidenceAttachmentSet, BridgeTemporalAsyncFailureClass,
    BridgeTemporalAsyncFailureCounters, BridgeTemporalAsyncFailureSubcode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeFailureLocalizationRequest {
    TemporalBasisDenial(BridgeTemporalBasisDenial),
    HistoricalTemporalReplayRejection(BridgeHistoricalTemporalReplayRejection),
    TemporalWakeRoutingRejection(BridgeTemporalWakeRoutingRejection),
    ResumeBasisRejection(BridgeSubscriptionResumeBasisRejection),
    PreviewDiscardRejection(BridgeSubscriptionPreviewLifecycleDiscardRejection),
    PreviewPromotionRejection(BridgeSubscriptionPreviewLifecyclePromotionRejection),
    MixedCauseDenied(BridgeDeniedMixedCause),
    MixedCauseSuppressed(BridgeSuppressedMixedCause),
    AsyncRequestIdentityRejection(BridgeAsyncRequestIdentityRejection),
    AsyncCompletionRejection(BridgeAsyncCompletionRejection),
    AsyncClassifiedDeniedCompletion(BridgeAsyncClassifiedDeniedCompletion),
    AsyncForwardCausalityRejection(BridgeAsyncForwardCausalityRejection),
    AsyncWritebackRejection(BridgeAsyncWritebackRejection),
    AsyncWritebackCommitReport(BridgeAsyncWritebackCommitReport),
    PolicyRejection(BridgePolicyRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeTemporalAsyncFailureLocalizationRejectionKind {
    UnsupportedFailureArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncFailureLocalizationRejection {
    kind: BridgeTemporalAsyncFailureLocalizationRejectionKind,
    detail: Arc<str>,
}

impl BridgeTemporalAsyncFailureLocalizationRejection {
    pub fn new(
        kind: BridgeTemporalAsyncFailureLocalizationRejectionKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> BridgeTemporalAsyncFailureLocalizationRejectionKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLocalizedTemporalAsyncFailure {
    failure_class: BridgeTemporalAsyncFailureClass,
    subcode: BridgeTemporalAsyncFailureSubcode,
    attachment_set: BridgeFailureEvidenceAttachmentSet,
    detail: Arc<str>,
    counters: BridgeTemporalAsyncFailureCounters,
    digest: Arc<str>,
}

pub(super) type LocalizedFailureParts = (
    BridgeTemporalAsyncFailureClass,
    BridgeTemporalAsyncFailureSubcode,
    Vec<super::BridgeFailureEvidenceAttachment>,
    Arc<str>,
);

impl BridgeLocalizedTemporalAsyncFailure {
    pub(crate) fn localize(
        request: BridgeFailureLocalizationRequest,
    ) -> Result<Self, BridgeTemporalAsyncFailureLocalizationRejection> {
        let (failure_class, subcode, attachments, detail) = match request {
            BridgeFailureLocalizationRequest::TemporalBasisDenial(denial) => {
                localize_temporal_basis_denial(denial)
            }
            BridgeFailureLocalizationRequest::HistoricalTemporalReplayRejection(rejection) => {
                localize_historical_temporal_replay_rejection(rejection)
            }
            BridgeFailureLocalizationRequest::TemporalWakeRoutingRejection(rejection) => {
                localize_temporal_wake_routing_rejection(rejection)
            }
            BridgeFailureLocalizationRequest::ResumeBasisRejection(rejection) => {
                localize_resume_basis(rejection)
            }
            BridgeFailureLocalizationRequest::PreviewDiscardRejection(rejection) => {
                localize_preview_discard(rejection)
            }
            BridgeFailureLocalizationRequest::PreviewPromotionRejection(rejection) => {
                localize_preview_promotion(rejection)
            }
            BridgeFailureLocalizationRequest::MixedCauseDenied(denied) => {
                localize_mixed_cause_denial(denied)
            }
            BridgeFailureLocalizationRequest::MixedCauseSuppressed(suppressed) => {
                localize_mixed_cause_suppression(suppressed)
            }
            BridgeFailureLocalizationRequest::AsyncRequestIdentityRejection(rejection) => {
                localize_async_request_identity(rejection)
            }
            BridgeFailureLocalizationRequest::AsyncCompletionRejection(rejection) => {
                localize_async_completion_rejection(rejection)
            }
            BridgeFailureLocalizationRequest::AsyncClassifiedDeniedCompletion(classified) => {
                localize_classified_completion(classified)
            }
            BridgeFailureLocalizationRequest::AsyncForwardCausalityRejection(rejection) => {
                localize_async_forward_causality_rejection(rejection)
            }
            BridgeFailureLocalizationRequest::AsyncWritebackRejection(rejection) => {
                localize_async_writeback(rejection)
            }
            BridgeFailureLocalizationRequest::AsyncWritebackCommitReport(report) => {
                localize_async_writeback_commit_report(report)
            }
            BridgeFailureLocalizationRequest::PolicyRejection(rejection) => {
                localize_policy_rejection(rejection)
            }
        }?;

        let attachment_set = BridgeFailureEvidenceAttachmentSet::new(attachments);
        let canonical_basis = format!(
            "bridge-localized-temporal-async-failure|class={}|subcode={}|attachments={}",
            failure_class.as_str(),
            subcode.as_str(),
            attachment_set.digest(),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            failure_class,
            subcode,
            attachment_set,
            detail,
            counters: BridgeTemporalAsyncFailureCounters::localized(),
            digest: Arc::from(format!(
                "bridge-localized-temporal-async-failure:sha256:{digest:x}"
            )),
        })
    }

    pub fn failure_class(&self) -> BridgeTemporalAsyncFailureClass {
        self.failure_class
    }

    pub fn subcode(&self) -> BridgeTemporalAsyncFailureSubcode {
        self.subcode
    }

    pub fn attachment_set(&self) -> &BridgeFailureEvidenceAttachmentSet {
        &self.attachment_set
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn counters(&self) -> &BridgeTemporalAsyncFailureCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
