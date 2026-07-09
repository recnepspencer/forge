use crate::evidence::{UiAllocationNeighborhood, UiMeasurementBasis};
use crate::graph::allocation_neighborhood::handoff::admit_neighborhood_for_touch;
#[cfg(any(test, feature = "certification-support"))]
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

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn admit_allocation_neighborhood_from_graph(
        &self,
        snapshot: &UiGraphSnapshot,
    ) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
        admit_neighborhood_from_graph(snapshot, self)
    }
}
