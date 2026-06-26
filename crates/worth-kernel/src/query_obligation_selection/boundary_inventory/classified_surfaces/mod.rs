mod forge_query_authority;
mod primitive_construction;
mod spatial_query_adoption;
mod topology_operator;
mod topology_primitive_construction;

use super::inventory_record::{
    QuerySelectionBoundaryInventory, QuerySelectionBoundaryInventoryRow,
};

pub fn query_selection_boundary_inventory() -> QuerySelectionBoundaryInventory {
    QuerySelectionBoundaryInventory::new(classified_query_selection_surfaces())
}

fn classified_query_selection_surfaces() -> Vec<QuerySelectionBoundaryInventoryRow> {
    let mut rows = Vec::new();
    rows.extend(forge_query_authority::rows());
    rows.extend(primitive_construction::rows());
    rows.extend(topology_operator::rows());
    rows.extend(topology_primitive_construction::rows());
    rows.extend(spatial_query_adoption::rows());
    rows
}
