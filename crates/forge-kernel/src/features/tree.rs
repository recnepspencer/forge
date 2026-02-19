//! Parametric Feature Tree powered by forge-signal.
//!
//! DOMAIN: Managing the dependency graph of high-level features.
//!
//! INVARIANTS:
//! - All feature evaluation is pure (output depends only on inputs)
//! - Dependencies are tracked via `forge-signal`
//! - Topology is immutable (passed as snapshots)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_signal::graph::SignalGraph;
use forge_signal::handles::NodeId;
use forge_signal::schema::{AspectVersion, Aspect};

pub use super::traits::{Feature, FeatureOutput};
use super::wrappers::{MakeCubeFeature, BooleanFeature};

/// A concrete enum implementation of all supported features for serialization.
///
/// This replaces `Box<dyn Feature>` to allow native `serde` support without
/// complex dynamic dispatch serialization crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NativeFeature {
    MakeCube(MakeCubeFeature),
    Boolean(BooleanFeature),
}

impl Feature for NativeFeature {
    fn evaluate(
        &self,
        inputs: &HashMap<NodeId, FeatureOutput>,
    ) -> Result<FeatureOutput, KernelError> {
        match self {
            NativeFeature::MakeCube(f) => f.evaluate(inputs),
            NativeFeature::Boolean(f) => f.evaluate(inputs),
        }
    }

    fn dependencies(&self) -> Vec<NodeId> {
        match self {
            NativeFeature::MakeCube(f) => f.dependencies(),
            NativeFeature::Boolean(f) => f.dependencies(),
        }
    }

    fn name(&self) -> &str {
        match self {
            NativeFeature::MakeCube(f) => f.name(),
            NativeFeature::Boolean(f) => f.name(),
        }
    }
}

/// The Feature Tree manager.
///
/// Owns the signal graph and the storage for feature data.
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTree {
    /// The reactive dependency graph.
    graph: SignalGraph,
    /// Map from NodeId to the Feature implementation.
    features: HashMap<NodeId, NativeFeature>,
    /// Cache of feature outputs.
    outputs: HashMap<NodeId, FeatureOutput>,
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
            outputs: HashMap::new(),
            names: HashMap::new(),
        }
    }

    /// Register a new feature in the tree.
    ///
    /// 1. Allocates a NodeId in the signal graph.
    /// 2. Registers dependencies.
    /// 3. Stores the feature logic.
    pub fn register_feature(
        &mut self,
        feature: NativeFeature,
    ) -> Result<NodeId, KernelError> {
        let node_id = self.graph.create_node();
        let deps = feature.dependencies();

        for dep_id in deps {
            self.graph.add_dependency(node_id, dep_id, Aspect::Topology)?;
            self.graph.add_dependency(node_id, dep_id, Aspect::Geometry)?;
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

        let new_feature = self.features.get(&node_id).ok_or_else(|| KernelError::InternalError {
            message: format!("Feature missing after insert for node {}", node_id),
            context: None,
        })?;
        for dep_id in new_feature.dependencies() {
             self.graph.add_dependency(node_id, dep_id, Aspect::Topology)?;
             self.graph.add_dependency(node_id, dep_id, Aspect::Geometry)?;
        }

        forge_signal::evaluation::mark_dirty(&mut self.graph, node_id, Aspect::Topology)?;
        forge_signal::evaluation::mark_dirty(&mut self.graph, node_id, Aspect::Geometry)?;

        Ok(())
    }

    /// Evaluate a specific feature (and its dependencies) to get the latest output.
    ///
    /// Implements a two-phase trace flush:
    /// 1. During evaluation, each node's `DecisionLog` is converted to a
    ///    `TraceSummary` and collected in a side-channel map.
    /// 2. After evaluation completes, summaries are flushed to the signal
    ///    graph's `NodeEntry` records for subsequent diffing.
    pub fn evaluate_feature(&mut self, node_id: NodeId) -> Result<FeatureOutput, KernelError> {
        let graph = &mut self.graph;
        let features = &self.features;
        let outputs = &mut self.outputs;

        let mut pending_traces: HashMap<NodeId, forge_core::TraceSummary> = HashMap::new();

        let mut compute = |id: NodeId, _graph_ref: &SignalGraph| -> Result<AspectVersion, KernelError> {
            let feature = features.get(&id).ok_or_else(|| KernelError::InvalidInput {
                message: format!("Feature logic not found for node {}", id),
                context: None,
            })?;

            let mut inputs = HashMap::new();
            for dep_id in feature.dependencies() {
                if let Some(output) = outputs.get(&dep_id) {
                     inputs.insert(dep_id, output.clone());
                } else {
                    return Err(KernelError::InvalidInput {
                         message: format!("Dependency output missing for node {}", dep_id),
                         context: None,
                    });
                }
            }

            let output = feature.evaluate(&inputs)?;

            let state_hash = forge_topo::hashing::compute_arena_topology_hash(
                output.topology.arena(),
            );
            let summary = output.decision_log.to_summary(state_hash);
            pending_traces.insert(id, summary);

            outputs.insert(id, output);

            Ok(AspectVersion::new(1, 1))
        };

        forge_signal::evaluation::evaluate(graph, node_id, &mut compute)?;

        for (id, summary) in pending_traces {
            if let Ok(entry) = graph.get_entry_mut(id) {
                entry.set_trace_summary(Some(summary));
            }
        }

        outputs.get(&node_id).cloned().ok_or_else(|| KernelError::InternalError {
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
}
