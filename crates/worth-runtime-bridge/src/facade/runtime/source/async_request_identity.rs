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

    /// Admits one managed execution intent through a fresh Signal request and
    /// binds that attempt to one materialized truth view.
    ///
    /// The returned authority is move-only and reserves the exact managed
    /// intent until Signal is explicitly terminalized or the basis is dropped.
    pub fn admit_managed_execution_basis(
        &self,
        intent: BridgeManagedExecutionIntent,
        step_contract: BridgeManagedExecutionStepContract,
        truth_basis: BridgeAsyncRequestTruthViewBasis,
        planned: PlannedTruthViewPacket,
    ) -> Result<BridgeBoundExecutionBasis, BridgeExecutionBasisDenial> {
        crate::execution_basis::admit_managed_execution_basis(
            self,
            intent,
            step_contract,
            truth_basis,
            planned,
        )
    }

    pub fn preflight_yielded_execution_basis(
        &self,
        yielded: BridgeYieldedExecutionBasis,
        operation_binding_identity: &str,
    ) -> Result<
        crate::execution_basis::BridgeYieldedExecutionBasisPreflight,
        BridgeExecutionBasisReadmissionDenied,
    > {
        crate::execution_basis::preflight_yielded_execution_basis(
            self,
            yielded,
            operation_binding_identity,
        )
    }

    pub fn readmit_yielded_execution_basis(
        &self,
        preflight: crate::execution_basis::BridgeYieldedExecutionBasisPreflight,
        fresh_intent: BridgeManagedExecutionIntent,
    ) -> BridgeExecutionBasisReadmissionOutcome {
        crate::execution_basis::readmit_yielded_execution_basis(self, preflight, fresh_intent)
    }
}
