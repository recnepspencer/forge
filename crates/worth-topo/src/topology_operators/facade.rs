use forge_relational::facade::history::BranchId;
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent};

use super::contracts::{TopologyEditContract, TopologyEditFamily, TopologyEditNamingReport};
use super::{topology_edit_families_for_contracts, topology_edit_naming_report_for_contracts};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyEditApplicationMode {
    Mainline,
    BranchLocal(BranchId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyEditBatch {
    contracts: Vec<TopologyEditContract>,
}

impl TopologyEditBatch {
    pub fn new(contracts: Vec<TopologyEditContract>) -> Result<Self, TopologyEditBatchError> {
        if contracts.is_empty() {
            return Err(TopologyEditBatchError::EmptyBatch);
        }
        Ok(Self { contracts })
    }

    pub fn contracts(&self) -> &[TopologyEditContract] {
        &self.contracts
    }

    pub fn naming_report(&self) -> TopologyEditNamingReport {
        topology_edit_naming_report_for_contracts(&self.contracts)
    }

    pub fn families(&self) -> Vec<TopologyEditFamily> {
        topology_edit_families_for_contracts(&self.contracts)
    }

    pub fn into_raw_intent(self, mode: &TopologyEditApplicationMode) -> RawTopologyIntent {
        raw_topology_intent_for_contracts(self.contracts, mode)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyEditBatchError {
    EmptyBatch,
}

impl std::fmt::Display for TopologyEditBatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyBatch => write!(f, "topology edit batch must contain at least one contract"),
        }
    }
}

impl std::error::Error for TopologyEditBatchError {}

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
