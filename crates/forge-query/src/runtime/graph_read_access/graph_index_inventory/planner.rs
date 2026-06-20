use super::{
    forge_query_graph_index_inventory, ForgeQueryGraphIndexInventory,
    ForgeQueryGraphIndexInventoryMatchReport,
};
use crate::runtime::ForgeQueryGraphReadAccessRequirementSet;

pub(crate) fn match_graph_index_inventory_for_requirements(
    requirements: &ForgeQueryGraphReadAccessRequirementSet,
    inventory: &ForgeQueryGraphIndexInventory,
) -> ForgeQueryGraphIndexInventoryMatchReport {
    ForgeQueryGraphIndexInventoryMatchReport::match_requirements(requirements, inventory)
}

pub fn match_current_graph_index_inventory_for_requirements(
    requirements: &ForgeQueryGraphReadAccessRequirementSet,
) -> ForgeQueryGraphIndexInventoryMatchReport {
    let inventory = forge_query_graph_index_inventory();
    ForgeQueryGraphIndexInventoryMatchReport::match_requirements(requirements, &inventory)
}
