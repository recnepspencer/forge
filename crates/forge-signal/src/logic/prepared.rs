use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{NodeEntry, NodeState};
use crate::data::output::{
    ComputationFamily, ComputationKey, IntoNodeEvaluationResult, MemoizedResultOrigin,
    NodeEvaluationResult, PartitionSubscription, StructuralMemoKey,
};
use crate::data::trace::{CausalityMetadata, TraceSummary};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PreparedDependencyCapture {
    edges: Vec<PreparedDependencyEdge>,
}

impl PreparedDependencyCapture {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, source: NodeId, aspect: Aspect, scope: Option<PartitionSubscription>) {
        let edge = PreparedDependencyEdge {
            source,
            aspect,
            scope,
        };
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }

    pub fn as_slice(&self) -> &[PreparedDependencyEdge] {
        &self.edges
    }

    pub fn into_sorted_unique(mut self) -> Self {
        self.edges.sort_by_key(|edge| {
            (
                edge.source.index(),
                edge.source.generation(),
                edge.aspect.index(),
                edge.scope
                    .as_ref()
                    .map(|scope| {
                        (
                            scope.partition.0.clone(),
                            scope.detail.clone().unwrap_or_default(),
                            scope.match_mode as u8,
                        )
                    })
                    .unwrap_or_default(),
            )
        });
        self.edges.dedup();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedDependencyEdge {
    pub source: NodeId,
    pub aspect: Aspect,
    #[serde(default)]
    pub scope: Option<PartitionSubscription>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PreparedTraceData {
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub causality: Option<CausalityMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PreparedEvaluationOutcome {
    #[default]
    Evaluate,
    ValidatedClean,
    DeferredByCondition,
    RevertedCleanByCondition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PreparedEvaluationOrigin {
    #[default]
    DirectPrecompute,
    MemoizedReuse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PreparedMemoDecision {
    #[default]
    None,
    Hit,
    Miss,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PreparedKeyedContext {
    #[serde(default)]
    pub family: Option<ComputationFamily>,
    #[serde(default)]
    pub key: Option<ComputationKey>,
    #[serde(default)]
    pub memo_key: Option<StructuralMemoKey>,
    pub memoized_origin: MemoizedResultOrigin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedEvaluation {
    pub result: NodeEvaluationResult,
    pub dependencies: PreparedDependencyCapture,
    pub trace_data: PreparedTraceData,
    pub outcome: PreparedEvaluationOutcome,
    pub origin: PreparedEvaluationOrigin,
    pub memo_decision: PreparedMemoDecision,
    #[serde(default)]
    pub keyed: Option<PreparedKeyedContext>,
}

impl PreparedEvaluation {
    pub fn from_result(result: impl IntoNodeEvaluationResult) -> Self {
        Self {
            result: result.into_evaluation_result(),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::Evaluate,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub fn validated_clean() -> Self {
        Self {
            result: NodeEvaluationResult::from_version(AspectVersion::zero()),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::ValidatedClean,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub fn deferred_by_condition() -> Self {
        Self {
            result: NodeEvaluationResult::from_version(AspectVersion::zero()),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::DeferredByCondition,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub fn reverted_clean_by_condition() -> Self {
        Self {
            result: NodeEvaluationResult::from_version(AspectVersion::zero()),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::RevertedCleanByCondition,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub fn with_dependencies(mut self, dependencies: PreparedDependencyCapture) -> Self {
        self.dependencies = dependencies.into_sorted_unique();
        self
    }

    pub fn with_trace_data(mut self, trace_data: PreparedTraceData) -> Self {
        self.trace_data = trace_data;
        self
    }

    pub fn with_origin(mut self, origin: PreparedEvaluationOrigin) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_memo_decision(mut self, memo_decision: PreparedMemoDecision) -> Self {
        self.memo_decision = memo_decision;
        self
    }

    pub fn with_keyed(mut self, keyed: PreparedKeyedContext) -> Self {
        self.keyed = Some(keyed);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PreparedStage {
    pub tasks: Vec<PreparedTaskRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedTaskRecord {
    pub node: NodeId,
    pub prepared: PreparedEvaluation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StageApplyResult {
    pub applied_tasks: u32,
    pub dependency_edge_updates: u32,
}

pub struct ExecutionSnapshot<'a> {
    graph: &'a SignalGraph,
}

impl<'a> ExecutionSnapshot<'a> {
    pub fn new(graph: &'a SignalGraph) -> Self {
        Self { graph }
    }

    pub fn graph(&self) -> &'a SignalGraph {
        self.graph
    }

    pub fn read_view(&'a self, evaluating: NodeId) -> ExecutionReadView<'a> {
        ExecutionReadView {
            snapshot: self,
            evaluating,
            capture: RefCell::new(PreparedDependencyCapture::default()),
        }
    }
}

pub type StageSnapshot<'a> = ExecutionSnapshot<'a>;

pub struct ExecutionReadView<'a> {
    snapshot: &'a ExecutionSnapshot<'a>,
    evaluating: NodeId,
    capture: RefCell<PreparedDependencyCapture>,
}

impl<'a> ExecutionReadView<'a> {
    pub fn graph(&self) -> &'a SignalGraph {
        self.snapshot.graph()
    }

    pub fn evaluating(&self) -> NodeId {
        self.evaluating
    }

    pub fn node(&self, node: NodeId) -> Result<SnapshotNodeView<'a>, SignalError> {
        let entry = self.graph().get_entry(node)?;
        Ok(SnapshotNodeView { node, entry })
    }

    pub fn capture_dependency(&self, source: NodeId, aspect: Aspect) {
        self.capture.borrow_mut().record(source, aspect, None);
    }

    pub fn capture_partition_dependency(
        &self,
        source: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
    ) {
        self.capture
            .borrow_mut()
            .record(source, aspect, Some(scope));
    }

    pub fn read_aspect_version(
        &self,
        source: NodeId,
        aspect: Aspect,
    ) -> Result<AspectVersion, SignalError> {
        self.capture_dependency(source, aspect);
        Ok(self.graph().get_entry(source)?.get_aspect_version())
    }

    pub fn read_partitioned_aspect_version(
        &self,
        source: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
    ) -> Result<AspectVersion, SignalError> {
        self.capture_partition_dependency(source, aspect, scope);
        Ok(self.graph().get_entry(source)?.get_aspect_version())
    }

    pub fn finish(&self, result: impl IntoNodeEvaluationResult) -> PreparedEvaluation {
        PreparedEvaluation::from_result(result)
            .with_dependencies(std::mem::take(&mut *self.capture.borrow_mut()))
    }

    pub fn finish_with(
        &self,
        result: impl IntoNodeEvaluationResult,
        trace_data: PreparedTraceData,
    ) -> PreparedEvaluation {
        self.finish(result).with_trace_data(trace_data)
    }
}

pub struct SnapshotNodeView<'a> {
    node: NodeId,
    entry: &'a NodeEntry,
}

impl<'a> SnapshotNodeView<'a> {
    pub fn id(&self) -> NodeId {
        self.node
    }

    pub fn state(&self) -> NodeState {
        *self.entry.get_state()
    }

    pub fn aspect_version(&self) -> AspectVersion {
        self.entry.get_aspect_version()
    }

    pub fn trace_summary(&self) -> Option<&'a TraceSummary> {
        self.entry.get_trace_summary()
    }

    pub fn causality(&self) -> Option<&'a CausalityMetadata> {
        self.entry.get_causality()
    }

    pub fn dependencies(&self) -> impl Iterator<Item = SnapshotDependencyView<'a>> + 'a {
        self.entry
            .get_dependencies()
            .iter()
            .map(|dependency| SnapshotDependencyView { edge: dependency })
    }
}

pub struct SnapshotDependencyView<'a> {
    edge: &'a DependencyEdge,
}

impl<'a> SnapshotDependencyView<'a> {
    pub fn source(&self) -> NodeId {
        self.edge.source()
    }

    pub fn aspect(&self) -> Aspect {
        self.edge.aspect()
    }

    pub fn scope(&self) -> Option<&'a PartitionSubscription> {
        self.edge.scope_ref()
    }
}
