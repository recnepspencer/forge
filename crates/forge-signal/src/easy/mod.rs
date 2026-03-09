use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Mutex;

use crate::facade::{
    mark_dirty, Aspect, AspectMask, AspectVersion, ExecutionReadView, NodeEvaluationResult, NodeId,
    NodeState, PreparedEvaluation, SignalError, SignalGraph,
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
        view: &ExecutionReadView<'_>,
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
        view: &ExecutionReadView<'_>,
    ) -> Result<(Box<dyn Any + Send + Sync>, PreparedEvaluation), SignalError> {
        let mut context = ComputeContext {
            values,
            view,
        };
        let value = (self.closure)(&mut context);
        let current = view
            .graph()
            .get_entry(view.evaluating())?
            .get_aspect_version()
            .get(DEFAULT_ASPECT);
        let next_version = AspectVersion::zero().with(DEFAULT_ASPECT, current + 1);
        let prepared = view.finish(NodeEvaluationResult::from_version(next_version));
        Ok((Box::new(value), prepared))
    }
}

pub struct ComputeContext<'a> {
    values: &'a HashMap<NodeId, Box<dyn Any + Send + Sync>>,
    view: &'a ExecutionReadView<'a>,
}

impl<'a> ComputeContext<'a> {
    pub fn get<T: Clone + Send + Sync + 'static>(&mut self, signal: Signal<T>) -> T {
        self.view.capture_dependency(signal.node, DEFAULT_ASPECT);
        self.values[&signal.node]
            .downcast_ref::<T>()
            .expect("easy-mode signal type mismatch")
            .clone()
    }
}

pub struct ReactiveGraph {
    graph: SignalGraph,
    values: HashMap<NodeId, Box<dyn Any + Send + Sync>>,
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
        self.computed.insert(
            node,
            Box::new(Computed {
                closure: compute,
                marker: PhantomData,
            }),
        );
        Signal::new(node)
    }

    pub fn get<T: Clone + Send + Sync + 'static>(&mut self, signal: Signal<T>) -> T {
        self.ensure_evaluated(signal.node)
            .expect("easy-mode evaluation failed");
        self.read_value(signal)
    }

    pub fn set<T: Clone + Send + Sync + 'static>(&mut self, signal: InputSignal<T>, value: T) {
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

        let plan = self
            .graph
            .build_evaluation_plan(&[node], crate::logic::evaluation::EvaluationRequestMode::Default)?;
        let staged_values: Mutex<HashMap<NodeId, Box<dyn Any + Send + Sync>>> =
            Mutex::new(HashMap::new());
        let graph = &mut self.graph;
        let computed = &self.computed;
        let values = &self.values;
        graph.execute_prepared_plan(
            &plan,
            &|current, view| {
                if let Some(computed) = computed.get(&current) {
                    let (value, prepared) = computed.precompute(values, view)?;
                    staged_values
                        .lock()
                        .map_err(|_| SignalError::internal("easy-mode staged value mutex poisoned"))?
                        .insert(current, value);
                    Ok(prepared)
                } else {
                    Ok(PreparedEvaluation::validated_clean())
                }
            },
        )?;

        let staged_values = staged_values
            .into_inner()
            .map_err(|_| SignalError::internal("easy-mode staged value mutex poisoned"))?;
        for (node, value) in staged_values {
            self.values.insert(node, value);
        }
        Ok(())
    }

    fn read_value<T: Clone + Send + Sync + 'static>(&self, signal: Signal<T>) -> T {
        self.values[&signal.node]
            .downcast_ref::<T>()
            .expect("easy-mode signal type mismatch")
            .clone()
    }
}
