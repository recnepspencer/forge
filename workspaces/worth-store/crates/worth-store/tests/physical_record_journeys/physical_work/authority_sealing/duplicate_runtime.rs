use super::assert_sources_exclude;

const PHYSICAL_RUNTIME: &str = "src/physical_runtime";

#[test]
fn a_second_pending_work_registry_is_forbidden() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "duplicate-work-registry",
        &["PendingWorkRegistry"],
    );
}

#[test]
fn store_local_async_registries_are_forbidden() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "store-local-async-registry",
        &[
            "TimerWheel",
            "RetryQueue",
            "TimeoutRegistry",
            "PolicyRegistry",
        ],
    );
}

#[test]
fn a_second_physical_lifecycle_is_forbidden() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "lifecycle-duplication",
        &["DuplicatePhysicalLifecycle"],
    );
}
