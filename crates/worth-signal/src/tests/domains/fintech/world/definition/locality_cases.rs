use super::{FinancialWorldDefinition, FinancialWorldDefinitionKind};
use crate::tests::domains::fintech::world::{
    DensityRatio, FinancialLocalityDefinition, LocalityCaseContract, LocalityScaleTuple,
    SparseFanoutAxis,
};

impl FinancialWorldDefinition {
    pub(in crate::tests::domains::fintech) fn sparse_book_fanout(
        seed: u64,
        total_outputs: u32,
        axis: SparseFanoutAxis,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(FinancialLocalityDefinition::generate(
                seed,
                LocalityScaleTuple::SparseBookFanout {
                    total_outputs,
                    axis,
                },
            )),
        }
    }

    pub(crate) fn partitioned_curve_universe(
        seed: u64,
        regions: u16,
        matching_memberships: u16,
        instruments_per_matching_region: u16,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(FinancialLocalityDefinition::generate(
                seed,
                LocalityScaleTuple::PartitionedCurveUniverse {
                    regions,
                    matching_memberships,
                    instruments_per_matching_region,
                },
            )),
        }
    }

    pub(crate) fn partitioned_curve_universe_performance(
        seed: u64,
        regions: u16,
        matching_memberships: u16,
        instruments_per_matching_region: u16,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(
                FinancialLocalityDefinition::generate_partitioned_performance(
                    seed,
                    LocalityScaleTuple::PartitionedCurveUniverse {
                        regions,
                        matching_memberships,
                        instruments_per_matching_region,
                    },
                ),
            ),
        }
    }

    pub(in crate::tests::domains::fintech) fn locality_case(
        seed: u64,
        case: LocalityCaseContract,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(
                FinancialLocalityDefinition::generate_case(seed, case),
            ),
        }
    }

    pub(in crate::tests::domains::fintech) fn convergent_factor_batch(
        seed: u64,
        duplicate_admissions: u8,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(FinancialLocalityDefinition::generate(
                seed,
                LocalityScaleTuple::ConvergentFactorBatch {
                    producer_permutations: 24,
                    duplicate_admissions,
                    canonical_seeds: 1,
                },
            )),
        }
    }

    pub(crate) fn dense_market_close(
        seed: u64,
        total_outputs: u32,
        affected_ratio: DensityRatio,
    ) -> Self {
        Self {
            kind: FinancialWorldDefinitionKind::Locality(FinancialLocalityDefinition::generate(
                seed,
                LocalityScaleTuple::DenseMarketClose {
                    total_outputs,
                    affected_ratio,
                },
            )),
        }
    }
}
