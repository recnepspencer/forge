use super::row::PlanarBooleanOverlapAdjacencyRow;

pub(crate) fn adjacency_neighborhood_identity(
    request_identity: &str,
    chain_identities: &[String],
    lineage_identities: &[String],
    participating_loop_identities: &[String],
    participating_island_identities: &[String],
) -> String {
    format!(
        "overlap-adjacency:neighborhood:{request_identity}:{}:{}:{}:{}",
        chain_identities.join("|"),
        lineage_identities.join("|"),
        participating_loop_identities.join("|"),
        participating_island_identities.join("|")
    )
}

pub(crate) fn adjacency_row_identity(neighborhood_identity: &str) -> String {
    format!("overlap-adjacency:row:{neighborhood_identity}")
}

pub(crate) fn adjacency_index_identity(
    request_identity: &str,
    row_identities: &[String],
) -> String {
    format!(
        "overlap-adjacency:index:{request_identity}:{}",
        row_identities.join("|")
    )
}

pub(crate) fn adjacency_ordering_basis_identity(
    request_identity: &str,
    adjacency_index_identity: &str,
    ordered_neighborhood_identities: &[String],
) -> String {
    format!(
        "overlap-adjacency:ordering:{request_identity}:{adjacency_index_identity}:{}",
        ordered_neighborhood_identities.join("|")
    )
}

pub(crate) fn neighborhood_group_identity(row: &PlanarBooleanOverlapAdjacencyRow) -> String {
    row.neighborhood_identity().to_string()
}
