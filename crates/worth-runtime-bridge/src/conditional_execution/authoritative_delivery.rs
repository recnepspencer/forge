use super::{
    BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeInstalledConditionalLowering,
    BridgeOwnedSignalRuntime,
};

impl BridgeOwnedSignalRuntime {
    pub fn deliver_authoritative_change(
        &mut self,
        lowering: &BridgeInstalledConditionalLowering,
        dependency_ordinal: usize,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<crate::correspondence::CorrespondenceDeliveryOutcome, BridgeConditionalDenial> {
        if lowering.signal_contract.graph_instance_id()
            != self.graph.installed_graph_capability().graph_instance_id()
        {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::StaleLowering,
                "conditional lowering belongs to another owned Signal graph",
            ));
        }
        let correspondence = lowering
            .correspondences
            .iter()
            .find(|item| item.dependency().dependency_ordinal() == dependency_ordinal)
            .ok_or_else(|| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::DependencyOrdinalMismatch,
                    "conditional declaration does not retain that dependency ordinal",
                )
            })?;
        Ok(self
            .bridge
            .deliver_installed_correspondence(correspondence, &mut self.graph, request))
    }
}
