use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::super::super::evidence_identities::typed_identity_drift;
use super::super::super::validation_evidence::validation_evidence_identity_label;
use super::super::stage::QuerySubscriptionDiagnosticStage;
use super::super::trace::QuerySubscriptionDiagnosticTrace;
use super::failure::{
    QuerySubscriptionDiagnosticBundleError, QuerySubscriptionDiagnosticBundleErrorKind,
};

pub(super) fn validate_trace_terminal_stage(
    trace: &QuerySubscriptionDiagnosticTrace,
    expected_terminal_stage: QuerySubscriptionDiagnosticStage,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if trace.terminal_stage() != &expected_terminal_stage {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
            "diagnostic bundle assembly requires the trace terminal stage to match the assembled outcome",
            &[
                format!("trace_terminal_stage:{}", trace.terminal_stage().as_str()),
                format!("expected_terminal_stage:{}", expected_terminal_stage.as_str()),
            ],
        ));
    }
    Ok(())
}

pub(super) fn validate_trace_stage_source(
    trace: &QuerySubscriptionDiagnosticTrace,
    stage: QuerySubscriptionDiagnosticStage,
    expected_source_identity: &WorthQueryEvidenceIdentity,
    message: &'static str,
    error_kind: QuerySubscriptionDiagnosticBundleErrorKind,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    let stage_trace = trace_stage(trace, stage).ok_or_else(|| {
        QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
            "diagnostic bundle assembly requires the trace to carry every stage that the assembled artifacts claim",
            &[
                format!("trace_terminal_stage:{}", trace.terminal_stage().as_str()),
                format!("missing_stage:{}", stage.as_str()),
            ],
        )
    })?;

    if typed_identity_drift(stage_trace.source_identity(), expected_source_identity) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            error_kind,
            message,
            &[
                format!("stage:{}", stage.as_str()),
                format!(
                    "trace_source:{}",
                    validation_evidence_identity_label(stage_trace.source_identity())
                ),
                format!(
                    "expected_source:{}",
                    validation_evidence_identity_label(expected_source_identity)
                ),
            ],
        ));
    }

    Ok(())
}

pub(super) fn validate_optional_trace_stage_source(
    trace: &QuerySubscriptionDiagnosticTrace,
    stage: QuerySubscriptionDiagnosticStage,
    expected_source_identity: Option<&WorthQueryEvidenceIdentity>,
    message: &'static str,
    error_kind: QuerySubscriptionDiagnosticBundleErrorKind,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    match (trace_stage(trace, stage), expected_source_identity) {
        (Some(stage_trace), Some(expected_source_identity)) => {
            if typed_identity_drift(stage_trace.source_identity(), expected_source_identity) {
                return Err(QuerySubscriptionDiagnosticBundleError::new(
                    error_kind,
                    message,
                    &[
                        format!("stage:{}", stage.as_str()),
                        format!(
                            "trace_source:{}",
                            validation_evidence_identity_label(stage_trace.source_identity())
                        ),
                        format!(
                            "expected_source:{}",
                            validation_evidence_identity_label(expected_source_identity)
                        ),
                    ],
                ));
            }
        }
        (Some(_), None) => {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
                message,
                &[format!("unexpected_stage:{}", stage.as_str())],
            ));
        }
        (None, Some(_)) => {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
                "diagnostic bundle assembly requires the trace to carry every optional stage claimed by the assembled artifacts",
                &[format!("missing_stage:{}", stage.as_str())],
            ));
        }
        (None, None) => {}
    }

    Ok(())
}

pub(super) fn trace_stage(
    trace: &QuerySubscriptionDiagnosticTrace,
    stage: QuerySubscriptionDiagnosticStage,
) -> Option<&super::super::trace::QuerySubscriptionDiagnosticStageTrace> {
    trace
        .stage_traces()
        .iter()
        .find(|stage_trace| stage_trace.stage() == &stage)
}
