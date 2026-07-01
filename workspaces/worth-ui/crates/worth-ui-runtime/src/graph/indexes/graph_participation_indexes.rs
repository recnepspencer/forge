use crate::graph::{UiGraphNode, UiGraphPageParticipationIndex, UiGraphTopology};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphParticipationIndexes {
    page_participation_index: UiGraphPageParticipationIndex,
}

impl UiGraphParticipationIndexes {
    pub(crate) fn build(nodes: &[UiGraphNode], topology: &UiGraphTopology) -> Self {
        Self {
            page_participation_index: UiGraphPageParticipationIndex::build(nodes, topology),
        }
    }

    pub fn page_participation(&self) -> &UiGraphPageParticipationIndex {
        &self.page_participation_index
    }
}
