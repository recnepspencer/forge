use forge_store_blob_chunks::certification_test_authority::{
    execute_s7_blob_harness, BlobHarnessExecutedWitness, BlobHarnessExecutionInput,
};
use forge_store_physical_certification::{
    synthetic_blob_harness_coverage_matrix_for_test_support,
    synthetic_blob_harness_replay_bundle_for_test_support, BlobHarnessScenarioSeed,
    GeneratedCoverageMatrix, SimulationReplayBundle,
};

pub fn execute_s7_blob_harness_scenario(seed: BlobHarnessScenarioSeed) -> BlobHarnessExecutedWitness {
    execute_s7_blob_harness(BlobHarnessExecutionInput::new(
        seed.profile().envelope().profile(),
        seed.size_class(),
        seed.placement_class(),
        seed.security_scope(),
        seed.access_mode(),
        seed.failure_point(),
        seed.actor_mix(),
        seed.topology(),
    ))
}

pub fn synthetic_s7_blob_harness_replay_bundle(
    seed: BlobHarnessScenarioSeed,
) -> SimulationReplayBundle {
    synthetic_blob_harness_replay_bundle_for_test_support(seed)
}

pub fn synthetic_s7_blob_harness_coverage_matrix(
    seed: BlobHarnessScenarioSeed,
) -> GeneratedCoverageMatrix {
    synthetic_blob_harness_coverage_matrix_for_test_support(seed)
}
