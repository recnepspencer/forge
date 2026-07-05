pub(crate) fn arrangement_neighborhood_identity(
    request_identity: &str,
    neighborhood_identity: &str,
    boundary_component_identities: &[String],
) -> String {
    format!(
        "overlap-arrangement:neighborhood:{request_identity}:{neighborhood_identity}:{}",
        boundary_component_identities.join("|")
    )
}

pub(crate) fn arrangement_boundary_segment_identity(
    arrangement_neighborhood_identity: &str,
    ordinal: usize,
    source_loop_identity: &str,
    source_edge_identity: &str,
) -> String {
    format!(
        "overlap-arrangement:boundary-segment:{arrangement_neighborhood_identity}:{ordinal}:{source_loop_identity}:{source_edge_identity}"
    )
}

pub(crate) fn arrangement_boundary_component_identity(
    arrangement_neighborhood_identity: &str,
    source_loop_identities: &[String],
    ordinal: usize,
    boundary_cycle_identities: &[String],
) -> String {
    format!(
        "overlap-arrangement:boundary-component:{arrangement_neighborhood_identity}:{}:{ordinal}:{}",
        source_loop_identities.join("|"),
        boundary_cycle_identities.join("|")
    )
}

pub(crate) fn arrangement_cell_identity(
    arrangement_neighborhood_identity: &str,
    source_loop_identities: &[String],
    boundary_component_identities: &[String],
) -> String {
    format!(
        "overlap-arrangement:cell:{arrangement_neighborhood_identity}:{}:{}",
        source_loop_identities.join("|"),
        boundary_component_identities.join("|")
    )
}

pub(crate) fn arrangement_graph_identity(
    request_identity: &str,
    arrangement_neighborhood_identities: &[String],
) -> String {
    format!(
        "overlap-arrangement:graph:{request_identity}:{}",
        arrangement_neighborhood_identities.join("|")
    )
}

pub(crate) fn arrangement_cell_set_identity(
    request_identity: &str,
    arrangement_graph_identity: &str,
    cell_identities: &[String],
) -> String {
    format!(
        "overlap-arrangement:cell-set:{request_identity}:{arrangement_graph_identity}:{}",
        cell_identities.join("|")
    )
}
