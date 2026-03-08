//! FeatureRegistry trait — the abstraction over concrete feature sets.
//!
//! DOMAIN: Allows FeatureTree to be generic over different sets of
//! features (e.g., NativeFeature for B-Rep, SdfFeatureSet for SDF).

use std::collections::HashMap;
use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use forge_core::envelope::OperationResult;
use forge_core::KernelError;
use forge_signal::facade::NodeId;

use super::super::output::solid_envelope::SolidEnvelope;
use super::feature_dependency::FeatureDependency;
use super::feature_signal_policy::FeatureSignalPolicy;
use crate::configuration::facade::KernelConfig;

/// Trait that concrete feature enums must implement to be used with `FeatureTree`.
///
/// This is the abstraction barrier between the engine (generic infrastructure)
/// and the registry (concrete feature wiring). The engine never knows which
/// specific features exist — it only knows how to execute and manage them
/// through this trait.
pub trait FeatureRegistry: Debug + Clone + Serialize + DeserializeOwned {
    /// Execute this feature through the pipeline.
    ///
    /// Takes ownership of inputs — the pipeline performs coordinate
    /// conditioning on the owned geometry before passing to the feature.
    fn execute_via_pipeline(
        &self,
        inputs: HashMap<NodeId, SolidEnvelope>,
        session_config: &KernelConfig,
    ) -> Result<OperationResult<SolidEnvelope>, KernelError>;

    /// Return the NodeIds this feature depends on.
    fn dependencies(&self) -> Vec<NodeId>;

    /// Return aspect-aware dependency declarations for this feature.
    ///
    /// Kernel callers should prefer this over `dependencies()` when wiring
    /// `forge-signal`, so semantic invalidation remains precise.
    fn dependency_bindings(&self) -> Vec<FeatureDependency> {
        self.dependencies()
            .into_iter()
            .map(FeatureDependency::topology_and_geometry)
            .collect()
    }

    /// Static signal policy for this feature node.
    ///
    /// Core execution nodes remain `Always` + static by default. Features can
    /// override this to opt into explicit comparator or condition settings when
    /// the kernel is ready to use them intentionally.
    fn signal_policy(&self) -> FeatureSignalPolicy {
        FeatureSignalPolicy::default()
    }

    /// Human-readable name for this feature.
    fn name(&self) -> &str;
}
