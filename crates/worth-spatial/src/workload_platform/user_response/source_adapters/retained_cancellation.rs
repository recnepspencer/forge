use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::{
    retained_cancellation_chain::{
        RetainedCancellationChainError, RetainedCancellationChainReceipt,
    },
    user_response::{
        choices::overlap_policy_choices, source::WorthUserResponseSourceKind,
        WorthUserOutcomeCauseKind, WorthUserResponseSource,
    },
};

impl WorthUserResponseSource {
    pub fn from_retained_cancellation_chain(receipt: &RetainedCancellationChainReceipt) -> Self {
        let trigger_detail = receipt
            .trigger_checkpoint()
            .map(|checkpoint| {
                format!(
                    " The retained trigger was checkpoint {}.",
                    checkpoint.step_index()
                )
            })
            .unwrap_or_default();
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: format!(
                    "Retained cancellation chain was certified through {} retained checkpoints and {} retained replay samples.{trigger_detail}",
                    receipt.counters().checkpoint_count(),
                    receipt.counters().replayed_checkpoint_count()
                ),
                evidence_digest: receipt.chain_digest().to_string(),
                source_identity: receipt.workload_identity().to_string(),
            },
        }
    }

    pub fn from_retained_cancellation_chain_error(error: RetainedCancellationChainError) -> Self {
        let cause_kind = retained_cancellation_error_cause(&error);
        let evidence_digest = retained_cancellation_error_digest(&error);
        if matches!(error, RetainedCancellationChainError::PolicyRequired { .. }) {
            return Self {
                kind: WorthUserResponseSourceKind::PolicyRequired {
                    message: error.human_reason(),
                    evidence_digest: evidence_digest.clone(),
                    source_identity: evidence_digest,
                    choices: overlap_policy_choices(),
                },
            };
        }
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message: error.human_reason(),
                evidence_digest: evidence_digest.clone(),
                source_identity: evidence_digest,
            },
        }
    }
}

fn retained_cancellation_error_cause(
    error: &RetainedCancellationChainError,
) -> WorthUserOutcomeCauseKind {
    match error {
        RetainedCancellationChainError::PredicateUncertain { .. } => {
            WorthUserOutcomeCauseKind::PredicateUncertain
        }
        RetainedCancellationChainError::TransformInvalidation { .. } => {
            WorthUserOutcomeCauseKind::DeniedMovementOrRotation
        }
        RetainedCancellationChainError::ProjectionConsumedFactMismatch { .. }
        | RetainedCancellationChainError::RetainedReplayMismatch { .. } => {
            WorthUserOutcomeCauseKind::IntegrityMismatch
        }
        RetainedCancellationChainError::PolicyRequired { .. } => {
            WorthUserOutcomeCauseKind::PolicyRequired
        }
        RetainedCancellationChainError::LiveExtractionForbidden => {
            WorthUserOutcomeCauseKind::UnsupportedInput
        }
        _ => WorthUserOutcomeCauseKind::MissingEvidence,
    }
}

fn retained_cancellation_error_digest(error: &RetainedCancellationChainError) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "retained-cancellation-chain-error".to_string(),
            format!("{error:?}"),
            error.human_reason(),
        ],
    )
}
