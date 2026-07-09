use crate::boundary::errors::WORTHSignalJsError;

use super::canonical_worker_certification_digest;
use super::worker_main_thread_hosted_callback_boundary::{
    WorkerMainThreadHostedCallbackOutcome, WorkerMainThreadHostedCallbackRequestEnvelope,
    WorkerMainThreadHostedCallbackResult,
};

pub(super) fn validate_main_thread_hosted_callback_result(
    request: &WorkerMainThreadHostedCallbackRequestEnvelope,
    result: &WorkerMainThreadHostedCallbackResult,
) -> Result<(), WORTHSignalJsError> {
    validate_main_thread_hosted_callback_request_envelope(request)?;
    if result.request_digest != request.request_digest {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback result must acknowledge the issued request digest",
        ));
    }
    if result.callback_id != request.callback_id {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback result target must match the issued request",
        ));
    }
    if result.outcome == WorkerMainThreadHostedCallbackOutcome::Completed && result.value.is_none()
    {
        return Err(WORTHSignalJsError::invalid_input(
            "completed main-thread-hosted callback result requires a value",
        ));
    }
    if result.outcome != WorkerMainThreadHostedCallbackOutcome::Completed
        && (result.value.is_some()
            || !result.captured_read_ids.is_empty()
            || !result.captured_host_capability_reads.is_empty())
    {
        return Err(WORTHSignalJsError::invalid_input(
            "failed denied or unavailable main-thread-hosted callback artifacts cannot mutate worker runtime truth",
        ));
    }

    Ok(())
}

pub(super) fn validate_main_thread_hosted_callback_request_envelope(
    request: &WorkerMainThreadHostedCallbackRequestEnvelope,
) -> Result<(), WORTHSignalJsError> {
    if request.envelope_family != "mainThreadHostedCallbackExecution"
        || request.host_execution_boundary != "mainThreadHostedCallback"
    {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback result requires a callback execution request envelope",
        ));
    }
    if request.closed_input_count != request.closed_input_ids.len() as u64 {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback request closed input count must match the closed frontier",
        ));
    }
    let expected_request_digest = canonical_worker_certification_digest(&(
        "mainThreadHostedCallbackRequest",
        request.callback_id.as_str(),
        &request.closed_input_ids,
        request.host_capability_read_count,
        request.closed_payload_digest.as_str(),
    ))?;
    if expected_request_digest != request.request_digest {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback request digest must match its closed frontier envelope",
        ));
    }

    Ok(())
}
