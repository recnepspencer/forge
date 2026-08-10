use crate::data::error::SignalError;
use crate::data::output::NodeEvaluationResult;
use crate::logic::evaluation::{
    EvaluationVerdict, PreviousArtifactWarmSnapshot, SuppressionReason,
};

pub(crate) fn verdict_for_evaluated_result(
    previous_trace: Option<&PreviousArtifactWarmSnapshot>,
    result: &NodeEvaluationResult,
    meaningful_output_change: bool,
) -> Result<EvaluationVerdict, SignalError> {
    let previous_output_identity = previous_trace.and_then(|trace| trace.output_identity.as_ref());
    let previous_continuity_token =
        previous_trace.and_then(|trace| trace.continuity_token.as_ref());

    let output_identity_unchanged = matches!(
        (previous_output_identity, result.output_identity.as_ref()),
        (Some(previous), Some(current)) if previous == current
    );
    let continuity_token_unchanged = matches!(
        (previous_continuity_token, result.continuity_token.as_ref()),
        (Some(previous), Some(current)) if previous == current
    );

    // Suppression precedence is part of the runtime contract:
    // 1. output identity continuity
    // 2. explicit continuity token continuity
    // 3. comparator-match suppression when no recompute occurred
    // 4. otherwise the result is authoritative recomputation
    let verdict = if meaningful_output_change {
        EvaluationVerdict::Recomputed
    } else if output_identity_unchanged {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::OutputIdentityUnchanged,
        }
    } else if continuity_token_unchanged
        && previous_output_identity.is_none()
        && result.output_identity.is_none()
    {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::ContinuityTokenUnchanged,
        }
    } else {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::ComparatorMatch,
        }
    };

    Ok(verdict)
}
