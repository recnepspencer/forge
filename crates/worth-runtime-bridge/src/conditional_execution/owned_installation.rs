use crate::correspondence::{
    BridgeSemanticCorrespondenceRegistration, BridgeSemanticDependencyCandidate,
    BridgeSignalAspectTargetDeclaration,
};

use super::{
    BridgeConditionalContract, BridgeConditionalDenial, BridgeConditionalDenialKind,
    BridgeConditionalInstallationRequest, BridgeConditionalLocation, BridgeConditionalProviderSet,
    BridgeInstalledConditionalLowering, BridgeOwnedSignalRuntime,
};

/// Bridge-owned installation input for callers that possess semantic
/// dependencies but no Signal topology authority.
pub struct BridgeOwnedConditionalInstallationRequest {
    pub contract: BridgeConditionalContract,
    pub location: BridgeConditionalLocation,
    pub dependencies: Vec<BridgeSemanticDependencyCandidate>,
    pub providers: BridgeConditionalProviderSet,
}

impl BridgeOwnedSignalRuntime {
    /// Allocates the volatile Signal node, partitions, and aspect targets
    /// inside Bridge before entering the ordinary conditional installation
    /// lane. No raw Signal capability crosses this boundary.
    pub fn install_owned_conditional(
        &mut self,
        request: BridgeOwnedConditionalInstallationRequest,
    ) -> Result<std::sync::Arc<BridgeInstalledConditionalLowering>, BridgeConditionalDenial> {
        validate_dependency_shape(&request)?;
        let registrations = self.owned_correspondence_registrations(
            request.location.node_identity(),
            request.dependencies,
        )?;
        self.install(BridgeConditionalInstallationRequest {
            contract: request.contract,
            location: request.location,
            registrations,
            providers: request.providers,
        })
    }

    pub fn retire_owned_conditional(
        &mut self,
        lowering: &std::sync::Arc<BridgeInstalledConditionalLowering>,
    ) -> Result<(), BridgeConditionalDenial> {
        let node = lowering.signal_node();
        let retained = self.conditional_lowerings.get(&node).ok_or_else(|| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::ForeignSignalGraph,
                "owned conditional retirement did not match this Bridge runtime",
            )
        })?;
        if !std::sync::Arc::ptr_eq(retained, lowering) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::ForeignSignalGraph,
                "owned conditional retirement did not carry the retained lowering",
            ));
        }
        self.signal_runtime
            .graph_mut()
            .unregister_node(node)
            .map_err(|error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::SignalContractInstallation,
                    format!("Signal denied owned conditional retirement: {error:?}"),
                )
            })?;
        self.owned_conditional_targets.unregister(lowering);
        self.conditional_lowerings.remove(&node);
        loop {
            let next = self
                .conditional_observations
                .range((
                    std::ops::Bound::Included((node, 0, None)),
                    std::ops::Bound::Unbounded,
                ))
                .next()
                .map(|(key, _)| key.clone())
                .filter(|(observed, _, _)| *observed == node);
            let Some(key) = next else {
                break;
            };
            self.conditional_observations.remove(&key);
        }
        Ok(())
    }

    fn owned_correspondence_registrations(
        &mut self,
        node_identity: &str,
        dependencies: Vec<BridgeSemanticDependencyCandidate>,
    ) -> Result<Vec<BridgeSemanticCorrespondenceRegistration>, BridgeConditionalDenial> {
        let node = self.signal_runtime.graph_mut().node().build();
        dependencies
            .into_iter()
            .map(|dependency| {
                let mapping = crate::correspondence::unique_mapping_id_for_dependency(
                    &self.bridge,
                    &dependency,
                )
                .map_err(|kind| correspondence_denial(kind, node_identity))?;
                let worth_proof::TransitionOutcome::Success(node_capability) =
                    self.signal_runtime.graph_mut().admit_installed_node(node)
                else {
                    return Err(correspondence_denial(
                        crate::correspondence::BridgeCorrespondenceDenialKind::MissingOrStaleSignalNode,
                        node_identity,
                    ));
                };
                let partition = dependency.owned_signal_partition();
                let target = BridgeSignalAspectTargetDeclaration::allocate(
                    mapping,
                    partition,
                    node_capability,
                );
                BridgeSemanticCorrespondenceRegistration::new(dependency, vec![target])
                    .map_err(|denial| correspondence_denial(denial.kind(), node_identity))
            })
            .collect()
    }
}

fn validate_dependency_shape(
    request: &BridgeOwnedConditionalInstallationRequest,
) -> Result<(), BridgeConditionalDenial> {
    if request.dependencies.len() != request.contract.dependency_count()
        || request
            .dependencies
            .iter()
            .enumerate()
            .any(|(ordinal, dependency)| {
                dependency.dependency_ordinal() != ordinal
                    || dependency.source_node_identity() != request.location.node_identity()
                    || dependency.source_stage_identity() != request.location.stage_identity()
            })
    {
        return Err(BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::DeclarationCorrespondenceMismatch,
            "Bridge-owned conditional dependencies do not exactly match their declaration",
        ));
    }
    Ok(())
}

fn correspondence_denial(
    kind: crate::correspondence::BridgeCorrespondenceDenialKind,
    node_identity: &str,
) -> BridgeConditionalDenial {
    BridgeConditionalDenial::new(
        BridgeConditionalDenialKind::CorrespondenceAdmission,
        format!("conditional node `{node_identity}` correspondence was denied: {kind:?}"),
    )
}
