use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{CheckpointNodeImage, NodeEntry, NodeEvaluationConfig};

use crate::data::graph::node_builder::NodeBuilder;

impl SignalGraph {
    #[doc(hidden)]
    pub fn create_node(&mut self) -> NodeId {
        let entry = NodeEntry::new();
        self.allocate_node(entry)
    }

    pub(crate) fn create_node_from_checkpoint_image(
        &mut self,
        image: CheckpointNodeImage,
    ) -> NodeId {
        self.allocate_node(NodeEntry::from_checkpoint_image(image))
    }

    pub fn node(&mut self) -> NodeBuilder<'_> {
        NodeBuilder::new(self)
    }

    #[doc(hidden)]
    pub fn create_node_with_config(&mut self, config: NodeEvaluationConfig) -> NodeId {
        let mut entry = NodeEntry::new();
        entry.set_eval_config(config);
        self.allocate_node(entry)
    }
}
