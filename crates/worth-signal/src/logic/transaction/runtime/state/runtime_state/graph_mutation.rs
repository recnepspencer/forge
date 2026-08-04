use super::SignalRuntime;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use std::ops::{Deref, DerefMut};
pub struct SignalGraphMut<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime::state::runtime_state) runtime:
        &'a mut SignalRuntime<D, I, E, Ctx, T>,
}

impl<D, I, E, Ctx, T> SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn unregister_node(
        &mut self,
        node: NodeId,
    ) -> Result<crate::data::temporal::TemporalWakeRetirementBatch, crate::data::error::SignalError>
    {
        self.runtime.unregister_node(node)
    }

    pub fn replace_node_from_checkpoint_image(
        &mut self,
        node: NodeId,
        image: crate::data::node::CheckpointNodeImage,
    ) -> Result<crate::data::temporal::TemporalWakeRetirementBatch, crate::data::error::SignalError>
    {
        self.runtime.replace_node_from_checkpoint_image(node, image)
    }

    pub fn replace_node_evaluation_config(
        &mut self,
        node: NodeId,
        eval_config: crate::data::node::NodeEvaluationConfig,
    ) -> Result<crate::data::temporal::TemporalWakeRetirementBatch, crate::data::error::SignalError>
    {
        self.runtime
            .replace_node_evaluation_config(node, eval_config)
    }
}

impl<D, I, E, Ctx, T> Deref for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    type Target = SignalGraph;

    fn deref(&self) -> &Self::Target {
        &self.runtime.graph
    }
}

impl<D, I, E, Ctx, T> DerefMut for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.runtime.graph
    }
}

impl<D, I, E, Ctx, T> Drop for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn drop(&mut self) {
        self.runtime
            .config
            .prune_stale_node_meta(&self.runtime.graph);
    }
}
