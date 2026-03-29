use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::facade::{AspectVersion, NodeEvaluationResult, NodeId, SignalError};
use crate::logic::prepared::{PreparedDependencyCapture, PreparedEvaluation};

use super::signal::{Signal, DEFAULT_ASPECT};

pub(crate) trait ErasedComputed: Send + Sync {
    fn precompute(
        &self,
        values: &HashMap<NodeId, Box<dyn Any + Send + Sync>>,
        staged_values: &HashMap<NodeId, Box<dyn Any + Send + Sync>>,
        current_version: u64,
    ) -> Result<(Box<dyn Any + Send + Sync>, PreparedEvaluation), SignalError>;
}

pub(crate) struct Computed<T, F>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&mut SignalContext<'_>) -> T + Send + Sync + 'static,
{
    pub(crate) closure: F,
    pub(crate) marker: PhantomData<fn() -> T>,
}

impl<T, F> ErasedComputed for Computed<T, F>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&mut SignalContext<'_>) -> T + Send + Sync + 'static,
{
    fn precompute(
        &self,
        values: &HashMap<NodeId, Box<dyn Any + Send + Sync>>,
        staged_values: &HashMap<NodeId, Box<dyn Any + Send + Sync>>,
        current_version: u64,
    ) -> Result<(Box<dyn Any + Send + Sync>, PreparedEvaluation), SignalError> {
        let mut capture = PreparedDependencyCapture::default();
        let mut context = SignalContext {
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

pub struct SignalContext<'a> {
    pub(crate) values: &'a HashMap<NodeId, Box<dyn Any + Send + Sync>>,
    pub(crate) staged_values: &'a HashMap<NodeId, Box<dyn Any + Send + Sync>>,
    pub(crate) capture: &'a mut PreparedDependencyCapture,
}

impl<'a> SignalContext<'a> {
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
