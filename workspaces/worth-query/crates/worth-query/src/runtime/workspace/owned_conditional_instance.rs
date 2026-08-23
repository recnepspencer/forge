use super::WorthQueryWorkspace;

impl WorthQueryWorkspace {
    pub(crate) fn resolve_owned_conditional_instance<D: 'static, O: 'static, F: 'static>(
        &self,
        instance: &crate::runtime::WorthQueryInstalledOwnedConditionalInstance,
    ) -> Result<
        std::sync::Arc<crate::domain_installation::WorthQueryInstalledConditionalNode>,
        crate::runtime::WorthQueryOwnedConditionalInstanceDenial,
    > {
        if instance.runtime_authority() != self.runtime.authority_identity.as_u64() {
            return Err(crate::runtime::WorthQueryOwnedConditionalInstanceDenial::ForeignRuntime);
        }
        let signal =
            self.runtime.conditional_signal_runtime.as_ref().ok_or(
                crate::runtime::WorthQueryOwnedConditionalInstanceDenial::MissingOwnedRuntime,
            )?;
        if instance.signal_graph_instance() != signal.owned_signal_graph_instance_id() {
            return Err(crate::runtime::WorthQueryOwnedConditionalInstanceDenial::SuccessorRuntime);
        }
        self.runtime
            .conditional_execution_registry
            .owned_instance::<D, O, F>(instance.instance_identity())
            .ok_or(crate::runtime::WorthQueryOwnedConditionalInstanceDenial::ForeignRuntime)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn install_owned_conditional_instance<D, O, F, G, P>(
        &mut self,
        _domain: D,
        _operation: O,
        _family: F,
        _graph: G,
        location: crate::domain_installation::WorthQueryConditionalNodeLocation,
        dependencies: Vec<
            crate::domain_installation::WorthQueryOwnedConditionalDependencyInstallation,
        >,
        providers: worth_runtime_bridge::facade::BridgeConditionalProviderSet,
        compute: P,
    ) -> Result<
        crate::runtime::WorthQueryInstalledOwnedConditionalInstance,
        crate::runtime::WorthQueryOwnedConditionalInstanceDenial,
    >
    where
        D: 'static,
        O: 'static,
        F: 'static,
        G: 'static,
        P: crate::domain_installation::WorthQueryConditionalNodeComputeProvider<D, O, F>,
    {
        let runtime_authority = self.runtime.authority_identity.as_u64();
        let signal =
            self.runtime.conditional_signal_runtime.as_mut().ok_or(
                crate::runtime::WorthQueryOwnedConditionalInstanceDenial::MissingOwnedRuntime,
            )?;
        let graph_instance = signal.owned_signal_graph_instance_id();
        let pending = crate::domain_installation::PendingOwnedConditionalNode::<D, O, F, G, P>::new(
            location,
            dependencies,
            providers,
            compute,
        );
        let (identity, _) = pending
            .install_owned_instance(
                &self.runtime.domain_installation_registry,
                &self.runtime.graph_participation_registry,
                signal,
                &mut self.runtime.conditional_execution_registry,
            )
            .map_err(crate::runtime::WorthQueryOwnedConditionalInstanceDenial::Installation)?;
        Ok(
            crate::runtime::WorthQueryInstalledOwnedConditionalInstance::new(
                runtime_authority,
                graph_instance,
                identity,
            ),
        )
    }

    pub fn publish_owned_conditional_instance_change<D: 'static, O: 'static, F: 'static>(
        &mut self,
        _domain: D,
        _operation: O,
        _family: F,
        instance: &crate::runtime::WorthQueryInstalledOwnedConditionalInstance,
        dependency_ordinal: usize,
    ) -> Result<
        worth_runtime_bridge::facade::CorrespondenceDeliveryOutcome,
        crate::runtime::WorthQueryOwnedConditionalInstanceDenial,
    > {
        let node = self.resolve_owned_conditional_instance::<D, O, F>(instance)?;
        let signal =
            self.runtime.conditional_signal_runtime.as_mut().ok_or(
                crate::runtime::WorthQueryOwnedConditionalInstanceDenial::MissingOwnedRuntime,
            )?;
        let outcome = signal
            .deliver_owned_authoritative_change(&node.lowering, dependency_ordinal)
            .map_err(|denial| {
                crate::runtime::WorthQueryOwnedConditionalInstanceDenial::Delivery(
                    crate::domain_installation::WorthQueryConditionalDeliveryDenial::bridge(denial),
                )
            })?;
        if let worth_proof::TransitionOutcome::Success(receipt) = &outcome {
            self.runtime
                .stage_conditional_owner_delivery::<D, O, F>(receipt);
        }
        Ok(outcome)
    }

    pub fn retire_owned_conditional_instance<D: 'static, O: 'static, F: 'static>(
        &mut self,
        _domain: D,
        _operation: O,
        _family: F,
        instance: &crate::runtime::WorthQueryInstalledOwnedConditionalInstance,
    ) -> Result<(), crate::runtime::WorthQueryOwnedConditionalInstanceDenial> {
        let node = self.resolve_owned_conditional_instance::<D, O, F>(instance)?;
        let signal =
            self.runtime.conditional_signal_runtime.as_mut().ok_or(
                crate::runtime::WorthQueryOwnedConditionalInstanceDenial::MissingOwnedRuntime,
            )?;
        signal
            .retire_owned_conditional(&node.lowering)
            .map_err(|denial| {
                crate::runtime::WorthQueryOwnedConditionalInstanceDenial::Delivery(
                    crate::domain_installation::WorthQueryConditionalDeliveryDenial::bridge(denial),
                )
            })?;
        self.runtime
            .conditional_execution_registry
            .remove_owned_instance::<D, O, F>(instance.instance_identity())
            .ok_or(crate::runtime::WorthQueryOwnedConditionalInstanceDenial::ForeignRuntime)?;
        Ok(())
    }
}
