use worth_signal::facade::{
    InstalledSignalComparatorIdentity, InstalledSignalConditionDecision,
    InstalledSignalConditionIdentity,
};

use super::{
    BridgeConditionalDenial, BridgeConditionalResolverContext,
    BridgeConditionalSemanticObservation, BridgeInstalledConditionalLowering,
};

pub(super) struct ConditionAdapter<'a> {
    lowering: &'a BridgeInstalledConditionalLowering,
    snapshot: Option<
        &'a crate::snapshot::AdmittedSnapshotContext<Box<dyn crate::snapshot::TruthSnapshotReader>>,
    >,
    previous: &'a std::collections::BTreeMap<
        (worth_signal::facade::NodeId, usize),
        worth_foundational::facade::ContractValidatedAspectArtifact,
    >,
    observations: std::sync::Arc<[BridgeConditionalSemanticObservation]>,
    observation_denial: Option<BridgeConditionalDenial>,
}

impl<'a> ConditionAdapter<'a> {
    pub(super) fn new(
        lowering: &'a BridgeInstalledConditionalLowering,
        snapshot: Option<
            &'a crate::snapshot::AdmittedSnapshotContext<
                Box<dyn crate::snapshot::TruthSnapshotReader>,
            >,
        >,
        previous: &'a std::collections::BTreeMap<
            (worth_signal::facade::NodeId, usize),
            worth_foundational::facade::ContractValidatedAspectArtifact,
        >,
    ) -> Self {
        Self {
            lowering,
            snapshot,
            previous,
            observations: std::sync::Arc::from([]),
            observation_denial: None,
        }
    }

    pub(super) fn take_observation_denial(&mut self) -> Option<BridgeConditionalDenial> {
        self.observation_denial.take()
    }

    pub(super) fn observation_count(&self) -> usize {
        self.observations.len()
    }

    pub(super) fn take_observations(
        &mut self,
    ) -> std::sync::Arc<[BridgeConditionalSemanticObservation]> {
        std::mem::take(&mut self.observations)
    }
}

impl worth_signal::facade::InstalledSignalConditionResolver for ConditionAdapter<'_> {
    fn resolve(
        &mut self,
        identity: InstalledSignalConditionIdentity,
        context: &worth_signal::facade::ConditionEvaluationContext,
    ) -> Result<InstalledSignalConditionDecision, worth_signal::facade::SignalError> {
        if !matches!(
            self.lowering.signal_contract.condition(),
            worth_signal::facade::EvaluationCondition::Installed(expected) if *expected == identity
        ) {
            return Err(worth_signal::facade::SignalError::invalid_input(
                "installed condition identity did not match its retained Bridge lowering",
            ));
        }
        self.observations = match super::semantic_observations::read_condition_observations(
            self.snapshot,
            self.lowering,
            self.previous,
        ) {
            Ok(observations) => observations.into(),
            Err(denial) => {
                let detail = denial.detail().to_string();
                self.observation_denial = Some(denial);
                return Err(worth_signal::facade::SignalError::invalid_input(detail));
            }
        };
        let bridge_context = BridgeConditionalResolverContext::new(
            context.dirty_aspects,
            context.max_dependency_delta,
            std::sync::Arc::clone(&self.observations),
        );
        if let worth_signal::facade::SignalConditionalCondition::DeltaThreshold(threshold) =
            self.lowering.signal_contract.semantic_condition()
        {
            let observation = self.observations.first().ok_or_else(|| {
                worth_signal::facade::SignalError::invalid_input(
                    "installed semantic threshold retained no admitted observation",
                )
            })?;
            let current = scalar_value(observation.current())?;
            let previous = observation.previous().map(scalar_value).transpose()?;
            return worth_signal::facade::resolve_signal_delta_threshold(
                threshold, previous, current,
            );
        }
        if let Some(provider) = &self.lowering.providers.condition {
            return provider
                .resolve(&self.lowering.declaration, bridge_context)
                .map_err(worth_signal::facade::SignalError::invalid_input);
        }
        if let Some(provider) = &self.lowering.providers.wake {
            return provider
                .resolve(&self.lowering.declaration, bridge_context)
                .map_err(worth_signal::facade::SignalError::invalid_input);
        }
        Err(worth_signal::facade::SignalError::invalid_input(
            "installed condition lost its exact Bridge provider",
        ))
    }
}

fn scalar_value(
    artifact: &worth_foundational::facade::ContractValidatedAspectArtifact,
) -> Result<&worth_foundational::facade::AspectValue, worth_signal::facade::SignalError> {
    match artifact.payload().view() {
        worth_foundational::facade::ContractValidatedAspectValueView::Scalar(value) => Ok(value),
        worth_foundational::facade::ContractValidatedAspectValueView::Struct(_) => {
            Err(worth_signal::facade::SignalError::invalid_input(
                "semantic threshold observation was not the admitted scalar projection",
            ))
        }
    }
}

pub(super) struct ComparatorAdapter<'a> {
    lowering: &'a BridgeInstalledConditionalLowering,
}

impl<'a> ComparatorAdapter<'a> {
    pub(super) fn new(lowering: &'a BridgeInstalledConditionalLowering) -> Self {
        Self { lowering }
    }
}

impl worth_signal::facade::VersionComparatorResolver for ComparatorAdapter<'_> {
    fn resolve(
        &mut self,
        _key: &str,
        _aspect: worth_signal::facade::Aspect,
        _cached: u64,
        _current: u64,
    ) -> Result<bool, worth_signal::facade::SignalError> {
        Err(worth_signal::facade::SignalError::invalid_input(
            "portable comparator strings are not installed Bridge authority",
        ))
    }

    fn resolve_installed(
        &mut self,
        identity: InstalledSignalComparatorIdentity,
        aspect: worth_signal::facade::Aspect,
        cached: u64,
        current: u64,
    ) -> Result<bool, worth_signal::facade::SignalError> {
        let dependency =
            installed_comparator(self.lowering.signal_contract.dependency_comparator());
        let output = installed_comparator(self.lowering.signal_contract.output_comparator());
        let reuse = match self.lowering.signal_contract.artifact_reuse() {
            worth_signal::facade::SignalConditionalArtifactReusePolicy::Installed(identity) => {
                Some(*identity)
            }
            _ => None,
        };
        let provider = if dependency == Some(identity) {
            self.lowering.providers.dependency_comparator.as_ref()
        } else if output == Some(identity) {
            self.lowering.providers.output_comparator.as_ref()
        } else if reuse == Some(identity) {
            self.lowering.providers.reuse_comparator.as_ref()
        } else {
            None
        }
        .ok_or_else(|| {
            worth_signal::facade::SignalError::invalid_input(
                "installed comparator identity did not match its retained Bridge role",
            )
        })?;
        provider
            .has_meaningful_change(aspect, cached, current)
            .map_err(worth_signal::facade::SignalError::invalid_input)
    }
}

fn installed_comparator(
    policy: &worth_signal::facade::VersionComparatorPolicy,
) -> Option<InstalledSignalComparatorIdentity> {
    match policy {
        worth_signal::facade::VersionComparatorPolicy::Installed { identity } => Some(*identity),
        _ => None,
    }
}

impl worth_signal::facade::ComparatorPolicyResolver for ComparatorAdapter<'_> {
    fn policy_for_node(
        &self,
        _node: worth_signal::facade::NodeId,
        node_override: Option<&worth_signal::facade::VersionComparatorPolicy>,
    ) -> worth_signal::facade::VersionComparatorPolicy {
        node_override.cloned().unwrap_or_default()
    }
}
