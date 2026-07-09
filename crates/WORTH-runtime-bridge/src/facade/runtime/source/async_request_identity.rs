use super::*;

impl RuntimeBridge {
    /// Binds one lowered bridge async source declaration to one explicit
    /// truth-view basis before any Signal request generation is admitted.
    pub fn bind_async_request_basis(
        &self,
        lowered: &LoweredBridgeAsyncSourceDeclaration,
        truth_view_basis: BridgeAsyncRequestTruthViewBasis,
    ) -> ValidatedBridgeAsyncRequestBasisBinding {
        let _ = self;
        ValidatedBridgeAsyncRequestBasisBinding::bind(lowered, truth_view_basis)
    }

    /// Admits one bridge-visible async request identity by binding the lowered
    /// bridge declaration to one authoritative Signal request generation.
    pub fn admit_async_request_identity(
        &self,
        request: BridgeAsyncRequestAdmissionRequest,
    ) -> Result<AdmittedBridgeAsyncRequestIdentity, BridgeAsyncRequestIdentityRejection> {
        crate::source::with_async_request_signal_runtime(self.signal_runtime_key, |signal_runtime| {
            AdmittedBridgeAsyncRequestIdentity::admit(
                self.signal_runtime_key,
                signal_runtime,
                request,
            )
        })
        .map_err(|error| {
            BridgeAsyncRequestIdentityRejection::new(
                BridgeAsyncRequestIdentityRejectionKind::SignalRuntimeThreadAffinityViolation,
                format!(
                    "bridge async request identity runtime {} is already bound to thread {:?} and cannot admit from thread {:?}",
                    error.runtime_key(),
                    error.owner(),
                    error.current()
                ),
            )
        })?
    }
}
