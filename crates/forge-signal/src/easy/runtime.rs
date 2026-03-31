use std::any::Any;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use std::sync::Mutex;

use crate::data::trace::{RuntimeArtifactHot, RuntimeArtifactState, RuntimeArtifactWarm};
use crate::facade::runtime::mark_dirty_batch;
use crate::facade::{
    AspectMask, BatchChange, ChangedRegion, DependencyEdge, NodeId, NodeState, SignalError,
    SignalGraph,
};
use crate::logic::prepared::PreparedEvaluation;

use super::compute::{Computed, ErasedComputed, SignalContext};
use super::signal::{ComputedSignal, InputSignal, Signal, DEFAULT_ASPECT};

/// Convenience app wrapper for lightweight typed experiments and examples.
///
/// This surface intentionally trades execution-model explicitness for ergonomics.
/// It is not the canonical kernel-grade runtime interface.
pub struct SignalApp {
    graph: SignalGraph,
    values: HashMap<NodeId, Box<dyn Any + Send + Sync>>,
    computed: HashMap<NodeId, Box<dyn ErasedComputed>>,
    batch_depth: usize,
    batched_dirty_nodes: BTreeSet<NodeId>,
    batch_value_undo: HashMap<NodeId, Option<Box<dyn Any + Send + Sync>>>,
    batch_entry_undo: HashMap<NodeId, crate::data::node::NodeEntry>,
}

impl Default for SignalApp {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalApp {
    pub fn new() -> Self {
        Self {
            graph: SignalGraph::new(),
            values: HashMap::new(),
            computed: HashMap::new(),
            batch_depth: 0,
            batched_dirty_nodes: BTreeSet::new(),
            batch_value_undo: HashMap::new(),
            batch_entry_undo: HashMap::new(),
        }
    }

    pub fn input<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> InputSignal<T> {
        let node = self.graph.node().build();
        self.values.insert(node, Box::new(value));

        let mut entry = self
            .graph
            .get_entry_mut(node)
            .expect("newly created input node should be available");
        entry.set_state(NodeState::Clean);
        entry.set_dirty_aspects(AspectMask::EMPTY);
        let version = entry.get_aspect_version();
        entry.set_runtime_artifact_state(Some(easy_seed_runtime_artifact_state(version, 0, false)));

        Signal::new(node)
    }

    pub fn computed<T, F>(&mut self, compute: F) -> ComputedSignal<T>
    where
        T: Clone + Send + Sync + 'static,
        F: Fn(&mut SignalContext<'_>) -> T + Send + Sync + 'static,
    {
        let node = self.graph.node().on_demand().build();
        let erased: Box<dyn ErasedComputed> = Box::new(Computed {
            closure: compute,
            marker: PhantomData,
        });
        self.computed.insert(node, erased);
        self.seed_computed_if_possible(node)
            .expect("easy path failed to seed computed node");
        Signal::new(node)
    }

    pub fn get<T: Clone + Send + Sync + 'static>(&mut self, signal: Signal<T>) -> T {
        self.try_get(signal).expect("easy path evaluation failed")
    }

    pub fn try_get<T: Clone + Send + Sync + 'static>(
        &mut self,
        signal: Signal<T>,
    ) -> Result<T, SignalError> {
        self.ensure_evaluated(signal.node)
            .map_err(|err| SignalError::internal(format!("easy path evaluation failed: {err}")))?;
        self.try_read_value(signal)
    }

    pub fn set<T: Clone + Send + Sync + 'static>(&mut self, signal: InputSignal<T>, value: T) {
        self.try_set(signal, value).expect("easy path set failed");
    }

    pub fn try_set<T: Clone + Send + Sync + 'static>(
        &mut self,
        signal: InputSignal<T>,
        value: T,
    ) -> Result<(), SignalError> {
        if self.batch_depth > 0 {
            self.batch_value_undo
                .entry(signal.node)
                .or_insert_with(|| self.values.remove(&signal.node));
            self.batch_entry_undo.entry(signal.node).or_insert_with(|| {
                self.graph
                    .get_entry(signal.node)
                    .expect("input node should exist")
                    .clone()
            });
        }
        self.values.insert(signal.node, Box::new(value));
        {
            let mut entry = self.graph.get_entry_mut(signal.node)?;
            let current = entry.get_aspect_version().get(DEFAULT_ASPECT);
            let next = entry.get_aspect_version().with(DEFAULT_ASPECT, current + 1);
            entry.set_aspect_version(next);
            entry.set_state(NodeState::Clean);
            entry.set_dirty_aspects(AspectMask::EMPTY);
            entry
                .set_runtime_artifact_state(Some(easy_seed_runtime_artifact_state(next, 0, false)));
        }

        if self.batch_depth > 0 {
            self.batched_dirty_nodes.insert(signal.node);
        } else {
            mark_dirty_batch(
                &mut self.graph,
                &BatchChange::singleton(signal.node, DEFAULT_ASPECT, Vec::<ChangedRegion>::new()),
            )?;
        }
        Ok(())
    }

    pub fn batch<F>(&mut self, apply: F)
    where
        F: FnOnce(&mut Self),
    {
        self.try_batch(|graph| {
            apply(graph);
            Ok(())
        })
        .expect("easy path batch failed");
    }

    pub fn try_batch<F>(&mut self, apply: F) -> Result<(), SignalError>
    where
        F: FnOnce(&mut Self) -> Result<(), SignalError>,
    {
        self.batch_depth += 1;
        let apply_result = apply(self);
        self.batch_depth -= 1;

        if let Err(err) = apply_result {
            if self.batch_depth == 0 {
                self.batched_dirty_nodes.clear();
                self.restore_batch_undo();
            }
            return Err(err);
        }

        if self.batch_depth == 0 {
            let dirty_nodes = std::mem::take(&mut self.batched_dirty_nodes);
            if let Err(err) = mark_dirty_batch(
                &mut self.graph,
                &BatchChange::from_sources(
                    dirty_nodes.into_iter().map(|node| (node, DEFAULT_ASPECT)),
                ),
            ) {
                self.restore_batch_undo();
                return Err(err);
            }
            self.clear_batch_undo();
        }
        Ok(())
    }

    fn ensure_evaluated(&mut self, node: NodeId) -> Result<(), SignalError> {
        if !self.computed.contains_key(&node) {
            return Ok(());
        }

        let plan = self.graph.build_evaluation_plan(
            &[node],
            crate::logic::evaluation::EvaluationRequestMode::Default,
        )?;
        let staged_values: Mutex<HashMap<NodeId, Box<dyn Any + Send + Sync>>> =
            Mutex::new(HashMap::new());
        let graph = &mut self.graph;
        let computed = &self.computed;
        let values = &self.values;
        graph.execute_prepared_plan_with_precompute(&plan, &|current, view| {
            if let Some(computed) = computed.get(&current) {
                let current_version = view
                    .graph()
                    .get_entry(current)?
                    .get_aspect_version()
                    .get(DEFAULT_ASPECT);
                let staged_guard = staged_values
                    .lock()
                    .map_err(|_| SignalError::internal("easy path staged value mutex poisoned"))?;
                let (value, prepared) =
                    computed.precompute(values, &staged_guard, current_version)?;
                drop(staged_guard);
                staged_values
                    .lock()
                    .map_err(|_| SignalError::internal("easy path staged value mutex poisoned"))?
                    .insert(current, value);
                Ok(prepared)
            } else {
                Ok(PreparedEvaluation::validated_clean())
            }
        })?;

        let staged_values = staged_values
            .into_inner()
            .map_err(|_| SignalError::internal("easy path staged value mutex poisoned"))?;
        for (node, value) in staged_values {
            self.values.insert(node, value);
        }
        Ok(())
    }

    fn seed_computed_if_possible(&mut self, node: NodeId) -> Result<(), SignalError> {
        let Some(computed) = self.computed.get(&node) else {
            return Ok(());
        };
        let staged = HashMap::new();
        let Ok((value, prepared)) = computed.precompute(&self.values, &staged, 0) else {
            return Ok(());
        };
        self.values.insert(node, value);
        let mut dep_snapshot = crate::data::dependency::DependencySnapshot::empty();
        for dependency in prepared.dependencies.as_slice() {
            let current_version = self
                .graph
                .get_entry(dependency.source)?
                .get_aspect_version()
                .get(dependency.aspect);
            dep_snapshot.record(dependency.source, dependency.aspect, current_version, None);
        }
        {
            let mut entry = self.graph.get_entry_mut(node)?;
            entry.set_aspect_version(prepared.result.aspect_version);
            entry.set_state(NodeState::Clean);
            entry.set_dirty_aspects(AspectMask::EMPTY);
            entry.clear_dirty_partition_scopes();
            entry.set_runtime_artifact_state(Some(easy_seed_runtime_artifact_state(
                prepared.result.aspect_version,
                prepared.dependencies.len() as u32,
                true,
            )));
        }
        self.graph.set_dependencies(
            node,
            prepared
                .dependencies
                .as_slice()
                .iter()
                .map(|dependency| match &dependency.scope {
                    Some(scope) => DependencyEdge::with_partition_scope(
                        dependency.source,
                        dependency.aspect,
                        scope.clone(),
                    ),
                    None => DependencyEdge::new(dependency.source, dependency.aspect),
                }),
        )?;
        self.graph.set_dep_snapshot(node, dep_snapshot)?;
        Ok(())
    }

    fn restore_batch_undo(&mut self) {
        let entry_undo = std::mem::take(&mut self.batch_entry_undo);
        for (node, entry) in entry_undo {
            if let Ok(mut slot) = self.graph.get_entry_mut(node) {
                *slot = entry;
            }
        }
        let value_undo = std::mem::take(&mut self.batch_value_undo);
        for (node, previous) in value_undo {
            match previous {
                Some(value) => {
                    self.values.insert(node, value);
                }
                None => {
                    self.values.remove(&node);
                }
            }
        }
    }

    fn clear_batch_undo(&mut self) {
        self.batch_value_undo.clear();
        self.batch_entry_undo.clear();
    }

    fn try_read_value<T: Clone + Send + Sync + 'static>(
        &self,
        signal: Signal<T>,
    ) -> Result<T, SignalError> {
        let stored = self
            .values
            .get(&signal.node)
            .ok_or_else(|| SignalError::invalid_input("easy signal has no stored value"))?;
        stored
            .downcast_ref::<T>()
            .cloned()
            .ok_or_else(|| SignalError::invalid_input("easy signal type mismatch"))
    }
}

fn easy_seed_runtime_artifact_state(
    version: crate::facade::AspectVersion,
    dependency_count: u32,
    recomputed: bool,
) -> RuntimeArtifactState {
    let mut hot = RuntimeArtifactHot::default();
    hot.dependency_count = dependency_count;
    hot.recomputed = recomputed;
    hot.changed_partition_count = 0;
    hot.meaningful_input_changes = 0;
    hot.propagation_suppressed = false;
    hot.output_hash = crate::data::core_profile::StableHashValue::from(version.slots()[0]);
    let mut warm = RuntimeArtifactWarm::default();
    warm.memoized_origin = crate::data::output::MemoizedResultOrigin::DirectCompute;
    RuntimeArtifactState::new(hot, warm)
}
