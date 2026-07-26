use super::*;

impl RuntimeBridge {
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

    pub fn commit_yielded_execution_basis_readmission(
        &self,
        pending: BridgeExecutionBasisReadmissionPending,
    ) -> BridgeBoundExecutionBasis {
        assert_eq!(
            pending.runtime_key(),
            self.signal_runtime_key,
            "only the RuntimeBridge that admitted a yielded-basis readmission may commit it",
        );
        pending.commit()
    }
}
