use forge_relational::facade::history::BranchId;
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent};

use super::contracts::{TopologyEditContract, TopologyEditFamily, TopologyEditNamingReport};
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
        let rows = self
            .contracts
            .iter()
            .flat_map(|contract| contract.naming_report().rows)
            .collect();
        TopologyEditNamingReport { rows }
    }

    pub fn families(&self) -> Vec<TopologyEditFamily> {
        self.contracts
            .iter()
            .map(|contract| contract.family)
            .collect()
    }

    pub fn into_raw_intent(self, mode: &TopologyEditApplicationMode) -> RawTopologyIntent {
        let mutations = self
            .contracts
            .into_iter()
            .flat_map(|contract| contract.lowered_mutations().to_vec())
            .collect();
        RawTopologyIntent::new(mutations, mutation_origin_for_mode(mode))
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




