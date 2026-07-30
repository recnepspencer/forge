use super::{verify_allocation, verify_materialization, DIMENSION_LIMITS};
use crate::courtroom_campaign::bounded_residency_siege::{
    protocol::{
        BoundedResidencyAllocationBoundaryObservation,
        BoundedResidencyAllocationDimensionObservation, BoundedResidencyAllocationObservation,
        BoundedResidencyAllocationTraceObservation, BoundedResidencyScopeObservation,
    },
    world::OPERATION_BYTES,
};

const STORE: [u8; 16] = [0x11; 16];
const PROCESS: u32 = 41;
const LAST_WORK_OPERATION: u64 = 100;

#[test]
fn exact_allocation_evidence_is_accepted() {
    assert!(verify(&accepted()).is_ok());
}

#[test]
fn scope_oracle_rejects_cross_scope_or_global_envelope_drift() {
    let mut missing_scope = accepted();
    missing_scope.scopes.exact_scope_denials = 6;
    assert!(verify(&missing_scope).is_err());

    let mut global = accepted();
    global.scopes.global_denial_current -= 1;
    assert!(verify(&global).is_err());

    let mut skipped = accepted();
    let blob = skipped
        .dimensions
        .iter_mut()
        .find(|dimension| dimension.name == "scope-blob")
        .unwrap();
    blob.attempts = blob.admissions;
    blob.denials = 0;
    blob.denied_units = 0;
    assert!(verify(&skipped).is_err());

    let mut unsaturated = accepted();
    let scrub = unsaturated
        .dimensions
        .iter_mut()
        .find(|dimension| dimension.name == "scope-scrub")
        .unwrap();
    scrub.peak_units -= 1;
    assert!(verify(&unsaturated).is_err());

    let mut foreign_limits = accepted();
    set_dimension_capacity(&mut foreign_limits, "scope-recovery", 1_835_008);
    set_dimension_capacity(&mut foreign_limits, "scope-scrub", 2_359_296);
    assert_denied_at(&foreign_limits, "scope-recovery");
}

#[test]
fn dimension_oracle_rejects_duplicates_failures_and_counter_drift() {
    let mut duplicate = accepted();
    duplicate.dimensions[1].name = duplicate.dimensions[0].name;
    assert!(verify(&duplicate).is_err());

    let mut allocator_failure = accepted();
    allocator_failure.dimensions[0].allocator_failures = 1;
    assert!(verify(&allocator_failure).is_err());

    let mut active = accepted();
    active.dimensions[0].active_units = 1;
    assert!(verify(&active).is_err());

    let mut limit = accepted();
    limit.dimensions[0].limit_units += 1;
    assert!(verify(&limit).is_err());
}

#[test]
fn trace_oracle_rejects_wrong_scope_units_process_operation_and_omission() {
    let mut wrong_scope = accepted();
    resident_materialization(&mut wrong_scope).scope = Some("blob");
    assert_denied_at(&wrong_scope, "false attribution");

    let mut swapped_units = accepted();
    let materialization = resident_materialization(&mut swapped_units);
    materialization.requested_units += 1;
    std::mem::swap(
        &mut materialization.requested_units,
        &mut materialization.actual_units,
    );
    assert_denied_at(&swapped_units, "transposed requested and actual");

    let mut wrong_process = accepted();
    wrong_process.trace.events[0].process += 1;
    assert_denied_at(&wrong_process, "sequence or process identity");

    let mut wrong_operation = accepted();
    resident_materialization(&mut wrong_operation).physical_operation =
        Some(LAST_WORK_OPERATION + 1);
    assert_denied_at(&wrong_operation, "foreign or duplicate operation");

    let mut omitted = accepted();
    let omitted_index = omitted
        .trace
        .events
        .iter()
        .position(|event| {
            event.kind == "actualization" && event.dimension == "dirty-replacement-bytes"
        })
        .unwrap();
    let mut events = omitted.trace.events.into_vec();
    events.remove(omitted_index);
    for (index, event) in events.iter_mut().enumerate() {
        event.sequence = index as u64 + 1;
    }
    omitted.trace.event_count -= 1;
    omitted.trace.events = events.into_boxed_slice();
    assert_denied_at(
        &omitted,
        "dirty-replacement-bytes` aggregate or materialization",
    );
}

#[test]
fn trace_oracle_rejects_materialization_before_matching_admission() {
    for dimension in [
        "metadata-bytes",
        "resident-bytes",
        "dirty-replacement-bytes",
    ] {
        let mut reordered = accepted();
        move_actualization_before_admission(&mut reordered, dimension);
        assert_denied_at(
            &reordered,
            &format!("`{dimension}` materialized before matching admission"),
        );
    }
}

#[test]
fn recovery_materialization_requires_explicit_pre_work_operation_absence() {
    let event = BoundedResidencyAllocationBoundaryObservation {
        sequence: 1,
        kind: "actualization",
        dimension: "resident-bytes",
        scope: Some("recovery"),
        requested_units: 74,
        actual_units: 74,
        process: PROCESS,
        physical_operation: None,
    };
    assert!(verify_materialization(&event).is_ok());
    assert!(
        verify_materialization(&BoundedResidencyAllocationBoundaryObservation {
            physical_operation: Some(1),
            ..event
        })
        .is_err()
    );
}

#[test]
fn foreground_write_materialization_accepts_pre_work_and_causal_attribution() {
    let event = BoundedResidencyAllocationBoundaryObservation {
        sequence: 1,
        kind: "actualization",
        dimension: "resident-bytes",
        scope: Some("foreground-write"),
        requested_units: 74,
        actual_units: 73,
        process: PROCESS,
        physical_operation: None,
    };
    assert!(verify_materialization(&event).is_ok());
    assert!(
        verify_materialization(&BoundedResidencyAllocationBoundaryObservation {
            physical_operation: Some(1),
            ..event
        })
        .is_ok()
    );
}

fn accepted() -> BoundedResidencyAllocationObservation {
    let dimensions = std::array::from_fn(|index| {
        let (name, limit) = DIMENSION_LIMITS[index];
        let scope = name.starts_with("scope-");
        BoundedResidencyAllocationDimensionObservation {
            name,
            attempts: if scope { 2 } else { 1 },
            admissions: 1,
            releases: 1,
            denials: u64::from(scope),
            allocator_failures: 0,
            admitted_units: limit,
            released_units: limit,
            denied_units: u64::from(scope),
            active_units: 0,
            current_units: 0,
            peak_units: limit,
            limit_units: limit,
        }
    });
    let mut events = Vec::new();
    for dimension in &dimensions {
        let scope = event_scope(dimension.name);
        push_event(
            &mut events,
            "admission",
            dimension.name,
            scope,
            dimension.admitted_units,
            dimension.admitted_units,
            None,
        );
        if matches!(
            dimension.name,
            "metadata-bytes" | "resident-bytes" | "dirty-replacement-bytes"
        ) {
            push_event(
                &mut events,
                "actualization",
                dimension.name,
                scope,
                dimension.admitted_units,
                dimension.admitted_units,
                (dimension.name == "resident-bytes").then_some(10),
            );
        }
        push_event(
            &mut events,
            "release",
            dimension.name,
            scope,
            dimension.released_units,
            dimension.released_units,
            None,
        );
        if dimension.denials != 0 {
            push_event(
                &mut events,
                "denial",
                dimension.name,
                scope,
                dimension.denied_units,
                0,
                None,
            );
        }
    }
    BoundedResidencyAllocationObservation {
        scopes: BoundedResidencyScopeObservation {
            admitted_scopes: 7,
            exact_scope_denials: 7,
            global_envelope_denied: true,
            global_denial_requested: 1,
            global_denial_current: OPERATION_BYTES,
            global_denial_limit: OPERATION_BYTES,
            peak_operation_bytes: OPERATION_BYTES,
            terminal_operation_bytes: 0,
            all_effect_free: true,
        },
        dimensions,
        trace: BoundedResidencyAllocationTraceObservation {
            store: STORE,
            pool_incarnation: 7,
            event_count: events.len() as u64,
            process: PROCESS,
            attributed_actualizations: 1,
            events: events.into_boxed_slice(),
        },
    }
}

fn set_dimension_capacity(
    allocation: &mut BoundedResidencyAllocationObservation,
    name: &str,
    capacity: u64,
) {
    let dimension = allocation
        .dimensions
        .iter_mut()
        .find(|dimension| dimension.name == name)
        .expect("the accepted allocation world contains every scope");
    dimension.admitted_units = capacity;
    dimension.released_units = capacity;
    dimension.peak_units = capacity;
    dimension.limit_units = capacity;
}

fn verify(allocation: &BoundedResidencyAllocationObservation) -> Result<(), String> {
    verify_allocation(allocation, STORE, PROCESS, LAST_WORK_OPERATION)
}

fn assert_denied_at(allocation: &BoundedResidencyAllocationObservation, predicate: &str) {
    let denial = verify(allocation).expect_err("hostile allocation evidence must be rejected");
    assert!(
        denial.contains(predicate),
        "expected `{predicate}` denial, observed `{denial}`"
    );
}

fn push_event(
    events: &mut Vec<BoundedResidencyAllocationBoundaryObservation>,
    kind: &'static str,
    dimension: &'static str,
    scope: Option<&'static str>,
    requested_units: u64,
    actual_units: u64,
    physical_operation: Option<u64>,
) {
    events.push(BoundedResidencyAllocationBoundaryObservation {
        sequence: events.len() as u64 + 1,
        kind,
        dimension,
        scope,
        requested_units,
        actual_units,
        process: PROCESS,
        physical_operation,
    });
}

fn event_scope(dimension: &'static str) -> Option<&'static str> {
    match dimension {
        "resident-bytes" | "frame-entries" | "operation-bytes" => Some("foreground-read"),
        "dirty-replacement-bytes" => Some("foreground-write"),
        "scope-foreground-read" => Some("foreground-read"),
        "scope-foreground-write" => Some("foreground-write"),
        "scope-recovery" => Some("recovery"),
        "scope-scrub" => Some("scrub"),
        "scope-maintenance" => Some("maintenance"),
        "scope-verification" => Some("verification"),
        "scope-blob" => Some("blob"),
        _ => None,
    }
}

fn resident_materialization(
    allocation: &mut BoundedResidencyAllocationObservation,
) -> &mut BoundedResidencyAllocationBoundaryObservation {
    allocation
        .trace
        .events
        .iter_mut()
        .find(|event| event.kind == "actualization" && event.dimension == "resident-bytes")
        .unwrap()
}

fn move_actualization_before_admission(
    allocation: &mut BoundedResidencyAllocationObservation,
    dimension: &str,
) {
    let mut events = allocation.trace.events.to_vec();
    let admission = events
        .iter()
        .position(|event| event.kind == "admission" && event.dimension == dimension)
        .expect("the accepted trace contains the materialized dimension admission");
    let actualization = events
        .iter()
        .position(|event| event.kind == "actualization" && event.dimension == dimension)
        .expect("the accepted trace contains the dimension actualization");
    assert!(admission < actualization);
    let event = events.remove(actualization);
    events.insert(admission, event);
    for (index, event) in events.iter_mut().enumerate() {
        event.sequence = index as u64 + 1;
    }
    allocation.trace.events = events.into_boxed_slice();
}
