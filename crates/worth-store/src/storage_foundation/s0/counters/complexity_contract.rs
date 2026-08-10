use super::super::evidence::S0ComplexityContractName;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum S0ComplexityStatus {
    Declared,
    Verified,
    Debt,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ComplexityContract {
    name: &'static str,
    status: S0ComplexityStatus,
    max_global_scans: u64,
    max_unindexed_repo_passes: u64,
}

impl S0ComplexityContract {
    pub fn declared(name: &'static str) -> Self {
        Self {
            name,
            status: S0ComplexityStatus::Declared,
            max_global_scans: 0,
            max_unindexed_repo_passes: 0,
        }
    }

    pub fn verified(
        name: &'static str,
        max_global_scans: u64,
        max_unindexed_repo_passes: u64,
    ) -> Self {
        Self {
            name,
            status: S0ComplexityStatus::Verified,
            max_global_scans,
            max_unindexed_repo_passes,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn status(&self) -> S0ComplexityStatus {
        self.status
    }

    pub fn max_global_scans(&self) -> u64 {
        self.max_global_scans
    }

    pub fn max_unindexed_repo_passes(&self) -> u64 {
        self.max_unindexed_repo_passes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ComplexityContractReport {
    required_contract_count: u64,
    observed_contract_count: u64,
    missing_required_contracts: Vec<&'static str>,
    duplicate_contracts: Vec<&'static str>,
    debt_contracts: Vec<&'static str>,
    max_global_scans: u64,
    max_unindexed_repo_passes: u64,
}

impl S0ComplexityContractReport {
    pub fn from_contracts(
        required_contracts: impl IntoIterator<Item = S0ComplexityContractName>,
        observed_contracts: impl IntoIterator<Item = S0ComplexityContract>,
    ) -> Self {
        let required = required_contracts
            .into_iter()
            .map(|name| name.as_str())
            .collect::<BTreeSet<_>>();
        let mut duplicate_names = BTreeSet::new();
        let mut observed = BTreeMap::new();
        for contract in observed_contracts {
            if observed.insert(contract.name(), contract.clone()).is_some() {
                duplicate_names.insert(contract.name());
            }
        }
        let missing_required_contracts = required
            .iter()
            .filter(|name| !observed.contains_key(**name))
            .copied()
            .collect::<Vec<_>>();
        let duplicate_contracts = duplicate_names.into_iter().collect::<Vec<_>>();
        let debt_contracts = observed
            .values()
            .filter(|contract| contract.status() != S0ComplexityStatus::Verified)
            .map(S0ComplexityContract::name)
            .collect::<Vec<_>>();
        let max_global_scans = observed
            .values()
            .map(S0ComplexityContract::max_global_scans)
            .sum();
        let max_unindexed_repo_passes = observed
            .values()
            .map(S0ComplexityContract::max_unindexed_repo_passes)
            .sum();
        Self {
            required_contract_count: required.len() as u64,
            observed_contract_count: observed.len() as u64,
            missing_required_contracts,
            duplicate_contracts,
            debt_contracts,
            max_global_scans,
            max_unindexed_repo_passes,
        }
    }

    pub fn observed_contract_count(&self) -> u64 {
        self.observed_contract_count
    }

    pub fn required_contract_count(&self) -> u64 {
        self.required_contract_count
    }

    pub fn missing_complexity_contract_count(&self) -> u64 {
        self.missing_required_contracts.len() as u64
    }

    pub fn duplicate_complexity_contract_count(&self) -> u64 {
        self.duplicate_contracts.len() as u64
    }

    pub fn complexity_debt_count(&self) -> u64 {
        self.debt_contracts.len() as u64
    }

    pub fn max_global_scans(&self) -> u64 {
        self.max_global_scans
    }

    pub fn max_unindexed_repo_passes(&self) -> u64 {
        self.max_unindexed_repo_passes
    }
}
