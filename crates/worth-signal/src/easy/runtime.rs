use std::any::Any;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;

use crate::facade::runtime::mark_dirty_batch;
use crate::facade::{
    AspectMask, AspectVersion, BatchChange, ChangedRegion, NodeId, NodeState, SignalError,
    SignalGraph,
};
use crate::logic::transaction::RuntimeObservationRegistry;

use super::compute::{Computed, ErasedComputed, SignalContext};
use super::signal::{ComputedSignal, InputSignal, Signal, DEFAULT_ASPECT};

mod batching;
mod evaluation;

/// Convenience app wrapper for lightweight typed experiments and examples.
///
/// This surface intentionally trades execution-model explicitness for ergonomics.
/// It is not the canonical kernel-grade runtime interface.
pub struct SignalApp {
    pub(super) graph: SignalGraph,
    pub(super) values: HashMap<NodeId, Box<dyn Any + Send + Sync>>,
    pub(super) computed: HashMap<NodeId, Box<dyn ErasedComputed>>,
    pub(super) observations: RuntimeObservationRegistry<(), (), (), (), ()>,
    batch_depth: usize,
    batched_dirty_nodes: BTreeSet<NodeId>,
    batch_value_undo: HashMap<NodeId, Option<Box<dyn Any + Send + Sync>>>,
    batch_entry_undo: HashMap<NodeId, crate::data::node::NodeEntry>,
    pending_input_versions: HashMap<NodeId, AspectVersion>,
    batch_pending_input_undo: HashMap<NodeId, Option<AspectVersion>>,
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
            observations: RuntimeObservationRegistry::default(),
            batch_depth: 0,
            batched_dirty_nodes: BTreeSet::new(),
            batch_value_undo: HashMap::new(),
            batch_entry_undo: HashMap::new(),
            pending_input_versions: HashMap::new(),
            batch_pending_input_undo: HashMap::new(),
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
        entry.set_runtime_artifact_state(Some(evaluation::easy_seed_runtime_artifact_state(
            version, 0, false,
        )));
        drop(entry);
        self.graph
            .transition_node_clean(node)
            .expect("newly created input node should admit clean authority");

        Signal::new(node)
    }

    pub fn computed<T, F>(&mut self, compute: F) -> ComputedSignal<T>
    where
        T: Clone + PartialEq + Send + Sync + 'static,
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
            self.batch_pending_input_undo
                .entry(signal.node)
                .or_insert_with(|| self.pending_input_versions.get(&signal.node).copied());
        }
        self.values.insert(signal.node, Box::new(value));
        let current = self
            .pending_input_versions
            .get(&signal.node)
            .copied()
            .unwrap_or(self.graph.node_aspect_version(signal.node)?);
        let next = current.with(DEFAULT_ASPECT, current.get(DEFAULT_ASPECT) + 1);
        self.pending_input_versions.insert(signal.node, next);

        if self.batch_depth > 0 {
            self.batched_dirty_nodes.insert(signal.node);
        } else {
            mark_dirty_batch(
                &mut self.graph,
                &BatchChange::singleton(signal.node, DEFAULT_ASPECT, Vec::<ChangedRegion>::new()),
            )?;
            self.ensure_evaluated(signal.node)?;
            let mut changed_nodes = BTreeSet::new();
            changed_nodes.insert(signal.node);
            super::observation::deliver_observation_boundary(self, changed_nodes)?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn graph(&self) -> &SignalGraph {
        &self.graph
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
