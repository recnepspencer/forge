use std::num::{NonZeroU32, NonZeroU64};

use worth_store::physical_runtime::{
    CertificationScopeAdmissionFailure, CertificationScopePressure,
    PhysicalOperationAllocationScope as Scope, PhysicalRecordInitialization,
    PhysicalRecordResidencyPolicy, PhysicalResidencyDimension,
    PhysicalSpeculativeWorkKind as Speculation,
};

use super::{configuration, media, success};

const PAGE_BYTES: u64 = 16_384;
const SUCCESSOR_SCOPES: [Scope; 5] = [
    Scope::Recovery,
    Scope::Scrub,
    Scope::Maintenance,
    Scope::Verification,
    Scope::Blob,
];

#[test]
fn successor_scopes_are_exact_isolated_global_and_released() {
    let root = tempfile::tempdir().unwrap();
    let (format, placement, access) = configuration();
    let serving = success(
        media(root.path()).initialize_record_store(
            PhysicalRecordInitialization::new(format, placement, access)
                .with_residency_policy(successor_policy(format)),
        ),
    );
    let certification = serving.certification_physical_residency();
    let store = serving.store_identity();

    for scope in SUCCESSOR_SCOPES {
        let exact = certification
            .admit_operation_scope(scope, bytes(PAGE_BYTES))
            .expect("the exact successor scope ceiling must be admitted");
        assert_eq!(exact.store_identity(), store);
        assert_eq!(exact.scope(), scope);
        assert_eq!(exact.bytes(), PAGE_BYTES);
        assert_eq!(
            serving
                .residency_observation()
                .counters()
                .active_operation_bytes_for(scope),
            PAGE_BYTES,
        );

        let pressure = scope_pressure(
            certification
                .admit_operation_scope(scope, bytes(1))
                .expect_err("one byte past the live scope ceiling must be denied"),
        );
        assert_pressure(
            pressure,
            ExpectedScopePressure {
                store,
                dimension: PhysicalResidencyDimension::OperationScope(scope),
                scope,
                requested: 1,
                current: PAGE_BYTES,
                limit: PAGE_BYTES,
            },
        );

        drop(exact);
        assert_eq!(
            serving
                .residency_observation()
                .counters()
                .active_operation_bytes_for(scope),
            0,
        );
    }

    let recovery = certification
        .admit_operation_scope(Scope::Recovery, bytes(PAGE_BYTES))
        .unwrap();
    let scrub = certification
        .admit_operation_scope(Scope::Scrub, bytes(PAGE_BYTES))
        .expect("Recovery saturation must not consume Scrub's ceiling");
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

    let held = SUCCESSOR_SCOPES
        .into_iter()
        .map(|scope| {
            certification
                .admit_operation_scope(scope, bytes(PAGE_BYTES))
                .expect("all five successor scopes must coexist at their exact ceilings")
        })
        .collect::<Vec<_>>();
    let counters = serving.residency_observation().counters();
    assert_eq!(
        counters.active_operation_bytes(),
        PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64,
    );
    for scope in SUCCESSOR_SCOPES {
        assert_eq!(counters.active_operation_bytes_for(scope), PAGE_BYTES);
    }

    let pressure = scope_pressure(
        certification
            .admit_operation_scope(Scope::ForegroundRead, bytes(1))
            .expect_err("the combined successor scopes must stop at the global envelope"),
    );
    assert_pressure(
        pressure,
        ExpectedScopePressure {
            store,
            dimension: PhysicalResidencyDimension::OperationBytes,
            scope: Scope::ForegroundRead,
            requested: 1,
            current: PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64,
            limit: PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64,
        },
    );

    drop(held);
    let released = serving.residency_observation().counters();
    assert_eq!(released.active_operation_bytes(), 0);
    assert_eq!(
        released.peak_operation_bytes(),
        PAGE_BYTES * SUCCESSOR_SCOPES.len() as u64,
    );
    for scope in SUCCESSOR_SCOPES {
        assert_eq!(released.active_operation_bytes_for(scope), 0);
        assert_eq!(released.peak_operation_bytes_for(scope), PAGE_BYTES);
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
        builder = builder.scope_bytes(scope, bytes(PAGE_BYTES));
    }
    builder
        .speculative_frames(Speculation::Prefetch, frames(8))
        .speculative_frames(Speculation::ReadAhead, frames(8))
        .speculative_frames(Speculation::WriteBehind, frames(4))
        .admit(format)
        .into_result()
        .unwrap()
}

fn scope_pressure(failure: CertificationScopeAdmissionFailure) -> CertificationScopePressure {
    match failure {
        CertificationScopeAdmissionFailure::Pressure(pressure) => pressure,
        CertificationScopeAdmissionFailure::Residency(failure) => {
            panic!("scope admission failed outside pressure: {failure:?}")
        }
    }
}

struct ExpectedScopePressure {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    dimension: PhysicalResidencyDimension,
    scope: Scope,
    requested: u64,
    current: u64,
    limit: u64,
}

fn assert_pressure(pressure: CertificationScopePressure, expected: ExpectedScopePressure) {
    assert_eq!(pressure.store_identity(), expected.store);
    assert_eq!(pressure.dimension(), expected.dimension);
    assert_eq!(pressure.scope(), expected.scope);
    assert_eq!(pressure.requested(), expected.requested);
    assert_eq!(pressure.current(), expected.current);
    assert_eq!(pressure.limit(), expected.limit);
    assert!(!pressure.effect_may_have_started());
}

fn bytes(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn frames(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).unwrap()
}
