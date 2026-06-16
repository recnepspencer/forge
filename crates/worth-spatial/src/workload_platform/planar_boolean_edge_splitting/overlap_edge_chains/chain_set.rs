use super::chain_row::PlanarBooleanOverlapEdgeChain;
use super::counters::PlanarBooleanOverlapEdgeChainCounters;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanOverlapEdgeChainSet {
    chain_set_identity: String,
    interval_subdivision_schedule_set_identity: String,
    split_edge_fragment_set_identity: String,
    chains: Vec<PlanarBooleanOverlapEdgeChain>,
    counters: PlanarBooleanOverlapEdgeChainCounters,
}

impl PlanarBooleanOverlapEdgeChainSet {
    pub(crate) fn new(
        chain_set_identity: String,
        interval_subdivision_schedule_set_identity: String,
        split_edge_fragment_set_identity: String,
        chains: Vec<PlanarBooleanOverlapEdgeChain>,
        counters: PlanarBooleanOverlapEdgeChainCounters,
    ) -> Self {
        Self {
            chain_set_identity,
            interval_subdivision_schedule_set_identity,
            split_edge_fragment_set_identity,
            chains,
            counters,
        }
    }

    pub fn chain_set_identity(&self) -> &str {
        &self.chain_set_identity
    }
    pub fn interval_subdivision_schedule_set_identity(&self) -> &str {
        &self.interval_subdivision_schedule_set_identity
    }
    pub fn split_edge_fragment_set_identity(&self) -> &str {
        &self.split_edge_fragment_set_identity
    }
    pub fn chains(&self) -> &[PlanarBooleanOverlapEdgeChain] {
        &self.chains
    }
    pub fn counters(&self) -> PlanarBooleanOverlapEdgeChainCounters {
        self.counters
    }
    pub fn certifies_prepared_overlap_chains(&self) -> bool {
        self.counters.chains_emitted() == self.chains.len()
            && self.counters.chain_members_emitted()
                == self
                    .chains
                    .iter()
                    .map(|chain| chain.members().len())
                    .sum::<usize>()
            && !self.emits_topology_truth()
    }
    pub fn emits_topology_truth(&self) -> bool {
        self.counters.topology_products_emitted() != 0
    }
}
