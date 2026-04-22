use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::speculation::BridgePreviewPromotionRecord;

use super::{
    BridgeAdmittedSubscriptionIdentity, BridgePreviewActiveSubscription,
    BridgePreviewActiveSubscriptionIdentity, BridgeSubscriptionActivationReady,
    BridgeSubscriptionCounters, BridgeSubscriptionLifecycleIdentity,
    BridgeSubscriptionPreviewBasisIdentity, BridgeSubscriptionPreviewLifecycleIdentity,
    BridgeSubscriptionPreviewPromotionRecordIdentity, BridgeSubscriptionPreviewScopeIdentity,
    BridgeSubscriptionPreviewWorkTrace, BridgeSubscriptionPreviewWorkTraceIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewPromotionOutcomeClass {
    PromotedAuthoritativeBoundary,
}

impl BridgeSubscriptionPreviewPromotionOutcomeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromotedAuthoritativeBoundary => "promoted_authoritative_boundary",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewPromotionRejectionKind {
    PromotionSessionMismatch,
    PromotionExecutionRecordMismatch,
    PromotedSubscriptionMismatch,
    PreviewWorkTraceMismatch,
}

impl BridgeSubscriptionPreviewPromotionRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromotionSessionMismatch => "promotion_session_mismatch",
            Self::PromotionExecutionRecordMismatch => "promotion_execution_record_mismatch",
            Self::PromotedSubscriptionMismatch => "promoted_subscription_mismatch",
            Self::PreviewWorkTraceMismatch => "preview_work_trace_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewPromotionRejection {
    rejection_kind: BridgeSubscriptionPreviewPromotionRejectionKind,
    rejection_context: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewPromotionRejection {
    fn new(
        rejection_kind: BridgeSubscriptionPreviewPromotionRejectionKind,
        rejection_context: impl Into<Arc<str>>,
    ) -> Self {
        let rejection_context = rejection_context.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-promotion-rejection|kind={}|context={}",
            rejection_kind.as_str(),
            rejection_context.as_ref()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            rejection_context,
            counters: BridgeSubscriptionCounters::from_subscription_preview_promotion_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-promotion-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionPreviewPromotionRejectionKind {
        self.rejection_kind
    }

    pub fn rejection_context(&self) -> &str {
        self.rejection_context.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewPromotionRecord {
    promotion_record_identity: BridgeSubscriptionPreviewPromotionRecordIdentity,
    outcome_class: BridgeSubscriptionPreviewPromotionOutcomeClass,
    preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    preview_basis_identity: BridgeSubscriptionPreviewBasisIdentity,
    preview_scope_identity: BridgeSubscriptionPreviewScopeIdentity,
    preview_lifecycle_identity: BridgeSubscriptionPreviewLifecycleIdentity,
    preview_work_trace_identity: BridgeSubscriptionPreviewWorkTraceIdentity,
    preview_work_trace_digest: Arc<str>,
    promoted_admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    promoted_lifecycle_identity: BridgeSubscriptionLifecycleIdentity,
    speculation_promotion_record_digest: Arc<str>,
    authoritative_commit_boundary_digest: Arc<str>,
    authoritative_artifact_digest: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewPromotionRecord {
    pub(crate) fn promote(
        preview_active: BridgePreviewActiveSubscription,
        preview_work_trace: &BridgeSubscriptionPreviewWorkTrace,
        promotion_record: &BridgePreviewPromotionRecord,
        promoted_activation_ready: &BridgeSubscriptionActivationReady,
    ) -> Result<Self, BridgeSubscriptionPreviewPromotionRejection> {
        if preview_work_trace.preview_active_subscription_identity()
            != preview_active.preview_active_subscription_identity()
        {
            return Err(BridgeSubscriptionPreviewPromotionRejection::new(
                BridgeSubscriptionPreviewPromotionRejectionKind::PreviewWorkTraceMismatch,
                format!(
                    "preview-active={}|trace-preview-active={}",
                    preview_active
                        .preview_active_subscription_identity()
                        .as_str(),
                    preview_work_trace
                        .preview_active_subscription_identity()
                        .as_str()
                ),
            ));
        }
        if preview_work_trace.preview_scope_identity() != preview_active.preview_scope_identity() {
            return Err(BridgeSubscriptionPreviewPromotionRejection::new(
                BridgeSubscriptionPreviewPromotionRejectionKind::PreviewWorkTraceMismatch,
                format!(
                    "preview-active={}|preview-scope={}|trace-scope={}",
                    preview_active
                        .preview_active_subscription_identity()
                        .as_str(),
                    preview_active.preview_scope_identity().as_str(),
                    preview_work_trace.preview_scope_identity().as_str()
                ),
            ));
        }
        if promotion_record.preview_session_identity()
            != preview_active.preview_session_identity().as_str()
        {
            return Err(BridgeSubscriptionPreviewPromotionRejection::new(
                BridgeSubscriptionPreviewPromotionRejectionKind::PromotionSessionMismatch,
                format!(
                    "preview-active={}|preview-session={}|promotion-session={}",
                    preview_active
                        .preview_active_subscription_identity()
                        .as_str(),
                    preview_active.preview_session_identity().as_str(),
                    promotion_record.preview_session_identity()
                ),
            ));
        }
        if promotion_record.preview_execution_record_identity()
            != preview_active.preview_execution_record_identity()
        {
            return Err(BridgeSubscriptionPreviewPromotionRejection::new(
                BridgeSubscriptionPreviewPromotionRejectionKind::PromotionExecutionRecordMismatch,
                format!(
                    "preview-active={}|preview-execution={}|promotion-execution={}",
                    preview_active
                        .preview_active_subscription_identity()
                        .as_str(),
                    preview_active.preview_execution_record_identity().as_str(),
                    promotion_record
                        .preview_execution_record_identity()
                        .as_str()
                ),
            ));
        }
        if promoted_activation_ready
            .admitted()
            .admitted_subscription_identity()
            != preview_active.admitted_subscription_identity()
        {
            return Err(BridgeSubscriptionPreviewPromotionRejection::new(
                BridgeSubscriptionPreviewPromotionRejectionKind::PromotedSubscriptionMismatch,
                format!(
                    "preview-active={}|preview-admitted={}|promoted-admitted={}",
                    preview_active
                        .preview_active_subscription_identity()
                        .as_str(),
                    preview_active.admitted_subscription_identity().as_str(),
                    promoted_activation_ready
                        .admitted()
                        .admitted_subscription_identity()
                        .as_str()
                ),
            ));
        }

        let outcome_class =
            BridgeSubscriptionPreviewPromotionOutcomeClass::PromotedAuthoritativeBoundary;
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-promotion-record|outcome={}|preview-active={}|preview-digest={}|preview-basis={}|preview-scope={}|preview-lifecycle={}|preview-work-trace={}|preview-work-digest={}|preview-session={}|preview-execution={}|promoted-admitted={}|promoted-lifecycle={}|promotion-record={}|promotion-digest={}|commit-boundary={}|authoritative-artifact={}",
            outcome_class.as_str(),
            preview_active.preview_active_subscription_identity().as_str(),
            preview_active.digest(),
            preview_active.preview_basis_identity().as_str(),
            preview_active.preview_scope_identity().as_str(),
            preview_active.preview_lifecycle_identity().as_str(),
            preview_work_trace.preview_work_trace_identity().as_str(),
            preview_work_trace.digest(),
            preview_active.preview_session_identity().as_str(),
            preview_active.preview_execution_record_identity().as_str(),
            promoted_activation_ready
                .admitted()
                .admitted_subscription_identity()
                .as_str(),
            promoted_activation_ready
                .lifecycle_record()
                .lifecycle_identity()
                .as_str(),
            promotion_record.record_identity().as_str(),
            promotion_record.digest(),
            promotion_record.authoritative_commit_boundary_digest(),
            promotion_record.authoritative_artifact_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            promotion_record_identity: BridgeSubscriptionPreviewPromotionRecordIdentity::new(
                format!("bridge-subscription-preview-promotion-record-id:sha256:{digest:x}"),
            ),
            outcome_class,
            preview_active_subscription_identity: preview_active
                .preview_active_subscription_identity()
                .clone(),
            preview_basis_identity: preview_active.preview_basis_identity().clone(),
            preview_scope_identity: preview_active.preview_scope_identity().clone(),
            preview_lifecycle_identity: preview_active.preview_lifecycle_identity().clone(),
            preview_work_trace_identity: preview_work_trace.preview_work_trace_identity().clone(),
            preview_work_trace_digest: Arc::from(preview_work_trace.digest()),
            promoted_admitted_subscription_identity: promoted_activation_ready
                .admitted()
                .admitted_subscription_identity()
                .clone(),
            promoted_lifecycle_identity: promoted_activation_ready
                .lifecycle_record()
                .lifecycle_identity()
                .clone(),
            speculation_promotion_record_digest: Arc::from(promotion_record.digest()),
            authoritative_commit_boundary_digest: Arc::from(
                promotion_record.authoritative_commit_boundary_digest(),
            ),
            authoritative_artifact_digest: Arc::from(
                promotion_record.authoritative_artifact_digest(),
            ),
            counters: BridgeSubscriptionCounters::from_subscription_preview_promotion(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-promotion-record:sha256:{digest:x}"
            )),
        })
    }

    pub fn promotion_record_identity(&self) -> &BridgeSubscriptionPreviewPromotionRecordIdentity {
        &self.promotion_record_identity
    }

    pub fn outcome_class(&self) -> BridgeSubscriptionPreviewPromotionOutcomeClass {
        self.outcome_class
    }

    pub fn preview_active_subscription_identity(&self) -> &BridgePreviewActiveSubscriptionIdentity {
        &self.preview_active_subscription_identity
    }

    pub fn preview_basis_identity(&self) -> &BridgeSubscriptionPreviewBasisIdentity {
        &self.preview_basis_identity
    }

    pub fn preview_scope_identity(&self) -> &BridgeSubscriptionPreviewScopeIdentity {
        &self.preview_scope_identity
    }

    pub fn preview_lifecycle_identity(&self) -> &BridgeSubscriptionPreviewLifecycleIdentity {
        &self.preview_lifecycle_identity
    }

    pub fn preview_work_trace_identity(&self) -> &BridgeSubscriptionPreviewWorkTraceIdentity {
        &self.preview_work_trace_identity
    }

    pub fn preview_work_trace_digest(&self) -> &str {
        self.preview_work_trace_digest.as_ref()
    }

    pub fn promoted_admitted_subscription_identity(&self) -> &BridgeAdmittedSubscriptionIdentity {
        &self.promoted_admitted_subscription_identity
    }

    pub fn promoted_lifecycle_identity(&self) -> &BridgeSubscriptionLifecycleIdentity {
        &self.promoted_lifecycle_identity
    }

    pub fn speculation_promotion_record_digest(&self) -> &str {
        self.speculation_promotion_record_digest.as_ref()
    }

    pub fn authoritative_commit_boundary_digest(&self) -> &str {
        self.authoritative_commit_boundary_digest.as_ref()
    }

    pub fn authoritative_artifact_digest(&self) -> &str {
        self.authoritative_artifact_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
