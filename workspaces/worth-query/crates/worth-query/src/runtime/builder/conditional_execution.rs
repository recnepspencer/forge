use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub(crate) fn conditional_runtime_for_test(
        mut self,
        bridge: worth_runtime_bridge::facade::RuntimeBridge,
        graph: worth_signal::facade::SignalGraph,
    ) -> Self {
        self.conditional_runtime_bridge = Some(bridge);
        self.conditional_signal_graph = Some(graph);
        self
    }

    /// Supplies the one Signal graph retained by the Query runtime's existing
    /// Runtime Bridge. Conditional declarations cannot build without it.
    pub fn conditional_signal_graph(mut self, graph: worth_signal::facade::SignalGraph) -> Self {
        self.conditional_signal_graph = Some(graph);
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn conditional_node<D, O, F, G, P>(
        mut self,
        _domain: D,
        _operation: O,
        _family: F,
        _graph: G,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependencies: Vec<crate::domain_installation::WorthQueryConditionalDependencyInstallation>,
        providers: worth_runtime_bridge::facade::BridgeConditionalProviderSet,
        compute: P,
    ) -> Self
    where
        D: 'static,
        O: 'static,
        F: 'static,
        G: 'static,
        P: crate::domain_installation::WorthQueryConditionalNodeComputeProvider<D, O, F>,
    {
        self.pending_conditional_installations.push(Box::new(
            crate::domain_installation::PendingConditionalNode::<D, O, F, G, P>::new(
                location,
                dependencies,
                providers,
                compute,
            ),
        ));
        self
    }

    pub(crate) fn owned_conditional_runtime_for_test(
        mut self,
        bridge: worth_runtime_bridge::facade::RuntimeBridge,
    ) -> Self {
        self.conditional_runtime_bridge = Some(bridge);
        self
    }

    pub fn owned_topology_conditional_node<D, O, F, G, P>(
        mut self,
        _domain: D,
        _operation: O,
        _family: F,
        _graph: G,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependencies: Vec<
            crate::domain_installation::WorthQueryOwnedConditionalDependencyInstallation,
        >,
        providers: worth_runtime_bridge::facade::BridgeConditionalProviderSet,
        compute: P,
    ) -> Self
    where
        D: 'static,
        O: 'static,
        F: 'static,
        G: 'static,
        P: crate::domain_installation::WorthQueryConditionalNodeComputeProvider<D, O, F>,
    {
        self.pending_conditional_installations.push(Box::new(
            crate::domain_installation::PendingOwnedConditionalNode::<D, O, F, G, P>::new(
                location,
                dependencies,
                providers,
                compute,
            ),
        ));
        self
    }

    pub fn owned_topology_conditional_instances<D, O, F, G, P>(
        mut self,
        _domain: D,
        _operation: O,
        _family: F,
        _graph: G,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        compute_contract: P,
    ) -> Self
    where
        D: 'static,
        O: 'static,
        F: 'static,
        G: 'static,
        P: crate::domain_installation::WorthQueryConditionalNodeComputeProvider<D, O, F>,
    {
        self.pending_conditional_installations.push(Box::new(
            crate::domain_installation::PendingOwnedConditionalInstanceFamily::<D, O, F, G, P>::new(
                location,
                compute_contract,
            ),
        ));
        self
    }

    pub(super) fn install_conditional_execution(
        conditional_runtime_bridge: Option<worth_runtime_bridge::facade::RuntimeBridge>,
        conditional_signal_graph: Option<worth_signal::facade::SignalGraph>,
        pending_conditional_installations: &[Box<
            dyn crate::domain_installation::PendingConditionalInstallation,
        >],
        domains: &crate::domain_installation::WorthQueryDomainInstallationRegistry,
        graphs: &crate::domain_installation::WorthQueryInstalledGraphParticipationRegistry,
    ) -> Result<
        (
            Option<worth_runtime_bridge::facade::BridgeOwnedSignalRuntime>,
            crate::domain_installation::WorthQueryConditionalExecutionRegistry,
        ),
        crate::runtime::WorthQueryRuntimeError,
    > {
        let expected = domains
            .execution_index()
            .domain_operation_execution_descriptors()
            .iter()
            .map(|descriptor| descriptor.conditional_node_count)
            .sum::<usize>();
        if expected != pending_conditional_installations.len() {
            return Err(conditional_installation_error(format!(
                "installed declarations require {expected} conditional registrations, found {}",
                pending_conditional_installations.len()
            )));
        }
        if expected == 0 {
            if conditional_signal_graph.is_some() {
                return Err(conditional_installation_error(
                    "a conditional Signal graph was supplied without conditional declarations",
                ));
            }
            return Ok((None, Default::default()));
        }
        let bridge = conditional_runtime_bridge.ok_or_else(|| {
            conditional_installation_error(
                "conditional declarations require the exact Runtime Bridge selected by runtime_bridge(...)"
            )
        })?;
        let requires_external_graph = pending_conditional_installations
            .iter()
            .any(|pending| pending.requires_external_signal_graph());
        let mut signal = match (requires_external_graph, conditional_signal_graph) {
            (true, Some(graph)) => {
                worth_runtime_bridge::facade::BridgeOwnedSignalRuntime::new(bridge, graph)
            }
            (true, None) => {
                return Err(conditional_installation_error(
                    "conditional declarations with exact targets require one owned Signal graph",
                ));
            }
            (false, None) => {
                worth_runtime_bridge::facade::BridgeOwnedSignalRuntime::with_owned_signal_graph(
                    bridge,
                )
            }
            (false, Some(_)) => {
                return Err(conditional_installation_error(
                    "owned-topology conditional declarations reject caller-supplied Signal graphs",
                ));
            }
        }
        .map_err(|denial| {
            conditional_installation_error(format!("{:?}: {}", denial.kind(), denial.detail()))
        })?;
        let mut installed =
            crate::domain_installation::WorthQueryConditionalExecutionRegistry::default();
        for pending in pending_conditional_installations {
            pending
                .install(domains, graphs, &mut signal, &mut installed)
                .map_err(|denial| conditional_installation_error(format!("{denial:?}")))?;
        }
        let expected_installed = pending_conditional_installations
            .iter()
            .map(|pending| pending.installed_node_count())
            .sum::<usize>();
        if installed.registration_len() != expected_installed {
            return Err(conditional_installation_error(
                "conditional registration set did not converge on the declared node set",
            ));
        }
        Ok((Some(signal), installed))
    }

    pub(crate) fn reinstall_conditional_execution(
        current: Option<&worth_runtime_bridge::facade::BridgeOwnedSignalRuntime>,
        retained_installations: &[Box<
            dyn crate::domain_installation::PendingConditionalInstallation,
        >],
        domains: &crate::domain_installation::WorthQueryDomainInstallationRegistry,
        graphs: &crate::domain_installation::WorthQueryInstalledGraphParticipationRegistry,
    ) -> Result<
        (
            Option<worth_runtime_bridge::facade::BridgeOwnedSignalRuntime>,
            crate::domain_installation::WorthQueryConditionalExecutionRegistry,
        ),
        crate::runtime::WorthQueryRuntimeError,
    > {
        if retained_installations.is_empty() {
            return Ok((None, Default::default()));
        }
        let mut signal = current
            .ok_or_else(|| {
                conditional_installation_error(
                    "conditional successor installation requires the current Bridge-owned Signal runtime",
                )
            })?
            .successor_installation_runtime()
            .map_err(|denial| {
                conditional_installation_error(format!("{:?}: {}", denial.kind(), denial.detail()))
            })?;
        let mut installed =
            crate::domain_installation::WorthQueryConditionalExecutionRegistry::default();
        for retained in retained_installations {
            retained
                .install(domains, graphs, &mut signal, &mut installed)
                .map_err(|denial| conditional_installation_error(format!("{denial:?}")))?;
        }
        let expected_installed = retained_installations
            .iter()
            .map(|pending| pending.installed_node_count())
            .sum::<usize>();
        if installed.registration_len() != expected_installed {
            return Err(conditional_installation_error(
                "conditional successor registration set did not converge",
            ));
        }
        Ok((Some(signal), installed))
    }
}

fn conditional_installation_error(
    message: impl Into<String>,
) -> crate::runtime::WorthQueryRuntimeError {
    crate::runtime::WorthQueryRuntimeError::InvariantRegistration {
        stage: "conditional_node_installation",
        message: message.into(),
    }
}
