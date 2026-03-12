use std::marker::PhantomData;
use std::rc::Rc;

use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
#[cfg(test)]
use std::cell::RefCell;
#[cfg(test)]
use crate::data::aspect::{Aspect, AspectVersion};
#[cfg(test)]
use crate::data::error::SignalError;
#[cfg(test)]
use crate::data::output::{IntoNodeEvaluationResult, PartitionSubscription};

#[cfg(test)]
use super::capture::PreparedDependencyCapture;
#[cfg(test)]
use super::evaluation::PreparedEvaluation;

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

    pub fn read_view(&'a self, _evaluating: NodeId) -> ExecutionReadView<'a> {
        ExecutionReadView {
            snapshot: self,
            #[cfg(test)]
            capture: RefCell::new(PreparedDependencyCapture::default()),
            not_send_or_sync: PhantomData,
        }
    }
}

pub struct ExecutionReadView<'a> {
    snapshot: &'a ExecutionSnapshot<'a>,
    #[cfg(test)]
    capture: RefCell<PreparedDependencyCapture>,
    not_send_or_sync: PhantomData<Rc<()>>,
}

impl<'a> ExecutionReadView<'a> {
    pub fn graph(&self) -> &'a SignalGraph {
        self.snapshot.graph()
    }

    #[cfg(test)]
    pub fn capture_dependency(&self, source: NodeId, aspect: Aspect) {
        self.capture.borrow_mut().record(source, aspect, None);
    }

    #[cfg(test)]
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

    #[cfg(test)]
    pub fn read_aspect_version(
        &self,
        source: NodeId,
        aspect: Aspect,
    ) -> Result<AspectVersion, SignalError> {
        self.capture_dependency(source, aspect);
        Ok(self.graph().get_entry(source)?.get_aspect_version())
    }

    #[cfg(test)]
    pub fn read_partitioned_aspect_version(
        &self,
        source: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
    ) -> Result<AspectVersion, SignalError> {
        self.capture_partition_dependency(source, aspect, scope.clone());
        Ok(self
            .graph()
            .get_entry(source)?
            .get_partitioned_aspect_version(&scope))
    }

    #[cfg(test)]
    pub fn finish(&self, result: impl IntoNodeEvaluationResult) -> PreparedEvaluation {
        PreparedEvaluation::from_result(result)
            .with_dependencies(std::mem::take(&mut *self.capture.borrow_mut()))
    }
}
