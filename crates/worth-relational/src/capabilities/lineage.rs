use crate::identity::data::LineageId;
use crate::lineage::data::LineageNode;
use crate::runtime::RelationalRuntime;

pub(crate) trait LineageNodeSource {
    fn lineage_node(&self, lineage_id: LineageId) -> Option<&LineageNode>;
    fn lineage_nodes_snapshot(&self) -> Vec<LineageNode>;
}

impl LineageNodeSource for RelationalRuntime {
    fn lineage_node(&self, lineage_id: LineageId) -> Option<&LineageNode> {
        self.lineage.nodes.get(&lineage_id)
    }

    fn lineage_nodes_snapshot(&self) -> Vec<LineageNode> {
        self.lineage.nodes.values().cloned().collect()
    }
}
