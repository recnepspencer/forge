use super::identity::adjacency_ordering_basis_identity;
use super::row::PlanarBooleanOverlapAdjacencyRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapAdjacencyOrderingBasis {
    basis_identity: String,
    request_identity: String,
    adjacency_index_identity: String,
    ordered_neighborhood_identities: Vec<String>,
}

impl PlanarBooleanOverlapAdjacencyOrderingBasis {
    pub(crate) fn new(
        request_identity: &str,
        adjacency_index_identity: &str,
        ordered_neighborhood_identities: Vec<String>,
    ) -> Self {
        let basis_identity = adjacency_ordering_basis_identity(
            request_identity,
            adjacency_index_identity,
            &ordered_neighborhood_identities,
        );
        Self {
            basis_identity,
            request_identity: request_identity.to_string(),
            adjacency_index_identity: adjacency_index_identity.to_string(),
            ordered_neighborhood_identities,
        }
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn adjacency_index_identity(&self) -> &str {
        &self.adjacency_index_identity
    }

    pub fn ordered_neighborhood_identities(&self) -> &[String] {
        &self.ordered_neighborhood_identities
    }
}

pub(crate) fn canonicalize_adjacency_rows(
    rows: &mut [PlanarBooleanOverlapAdjacencyRow],
) -> Vec<String> {
    rows.sort_by(|left, right| adjacency_order_key(left).cmp(&adjacency_order_key(right)));
    rows.iter()
        .map(|row| row.neighborhood_identity().to_string())
        .collect()
}

pub(crate) fn adjacency_order_key(row: &PlanarBooleanOverlapAdjacencyRow) -> String {
    format!(
        "{}|{}|{}|{}",
        row.chain_identities().join("|"),
        row.participating_loop_identities().join("|"),
        row.participating_island_identities().join("|"),
        row.lineage_identities().join("|"),
    )
}

pub(crate) fn loop_order_key(
    row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanLoopOverlapParticipationRow,
    connectivity_identity: &str,
) -> String {
    format!(
        "{}|{:?}|{}|{}|{:?}|{}|{}|{}",
        connectivity_identity,
        row.loop_role(),
        row.role_outcome_identity(),
        row.island_identity(),
        row.island_kind(),
        row.island_origin_loop_identity(),
        row.source_loop_identities().join("|"),
        row.propagated_persistent_name_identities().join("|"),
    )
}

pub(crate) fn island_order_key(
    row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanLoopIslandOverlapParticipationRow,
    connectivity_identity: &str,
) -> String {
    format!(
        "{}|{}|{:?}|{}|{}",
        connectivity_identity,
        row.island_origin_loop_identity(),
        row.island_kind(),
        row.member_loop_identities().join("|"),
        row.propagated_persistent_name_identities().join("|"),
    )
}
