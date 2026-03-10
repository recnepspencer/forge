//! Convenience-only typed wrapper around `SignalGraph`.
//!
//! This module optimizes for approachability and small examples, not kernel-grade
//! execution performance or fully static contracts. Heavyweight/runtime-critical
//! integrations should prefer the prepared/runtime APIs directly.

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Mutex;

use crate::facade::{
    mark_dirty, Aspect, AspectMask, AspectVersion, NodeEvaluationResult, NodeId, NodeState,
    PreparedDependencyCapture, PreparedEvaluation, SignalError, SignalGraph,
};

const DEFAULT_ASPECT: Aspect = Aspect::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Signal<T> {
    node: NodeId,
    marker: PhantomData<fn() -> T>,
}

impl<T> Signal<T> {
    fn new(node: NodeId) -> Self {
        Self {
            node,
            marker: PhantomData,
        }
    }
}

pub type InputSignal<T> = Signal<T>;
pub type ComputedSignal<T> = Signal<T>;

trait ErasedComputed: Send + Sync {
    fn precompute(
        &self,
        values: &HashMap<NodeId, Box<dyn Any + Send + Sync>>,
        staged_values: &HashMap<NodeId, Box<dyn Any + Send + Sync>>,
        current_version: u64,
    ) -> Result<(Box<dyn Any + Send + Sync>, PreparedEvaluation), SignalError>;
}

struct Computed<T, F>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&mut ComputeContext<'_>) -> T + Send + Sync + 'static,
{
    closure: F,
    marker: PhantomData<fn() -> T>,
}

impl<T, F> ErasedComputed for Computed<T, F>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&mut ComputeContext<'_>) -> T + Send + Sync + 'static,
{
    fn precompute(
        &self,
        values: &HashMap<NodeId, Box<dyn Any + Send + Sync>>,
        staged_values: &HashMap<NodeId, Box<dyn Any + Send + Sync>>,
        current_version: u64,
    ) -> Result<(Box<dyn Any + Send + Sync>, PreparedEvaluation), SignalError> {
        let mut capture = PreparedDependencyCapture::default();
        let mut context = ComputeContext {
            values,
            staged_values,
            capture: &mut capture,
        };
        let value = (self.closure)(&mut context);
        let next_version = AspectVersion::zero().with(DEFAULT_ASPECT, current_version + 1);
        let prepared =
            PreparedEvaluation::from_result(NodeEvaluationResult::from_version(next_version))
                .with_dependencies(capture);
        Ok((Box::new(value), prepared))
    }
}

pub struct ComputeContext<'a> {
    values: &'a HashMap<NodeId, Box<dyn Any + Send + Sync>>,
    staged_values: &'a HashMap<NodeId, Box<dyn Any + Send + Sync>>,
    capture: &'a mut PreparedDependencyCapture,
}

impl<'a> ComputeContext<'a> {
    pub fn get<T: Clone + Send + Sync + 'static>(&mut self, signal: Signal<T>) -> T {
        self.capture.record(signal.node, DEFAULT_ASPECT, None);
        self.staged_values
            .get(&signal.node)
            .or_else(|| self.values.get(&signal.node))
            .expect("easy-mode signal has no stored value")
            .downcast_ref::<T>()
            .expect("easy-mode signal type mismatch")
            .clone()
    }
}

/// Convenience graph wrapper for lightweight typed experiments and examples.
///
/// This surface intentionally trades execution-model explicitness for ergonomics.
/// It is not the canonical kernel-grade runtime interface.
pub struct ReactiveGraph {
    graph: SignalGraph,
    values: HashMap<NodeId, Box<dyn Any + Send + Sync>>,
    computed: HashMap<NodeId, Box<dyn ErasedComputed>>,
    batch_depth: usize,
    batched_dirty_nodes: Vec<NodeId>,
    batch_value_undo: HashMap<NodeId, Option<Box<dyn Any + Send + Sync>>>,
    batch_entry_undo: HashMap<NodeId, crate::data::node::NodeEntry>,
}

impl Default for ReactiveGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ReactiveGraph {
    pub fn new() -> Self {
        Self {
            graph: SignalGraph::new(),
            values: HashMap::new(),
            computed: HashMap::new(),
            batch_depth: 0,
            batched_dirty_nodes: Vec::new(),
            batch_value_undo: HashMap::new(),
            batch_entry_undo: HashMap::new(),
        }
    }

    pub fn input<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> InputSignal<T> {
        let node = self.graph.node().build();
        self.values.insert(node, Box::new(value));

        let entry = self
            .graph
            .get_entry_mut(node)
            .expect("newly created input node should be available");
        entry.set_state(NodeState::Clean);
        entry.set_dirty_aspects(AspectMask::EMPTY);

        Signal::new(node)
    }

    pub fn computed<T, F>(&mut self, compute: F) -> ComputedSignal<T>
    where
        T: Clone + Send + Sync + 'static,
        F: Fn(&mut ComputeContext<'_>) -> T + Send + Sync + 'static,
    {
        let node = self.graph.node().on_demand().build();
        let erased: Box<dyn ErasedComputed> = Box::new(Computed {
            closure: compute,
            marker: PhantomData,
        });
        self.computed.insert(node, erased);
        self.seed_computed_if_possible(node)
            .expect("easy-mode failed to seed computed node");
        Signal::new(node)
    }

    pub fn get<T: Clone + Send + Sync + 'static>(&mut self, signal: Signal<T>) -> T {
        self.try_get(signal).expect("easy-mode evaluation failed")
    }

    pub fn try_get<T: Clone + Send + Sync + 'static>(
        &mut self,
        signal: Signal<T>,
    ) -> Result<T, SignalError> {
        self.ensure_evaluated(signal.node)
            .map_err(|err| SignalError::internal(format!("easy-mode evaluation failed: {err}")))?;
        self.try_read_value(signal)
    }

    pub fn set<T: Clone + Send + Sync + 'static>(&mut self, signal: InputSignal<T>, value: T) {
        self.try_set(signal, value).expect("easy-mode set failed");
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
            let entry = self.graph.get_entry_mut(signal.node)?;
            let current = entry.get_aspect_version().get(DEFAULT_ASPECT);
            let next = entry.get_aspect_version().with(DEFAULT_ASPECT, current + 1);
            entry.set_aspect_version(next);
            entry.set_state(NodeState::Clean);
            entry.set_dirty_aspects(AspectMask::EMPTY);
        }

        if self.batch_depth > 0 {
            self.batched_dirty_nodes.push(signal.node);
        } else {
            mark_dirty(&mut self.graph, signal.node, DEFAULT_ASPECT)?;
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
        .expect("easy-mode batch failed");
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
            let mut dirty_nodes = std::mem::take(&mut self.batched_dirty_nodes);
            dirty_nodes.sort();
            dirty_nodes.dedup();
            for node in dirty_nodes {
                if let Err(err) = mark_dirty(&mut self.graph, node, DEFAULT_ASPECT) {
                    self.restore_batch_undo();
                    return Err(err);
                }
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
        graph.execute_prepared_plan(&plan, &|current, view| {
            if let Some(computed) = computed.get(&current) {
                let current_version = view
                    .graph()
                    .get_entry(current)?
                    .get_aspect_version()
                    .get(DEFAULT_ASPECT);
                let staged_guard = staged_values
                    .lock()
                    .map_err(|_| SignalError::internal("easy-mode staged value mutex poisoned"))?;
                let (value, prepared) =
                    computed.precompute(values, &staged_guard, current_version)?;
                drop(staged_guard);
                staged_values
                    .lock()
                    .map_err(|_| SignalError::internal("easy-mode staged value mutex poisoned"))?
                    .insert(current, value);
                Ok(prepared)
            } else {
                Ok(PreparedEvaluation::validated_clean())
            }
        })?;

        let staged_values = staged_values
            .into_inner()
            .map_err(|_| SignalError::internal("easy-mode staged value mutex poisoned"))?;
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
            let entry = self.graph.get_entry_mut(node)?;
            entry.set_aspect_version(prepared.result.aspect_version);
            entry.set_state(NodeState::Clean);
            entry.set_dirty_aspects(AspectMask::EMPTY);
            entry.clear_dirty_partition_scopes();
        }
        for dependency in prepared.dependencies.as_slice() {
            self.graph
                .add_dependency(node, dependency.source, dependency.aspect)?;
        }
        self.graph.set_dep_snapshot(node, dep_snapshot)?;
        Ok(())
    }

    fn restore_batch_undo(&mut self) {
        let entry_undo = std::mem::take(&mut self.batch_entry_undo);
        for (node, entry) in entry_undo {
            if let Ok(slot) = self.graph.get_entry_mut(node) {
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
            .ok_or_else(|| SignalError::invalid_input("easy-mode signal has no stored value"))?;
        stored
            .downcast_ref::<T>()
            .cloned()
            .ok_or_else(|| SignalError::invalid_input("easy-mode signal type mismatch"))
    }
}
