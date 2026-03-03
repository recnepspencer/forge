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

use crate::configuration::facade::KernelConfig;
use super::super::output::solid_envelope::SolidEnvelope;

/// Trait that concrete feature enums must implement to be used with `FeatureTree`.
///
/// This is the abstraction barrier between the engine (generic infrastructure)
/// and the registry (concrete feature wiring). The engine never knows which
/// specific features exist — it only knows how to execute and manage them
/// through this trait.
pub trait FeatureRegistry: Debug + Clone + Serialize + DeserializeOwned {
    /// Execute this feature through the pipeline.
    fn execute_via_pipeline(
        &self,
        inputs: &HashMap<NodeId, SolidEnvelope>,
        session_config: &KernelConfig,
    ) -> Result<OperationResult<SolidEnvelope>, KernelError>;

    /// Return the NodeIds this feature depends on.
    fn dependencies(&self) -> Vec<NodeId>;

    /// Human-readable name for this feature.
    fn name(&self) -> &str;
}
