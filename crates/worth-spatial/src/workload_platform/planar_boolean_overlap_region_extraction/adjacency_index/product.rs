use std::collections::BTreeMap;

use super::construction::build_adjacency_index;
use super::counters::PlanarBooleanOverlapAdjacencyIndexCounters;
use super::identity::neighborhood_group_identity;
use super::input::PlanarBooleanOverlapAdjacencyIndexInput;
use super::neighborhood::PlanarBooleanOverlapNeighborhoodView;
use super::ordering::PlanarBooleanOverlapAdjacencyOrderingBasis;
use super::row::PlanarBooleanOverlapAdjacencyRow;
use super::PlanarBooleanOverlapAdjacencyIndexDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionAdjacencyIndex {
    adjacency_index_identity: String,
    request_identity: String,
    loop_participation_map_identity: String,
    island_participation_map_identity: String,
    chain_lineage_map_identity: String,
    rows: Vec<PlanarBooleanOverlapAdjacencyRow>,
    ordering_basis: PlanarBooleanOverlapAdjacencyOrderingBasis,
    source_only_boundary_lane: bool,
    neighborhoods_by_chain: BTreeMap<String, Vec<usize>>,
    neighborhoods_by_loop: BTreeMap<String, Vec<usize>>,
    neighborhoods_by_island: BTreeMap<String, Vec<usize>>,
    counters: PlanarBooleanOverlapAdjacencyIndexCounters,
}

impl PlanarBooleanOverlapRegionAdjacencyIndex {
    pub fn admit(
        input: PlanarBooleanOverlapAdjacencyIndexInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapAdjacencyIndexDenial> {
        build_adjacency_index(input)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        adjacency_index_identity: String,
        request_identity: String,
        loop_participation_map_identity: String,
        island_participation_map_identity: String,
        chain_lineage_map_identity: String,
        rows: Vec<PlanarBooleanOverlapAdjacencyRow>,
        ordering_basis: PlanarBooleanOverlapAdjacencyOrderingBasis,
        counters: PlanarBooleanOverlapAdjacencyIndexCounters,
    ) -> Self {
        Self::new_with_source_only_boundary_lane(
            adjacency_index_identity,
            request_identity,
            loop_participation_map_identity,
            island_participation_map_identity,
            chain_lineage_map_identity,
            rows,
            ordering_basis,
            false,
            counters,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_with_source_only_boundary_lane(
        adjacency_index_identity: String,
        request_identity: String,
        loop_participation_map_identity: String,
        island_participation_map_identity: String,
        chain_lineage_map_identity: String,
        rows: Vec<PlanarBooleanOverlapAdjacencyRow>,
        ordering_basis: PlanarBooleanOverlapAdjacencyOrderingBasis,
        source_only_boundary_lane: bool,
        counters: PlanarBooleanOverlapAdjacencyIndexCounters,
    ) -> Self {
        let mut neighborhoods_by_chain = BTreeMap::<String, Vec<usize>>::new();
        let mut neighborhoods_by_loop = BTreeMap::<String, Vec<usize>>::new();
        let mut neighborhoods_by_island = BTreeMap::<String, Vec<usize>>::new();
        for (offset, row) in rows.iter().enumerate() {
            for chain_identity in row.chain_identities() {
                neighborhoods_by_chain
                    .entry(chain_identity.clone())
                    .or_default()
                    .push(offset);
            }
            for loop_identity in row.participating_loop_identities() {
                neighborhoods_by_loop
                    .entry(loop_identity.clone())
                    .or_default()
                    .push(offset);
            }
            for island_identity in row.participating_island_identities() {
                neighborhoods_by_island
                    .entry(island_identity.clone())
                    .or_default()
                    .push(offset);
            }
        }
        Self {
            adjacency_index_identity,
            request_identity,
            loop_participation_map_identity,
            island_participation_map_identity,
            chain_lineage_map_identity,
            rows,
            ordering_basis,
            source_only_boundary_lane,
            neighborhoods_by_chain,
            neighborhoods_by_loop,
            neighborhoods_by_island,
            counters,
        }
    }

    pub fn adjacency_index_identity(&self) -> &str {
        &self.adjacency_index_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn loop_participation_map_identity(&self) -> &str {
        &self.loop_participation_map_identity
    }

    pub fn island_participation_map_identity(&self) -> &str {
        &self.island_participation_map_identity
    }

    pub fn chain_lineage_map_identity(&self) -> &str {
        &self.chain_lineage_map_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapAdjacencyRow] {
        &self.rows
    }

    pub fn ordering_basis(&self) -> &PlanarBooleanOverlapAdjacencyOrderingBasis {
        &self.ordering_basis
    }

    pub fn source_only_boundary_lane(&self) -> bool {
        self.source_only_boundary_lane
    }

    pub fn counters(&self) -> PlanarBooleanOverlapAdjacencyIndexCounters {
        self.counters
    }

    pub fn neighborhoods_for_chain(
        &self,
        chain_identity: &str,
    ) -> Option<PlanarBooleanOverlapNeighborhoodView<'_>> {
        self.neighborhoods_by_chain
            .get(chain_identity)
            .map(|offsets| PlanarBooleanOverlapNeighborhoodView::new(self, chain_identity, offsets))
    }

    pub fn neighborhoods_for_loop(
        &self,
        canonical_loop_identity: &str,
    ) -> Option<PlanarBooleanOverlapNeighborhoodView<'_>> {
        self.neighborhoods_by_loop
            .get(canonical_loop_identity)
            .map(|offsets| {
                PlanarBooleanOverlapNeighborhoodView::new(self, canonical_loop_identity, offsets)
            })
    }

    pub fn neighborhoods_for_island(
        &self,
        island_identity: &str,
    ) -> Option<PlanarBooleanOverlapNeighborhoodView<'_>> {
        self.neighborhoods_by_island
            .get(island_identity)
            .map(|offsets| {
                PlanarBooleanOverlapNeighborhoodView::new(self, island_identity, offsets)
            })
    }

    pub fn neighborhood_groups(&self) -> impl Iterator<Item = String> + '_ {
        self.rows.iter().map(neighborhood_group_identity)
    }
}
