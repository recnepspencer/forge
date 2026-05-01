use forge_relational::facade::history::BranchId;
use worth_schema::facade::{RawWorthTopologyIntent, WorthMutationOrigin};

use super::types::{
    WorthTopologyEditContract, WorthTopologyEditFamily, WorthTopologyEditNamingReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorthTopologyEditApplicationMode {
    Mainline,
    BranchLocal(BranchId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyEditBatch {
    contracts: Vec<WorthTopologyEditContract>,
}

impl WorthTopologyEditBatch {
    pub fn new(
        contracts: Vec<WorthTopologyEditContract>,
    ) -> Result<Self, WorthTopologyEditBatchError> {
        if contracts.is_empty() {
            return Err(WorthTopologyEditBatchError::EmptyBatch);
        }
        Ok(Self { contracts })
    }

    pub fn contracts(&self) -> &[WorthTopologyEditContract] {
        &self.contracts
    }

    pub fn naming_report(&self) -> WorthTopologyEditNamingReport {
        let rows = self
            .contracts
            .iter()
            .flat_map(|contract| contract.naming_report().rows)
            .collect();
        WorthTopologyEditNamingReport { rows }
    }

    pub fn families(&self) -> Vec<WorthTopologyEditFamily> {
        self.contracts
            .iter()
            .map(|contract| contract.family)
            .collect()
    }

    pub fn into_raw_intent(
        self,
        mode: &WorthTopologyEditApplicationMode,
    ) -> RawWorthTopologyIntent {
        let mutations = self
            .contracts
            .into_iter()
            .flat_map(|contract| contract.lowered_mutations().to_vec())
            .collect();
        RawWorthTopologyIntent::new(mutations, mutation_origin_for_mode(mode))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorthTopologyEditBatchError {
    EmptyBatch,
}

impl std::fmt::Display for WorthTopologyEditBatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyBatch => write!(f, "topology edit batch must contain at least one contract"),
        }
    }
}

impl std::error::Error for WorthTopologyEditBatchError {}

fn mutation_origin_for_mode(mode: &WorthTopologyEditApplicationMode) -> WorthMutationOrigin {
    match mode {
        WorthTopologyEditApplicationMode::Mainline => WorthMutationOrigin::LocalEdit,
        WorthTopologyEditApplicationMode::BranchLocal(_) => {
            WorthMutationOrigin::BranchLocalApplication
        }
    }
}
