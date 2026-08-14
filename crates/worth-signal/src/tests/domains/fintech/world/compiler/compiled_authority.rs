use std::collections::BTreeMap;

use crate::data::aspect::AspectVersion;

use super::super::{
    FinancialEconomicSnapshot, FinancialSemanticProjection, FinancialWorldDefinition,
    SemanticOutputKey,
};
use super::locality_execution::CompiledFinancialLocalityWorld;
use super::{FinancialEvaluationLedger, FinancialRuntime, FinancialSemanticHandles};

pub(in crate::tests::domains::fintech) struct CompiledPortfolioFinancialWorld {
    pub(super) runtime: FinancialRuntime,
    pub(super) definition: FinancialWorldDefinition,
    pub(super) economic_snapshot: FinancialEconomicSnapshot,
    pub(super) projection: FinancialSemanticProjection,
    pub(super) handles: FinancialSemanticHandles,
    pub(super) ledger: FinancialEvaluationLedger,
    pub(super) baseline_dependency_revisions: BTreeMap<SemanticOutputKey, u64>,
    pub(super) baseline_aspect_versions: BTreeMap<SemanticOutputKey, AspectVersion>,
}

pub(super) enum CompiledFinancialWorldKind {
    Portfolio(CompiledPortfolioFinancialWorld),
    Locality(CompiledFinancialLocalityWorld),
}

pub(in crate::tests::domains::fintech) struct CompiledFinancialWorld {
    pub(super) kind: CompiledFinancialWorldKind,
}

impl std::ops::Deref for CompiledFinancialWorld {
    type Target = CompiledPortfolioFinancialWorld;

    fn deref(&self) -> &Self::Target {
        match &self.kind {
            CompiledFinancialWorldKind::Portfolio(portfolio) => portfolio,
            CompiledFinancialWorldKind::Locality(_) => {
                panic!("portfolio operation used with compiled locality world")
            }
        }
    }
}

impl std::ops::DerefMut for CompiledFinancialWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.kind {
            CompiledFinancialWorldKind::Portfolio(portfolio) => portfolio,
            CompiledFinancialWorldKind::Locality(_) => {
                panic!("portfolio mutation used with compiled locality world")
            }
        }
    }
}
