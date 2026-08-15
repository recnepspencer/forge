use super::super::locality_scale::{LocalityCaseContract, LocalityLane, LocalityScaleTuple};
use super::FinancialLocalityDefinition;

mod churn;
mod convergent;
mod dense;
mod partitioned;
mod restore;
mod sparse;

impl FinancialLocalityDefinition {
    pub(in crate::tests::domains::fintech::world) fn generate(
        seed: u64,
        scale: LocalityScaleTuple,
    ) -> Self {
        Self::generate_with_lane(seed, scale, LocalityLane::OrdinaryChangeGate)
    }

    pub(in crate::tests::domains::fintech::world) fn generate_case(
        seed: u64,
        case: LocalityCaseContract,
    ) -> Self {
        Self::generate_with_lane(seed, case.scale, case.lane)
    }

    fn generate_with_lane(seed: u64, scale: LocalityScaleTuple, lane: LocalityLane) -> Self {
        match scale {
            LocalityScaleTuple::SparseBookFanout {
                total_outputs,
                axis,
            } => sparse::generate(seed, scale, sparse_scale(total_outputs, axis), lane),
            LocalityScaleTuple::PartitionedCurveUniverse {
                regions,
                matching_memberships,
                instruments_per_matching_region,
            } => partitioned::generate(
                seed,
                scale,
                partition_scale(
                    regions,
                    matching_memberships,
                    instruments_per_matching_region,
                ),
                lane,
            ),
            LocalityScaleTuple::ConvergentFactorBatch {
                producer_permutations,
                duplicate_admissions,
                canonical_seeds,
            } => convergent::generate(
                seed,
                scale,
                convergent_scale(producer_permutations, duplicate_admissions, canonical_seeds),
                lane,
            ),
            LocalityScaleTuple::DenseMarketClose {
                total_outputs,
                affected_ratio,
            } => dense::generate(
                seed,
                scale,
                dense_scale(total_outputs, affected_ratio),
                lane,
            ),
            LocalityScaleTuple::PortfolioDependencyChurn {
                rounds,
                canonical_seeds,
            } => churn::generate(seed, scale, churn_scale(rounds, canonical_seeds), lane),
            LocalityScaleTuple::BranchRestoreLocalityReplay {
                posture,
                total_outputs,
                canonical_seeds,
            } => restore::generate(
                seed,
                scale,
                restore_scale(posture, total_outputs, canonical_seeds),
                lane,
            ),
        }
    }
}

fn sparse_scale(
    total_outputs: u32,
    axis: super::super::locality_scale::SparseFanoutAxis,
) -> sparse::SparseScale {
    sparse::SparseScale {
        total_outputs,
        axis,
    }
}

fn partition_scale(
    regions: u16,
    matching_memberships: u16,
    instruments_per_matching_region: u16,
) -> partitioned::PartitionScale {
    partitioned::PartitionScale {
        regions,
        matching_memberships,
        instruments_per_matching_region,
    }
}

fn convergent_scale(
    producer_permutations: u8,
    duplicate_admissions: u8,
    canonical_seeds: u16,
) -> convergent::ConvergentScale {
    convergent::ConvergentScale {
        producer_permutations,
        duplicate_admissions,
        canonical_seeds,
    }
}

fn dense_scale(
    total_outputs: u32,
    affected_ratio: super::super::locality_scale::DensityRatio,
) -> dense::DenseScale {
    dense::DenseScale {
        total_outputs,
        affected_ratio,
    }
}

fn churn_scale(rounds: u16, canonical_seeds: u16) -> churn::ChurnScale {
    churn::ChurnScale {
        rounds,
        canonical_seeds,
    }
}

fn restore_scale(
    posture: super::super::locality_scale::RestorePosture,
    total_outputs: u32,
    canonical_seeds: u16,
) -> restore::RestoreScale {
    restore::RestoreScale {
        posture,
        total_outputs,
        canonical_seeds,
    }
}
