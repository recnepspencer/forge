use super::*;

use crate::{
    PhysicalResidencyAllocationBoundaryEvent as Event,
    PhysicalResidencyAllocationBoundaryKind as Kind,
    PhysicalResidencyAllocationOperation as Operation,
};

#[test]
fn resident_materialization_requires_prior_admission_and_preserves_exact_trace() {
    let identity = store(115);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 1, 64, 3)).unwrap();
    let observer = pool.allocation_events();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let operation = Operation::new(NonZeroU64::new(77).unwrap());

    let lease = expect_fault(&pool, &allocation, key)
        .load_observed(Some(operation), |target| {
            let trace = observer.trace();
            let actualization = find_actualization(trace.events(), 77);
            assert_materialization_follows_admission(
                trace.events(),
                actualization,
                "resident-allocation-before-admission",
            );
            target.fill(5);
            Ok::<_, ()>(())
        })
        .unwrap();
    drop(lease);
    assert_eq!(pool.drain_unpinned_clean_frames(), 1);
    drop(allocation);

    let trace = observer.trace();
    assert_eq!(trace.store(), identity);
    assert_eq!(trace.pool(), pool.incarnation());
    assert_ordered_process_trace(trace.events());
    let actualization = find_actualization(trace.events(), 77);
    assert_eq!(
        actualization.dimension(),
        PhysicalResidencyDimension::ResidentBytes
    );
    assert_eq!(actualization.scope(), Some(READ_SCOPE));
    assert_eq!(actualization.requested_units(), 8);
    assert_eq!(actualization.actual_units(), 8);
    assert!(trace.events().iter().any(|event| {
        event.kind() == Kind::Release
            && event.dimension() == PhysicalResidencyDimension::ResidentBytes
            && event.scope() == Some(READ_SCOPE)
            && event.actual_units() == actualization.actual_units()
    }));
}

#[test]
fn metadata_materialization_requires_prior_admission() {
    let identity = store(117);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 1, 64, 3)).unwrap();
    let trace = pool.allocation_events().trace();
    let actualization =
        find_dimension_actualization(trace.events(), PhysicalResidencyDimension::MetadataBytes);
    assert_materialization_follows_admission(
        trace.events(),
        actualization,
        "metadata-allocation-before-admission",
    );
}

#[test]
fn bounded_actualization_retains_limit_actual_units_and_real_operation() {
    let identity = store(116);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 1, 64, 3)).unwrap();
    let observer = pool.allocation_events();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalBoundedFrameKey::new(
        identity,
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block: 2,
        },
        NonZeroU32::new(16).unwrap(),
    );
    let owner = match pool.access_bounded_frame(&allocation, key).unwrap() {
        PhysicalBoundedFrameAccess::Fault(owner) => owner,
        _ => panic!("expected bounded fault ownership"),
    };

    let lease = owner
        .load_observed(
            |_| Ok::<_, ()>(8),
            |target| {
                target.fill(9);
                Ok(Some(Operation::new(NonZeroU64::new(88).unwrap())))
            },
        )
        .unwrap();
    drop(lease);
    assert_eq!(pool.drain_unpinned_clean_frames(), 1);
    drop(allocation);

    let trace = observer.trace();
    assert_ordered_process_trace(trace.events());
    let actualization = find_actualization(trace.events(), 88);
    assert_eq!(actualization.scope(), Some(READ_SCOPE));
    assert_eq!(actualization.requested_units(), 16);
    assert!(actualization.actual_units() >= 8);
    assert!(actualization.actual_units() <= 16);
    assert!(trace.events().iter().any(|event| {
        event.kind() == Kind::Release
            && event.dimension() == PhysicalResidencyDimension::ResidentBytes
            && event.scope() == Some(READ_SCOPE)
            && event.actual_units() == actualization.actual_units()
    }));
}

fn find_dimension_actualization(events: &[Event], dimension: PhysicalResidencyDimension) -> Event {
    events
        .iter()
        .copied()
        .find(|event| event.kind() == Kind::Actualization && event.dimension() == dimension)
        .unwrap_or_else(|| panic!("missing {dimension:?} allocation actualization"))
}

fn find_actualization(events: &[Event], operation: u64) -> Event {
    events
        .iter()
        .copied()
        .find(|event| {
            event.kind() == Kind::Actualization
                && event.operation().map(Operation::get) == Some(operation)
        })
        .unwrap_or_else(|| panic!("missing allocation actualization for operation {operation}"))
}

fn assert_materialization_follows_admission(
    events: &[Event],
    actualization: Event,
    mutation_predicate: &str,
) {
    let actualization_index = events
        .iter()
        .position(|event| event.sequence() == actualization.sequence())
        .expect("the selected actualization belongs to this trace");
    let admitted_units = match actualization.dimension() {
        PhysicalResidencyDimension::MetadataBytes => actualization.actual_units(),
        _ => actualization.requested_units(),
    };
    let admitted = events[..actualization_index].iter().any(|event| {
        event.kind() == Kind::Admission
            && event.dimension() == actualization.dimension()
            && event.scope() == actualization.scope()
            && event.actual_units() == admitted_units
    });
    if !admitted {
        panic!("MUTANT_PREDICATE:{mutation_predicate}");
    }
}

fn assert_ordered_process_trace(events: &[Event]) {
    assert!(!events.is_empty());
    for (index, event) in events.iter().copied().enumerate() {
        assert_eq!(event.sequence(), index as u64 + 1);
        assert_eq!(event.process(), std::process::id());
        if event.kind() != Kind::Actualization {
            assert_ne!(
                event.requested_units(),
                0,
                "zero-unit accounting events are not physical transitions"
            );
        }
    }
    assert!(events.iter().any(|event| {
        event.kind() == Kind::Actualization
            && event.dimension() == PhysicalResidencyDimension::MetadataBytes
            && event.scope().is_none()
            && event.operation().is_none()
            && event.actual_units() >= event.requested_units()
    }));
}
