use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{IntoNodeEvaluationResult, PartitionSubscription};

use super::capture::PreparedDependencyCapture;
use super::evaluation::{PreparedEvaluation, PreparedTraceData};

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
            not_send_or_sync: PhantomData,
        }
    }
}

pub type StageSnapshot<'a> = ExecutionSnapshot<'a>;

pub struct ExecutionReadView<'a> {
    snapshot: &'a ExecutionSnapshot<'a>,
    evaluating: NodeId,
    capture: RefCell<PreparedDependencyCapture>,
    not_send_or_sync: PhantomData<Rc<()>>,
}

impl<'a> ExecutionReadView<'a> {
    pub fn graph(&self) -> &'a SignalGraph {
        self.snapshot.graph()
    }

    pub fn evaluating(&self) -> NodeId {
        self.evaluating
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
