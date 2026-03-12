//! Parametric Feature Tree powered by forge-signal.
//!
//! DOMAIN: Managing the dependency graph of high-level features.
//!
//! INVARIANTS:
//! - All feature evaluation goes through `FeaturePipeline::execute`
//! - `forge-signal` owns scheduling, invalidation, rollback, condition gating,
//!   and aspect-version comparison for the feature graph
//! - `forge-kernel` owns feature logic, semantic aspect derivation, and the
//!   canonical `OperationResult<SolidEnvelope>` envelope cache
//! - Every feature node must return meaningful monotonic aspect versions
//!   derived from host-owned envelope changes, never placeholder counters
//! - Raw structural graph rewiring is transitional and host-owned; evaluation
//!   and dirty propagation flow through `SignalRuntime` transactions
//! - Serialization persists committed graph state plus kernel-owned caches and
//!   reconstructs a fresh runtime shell on deserialize
//! - Topology is immutable (passed as snapshots)
//! - `FeatureTree<R>` is generic over the feature registry `R`

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use forge_core::envelope::OperationResult;
use forge_core::KernelError;
use forge_signal::facade::{
    evaluate_in_txn, Aspect, AspectMask, AspectVersion, CheckpointBarrier, DefaultComparatorResolver,
    DependencyEdge, NodeId, SignalError, SignalGraph, SignalRuntime, TraceSummary, TransactionOutcome,
};

use super::contracts::feature_dependency::FeatureAspect;
use super::contracts::feature_registry::FeatureRegistry;
use super::contracts::feature_signal_policy::{FeatureSignalPolicy, FeatureSignalTier};
use super::output::solid_envelope::SolidEnvelope;
use crate::configuration::facade::KernelConfig;
use crate::geometry::facade::GeometryStore;

const TOPOLOGY_ASPECT: Aspect = Aspect::new(0);
const GEOMETRY_ASPECT: Aspect = Aspect::new(1);

type FeatureSignalRuntime = SignalRuntime<(), (), (), (), FeatureSignalTier>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "R: Serialize",
    deserialize = "R: serde::de::DeserializeOwned",
))]
struct FeatureTreeSnapshot<R: FeatureRegistry> {
    graph: SignalGraph,
    node_tiers: HashMap<NodeId, FeatureSignalTier>,
    features: HashMap<NodeId, R>,
    envelopes: HashMap<NodeId, OperationResult<SolidEnvelope>>,
    names: HashMap<String, NodeId>,
    next_feature_seq: u64,
}

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
pub struct FeatureTree<R: FeatureRegistry> {
    /// Transactional signal runtime that owns the committed graph state.
    runtime: FeatureSignalRuntime,
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

impl<R: FeatureRegistry> fmt::Debug for FeatureTree<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FeatureTree")
            .field("graph", self.runtime.graph())
            .field("features", &self.features)
            .field("envelopes", &self.envelopes)
            .field("names", &self.names)
            .field("next_feature_seq", &self.next_feature_seq)
            .finish()
    }
}

impl<R> Serialize for FeatureTree<R>
where
    R: FeatureRegistry + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        FeatureTreeSnapshot {
            graph: self.runtime.graph().clone(),
            node_tiers: self
                .features
                .keys()
                .filter_map(|&node_id| self.signal_tier(node_id).map(|tier| (node_id, tier)))
                .collect(),
            features: self.features.clone(),
            envelopes: self.envelopes.clone(),
            names: self.names.clone(),
            next_feature_seq: self.next_feature_seq,
        }
        .serialize(serializer)
    }
}

impl<'de, R> Deserialize<'de> for FeatureTree<R>
where
    R: FeatureRegistry + serde::de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let snapshot = FeatureTreeSnapshot::<R>::deserialize(deserializer)?;
        let mut runtime = Self::new_runtime(snapshot.graph);
        for (node_id, tier) in &snapshot.node_tiers {
            runtime.set_node_tier(*node_id, *tier);
        }
        Ok(Self {
            runtime,
            features: snapshot.features,
            envelopes: snapshot.envelopes,
            names: snapshot.names,
            next_feature_seq: snapshot.next_feature_seq,
        })
    }
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

    fn signal_aspect(aspect: FeatureAspect) -> Aspect {
        match aspect {
            FeatureAspect::Topology => TOPOLOGY_ASPECT,
            FeatureAspect::Geometry => GEOMETRY_ASPECT,
        }
    }

    fn signal_to_kernel(err: SignalError) -> KernelError {
        KernelError::InternalError {
            message: err.to_string(),
            context: None,
        }
    }

    fn new_runtime(graph: SignalGraph) -> FeatureSignalRuntime {
        let mut runtime = SignalRuntime::builder(graph)
            .with_tiers::<FeatureSignalTier>()
            .checkpoint_barrier(CheckpointBarrier::PerOperation)
            .build();
        runtime.set_tier_policy(FeatureSignalPolicy::core_tier_policy());
        runtime
    }

    fn version_for_output(
        previous: Option<&OperationResult<SolidEnvelope>>,
        next: &OperationResult<SolidEnvelope>,
        prior_version: AspectVersion,
    ) -> AspectVersion {
        let next_topology = next.get_value().topology_fingerprint();
        let next_geometry = next.get_value().geometry_fingerprint();

        let (topology_changed, geometry_changed) = match previous {
            Some(previous_envelope) => {
                let previous_topology = previous_envelope.get_value().topology_fingerprint();
                let previous_geometry = previous_envelope.get_value().geometry_fingerprint();
                (
                    previous_topology != next_topology,
                    previous_geometry != next_geometry,
                )
            }
            None => (true, true),
        };

        let mut next_version = prior_version;
        if topology_changed {
            next_version = next_version.bump(TOPOLOGY_ASPECT);
        }
        if geometry_changed {
            next_version = next_version.bump(GEOMETRY_ASPECT);
        }
        next_version
    }

    fn dependency_edges_for_feature(feature: &R) -> Vec<DependencyEdge> {
        let mut desired = Vec::new();
        for binding in feature.dependency_bindings() {
            let upstream = binding.node_id();
            let aspects = binding.aspects();

            if aspects.intersects(AspectMask::from_aspect(TOPOLOGY_ASPECT)) {
                desired.push(DependencyEdge::new(upstream, TOPOLOGY_ASPECT));
            }
            if aspects.intersects(AspectMask::from_aspect(GEOMETRY_ASPECT)) {
                desired.push(DependencyEdge::new(upstream, GEOMETRY_ASPECT));
            }
        }
        desired
    }

    fn wire_dependency_bindings(
        graph: &mut SignalGraph,
        node_id: NodeId,
        feature: &R,
    ) -> Result<(), KernelError> {
        graph
            .set_dependencies(node_id, Self::dependency_edges_for_feature(feature))
            .map_err(Self::signal_to_kernel)
    }

    fn materialize_input(
        envelope: &OperationResult<SolidEnvelope>,
        binding: super::contracts::feature_dependency::FeatureDependency,
    ) -> SolidEnvelope {
        if binding
            .aspects()
            .intersects(AspectMask::from_aspect(GEOMETRY_ASPECT))
        {
            return envelope.get_value().clone();
        }

        SolidEnvelope::new(
            envelope.get_value().topology().clone(),
            GeometryStore::default(),
        )
    }

    /// Create a new empty feature tree.
    pub fn new() -> Self {
        Self {
            runtime: Self::new_runtime(SignalGraph::new()),
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
        let signal_policy = feature.signal_policy();
        signal_policy.validate_for_feature_tree()?;
        let node_id = self
            .runtime
            .graph_mut()
            .create_node_with_config(signal_policy.node_config().clone());
        if let Some(tier) = signal_policy.tier() {
            self.runtime.set_node_tier(node_id, tier);
        }
        Self::wire_dependency_bindings(self.runtime.graph_mut(), node_id, &feature)?;
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

        Ok(node_id)
    }

    /// Replace an existing feature with a new implementation.
    ///
    /// Preserves the NodeId but updates the logic and marks dependencies dirty.
    pub fn replace_feature(&mut self, node_id: NodeId, feature: R) -> Result<(), KernelError> {
        if !self.runtime.graph().is_alive(node_id) {
            return Err(KernelError::InvalidInput {
                message: format!("Node {} is not alive", node_id),
                context: None,
            });
        }

        self.features.insert(node_id, feature);

        let new_feature =
            self.features
                .get(&node_id)
                .ok_or_else(|| KernelError::InternalError {
                    message: format!("Feature missing after insert for node {}", node_id),
                    context: None,
                })?;
        self.runtime
            .graph_mut()
            .set_dependencies(node_id, Self::dependency_edges_for_feature(new_feature))
            .map_err(Self::signal_to_kernel)?;

        let mut txn = self.runtime.begin();
        txn.mark_dirty(node_id, TOPOLOGY_ASPECT)
            .map_err(Self::signal_to_kernel)?;
        txn.mark_dirty(node_id, GEOMETRY_ASPECT)
            .map_err(Self::signal_to_kernel)?;

        let mut runtime_ctx = ();
        match txn
            .commit(&mut runtime_ctx)
            .map_err(Self::signal_to_kernel)?
        {
            TransactionOutcome::Committed => {}
            TransactionOutcome::RolledBack | TransactionOutcome::Poisoned => {
                return Err(KernelError::InternalError {
                    message: format!(
                        "feature replacement invalidation failed for node {}",
                        node_id
                    ),
                    context: None,
                });
            }
        }

        Ok(())
    }

    /// Mark one feature node dirty for a specific semantic aspect.
    ///
    /// This is the host-facing bridge into `forge-signal` invalidation. Feature
    /// definitions and other kernel state remain host-owned; the signal runtime
    /// owns downstream scheduling and rollback-sensitive propagation.
    pub fn mark_feature_dirty(
        &mut self,
        node_id: NodeId,
        aspect: FeatureAspect,
    ) -> Result<(), KernelError> {
        let mut txn = self.runtime.begin();
        txn.mark_dirty(node_id, Self::signal_aspect(aspect))
            .map_err(Self::signal_to_kernel)?;

        let mut runtime_ctx = ();
        match txn
            .commit(&mut runtime_ctx)
            .map_err(Self::signal_to_kernel)?
        {
            TransactionOutcome::Committed => Ok(()),
            TransactionOutcome::RolledBack | TransactionOutcome::Poisoned => {
                Err(KernelError::InternalError {
                    message: format!("feature invalidation failed for node {}", node_id),
                    context: None,
                })
            }
        }
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
        let features = &self.features;
        let committed_envelopes = &self.envelopes;
        let mut pending_envelopes: HashMap<NodeId, OperationResult<SolidEnvelope>> = HashMap::new();
        let mut pending_traces: HashMap<NodeId, TraceSummary> = HashMap::new();

        let mut compute = |id: NodeId,
                           graph_ref: &SignalGraph|
         -> Result<AspectVersion, SignalError> {
            let feature = features
                .get(&id)
                .ok_or_else(|| KernelError::InvalidInput {
                    message: format!("Feature logic not found for node {}", id),
                    context: None,
                })
                .map_err(Self::kernel_to_signal)?;

            // Build input map by cloning SolidEnvelope from stored envelopes.
            // This is the single, unavoidable clone — the signal graph cache
            // owns the canonical data, features need their own copy. Topology
            // is Arc (O(1) clone), geometry is the real cost (O(V+F)).
            let mut input_map = HashMap::new();
            for binding in feature.dependency_bindings() {
                let dep_id = binding.node_id();
                if let Some(envelope) = pending_envelopes
                    .get(&dep_id)
                    .or_else(|| committed_envelopes.get(&dep_id))
                {
                    input_map.insert(dep_id, Self::materialize_input(envelope, binding));
                } else {
                    return Err(Self::kernel_to_signal(KernelError::InvalidInput {
                        message: format!("Dependency output missing for node {}", dep_id),
                        context: None,
                    }));
                }
            }

            let envelope = feature
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

            let prior_version = graph_ref.get_entry(id)?.get_aspect_version();
            let next_version =
                Self::version_for_output(committed_envelopes.get(&id), &envelope, prior_version);

            pending_envelopes.insert(id, envelope);

            Ok(next_version)
        };

        let mut txn = self.runtime.begin();
        if let Err(err) =
            evaluate_in_txn(&mut txn, node_id, &mut compute, DefaultComparatorResolver)
        {
            let mut runtime_ctx = ();
            let _ = txn.rollback(&mut runtime_ctx);
            return Err(Self::signal_to_kernel(err));
        }

        let mut runtime_ctx = ();
        match txn
            .commit(&mut runtime_ctx)
            .map_err(Self::signal_to_kernel)?
        {
            TransactionOutcome::Committed => {}
            TransactionOutcome::RolledBack | TransactionOutcome::Poisoned => {
                return Err(KernelError::InternalError {
                    message: format!("feature evaluation rollback for node {}", node_id),
                    context: None,
                });
            }
        }

        for (id, envelope) in pending_envelopes {
            self.envelopes.insert(id, envelope);
        }

        for (id, summary) in pending_traces {
            if let Ok(entry) = self.runtime.graph_mut().get_entry_mut(id) {
                entry.set_trace_summary(Some(summary));
            }
        }

        self.envelopes
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
        self.runtime.graph()
    }

    /// Read the assigned signal tier for one feature node, if any.
    pub fn signal_tier(&self, node_id: NodeId) -> Option<FeatureSignalTier> {
        self.runtime.config().node_meta().tier_for_node(node_id)
    }

    /// Read-only access to a stored envelope (for external audit inspection).
    pub fn get_envelope(&self, node_id: NodeId) -> Option<&OperationResult<SolidEnvelope>> {
        self.envelopes.get(&node_id)
    }
}
