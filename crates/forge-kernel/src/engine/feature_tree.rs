//! Parametric Feature Tree powered by forge-signal.
//!
//! DOMAIN: Managing the dependency graph of high-level features.
//!
//! INVARIANTS:
//! - All feature evaluation goes through `FeaturePipeline::execute`
//! - Dependencies are tracked via `forge-signal`
//! - Topology is immutable (passed as snapshots)
//! - `FeatureTree<R>` is generic over the feature registry `R`
//! - Per-node `OperationResult<SolidEnvelope>` envelopes are the
//!   canonical storage — they carry the full decision log, metrics,
//!   lineage, and warnings from each feature evaluation

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use forge_core::envelope::OperationResult;
use forge_core::KernelError;
use forge_signal::facade::{Aspect, AspectVersion, NodeId, SignalError, SignalGraph, TraceSummary};

use super::contracts::feature_registry::FeatureRegistry;
use super::output::solid_envelope::SolidEnvelope;
use crate::configuration::facade::KernelConfig;

const TOPOLOGY_ASPECT: Aspect = Aspect::new(0);
const GEOMETRY_ASPECT: Aspect = Aspect::new(1);

/// The Feature Tree manager.
///
/// Owns the signal graph and the storage for feature data.
/// Generic over `R: FeatureRegistry` — the concrete feature enum
/// is provided by the `registry` domain (e.g., `NativeFeature`).
///
/// Each evaluated node stores an `OperationResult<SolidEnvelope>` envelope
/// containing the domain output (topology + geometry) plus the full audit
/// trail (decision log, metrics, lineage, warnings). This is the canonical
/// metadata storage — no separate `Arc<DecisionLog>` fields needed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "R: Serialize",
    deserialize = "R: serde::de::DeserializeOwned",
))]
pub struct FeatureTree<R: FeatureRegistry> {
    /// The reactive dependency graph.
    graph: SignalGraph,
    /// Map from NodeId to the Feature implementation.
    features: HashMap<NodeId, R>,
    /// Cached envelopes carrying both domain output and audit metadata.
    envelopes: HashMap<NodeId, OperationResult<SolidEnvelope>>,
    /// Map of names to NodeIds (optional, for lookup).
    names: HashMap<String, NodeId>,
    /// Monotonic sequence counter for deterministic feature naming.
    ///
    /// Only increments, never resets — survives serialization, undo/redo,
    /// and multiple dispatcher lifetimes. Used by `CommandDispatcher` to
    /// generate unique names like `block_3`, `boolean_union_7`.
    next_feature_seq: u64,
}

impl<R: FeatureRegistry> Default for FeatureTree<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: FeatureRegistry> FeatureTree<R> {
    fn kernel_to_signal(err: KernelError) -> SignalError {
        SignalError::internal(err.to_string())
    }

    fn signal_to_kernel(err: SignalError) -> KernelError {
        KernelError::InternalError {
            message: err.to_string(),
            context: None,
        }
    }

    /// Create a new empty feature tree.
    pub fn new() -> Self {
        Self {
            graph: SignalGraph::new(),
            features: HashMap::new(),
            envelopes: HashMap::new(),
            names: HashMap::new(),
            next_feature_seq: 0,
        }
    }

    /// Return the next monotonic sequence number and increment.
    ///
    /// Used by dispatchers to generate deterministic, unique feature names.
    /// The counter is owned by the tree (not the dispatcher) so it survives
    /// dispatcher recreation, undo/redo, and serialization round-trips.
    pub fn next_seq(&mut self) -> u64 {
        let seq = self.next_feature_seq;
        self.next_feature_seq += 1;
        seq
    }

    /// Register a new feature in the tree.
    ///
    /// 1. Allocates a NodeId in the signal graph.
    /// 2. Registers dependencies.
    /// 3. Stores the feature logic.
    pub fn register_feature(&mut self, feature: R) -> Result<NodeId, KernelError> {
        let node_id = self.graph.create_node();
        let deps = feature.dependencies();

        for dep_id in deps {
            self.graph
                .add_dependency(node_id, dep_id, TOPOLOGY_ASPECT)
                .map_err(Self::signal_to_kernel)?;
            self.graph
                .add_dependency(node_id, dep_id, GEOMETRY_ASPECT)
                .map_err(Self::signal_to_kernel)?;
        }
        // Enforce feature name uniqueness — full path, not trailing segment.
        // Previous code used split('/').last() which silently overwrote
        // features sharing a trailing name segment.
        let name = feature.name().to_string();
        if self.names.contains_key(&name) {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "Duplicate feature name '{}'. Feature names must be unique.",
                    name
                ),
                context: None,
            });
        }
        self.names.insert(name, node_id);

        self.features.insert(node_id, feature);

        forge_signal::facade::mark_dirty(&mut self.graph, node_id, TOPOLOGY_ASPECT)
            .map_err(Self::signal_to_kernel)?;
        forge_signal::facade::mark_dirty(&mut self.graph, node_id, GEOMETRY_ASPECT)
            .map_err(Self::signal_to_kernel)?;

        Ok(node_id)
    }

    /// Replace an existing feature with a new implementation.
    ///
    /// Preserves the NodeId but updates the logic and marks dependencies dirty.
    pub fn replace_feature(&mut self, node_id: NodeId, feature: R) -> Result<(), KernelError> {
        if !self.graph.is_alive(node_id) {
            return Err(KernelError::InvalidInput {
                message: format!("Node {} is not alive", node_id),
                context: None,
            });
        }

        let old_feature = self.features.insert(node_id, feature);

        if let Some(old) = old_feature {
            for dep_id in old.dependencies().iter() {
                self.graph
                    .remove_dependency(node_id, *dep_id)
                    .map_err(Self::signal_to_kernel)?;
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
                .add_dependency(node_id, dep_id, TOPOLOGY_ASPECT)
                .map_err(Self::signal_to_kernel)?;
            self.graph
                .add_dependency(node_id, dep_id, GEOMETRY_ASPECT)
                .map_err(Self::signal_to_kernel)?;
        }

        forge_signal::facade::mark_dirty(&mut self.graph, node_id, TOPOLOGY_ASPECT)
            .map_err(Self::signal_to_kernel)?;
        forge_signal::facade::mark_dirty(&mut self.graph, node_id, GEOMETRY_ASPECT)
            .map_err(Self::signal_to_kernel)?;

        Ok(())
    }

    /// Evaluate a specific feature (and its dependencies) to get the latest output.
    ///
    /// Convenience wrapper over `evaluate_feature_with_context` that uses a
    /// default `ModelingContext`. Returns only the `SolidEnvelope` — callers
    /// that need the full envelope should use `evaluate_feature_with_context`.
    pub fn evaluate_feature(&mut self, node_id: NodeId) -> Result<SolidEnvelope, KernelError> {
        let session = KernelConfig::default();
        let envelope = self.evaluate_feature_with_config(node_id, &session)?;
        Ok(envelope.into_value())
    }

    /// Evaluate a feature with an explicit `KernelConfig`.
    ///
    /// Returns the full `OperationResult<SolidEnvelope>` envelope so callers
    /// can inspect the decision log, warnings, metrics, and lineage alongside
    /// the domain output.
    ///
    /// The envelope stored per-node is the canonical metadata record — the
    /// `ModelingContext`'s decision log is drained into each envelope by the
    /// `OperationFinalizer` during pipeline execution.
    pub fn evaluate_feature_with_config(
        &mut self,
        node_id: NodeId,
        session_config: &KernelConfig,
    ) -> Result<OperationResult<SolidEnvelope>, KernelError> {
        let graph = &mut self.graph;
        let features = &self.features;
        let envelopes = &mut self.envelopes;

        let mut pending_traces: HashMap<NodeId, TraceSummary> = HashMap::new();

        let mut compute =
            |id: NodeId, _graph_ref: &SignalGraph| -> Result<AspectVersion, SignalError> {
                let feature = features.get(&id).ok_or_else(|| KernelError::InvalidInput {
                    message: format!("Feature logic not found for node {}", id),
                    context: None,
                })
                .map_err(Self::kernel_to_signal)?;

                // Build input map by cloning SolidEnvelope from stored envelopes.
                // This is the single, unavoidable clone — the signal graph cache
                // owns the canonical data, features need their own copy. Topology
                // is Arc (O(1) clone), geometry is the real cost (O(V+F)).
                let mut input_map = HashMap::new();
                for dep_id in feature.dependencies() {
                    if let Some(envelope) = envelopes.get(&dep_id) {
                        input_map.insert(dep_id, envelope.get_value().clone());
                    } else {
                        return Err(Self::kernel_to_signal(KernelError::InvalidInput {
                            message: format!("Dependency output missing for node {}", dep_id),
                            context: None,
                        }));
                    }
                }

                let envelope =
                    feature
                        .execute_via_pipeline(input_map, session_config)
                        .map_err(Self::kernel_to_signal)?;

                // Build the trace summary from the envelope's decision log —
                // NOT from ctx, which was drained by the OperationFinalizer.
                let hash = forge_topo::transactions::compute_arena_topology_hash(
                    envelope.get_value().topology().arena(),
                );
                let core_summary = envelope.get_decision_log().to_summary(hash);
                let summary = TraceSummary {
                    output_hash: core_summary.get_state_hash(),
                    labels: vec![
                        format!("interesting={}", core_summary.get_interesting().len()),
                        format!("spans={}", core_summary.get_span_summaries().len()),
                    ],
                };
                pending_traces.insert(id, summary);

                // Store the full envelope — metadata is preserved.
                envelopes.insert(id, envelope);

                Ok(AspectVersion::from_updates([
                    (TOPOLOGY_ASPECT, 1),
                    (GEOMETRY_ASPECT, 1),
                ]))
            };

        forge_signal::facade::evaluate(graph, node_id, &mut compute)
            .map_err(Self::signal_to_kernel)?;

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
    pub fn get_envelope(&self, node_id: NodeId) -> Option<&OperationResult<SolidEnvelope>> {
        self.envelopes.get(&node_id)
    }
}
