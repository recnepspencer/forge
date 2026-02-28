//! Parametric Feature Tree powered by forge-signal.
//!
//! DOMAIN: Managing the dependency graph of high-level features.
//!
//! INVARIANTS:
//! - All feature evaluation goes through `FeaturePipeline::execute`
//! - Dependencies are tracked via `forge-signal`
//! - Topology is immutable (passed as snapshots)
//! - `NativeFeature` is a dispatch enum, NOT a `Feature` impl
//!   (because `Feature::type Inputs` differs per variant)
//! - Per-node `OperationResult<FeatureOutput>` envelopes are the
//!   canonical storage — they carry the full decision log, metrics,
//!   lineage, and warnings from each feature evaluation

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use forge_core::envelope::OperationResult;
use forge_core::KernelError;
use forge_signal::graph::SignalGraph;
use forge_signal::handles::NodeId;
use forge_signal::schema::{Aspect, AspectVersion};

use super::executor::FeaturePipeline;
pub use super::traits::{Feature, FeatureOutput};
use super::wrappers::BooleanFeature;
use crate::core::ModelingContext;
use crate::primitives::MakePrimitiveFeature;

/// A concrete enum implementation of all supported features for serialization.
///
/// This replaces `Box<dyn Feature>` to allow native `serde` support without
/// complex dynamic dispatch serialization crates.
///
/// `NativeFeature` does NOT implement `Feature` because each variant has
/// a different `type Inputs`. Instead, it dispatches to the concrete
/// `Feature` impl via `execute_via_pipeline`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NativeFeature {
    MakePrimitive(MakePrimitiveFeature),
    Boolean(BooleanFeature),
}

impl NativeFeature {
    /// Execute this feature through the pipeline, dispatching to the concrete type.
    ///
    /// Returns `OperationResult<FeatureOutput>` — the envelope carries the
    /// canonical decision log, warnings, metrics, and lineage from the
    /// pipeline's finalization step. The `ModelingContext`'s decision log
    /// is drained into the envelope by the `OperationFinalizer`.
    fn execute_via_pipeline(
        &self,
        inputs: &HashMap<NodeId, FeatureOutput>,
        ctx: &mut ModelingContext,
    ) -> Result<OperationResult<FeatureOutput>, KernelError> {
        match self {
            NativeFeature::MakePrimitive(f) => FeaturePipeline::execute(f, inputs, ctx),
            NativeFeature::Boolean(f) => FeaturePipeline::execute(f, inputs, ctx),
        }
    }

    /// Return the dependencies of the inner feature.
    pub fn dependencies(&self) -> Vec<NodeId> {
        match self {
            NativeFeature::MakePrimitive(f) => f.dependencies(),
            NativeFeature::Boolean(f) => f.dependencies(),
        }
    }

    /// Return the name of the inner feature.
    pub fn name(&self) -> &str {
        match self {
            NativeFeature::MakePrimitive(f) => f.name(),
            NativeFeature::Boolean(f) => f.name(),
        }
    }
}

/// The Feature Tree manager.
///
/// Owns the signal graph and the storage for feature data.
/// Each evaluated node stores an `OperationResult<FeatureOutput>` envelope
/// containing the domain output (topology + geometry) plus the full audit
/// trail (decision log, metrics, lineage, warnings). This is the canonical
/// metadata storage — no separate `Arc<DecisionLog>` fields needed.
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTree {
    /// The reactive dependency graph.
    graph: SignalGraph,
    /// Map from NodeId to the Feature implementation.
    features: HashMap<NodeId, NativeFeature>,
    /// Cached envelopes carrying both domain output and audit metadata.
    envelopes: HashMap<NodeId, OperationResult<FeatureOutput>>,
    /// Map of names to NodeIds (optional, for lookup).
    names: HashMap<String, NodeId>,
}

impl Default for FeatureTree {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureTree {
    /// Create a new empty feature tree.
    pub fn new() -> Self {
        Self {
            graph: SignalGraph::new(),
            features: HashMap::new(),
            envelopes: HashMap::new(),
            names: HashMap::new(),
        }
    }

    /// Register a new feature in the tree.
    ///
    /// 1. Allocates a NodeId in the signal graph.
    /// 2. Registers dependencies.
    /// 3. Stores the feature logic.
    pub fn register_feature(&mut self, feature: NativeFeature) -> Result<NodeId, KernelError> {
        let node_id = self.graph.create_node();
        let deps = feature.dependencies();

        for dep_id in deps {
            self.graph
                .add_dependency(node_id, dep_id, Aspect::Topology)?;
            self.graph
                .add_dependency(node_id, dep_id, Aspect::Geometry)?;
        }

        if let Some(name) = feature.name().split('/').last() {
            self.names.insert(name.to_string(), node_id);
        }

        self.features.insert(node_id, feature);

        forge_signal::evaluation::mark_dirty(&mut self.graph, node_id, Aspect::Topology)?;
        forge_signal::evaluation::mark_dirty(&mut self.graph, node_id, Aspect::Geometry)?;

        Ok(node_id)
    }

    /// Replace an existing feature with a new implementation.
    ///
    /// Preserves the NodeId but updates the logic and marks dependencies dirty.
    pub fn replace_feature(
        &mut self,
        node_id: NodeId,
        feature: NativeFeature,
    ) -> Result<(), KernelError> {
        if !self.graph.is_alive(node_id) {
            return Err(KernelError::InvalidInput {
                message: format!("Node {} is not alive", node_id),
                context: None,
            });
        }

        let old_feature = self.features.insert(node_id, feature);

        if let Some(old) = old_feature {
            for dep_id in old.dependencies().iter() {
                self.graph.remove_dependency(node_id, *dep_id)?;
            }
        }

        let new_feature =
            self.features
                .get(&node_id)
                .ok_or_else(|| KernelError::InternalError {
                    message: format!("Feature missing after insert for node {}", node_id),
                    context: None,
                })?;
        for dep_id in new_feature.dependencies() {
            self.graph
                .add_dependency(node_id, dep_id, Aspect::Topology)?;
            self.graph
                .add_dependency(node_id, dep_id, Aspect::Geometry)?;
        }

        forge_signal::evaluation::mark_dirty(&mut self.graph, node_id, Aspect::Topology)?;
        forge_signal::evaluation::mark_dirty(&mut self.graph, node_id, Aspect::Geometry)?;

        Ok(())
    }

    /// Evaluate a specific feature (and its dependencies) to get the latest output.
    ///
    /// Convenience wrapper over `evaluate_feature_with_context` that uses a
    /// default `ModelingContext`. Returns only the `FeatureOutput` — callers
    /// that need the full envelope should use `evaluate_feature_with_context`.
    pub fn evaluate_feature(&mut self, node_id: NodeId) -> Result<FeatureOutput, KernelError> {
        let mut ctx = ModelingContext::new();
        let envelope = self.evaluate_feature_with_context(node_id, &mut ctx)?;
        Ok(envelope.into_value())
    }

    /// Evaluate a feature with an explicit `ModelingContext`.
    ///
    /// Returns the full `OperationResult<FeatureOutput>` envelope so callers
    /// can inspect the decision log, warnings, metrics, and lineage alongside
    /// the domain output.
    ///
    /// The envelope stored per-node is the canonical metadata record — the
    /// `ModelingContext`'s decision log is drained into each envelope by the
    /// `OperationFinalizer` during pipeline execution.
    pub fn evaluate_feature_with_context(
        &mut self,
        node_id: NodeId,
        ctx: &mut ModelingContext,
    ) -> Result<OperationResult<FeatureOutput>, KernelError> {
        let graph = &mut self.graph;
        let features = &self.features;
        let envelopes = &mut self.envelopes;

        let mut pending_traces: HashMap<NodeId, forge_core::TraceSummary> = HashMap::new();

        let mut compute =
            |id: NodeId, _graph_ref: &SignalGraph| -> Result<AspectVersion, KernelError> {
                let feature = features.get(&id).ok_or_else(|| KernelError::InvalidInput {
                    message: format!("Feature logic not found for node {}", id),
                    context: None,
                })?;

                // Build input map by borrowing FeatureOutput from stored envelopes.
                // The clone is at the input boundary — necessary because
                // parse_inputs takes &HashMap<NodeId, FeatureOutput>.
                let mut input_map = HashMap::new();
                for dep_id in feature.dependencies() {
                    if let Some(envelope) = envelopes.get(&dep_id) {
                        input_map.insert(dep_id, envelope.get_value().clone());
                    } else {
                        return Err(KernelError::InvalidInput {
                            message: format!("Dependency output missing for node {}", dep_id),
                            context: None,
                        });
                    }
                }

                let envelope = feature.execute_via_pipeline(&input_map, ctx)?;

                // Build the trace summary from the envelope's decision log —
                // NOT from ctx, which was drained by the OperationFinalizer.
                let hash = forge_topo::transactions::compute_arena_topology_hash(
                    envelope.get_value().topology.arena(),
                );
                let summary = envelope.get_decision_log().to_summary(hash);
                pending_traces.insert(id, summary);

                // Store the full envelope — metadata is preserved.
                envelopes.insert(id, envelope);

                Ok(AspectVersion::new(1, 1))
            };

        forge_signal::evaluation::evaluate(graph, node_id, &mut compute)?;

        for (id, summary) in pending_traces {
            if let Ok(entry) = graph.get_entry_mut(id) {
                entry.set_trace_summary(Some(summary));
            }
        }

        envelopes
            .get(&node_id)
            .cloned()
            .ok_or_else(|| KernelError::InternalError {
                message: "Evaluation finished but output missing".to_string(),
                context: None,
            })
    }

    /// Get a feature ID by name.
    pub fn get_node_by_name(&self, name: &str) -> Option<NodeId> {
        self.names.get(name).copied()
    }

    /// Read-only access to the signal graph (for trace inspection).
    pub fn get_graph(&self) -> &SignalGraph {
        &self.graph
    }

    /// Read-only access to a stored envelope (for external audit inspection).
    pub fn get_envelope(&self, node_id: NodeId) -> Option<&OperationResult<FeatureOutput>> {
        self.envelopes.get(&node_id)
    }
}
