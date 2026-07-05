use std::collections::BTreeMap;

use super::construction::build_arrangement_graph;
use super::counters::PlanarBooleanOverlapArrangementGraphCounters;
use super::graph::{
    PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow,
    PlanarBooleanOverlapArrangementBoundaryComponentRow,
    PlanarBooleanOverlapArrangementBoundarySegmentRow, PlanarBooleanOverlapArrangementCellSet,
};
use super::input::PlanarBooleanOverlapArrangementGraphInput;
use super::PlanarBooleanOverlapArrangementGraphDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCoplanarOverlapArrangementGraph {
    arrangement_graph_identity: String,
    request_identity: String,
    adjacency_index_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow>,
    boundary_components: Vec<PlanarBooleanOverlapArrangementBoundaryComponentRow>,
    boundary_segments: Vec<PlanarBooleanOverlapArrangementBoundarySegmentRow>,
    cell_set: PlanarBooleanOverlapArrangementCellSet,
    neighborhoods_by_chain: BTreeMap<String, Vec<usize>>,
    counters: PlanarBooleanOverlapArrangementGraphCounters,
}

impl PlanarBooleanCoplanarOverlapArrangementGraph {
    pub fn admit(
        input: PlanarBooleanOverlapArrangementGraphInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapArrangementGraphDenial> {
        build_arrangement_graph(input)
    }

    pub(crate) fn new(
        arrangement_graph_identity: String,
        request_identity: String,
        adjacency_index_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow>,
        boundary_components: Vec<PlanarBooleanOverlapArrangementBoundaryComponentRow>,
        boundary_segments: Vec<PlanarBooleanOverlapArrangementBoundarySegmentRow>,
        cell_set: PlanarBooleanOverlapArrangementCellSet,
        counters: PlanarBooleanOverlapArrangementGraphCounters,
    ) -> Self {
        let mut neighborhoods_by_chain = BTreeMap::<String, Vec<usize>>::new();
        for (offset, row) in rows.iter().enumerate() {
            for chain_identity in row.chain_identities() {
                neighborhoods_by_chain
                    .entry(chain_identity.clone())
                    .or_default()
                    .push(offset);
            }
        }
        Self {
            arrangement_graph_identity,
            request_identity,
            adjacency_index_identity,
            ordering_basis_identity,
            rows,
            boundary_components,
            boundary_segments,
            cell_set,
            neighborhoods_by_chain,
            counters,
        }
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        &self.arrangement_graph_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn adjacency_index_identity(&self) -> &str {
        &self.adjacency_index_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow] {
        &self.rows
    }

    pub fn boundary_components(&self) -> &[PlanarBooleanOverlapArrangementBoundaryComponentRow] {
        &self.boundary_components
    }

    pub fn boundary_segments(&self) -> &[PlanarBooleanOverlapArrangementBoundarySegmentRow] {
        &self.boundary_segments
    }

    pub fn cell_set(&self) -> &PlanarBooleanOverlapArrangementCellSet {
        &self.cell_set
    }

    pub fn neighborhoods_for_chain(
        &self,
        chain_identity: &str,
    ) -> Option<impl Iterator<Item = &PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow>> {
        self.neighborhoods_by_chain
            .get(chain_identity)
            .map(|offsets| offsets.iter().map(|offset| &self.rows[*offset]))
    }

    pub fn counters(&self) -> PlanarBooleanOverlapArrangementGraphCounters {
        self.counters
    }
}
