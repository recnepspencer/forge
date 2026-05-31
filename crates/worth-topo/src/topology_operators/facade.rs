use forge_relational::facade::history::BranchId;
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent};

use super::contracts::TopologyEditContract;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyEditApplicationMode {
    Mainline,
    BranchLocal(BranchId),
}

fn mutation_origin_for_mode(mode: &TopologyEditApplicationMode) -> MutationOrigin {
    match mode {
        TopologyEditApplicationMode::Mainline => MutationOrigin::LocalEdit,
        TopologyEditApplicationMode::BranchLocal(_) => MutationOrigin::BranchLocalApplication,
    }
}

pub(crate) fn raw_topology_intent_for_contracts(
    contracts: Vec<TopologyEditContract>,
    mode: &TopologyEditApplicationMode,
) -> RawTopologyIntent {
    let mutations = contracts
        .into_iter()
        .flat_map(|contract| contract.lowered_mutations().to_vec())
        .collect();
    RawTopologyIntent::new(mutations, mutation_origin_for_mode(mode))
}
