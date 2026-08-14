use super::super::locality_scale::LocalityScaleTuple;
use super::FinancialLocalityDefinition;

mod partitioned;
mod sparse;

impl FinancialLocalityDefinition {
    pub(in crate::tests::domains::fintech::world) fn generate(
        seed: u64,
        scale: LocalityScaleTuple,
    ) -> Self {
        match scale {
            LocalityScaleTuple::SparseBookFanout {
                total_outputs,
                axis,
            } => sparse::generate(seed, scale, total_outputs, axis),
            LocalityScaleTuple::PartitionedCurveUniverse {
                regions,
                matching_memberships,
                instruments_per_matching_region,
            } => partitioned::generate(
                seed,
                scale,
                partitioned::PartitionScale {
                    regions,
                    matching_memberships,
                    instruments_per_matching_region,
                },
            ),
            _ => panic!("Phase 1 locality generator supports sparse and partition worlds"),
        }
    }
}
