use forge_store_blob_chunks::{
    certification_test_authority::{execute_s7_blob_harness, BlobHarnessExecutedWitness, BlobHarnessExecutionInput},
};

use super::s7_blob_harness_profiles::{
    heavy_multi_gb_s7_blob_harness_seed, local_s7_blob_harness_seed,
};

pub fn execute_s7_blob_harness_temp_file_fixture_smoke() -> BlobHarnessExecutedWitness {
    let seed = local_s7_blob_harness_seed().expect("local blob harness seed");
    execute_s7_blob_harness(
        BlobHarnessExecutionInput::new(
            seed.profile().envelope().profile(),
            seed.size_class(),
            seed.placement_class(),
            seed.security_scope(),
            seed.access_mode(),
            seed.failure_point(),
            seed.actor_mix(),
            seed.topology(),
        )
        .with_heavy_temp_file_materialization(),
    )
}

pub fn execute_s7_blob_harness_real_multi_gb_temp_file_fixture() -> BlobHarnessExecutedWitness {
    let seed = heavy_multi_gb_s7_blob_harness_seed().expect("heavy blob harness seed");
    execute_s7_blob_harness(
        BlobHarnessExecutionInput::new(
            seed.profile().envelope().profile(),
            seed.size_class(),
            seed.placement_class(),
            seed.security_scope(),
            seed.access_mode(),
            seed.failure_point(),
            seed.actor_mix(),
            seed.topology(),
        )
        .with_heavy_temp_file_materialization(),
    )
}
