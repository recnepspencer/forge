#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BridgeTemporalAsyncFailureSubcode {
    TemporalBasisMissing,
    TemporalBasisIncompatible,
    TemporalBasisStale,
    TemporalBasisCrossBranch,
    TemporalReadinessNotReady,
    TemporalReadinessWakeMissing,
    TemporalReadinessPreviousValueMissing,
    AsyncIdentitySourceMismatch,
    AsyncIdentityBasisMismatch,
    AsyncIdentityPreviewMismatch,
    AsyncIdentityGenerationDrift,
    AsyncIdentitySubscriptionInstanceDrift,
    CompletionAdmissionEnvelopeInvalid,
    CompletionAdmissionTransportRejected,
    CompletionAdmissionLifecycleDenied,
    SupersessionTruthBasis,
    SupersessionPreview,
    SupersessionBranch,
    SupersessionSubscriptionInstance,
    SupersessionGeneration,
    RetryRevalidationRetryRejected,
    RetryRevalidationRevalidationRejected,
    RetryRevalidationTimeout,
    RetryRevalidationCancelled,
    OrderingDuplicateCause,
    OrderingSuppressedCause,
    OrderingReplayDrift,
    ResumeBasisMissing,
    ResumeBasisIncompatible,
    ResumeBasisStale,
    ResumeBasisTruncated,
    PreviewBoundaryDiscardResidue,
    PreviewBoundaryPromotionMismatch,
    PreviewBoundaryPreviewCrossedCompletion,
    PolicyRemaskTenantDrift,
    PolicyRemaskPolicyDrift,
    PolicyRemaskSchemaContextDrift,
    WritebackBoundaryAuthorityRejected,
    WritebackBoundaryMapperFailed,
    WritebackBoundaryLoopPrevented,
    WritebackBoundaryIdempotentNoop,
}

impl BridgeTemporalAsyncFailureSubcode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemporalBasisMissing => "temporal_basis.missing",
            Self::TemporalBasisIncompatible => "temporal_basis.incompatible",
            Self::TemporalBasisStale => "temporal_basis.stale",
            Self::TemporalBasisCrossBranch => "temporal_basis.cross_branch",
            Self::TemporalReadinessNotReady => "temporal_readiness.not_ready",
            Self::TemporalReadinessWakeMissing => "temporal_readiness.wake_missing",
            Self::TemporalReadinessPreviousValueMissing => {
                "temporal_readiness.previous_value_missing"
            }
            Self::AsyncIdentitySourceMismatch => "async_identity.source_mismatch",
            Self::AsyncIdentityBasisMismatch => "async_identity.basis_mismatch",
            Self::AsyncIdentityPreviewMismatch => "async_identity.preview_mismatch",
            Self::AsyncIdentityGenerationDrift => "async_identity.generation_drift",
            Self::AsyncIdentitySubscriptionInstanceDrift => {
                "async_identity.subscription_instance_drift"
            }
            Self::CompletionAdmissionEnvelopeInvalid => "completion_admission.envelope_invalid",
            Self::CompletionAdmissionTransportRejected => "completion_admission.transport_rejected",
            Self::CompletionAdmissionLifecycleDenied => "completion_admission.lifecycle_denied",
            Self::SupersessionTruthBasis => "supersession.truth_basis",
            Self::SupersessionPreview => "supersession.preview",
            Self::SupersessionBranch => "supersession.branch",
            Self::SupersessionSubscriptionInstance => "supersession.subscription_instance",
            Self::SupersessionGeneration => "supersession.generation",
            Self::RetryRevalidationRetryRejected => "retry_revalidation.retry_rejected",
            Self::RetryRevalidationRevalidationRejected => {
                "retry_revalidation.revalidation_rejected"
            }
            Self::RetryRevalidationTimeout => "retry_revalidation.timeout",
            Self::RetryRevalidationCancelled => "retry_revalidation.cancelled",
            Self::OrderingDuplicateCause => "ordering.duplicate_cause",
            Self::OrderingSuppressedCause => "ordering.suppressed_cause",
            Self::OrderingReplayDrift => "ordering.replay_drift",
            Self::ResumeBasisMissing => "resume_basis.missing",
            Self::ResumeBasisIncompatible => "resume_basis.incompatible",
            Self::ResumeBasisStale => "resume_basis.stale",
            Self::ResumeBasisTruncated => "resume_basis.truncated",
            Self::PreviewBoundaryDiscardResidue => "preview_boundary.discard_residue",
            Self::PreviewBoundaryPromotionMismatch => "preview_boundary.promotion_mismatch",
            Self::PreviewBoundaryPreviewCrossedCompletion => {
                "preview_boundary.preview_crossed_completion"
            }
            Self::PolicyRemaskTenantDrift => "policy_remask.tenant_drift",
            Self::PolicyRemaskPolicyDrift => "policy_remask.policy_drift",
            Self::PolicyRemaskSchemaContextDrift => "policy_remask.schema_context_drift",
            Self::WritebackBoundaryAuthorityRejected => "writeback_boundary.authority_rejected",
            Self::WritebackBoundaryMapperFailed => "writeback_boundary.mapper_failed",
            Self::WritebackBoundaryLoopPrevented => "writeback_boundary.loop_prevented",
            Self::WritebackBoundaryIdempotentNoop => "writeback_boundary.idempotent_noop",
        }
    }
}
