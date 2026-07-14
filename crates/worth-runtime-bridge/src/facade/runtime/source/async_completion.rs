use super::*;

impl RuntimeBridge {
    /// Validates one raw async completion envelope against one admitted bridge
    /// async request identity before any Signal completion admission occurs.
    pub fn validate_async_completion_envelope(
        &self,
        request_identity: &AdmittedBridgeAsyncRequestIdentity,
        raw: worth_signal::facade::RawCompletionEnvelope,
    ) -> Result<ValidatedBridgeAsyncCompletionEnvelope, BridgeAsyncCompletionRejection> {
        let _ = self;
        ValidatedBridgeAsyncCompletionEnvelope::validate(request_identity, raw)
    }

    /// Admits one validated bridge async completion envelope against the
    /// persistent Signal runtime bound to this bridge instance.
    pub fn admit_async_completion(
        &self,
        request_identity: &AdmittedBridgeAsyncRequestIdentity,
        validated: &ValidatedBridgeAsyncCompletionEnvelope,
    ) -> Result<BridgeAsyncCompletionAdmissionReport, BridgeAsyncCompletionRejection> {
        crate::source::with_async_request_signal_runtime(self.signal_runtime_key, |signal_runtime| {
            validated.admit(signal_runtime, request_identity)
        })
        .map_err(|error| {
            BridgeAsyncCompletionRejection::new(
                BridgeAsyncCompletionRejectionKind::SignalRuntimeThreadAffinityViolation,
                format!(
                    "bridge async completion runtime {} is already bound to thread {:?} and cannot admit from thread {:?}",
                    error.runtime_key(),
                    error.owner(),
                    error.current()
                ),
            )
        })?
    }
}
