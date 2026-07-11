#[cfg(test)]
use forge_store_blob_chunks::{DeterministicBytePatternProfile, HeavyBlobFixturePlan};

#[cfg(test)]
use super::scenario_seed::BlobHarnessScenarioSeed;
#[cfg(test)]
use forge_store_blob_chunks::HeavyBlobFixtureMaterializationMode;

#[cfg(test)]
pub fn canonical_heavy_fixture_plan_for_seed(
    seed: &BlobHarnessScenarioSeed,
    materialization_mode: HeavyBlobFixtureMaterializationMode,
) -> Option<HeavyBlobFixturePlan> {
    HeavyBlobFixturePlan::canonical_for_profile(seed.size_class(), seed.topology())
        .map(|plan| plan.with_materialization_mode(materialization_mode))
}

#[cfg(test)]
pub fn canonical_heavy_fixture_pattern_plan_for_seed(
    seed: &BlobHarnessScenarioSeed,
    pattern: DeterministicBytePatternProfile,
    materialization_mode: HeavyBlobFixtureMaterializationMode,
) -> Option<HeavyBlobFixturePlan> {
    canonical_heavy_fixture_plan_for_seed(seed, materialization_mode)
        .map(|plan| plan.with_byte_pattern_profile(pattern))
}

#[cfg(test)]
pub fn non_canonical_chaos_stress_plan_for_seed(
    seed: &BlobHarnessScenarioSeed,
) -> HeavyBlobFixturePlan {
    HeavyBlobFixturePlan::ambient_chaos_corpus_stress_for_topology(seed.topology())
}
