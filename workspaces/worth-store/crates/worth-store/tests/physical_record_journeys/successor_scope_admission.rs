use std::num::{NonZeroU32, NonZeroU64};

use worth_store::physical_runtime::{
    PhysicalOperationAllocationScope as Scope, PhysicalRecordInitialization,
    PhysicalRecordPressureEvidence, PhysicalRecordResidencyPolicy, PhysicalResidencyDimension,
    PhysicalSpeculativeWorkKind as Speculation, ServingPhysicalRuntime,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{configuration, media, success};

const PAGE_BYTES: u64 = 16_384;
const SUCCESSOR_SCOPES: [Scope; 5] = [
    Scope::Recovery,
    Scope::Scrub,
    Scope::Maintenance,
    Scope::Verification,
    Scope::Blob,
];

macro_rules! assert_exact_scope_ceiling {
    ($serving:expr, $allocations:expr, $store:expr, $generation:expr, $admit:ident, $scope:expr) => {{
        let exact = $allocations
            .$admit(bytes(PAGE_BYTES * 2))
            .expect("the exact successor scope ceiling must be admitted");
        assert_eq!(exact.store_identity(), $store);
        assert_eq!(exact.runtime_identity(), $serving.runtime_identity());
        assert_eq!(exact.store_generation(), $generation);
        assert_eq!(exact.bytes(), PAGE_BYTES * 2);
        assert_eq!(
            $serving
                .residency_observation()
                .counters()
                .active_operation_bytes_for($scope),
            PAGE_BYTES * 2,
        );
        let pressure = $allocations
            .$admit(bytes(1))
            .expect_err("one byte past the live scope ceiling must be denied")
            .pressure()
            .expect("scope exhaustion must lower to Store pressure evidence");
        assert_pressure(
            pressure,
            ExpectedScopePressure {
                store: $store,
                dimension: PhysicalResidencyDimension::OperationScope($scope),
                scope: $scope,
                requested: 1,
                admitted: PAGE_BYTES * 2,
                limit: PAGE_BYTES * 2,
            },
        );
        drop(exact);
        assert_eq!(
            $serving
                .residency_observation()
                .counters()
                .active_operation_bytes_for($scope),
            0,
        );
    }};
}

#[test]
fn successor_scopes_are_exact_isolated_global_and_released() {
    let root = tempfile::tempdir().unwrap();
    let serving = successor_scope_runtime(root.path());
    assert_exact_scope_ceilings(&serving);
    assert_scope_isolation(&serving);
    assert_global_envelope_and_release(&serving);
}

fn successor_scope_runtime(root: &std::path::Path) -> ServingPhysicalRuntime {
    let (format, placement, access) = configuration();
    success(initialize_record_store!(media(root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
            .with_residency_policy(successor_policy(format))
    },))
}

fn assert_exact_scope_ceilings(serving: &ServingPhysicalRuntime) {
    let allocations = serving.physical_allocations();
    let store = serving.store_identity();
    let generation = serving.residency_observation().store_generation();
    assert_exact_scope_ceiling!(
        serving,
        allocations,
        store,
        generation,
        admit_recovery,
        Scope::Recovery
    );
    assert_exact_scope_ceiling!(
        serving,
        allocations,
        store,
        generation,
        admit_scrub,
        Scope::Scrub
    );
    assert_exact_scope_ceiling!(
        serving,
        allocations,
        store,
        generation,
        admit_maintenance,
        Scope::Maintenance
    );
    assert_exact_scope_ceiling!(
        serving,
        allocations,
        store,
        generation,
        admit_verification,
        Scope::Verification
    );
    assert_exact_scope_ceiling!(
        serving,
        allocations,
        store,
        generation,
        admit_blob,
        Scope::Blob
    );
}

fn assert_scope_isolation(serving: &ServingPhysicalRuntime) {
    let allocations = serving.physical_allocations();
    let recovery = allocations.admit_recovery(bytes(PAGE_BYTES)).unwrap();
    let scrub = allocations
        .admit_scrub(bytes(PAGE_BYTES))
        .expect("Recovery allocation must not consume Scrub's ceiling");
    assert_eq!(
        serving
            .residency_observation()
            .counters()
            .active_operation_bytes_for(Scope::Recovery),
        PAGE_BYTES,
    );
    assert_eq!(
        serving
            .residency_observation()
            .counters()
            .active_operation_bytes_for(Scope::Scrub),
        PAGE_BYTES,
    );
    drop((recovery, scrub));
}

fn assert_global_envelope_and_release(serving: &ServingPhysicalRuntime) {
    let allocations = serving.physical_allocations();
    let recovery = allocations.admit_recovery(bytes(PAGE_BYTES)).unwrap();
    let scrub = allocations.admit_scrub(bytes(PAGE_BYTES)).unwrap();
    let maintenance = allocations.admit_maintenance(bytes(PAGE_BYTES)).unwrap();
    let verification = allocations.admit_verification(bytes(PAGE_BYTES)).unwrap();
    let blob = allocations.admit_blob(bytes(PAGE_BYTES)).unwrap();
    let counters = serving.residency_observation().counters();
    assert_eq!(
        counters.active_operation_bytes(),
        PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64,
    );
    for scope in SUCCESSOR_SCOPES {
        assert_eq!(counters.active_operation_bytes_for(scope), PAGE_BYTES);
    }

    let pressure = allocations
        .admit_recovery(bytes(PAGE_BYTES))
        .expect_err("the combined successor scopes must stop at the global envelope")
        .pressure()
        .expect("global exhaustion must lower to Store pressure evidence");
    assert_pressure(
        pressure,
        ExpectedScopePressure {
            store: serving.store_identity(),
            dimension: PhysicalResidencyDimension::OperationBytes,
            scope: Scope::Recovery,
            requested: PAGE_BYTES,
            admitted: PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64,
            limit: PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64,
        },
    );

    drop((recovery, scrub, maintenance, verification, blob));
    assert_released_scope_counters(serving);
}

fn assert_released_scope_counters(serving: &ServingPhysicalRuntime) {
    let released = serving.residency_observation().counters();
    assert_eq!(released.active_operation_bytes(), 0);
    assert_eq!(
        released.peak_operation_bytes(),
        PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64,
    );
    for scope in SUCCESSOR_SCOPES {
        assert_eq!(released.active_operation_bytes_for(scope), 0);
        assert_eq!(released.peak_operation_bytes_for(scope), PAGE_BYTES * 2);
    }
}

fn successor_policy(
    format: worth_store::physical_runtime::AdmittedPhysicalRecordFormat,
) -> worth_store::physical_runtime::AdmittedPhysicalRecordResidencyPolicy {
    let operation = bytes(PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64);
    let mut builder = PhysicalRecordResidencyPolicy::builder()
        .total_bytes(bytes(512 * 1024))
        .resident_bytes(bytes(PAGE_BYTES))
        .metadata_bytes(bytes(256 * 1024))
        .frame_entries(frames(8))
        .pinned_frames(frames(8))
        .pin_leases(frames(8))
        .dirty_frames(frames(4))
        .dirty_replacement_bytes(bytes(PAGE_BYTES))
        .operation_bytes(operation)
        .scope_bytes(Scope::ForegroundRead, bytes(PAGE_BYTES))
        .scope_bytes(Scope::ForegroundWrite, bytes(PAGE_BYTES));
    for scope in SUCCESSOR_SCOPES {
        builder = builder.scope_bytes(scope, bytes(PAGE_BYTES * 2));
    }
    builder
        .speculative_frames(Speculation::Prefetch, frames(8))
        .speculative_frames(Speculation::ReadAhead, frames(8))
        .speculative_frames(Speculation::WriteBehind, frames(4))
        .admit(format)
        .into_result()
        .unwrap()
}

struct ExpectedScopePressure {
    store: StableStoreIdentity,
    dimension: PhysicalResidencyDimension,
    scope: Scope,
    requested: u64,
    admitted: u64,
    limit: u64,
}

fn assert_pressure(pressure: PhysicalRecordPressureEvidence, expected: ExpectedScopePressure) {
    assert_eq!(pressure.basis().store_identity(), expected.store);
    assert_eq!(pressure.dimension(), expected.dimension);
    assert_eq!(pressure.scope(), expected.scope);
    assert_eq!(pressure.requested(), expected.requested);
    assert_eq!(pressure.admitted(), expected.admitted);
    assert_eq!(pressure.limit(), expected.limit);
    assert!(!pressure.effect_may_have_started());
}

fn bytes(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn frames(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).unwrap()
}
