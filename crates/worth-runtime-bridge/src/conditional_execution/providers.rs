use std::sync::Arc;

use worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration;

use super::provider_semantics::{
    BridgeConditionalProviderSemanticContracts, BridgeErasedProviderSemanticContract,
};
use super::BridgeConditionalProviderSemantics;

#[derive(Debug, Clone)]
pub struct BridgeConditionalResolverContext {
    pub dirty_aspects: worth_signal::facade::AspectMask,
    /// Signal-version distance only. This is never a unitful semantic delta;
    /// typed threshold providers must use their installed domain observation.
    pub max_signal_version_delta: u64,
    observations: Arc<[BridgeConditionalSemanticObservation]>,
}

impl BridgeConditionalResolverContext {
    pub(super) fn new(
        dirty_aspects: worth_signal::facade::AspectMask,
        max_signal_version_delta: u64,
        observations: Arc<[BridgeConditionalSemanticObservation]>,
    ) -> Self {
        Self {
            dirty_aspects,
            max_signal_version_delta,
            observations,
        }
    }

    pub fn observations(&self) -> &[BridgeConditionalSemanticObservation] {
        &self.observations
    }

    pub fn observation(
        &self,
        dependency_ordinal: usize,
    ) -> Option<&BridgeConditionalSemanticObservation> {
        self.observations
            .iter()
            .find(|observation| observation.dependency_ordinal == dependency_ordinal)
    }
}

#[derive(Debug, Clone)]
pub struct BridgeConditionalSemanticObservation {
    dependency_ordinal: usize,
    previous: Option<worth_foundational::facade::ContractValidatedAspectArtifact>,
    current: worth_foundational::facade::ContractValidatedAspectArtifact,
}

impl BridgeConditionalSemanticObservation {
    pub(super) fn new(
        dependency_ordinal: usize,
        previous: Option<worth_foundational::facade::ContractValidatedAspectArtifact>,
        current: worth_foundational::facade::ContractValidatedAspectArtifact,
    ) -> Self {
        Self {
            dependency_ordinal,
            previous,
            current,
        }
    }

    pub const fn dependency_ordinal(&self) -> usize {
        self.dependency_ordinal
    }

    pub fn previous(&self) -> Option<&worth_foundational::facade::ContractValidatedAspectArtifact> {
        self.previous.as_ref()
    }

    pub fn current(&self) -> &worth_foundational::facade::ContractValidatedAspectArtifact {
        &self.current
    }
}

pub trait BridgeConditionalConditionProvider: Send + Sync + 'static {
    fn resolve(
        &self,
        declaration: &WorthQueryPortableConditionalNodeDeclaration,
        context: BridgeConditionalResolverContext,
    ) -> Result<worth_signal::facade::InstalledSignalConditionDecision, String>;
}

pub trait BridgeConditionalComparatorProvider: Send + Sync + 'static {
    fn has_meaningful_change(
        &self,
        aspect: worth_signal::facade::Aspect,
        cached: u64,
        current: u64,
    ) -> Result<bool, String>;
}

pub trait BridgeConditionalTriggerProvider: Send + Sync + 'static {
    fn requested(&self) -> bool;
}

pub trait BridgeConditionalWakeProvider: Send + Sync + 'static {
    fn resolve(
        &self,
        declaration: &WorthQueryPortableConditionalNodeDeclaration,
        context: BridgeConditionalResolverContext,
    ) -> Result<worth_signal::facade::InstalledSignalConditionDecision, String>;
}

pub trait BridgeConditionalComputeProvider: Send + Sync + 'static {
    fn compute(
        &self,
        context: &mut dyn std::any::Any,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String>;
}

#[derive(Clone, Default)]
pub struct BridgeConditionalProviderSet {
    pub(crate) condition: Option<Arc<dyn BridgeConditionalConditionProvider>>,
    pub(crate) dependency_comparator: Option<Arc<dyn BridgeConditionalComparatorProvider>>,
    pub(crate) output_comparator: Option<Arc<dyn BridgeConditionalComparatorProvider>>,
    pub(crate) reuse_comparator: Option<Arc<dyn BridgeConditionalComparatorProvider>>,
    pub(crate) trigger: Option<Arc<dyn BridgeConditionalTriggerProvider>>,
    pub(crate) wake: Option<Arc<dyn BridgeConditionalWakeProvider>>,
    pub(crate) compute: Option<Arc<dyn BridgeConditionalComputeProvider>>,
    pub(super) semantic_contracts: BridgeConditionalProviderSemanticContracts,
}

impl BridgeConditionalProviderSet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn condition<P>(mut self, provider: P) -> Self
    where
        P: BridgeConditionalConditionProvider + BridgeConditionalProviderSemantics,
    {
        self.semantic_contracts.condition =
            Some(BridgeErasedProviderSemanticContract::capture(&provider));
        self.condition = Some(Arc::new(provider));
        self
    }
    pub fn dependency_comparator<P>(mut self, provider: P) -> Self
    where
        P: BridgeConditionalComparatorProvider + BridgeConditionalProviderSemantics,
    {
        self.semantic_contracts.dependency_comparator =
            Some(BridgeErasedProviderSemanticContract::capture(&provider));
        self.dependency_comparator = Some(Arc::new(provider));
        self
    }
    pub fn output_comparator<P>(mut self, provider: P) -> Self
    where
        P: BridgeConditionalComparatorProvider + BridgeConditionalProviderSemantics,
    {
        self.semantic_contracts.output_comparator =
            Some(BridgeErasedProviderSemanticContract::capture(&provider));
        self.output_comparator = Some(Arc::new(provider));
        self
    }
    pub fn reuse_comparator<P>(mut self, provider: P) -> Self
    where
        P: BridgeConditionalComparatorProvider + BridgeConditionalProviderSemantics,
    {
        self.semantic_contracts.reuse_comparator =
            Some(BridgeErasedProviderSemanticContract::capture(&provider));
        self.reuse_comparator = Some(Arc::new(provider));
        self
    }
    pub fn trigger<P>(mut self, provider: P) -> Self
    where
        P: BridgeConditionalTriggerProvider + BridgeConditionalProviderSemantics,
    {
        self.semantic_contracts.trigger =
            Some(BridgeErasedProviderSemanticContract::capture(&provider));
        self.trigger = Some(Arc::new(provider));
        self
    }
    pub fn wake<P>(mut self, provider: P) -> Self
    where
        P: BridgeConditionalWakeProvider + BridgeConditionalProviderSemantics,
    {
        self.semantic_contracts.wake =
            Some(BridgeErasedProviderSemanticContract::capture(&provider));
        self.wake = Some(Arc::new(provider));
        self
    }
    pub fn compute<P>(mut self, provider: P) -> Self
    where
        P: BridgeConditionalComputeProvider + BridgeConditionalProviderSemantics,
    {
        self.semantic_contracts.compute =
            Some(BridgeErasedProviderSemanticContract::capture(&provider));
        self.compute = Some(Arc::new(provider));
        self
    }

    pub fn has_compute_provider(&self) -> bool {
        self.compute.is_some()
    }
}
