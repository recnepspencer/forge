use crate::evidence::{UiAllocationNeighborhood, UiMeasurementBasis};
use crate::graph::allocation_neighborhood::handoff::admit_neighborhood_for_touch;
#[cfg(test)]
use crate::graph::allocation_neighborhood::handoff::admit_neighborhood_from_graph;
use crate::graph::{UiAllocationNeighborhoodDenial, UiGraphSnapshot};
use crate::obligations::selection::UiSelectedObligationSet;

impl UiMeasurementBasis {
    pub(crate) fn admit_allocation_neighborhood(
        &self,
        snapshot: &UiGraphSnapshot,
        selected: &UiSelectedObligationSet,
    ) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
        admit_neighborhood_for_touch(snapshot, selected, self)
    }

    #[cfg(test)]
    pub(crate) fn admit_allocation_neighborhood_from_graph(
        &self,
        snapshot: &UiGraphSnapshot,
    ) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
        admit_neighborhood_from_graph(snapshot, self)
    }
}

impl UiGraphSnapshot {
    pub(crate) fn allocation_planning_node_identities(
        &self,
    ) -> impl Iterator<Item = crate::graph::UiGraphNodeIdentity> + '_ {
        self.nodes().iter().filter_map(|node| {
            let identity = node.graph_node_identity();
            let layout = node
                .participation_posture()
                .axis(crate::graph::UiGraphParticipationAxis::Layout);
            super::membership::layout_participates_in_planning(self, identity, layout)
                .then_some(identity)
        })
    }
}
