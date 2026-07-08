use forge_store_physical_certification::{
    BlobHarnessProfile, BlobHarnessScenarioSeed, BlobHarnessShortcutDenial,
};

pub fn local_s7_blob_harness_seed() -> Result<BlobHarnessScenarioSeed, BlobHarnessShortcutDenial> {
    seed_for_profile(BlobHarnessProfile::local())
}

pub fn ci_memory_envelope_s7_blob_harness_seed(
) -> Result<BlobHarnessScenarioSeed, BlobHarnessShortcutDenial> {
    seed_for_profile(BlobHarnessProfile::ci_memory_envelope_exceeding())
}

pub fn heavy_multi_gb_s7_blob_harness_seed(
) -> Result<BlobHarnessScenarioSeed, BlobHarnessShortcutDenial> {
    seed_for_profile(BlobHarnessProfile::heavy_multi_gb())
}

fn seed_for_profile(
    profile: BlobHarnessProfile,
) -> Result<BlobHarnessScenarioSeed, BlobHarnessShortcutDenial> {
    BlobHarnessScenarioSeed::builder()
        .profile(profile)
        .placement_external()
        .security_scope_preserving()
        .read_only_access()
        .seed_actor_mix()
        .build()
}
