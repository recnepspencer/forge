use super::*;

static TEST_STORE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn runtime_with_test_schema() -> RelationalRuntime {
    runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore)
}

pub(crate) fn runtime_with_declared_aspect_schema_profile(
    profile: RelationalRuntimeProfile,
    cascade_delete_policy: CascadeDeletePolicy,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(profile)
        .schema_registry(declared_aspect_schema_registry(cascade_delete_policy))
        .build()
}

pub(crate) fn runtime_with_test_schema_execution_model(
    execution_model: crate::facade::runtime::RelationalExecutionModel,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .execution_model(execution_model)
        .build()
}

pub(crate) fn runtime_with_test_schema_profile(
    profile: RelationalRuntimeProfile,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(profile)
        .schema_registry(test_schema_registry())
        .build()
}

pub(crate) fn persisted_runtime_with_test_schema() -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: unique_test_store_path("forge-relational-persisted"),
            segment_commit_capacity: 2,
        })
        .build()
}

pub(crate) fn persisted_runtime_with_declared_aspect_schema(
    cascade_delete_policy: CascadeDeletePolicy,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(cascade_delete_policy))
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: unique_test_store_path("forge-relational-persisted-aspects"),
            segment_commit_capacity: 2,
        })
        .build()
}

pub(crate) fn unique_test_store_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = TEST_STORE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("{prefix}-{nanos}-{counter}"));
    let _ = fs::remove_dir_all(&path);
    path
}

pub(crate) fn runtime_with_test_schema_and_chunks(
    entity_chunk_size: usize,
    relation_chunk_size: usize,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .storage_layout(StorageLayoutConfig {
            entity_chunk_size,
            relation_chunk_size,
            scan_packet_size: 64,
        })
        .build()
}

pub(crate) fn runtime_with_test_schema_and_invariants(
    invariant_catalog: InvariantCatalog,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .invariant_catalog(invariant_catalog)
        .build()
}
