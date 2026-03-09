use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::facade::{
    mark_dirty, Aspect, AspectMask, DependencyEdge, NodeId, NodeState, SignalError,
    SignalGraph,
};
use crate::data::dependency::DependencySnapshot;

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

trait ErasedComputed {
    fn evaluate(&self, graph: &mut ReactiveGraph, node: NodeId) -> Result<(), SignalError>;
}

struct Computed<T, F>
where
    T: Clone + 'static,
    F: Fn(&mut ComputeContext<'_>) -> T + 'static,
{
    closure: F,
    marker: PhantomData<fn() -> T>,
}

impl<T, F> ErasedComputed for Computed<T, F>
where
    T: Clone + 'static,
    F: Fn(&mut ComputeContext<'_>) -> T + 'static,
{
    fn evaluate(&self, graph: &mut ReactiveGraph, node: NodeId) -> Result<(), SignalError> {
        let old_dependencies = graph.graph.get_entry(node)?.get_dependencies().to_vec();
        let current_version = graph
            .graph
            .get_entry(node)?
            .get_aspect_version()
            .get(DEFAULT_ASPECT);

        let mut context = ComputeContext {
            graph,
            dependencies: Vec::new(),
        };
        let value = (self.closure)(&mut context);

        for dependency in old_dependencies {
            context
                .graph
                .graph
                .remove_dependency(node, dependency.source(), dependency.aspect())?;
        }
        for dependency in &context.dependencies {
            context
                .graph
                .graph
                .add_dependency(node, dependency.source(), dependency.aspect())?;
        }

        let mut snapshot = DependencySnapshot::empty();
        for dependency in &context.dependencies {
            let version = context
                .graph
                .graph
                .get_entry(dependency.source())?
                .get_aspect_version()
                .get(dependency.aspect());
            snapshot.record(
                dependency.source(),
                dependency.aspect(),
                version,
                dependency.scope_ref().cloned(),
            );
        }

        context.graph.values.insert(node, Box::new(value));
        let entry = context.graph.graph.get_entry_mut(node)?;
        let next_version = entry
            .get_aspect_version()
            .with(DEFAULT_ASPECT, current_version + 1);
        entry.set_aspect_version(next_version);
        entry.set_dep_snapshot(snapshot);
        entry.set_state(NodeState::Clean);
        entry.set_dirty_aspects(AspectMask::EMPTY);
        Ok(())
    }
}

pub struct ComputeContext<'a> {
    graph: &'a mut ReactiveGraph,
    dependencies: Vec<DependencyEdge>,
}

impl<'a> ComputeContext<'a> {
    pub fn get<T: Clone + 'static>(&mut self, signal: Signal<T>) -> T {
        self.dependencies
            .push(DependencyEdge::new(signal.node, DEFAULT_ASPECT));
        self.graph
            .ensure_evaluated(signal.node)
            .expect("easy-mode dependency evaluation failed");
        self.graph.read_value(signal)
    }
}

pub struct ReactiveGraph {
    graph: SignalGraph,
    values: HashMap<NodeId, Box<dyn Any>>,
    computed: HashMap<NodeId, Box<dyn ErasedComputed>>,
    batch_depth: usize,
    batched_dirty_nodes: Vec<NodeId>,
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
        }
    }

    pub fn input<T: Clone + 'static>(&mut self, value: T) -> InputSignal<T> {
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
        T: Clone + 'static,
        F: Fn(&mut ComputeContext<'_>) -> T + 'static,
    {
        let node = self
            .graph
            .node()
            .on_demand()
            .build();
        self.computed.insert(
            node,
            Box::new(Computed {
                closure: compute,
                marker: PhantomData,
            }),
        );
        Signal::new(node)
    }

    pub fn get<T: Clone + 'static>(&mut self, signal: Signal<T>) -> T {
        self.ensure_evaluated(signal.node)
            .expect("easy-mode evaluation failed");
        self.read_value(signal)
    }

    pub fn set<T: Clone + 'static>(&mut self, signal: InputSignal<T>, value: T) {
        self.values.insert(signal.node, Box::new(value));
        {
            let entry = self
                .graph
                .get_entry_mut(signal.node)
                .expect("input node should exist");
            let current = entry.get_aspect_version().get(DEFAULT_ASPECT);
            let next = entry.get_aspect_version().with(DEFAULT_ASPECT, current + 1);
            entry.set_aspect_version(next);
            entry.set_state(NodeState::Clean);
            entry.set_dirty_aspects(AspectMask::EMPTY);
        }

        if self.batch_depth > 0 {
            self.batched_dirty_nodes.push(signal.node);
        } else {
            mark_dirty(&mut self.graph, signal.node, DEFAULT_ASPECT)
                .expect("easy-mode invalidation failed");
        }
    }

    pub fn batch<F>(&mut self, apply: F)
    where
        F: FnOnce(&mut Self),
    {
        self.batch_depth += 1;
        apply(self);
        self.batch_depth -= 1;

        if self.batch_depth == 0 {
            let mut dirty_nodes = std::mem::take(&mut self.batched_dirty_nodes);
            dirty_nodes.sort();
            dirty_nodes.dedup();
            for node in dirty_nodes {
                mark_dirty(&mut self.graph, node, DEFAULT_ASPECT)
                    .expect("easy-mode batched invalidation failed");
            }
        }
    }

    fn ensure_evaluated(&mut self, node: NodeId) -> Result<(), SignalError> {
        if !self.computed.contains_key(&node) {
            return Ok(());
        }

        let dependencies = self.graph.get_entry(node)?.get_dependencies().to_vec();
        for dependency in dependencies {
            self.ensure_evaluated(dependency.source())?;
        }

        match *self.graph.get_entry(node)?.get_state() {
            NodeState::Clean => Ok(()),
            NodeState::MaybeStale => {
                if self.upstream_matches_snapshot(node)? {
                    let entry = self.graph.get_entry_mut(node)?;
                    entry.set_state(NodeState::Clean);
                    entry.set_dirty_aspects(AspectMask::EMPTY);
                    Ok(())
                } else {
                    self.recompute(node)
                }
            }
            NodeState::Dirty => self.recompute(node),
        }
    }

    fn recompute(&mut self, node: NodeId) -> Result<(), SignalError> {
        let compute = self
            .computed
            .remove(&node)
            .expect("computed node should exist while recomputing");
        let result = compute.evaluate(self, node);
        self.computed.insert(node, compute);
        result
    }

    fn upstream_matches_snapshot(&self, node: NodeId) -> Result<bool, SignalError> {
        let snapshot = self.graph.get_entry(node)?.get_dep_snapshot();
        for &(source, aspect, expected_version, _) in snapshot.entries() {
            let current_version = self.graph.get_entry(source)?.get_aspect_version().get(aspect);
            if current_version != expected_version {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn read_value<T: Clone + 'static>(&self, signal: Signal<T>) -> T {
        self.values[&signal.node]
            .downcast_ref::<T>()
            .expect("easy-mode signal type mismatch")
            .clone()
    }
}
