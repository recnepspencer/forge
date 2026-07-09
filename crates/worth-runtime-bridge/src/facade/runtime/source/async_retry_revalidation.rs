use super::*;

impl RuntimeBridge {
    /// Classifies one newer request as a retry lineage after timeout by
    /// consuming the authoritative Signal timeout and retry admission evidence.
    pub fn admit_async_retry_lineage_after_timeout(
        &self,
        request: BridgeAsyncRetryLineageRequest,
    ) -> Result<BridgeAsyncRetryLineage, BridgeAsyncForwardCausalityRejection> {
        crate::source::with_async_request_signal_runtime(self.signal_runtime_key, |signal_runtime| {
            crate::source::admit_retry_lineage(signal_runtime, request)
        })
        .map_err(|error| {
            BridgeAsyncForwardCausalityRejection::new(
                BridgeAsyncForwardCausalityRejectionKind::SignalRuntimeThreadAffinityViolation,
                format!(
                    "bridge async forward causality runtime {} is already bound to thread {:?} and cannot admit from thread {:?}",
                    error.runtime_key(),
                    error.owner(),
                    error.current()
                ),
            )
        })?
    }

    /// Classifies one newer request as a retry lineage after cancellation by
    /// consuming the authoritative Signal cancellation and retry admission evidence.
    pub fn admit_async_retry_lineage_after_cancellation(
        &self,
        request: BridgeAsyncRetryLineageRequest,
    ) -> Result<BridgeAsyncRetryLineage, BridgeAsyncForwardCausalityRejection> {
        crate::source::with_async_request_signal_runtime(self.signal_runtime_key, |signal_runtime| {
            crate::source::admit_retry_lineage(signal_runtime, request)
        })
        .map_err(|error| {
            BridgeAsyncForwardCausalityRejection::new(
                BridgeAsyncForwardCausalityRejectionKind::SignalRuntimeThreadAffinityViolation,
                format!(
                    "bridge async forward causality runtime {} is already bound to thread {:?} and cannot admit from thread {:?}",
                    error.runtime_key(),
                    error.owner(),
                    error.current()
                ),
            )
        })?
    }

    /// Classifies one newer request as a revalidation lineage by consuming the
    /// authoritative Signal revalidation admission evidence.
    pub fn admit_async_revalidation_lineage(
        &self,
        request: BridgeAsyncRevalidationLineageRequest,
    ) -> Result<BridgeAsyncRevalidationLineage, BridgeAsyncForwardCausalityRejection> {
        crate::source::with_async_request_signal_runtime(self.signal_runtime_key, |signal_runtime| {
            crate::source::admit_revalidation_lineage(signal_runtime, request)
        })
        .map_err(|error| {
            BridgeAsyncForwardCausalityRejection::new(
                BridgeAsyncForwardCausalityRejectionKind::SignalRuntimeThreadAffinityViolation,
                format!(
                    "bridge async forward causality runtime {} is already bound to thread {:?} and cannot admit from thread {:?}",
                    error.runtime_key(),
                    error.owner(),
                    error.current()
                ),
            )
        })?
    }
}
