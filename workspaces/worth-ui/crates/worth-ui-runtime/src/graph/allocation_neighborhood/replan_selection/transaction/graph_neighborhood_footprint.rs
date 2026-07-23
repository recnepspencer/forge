use crate::evidence::UiAllocationNeighborhood;
use crate::graph::UiGraphNodeIdentity;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct UiGraphNeighborhoodFootprint {
    members: Box<[UiGraphNodeIdentity]>,
}

impl UiGraphNeighborhoodFootprint {
    pub(super) fn seal(neighborhood: &UiAllocationNeighborhood) -> Self {
        let mut members = neighborhood
            .members()
            .iter()
            .map(crate::evidence::UiAllocationNeighborhoodMember::graph_node_identity)
            .collect::<Vec<_>>();
        members.sort_unstable();
        members.dedup();
        Self {
            members: members.into_boxed_slice(),
        }
    }

    pub(crate) fn members(&self) -> &[UiGraphNodeIdentity] {
        &self.members
    }
}
