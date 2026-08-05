//! Context-aware evaluation surface for dependency tracking.
//!
//! INVARIANTS:
//! - Context is passed explicitly, never through thread-locals.
//! - Domain context is framework-owned and ambient for the lifetime of evaluation.
//! - All upstream reads are recorded for graph wiring.

use std::collections::HashSet;

use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{IntoNodeEvaluationResult, PartitionSubscription};
use crate::data::trace::CausalityMetadata;
use crate::logic::evaluation::{EvaluationOutput, IntoEvaluationOutput};
use crate::logic::prepared::{PreparedDependencyCapture, PreparedTraceData};

pub struct EvaluationContext<'graph, Ctx> {
    graph: &'graph SignalGraph,
    node: NodeId,
    domain_context: &'graph Ctx,
    discovered_deps: PreparedDependencyCapture,
    discovered_dep_keys: HashSet<(NodeId, Aspect, Option<PartitionSubscription>)>,
}

impl<'graph, Ctx> EvaluationContext<'graph, Ctx> {
    pub fn new(graph: &'graph SignalGraph, node: NodeId, domain_context: &'graph Ctx) -> Self {
        Self {
            graph,
            node,
            domain_context,
            discovered_deps: PreparedDependencyCapture::default(),
            discovered_dep_keys: HashSet::new(),
        }
    }

    pub fn graph(&self) -> &'graph SignalGraph {
        self.graph
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn evaluating(&self) -> NodeId {
        self.node()
    }

    pub fn domain(&self) -> &'graph Ctx {
        self.domain_context
    }

    pub fn read(&mut self, signal: NodeId, aspect: Aspect) -> Result<u64, SignalError> {
        self.capture_dependency(signal, aspect);
        Ok(self.graph.node_aspect_version(signal)?.get(aspect))
    }

    pub fn read_aspect_version(
        &mut self,
        source: NodeId,
        aspect: Aspect,
    ) -> Result<AspectVersion, SignalError> {
        self.capture_dependency(source, aspect);
        self.graph.node_aspect_version(source)
    }

    pub fn read_partitioned_aspect_version(
        &mut self,
        source: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
    ) -> Result<AspectVersion, SignalError> {
        self.capture_partition_dependency(source, aspect, scope.clone());
        self.graph.node_partitioned_aspect_version(source, &scope)
    }

    pub fn capture_dependency(&mut self, source: NodeId, aspect: Aspect) {
        if self.discovered_dep_keys.insert((source, aspect, None)) {
            self.discovered_deps.record(source, aspect, None);
        }
    }

    pub fn capture_partition_dependency(
        &mut self,
        source: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
    ) {
        if self
            .discovered_dep_keys
            .insert((source, aspect, Some(scope.clone())))
        {
            self.discovered_deps.record(source, aspect, Some(scope));
        }
    }

    pub fn finish(&self, result: impl IntoEvaluationOutput) -> EvaluationOutput {
        result.into_evaluation_output()
    }

    pub fn finish_with(
        &self,
        result: impl IntoNodeEvaluationResult,
        trace_data: PreparedTraceData,
    ) -> EvaluationOutput {
        let mut output = EvaluationOutput::from_result(result);
        output.set_trace_data(trace_data);
        output
    }

    pub fn discovered_count(&self) -> usize {
        self.discovered_deps.len()
    }

    pub(crate) fn into_prepared(
        self,
        output: impl IntoEvaluationOutput,
    ) -> crate::logic::prepared::PreparedEvaluation {
        output
            .into_evaluation_output()
            .into_prepared(self.discovered_deps)
    }
}

impl EvaluationOutput {
    pub fn with_trace_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.trace_data
            .labels
            .extend(labels.into_iter().map(Into::into));
        self
    }

    pub fn with_causality_opt(mut self, causality: Option<CausalityMetadata>) -> Self {
        self.trace_data.causality = causality;
        self
    }

    pub(crate) fn set_trace_data(&mut self, trace_data: PreparedTraceData) {
        self.trace_data = trace_data;
    }
}
