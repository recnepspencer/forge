use super::{
    BridgeConditionalDenial, BridgeConditionalDenialKind,
    BridgeConditionalRuntimeReconstitutionReport, BridgeOwnedSignalRuntime,
};

impl BridgeOwnedSignalRuntime {
    pub fn successor_installation_runtime(&self) -> Result<Self, BridgeConditionalDenial> {
        let mut bridge = self.bridge.clone();
        let (graph, signal) = self
            .signal_runtime
            .graph()
            .reconstitute_for_runtime_rebind()
            .map_err(|error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::SignalContractInstallation,
                    format!("Signal checkpoint reconstitution failed: {error}"),
                )
            })?
            .into_parts();
        bridge.semantic_dependency_registry = self
            .baseline_semantic_dependency_registry
            .rebind_to_graph(&graph)
            .ok_or_else(|| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::CorrespondenceAdmission,
                    "Bridge baseline semantic registrations could not bind to the reconstructed Signal graph",
                )
            })?;
        bridge.aspect_registry = bridge
            .aspect_registry
            .reconstruct_derived_indexes()
            .map_err(|error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::CorrespondenceAdmission,
                    format!("Bridge aspect index reconstruction failed: {error:?}"),
                )
            })?;
        let allocations = self
            .baseline_correspondence_allocations
            .rebind_to_graph(graph.installed_graph_capability().graph_instance_id());
        bridge.correspondence_allocations =
            std::sync::Arc::new(std::sync::RwLock::new(allocations));
        let mut successor = Self::new(bridge, graph)?;
        successor
            .bridge
            .bind_signal_graph(&mut *successor.signal_runtime.graph_mut())
            .map_err(|error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::CorrespondenceAdmission,
                    format!("Bridge reconstructed Signal binding failed: {error:?}"),
                )
            })?;
        let correspondence = successor
            .bridge
            .rebuild_correspondence_allocation_index()
            .map_err(|error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::CorrespondenceAdmission,
                    format!("Bridge correspondence reconstruction failed: {error:?}"),
                )
            })?;
        successor.reconstitution_report = Some(BridgeConditionalRuntimeReconstitutionReport::new(
            signal,
            correspondence,
        ));
        Ok(successor)
    }

    #[cfg(test)]
    pub(crate) fn destroy_reconstitutable_indexes_for_test(&mut self) {
        self.bridge
            .semantic_dependency_registry
            .destroy_derived_indexes();
        self.baseline_semantic_dependency_registry
            .destroy_derived_indexes();
        self.bridge.aspect_registry.destroy_derived_indexes();
        self.baseline_correspondence_allocations
            .destroy_derived_indexes();
        self.bridge
            .correspondence_allocations
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .destroy_derived_indexes();
    }

    pub const fn reconstitution_report(
        &self,
    ) -> Option<BridgeConditionalRuntimeReconstitutionReport> {
        self.reconstitution_report
    }
}
