use crate::logic::planning::RelationalExecutionModel;
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

impl PreparationFallbackReason {
    pub const fn diagnostic_label(self) -> &'static str {
        match self {
            Self::ExecutionModelSerial => "execution_model_serial",
            Self::ProofRequiresSerial => "proof_requires_serial",
            Self::InsufficientPacketBreadth => "insufficient_packet_breadth",
            Self::ProfitabilityThreshold => "profitability_threshold",
        }
    }
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

pub(crate) const MIN_PARALLEL_PACKET_WIDTH: usize = 2;
pub(crate) const TARGET_PREPARATION_ITEMS_PER_PACKET: usize = 32;

pub(crate) const fn packet_width_is_profitable(
    packet_count: usize,
    min_parallel_packet_width: usize,
) -> bool {
    packet_count >= min_parallel_packet_width
}

pub(crate) const fn coarse_preparation_packet_count(
    item_count: usize,
    target_items_per_packet: usize,
) -> usize {
    if item_count == 0 {
        0
    } else {
        item_count.div_ceil(target_items_per_packet)
    }
}

pub(crate) fn strategy_for_parallel_packets(
    execution_model: RelationalExecutionModel,
    packet_count: usize,
) -> PreparationStrategy {
    if !matches!(
        execution_model,
        RelationalExecutionModel::StagedParallelPreparation
    ) {
        return PreparationStrategy::serial(PreparationFallbackReason::ExecutionModelSerial);
    }

    if !packet_width_is_profitable(packet_count, MIN_PARALLEL_PACKET_WIDTH) {
        return PreparationStrategy {
            parallel_legality: ParallelLegality::ProvenParallel,
            parallel_profitability: ParallelProfitability::NotProfitable,
            selected_mode: PreparationStrategySelection::Serial,
            fallback_reason: Some(PreparationFallbackReason::InsufficientPacketBreadth),
        };
    }

    PreparationStrategy {
        parallel_legality: ParallelLegality::ProvenParallel,
        parallel_profitability: ParallelProfitability::Profitable,
        selected_mode: PreparationStrategySelection::StagedParallel,
        fallback_reason: None,
    }
}
