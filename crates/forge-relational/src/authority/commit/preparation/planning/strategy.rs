use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelLegality {
    ProvenParallel,
    RequiresSerial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelProfitability {
    Profitable,
    NotProfitable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreparationStrategySelection {
    Serial,
    StagedParallel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreparationFallbackReason {
    ExecutionModelSerial,
    ProofRequiresSerial,
    InsufficientPacketBreadth,
    ProfitabilityThreshold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparationStrategy {
    pub(crate) parallel_legality: ParallelLegality,
    pub(crate) parallel_profitability: ParallelProfitability,
    pub(crate) selected_mode: PreparationStrategySelection,
    pub(crate) fallback_reason: Option<PreparationFallbackReason>,
}

impl PreparationStrategy {
    pub(crate) fn serial(reason: PreparationFallbackReason) -> Self {
        Self {
            parallel_legality: ParallelLegality::RequiresSerial,
            parallel_profitability: ParallelProfitability::NotProfitable,
            selected_mode: PreparationStrategySelection::Serial,
            fallback_reason: Some(reason),
        }
    }
}
