use std::collections::{BTreeMap, BTreeSet};

#[path = "allocation/materialization_order.rs"]
mod materialization_order;

use super::super::{
    protocol::{
        BoundedResidencyAllocationBoundaryObservation,
        BoundedResidencyAllocationDimensionObservation, BoundedResidencyAllocationObservation,
    },
    world::{
        BLOB_SCOPE_BYTES, DIRTY_FRAMES, DIRTY_REPLACEMENT_BYTES, FOREGROUND_READ_SCOPE_BYTES,
        FOREGROUND_WRITE_SCOPE_BYTES, FRAME_ENTRIES, MAINTENANCE_SCOPE_BYTES, METADATA_BYTES,
        OPERATION_BYTES, PINNED_FRAMES, PIN_LEASES, PREFETCH_FRAMES, READ_AHEAD_FRAMES,
        RECOVERY_SCOPE_BYTES, RESIDENT_BYTES, SCRUB_SCOPE_BYTES, TOTAL_BYTES,
        VERIFICATION_SCOPE_BYTES, WRITE_BEHIND_FRAMES,
    },
};

const DIMENSION_LIMITS: [(&str, u64); 19] = [
    ("total-bytes", TOTAL_BYTES),
    ("resident-bytes", RESIDENT_BYTES),
    ("metadata-bytes", METADATA_BYTES),
    ("frame-entries", FRAME_ENTRIES as u64),
    ("pinned-frames", PINNED_FRAMES as u64),
    ("pin-leases", PIN_LEASES as u64),
    ("dirty-frames", DIRTY_FRAMES as u64),
    ("dirty-replacement-bytes", DIRTY_REPLACEMENT_BYTES),
    ("operation-bytes", OPERATION_BYTES),
    ("scope-foreground-read", FOREGROUND_READ_SCOPE_BYTES),
    ("scope-foreground-write", FOREGROUND_WRITE_SCOPE_BYTES),
    ("scope-recovery", RECOVERY_SCOPE_BYTES),
    ("scope-scrub", SCRUB_SCOPE_BYTES),
    ("scope-maintenance", MAINTENANCE_SCOPE_BYTES),
    ("scope-verification", VERIFICATION_SCOPE_BYTES),
    ("scope-blob", BLOB_SCOPE_BYTES),
    ("speculative-read-ahead", READ_AHEAD_FRAMES as u64),
    ("speculative-prefetch", PREFETCH_FRAMES as u64),
    ("speculative-write-behind", WRITE_BEHIND_FRAMES as u64),
];

pub(super) fn verify_allocation(
    allocation: &BoundedResidencyAllocationObservation,
    expected_store: [u8; 16],
    expected_process: u32,
    exclusive_operation_limit: u64,
) -> Result<(), String> {
    verify_scopes(allocation)?;
    let mut names = BTreeSet::new();
    for dimension in allocation.dimensions.iter().copied() {
        if !names.insert(dimension.name) {
            return Err(format!(
                "Courtroom C duplicated allocation dimension `{}`",
                dimension.name
            ));
        }
        verify_dimension(dimension)?;
    }
    if names.len() != DIMENSION_LIMITS.len() {
        return Err("Courtroom C omitted an allocation dimension".to_owned());
    }
    verify_trace(
        allocation,
        expected_store,
        expected_process,
        exclusive_operation_limit,
    )
}

fn verify_scopes(allocation: &BoundedResidencyAllocationObservation) -> Result<(), String> {
    let scopes = allocation.scopes;
    let operation = allocation
        .dimensions
        .iter()
        .find(|dimension| dimension.name == "operation-bytes")
        .ok_or_else(|| "Courtroom C omitted operation-byte allocation evidence".to_owned())?;
    let named_scopes = allocation
        .dimensions
        .iter()
        .filter(|dimension| dimension.name.starts_with("scope-"))
        .collect::<Vec<_>>();
    if scopes.admitted_scopes != 7
        || scopes.exact_scope_denials != 7
        || !scopes.global_envelope_denied
        || scopes.global_denial_requested != 1
        || scopes.global_denial_current != OPERATION_BYTES
        || scopes.global_denial_limit != OPERATION_BYTES
        || scopes.peak_operation_bytes != OPERATION_BYTES
        || scopes.terminal_operation_bytes != 0
        || !scopes.all_effect_free
        || operation.peak_units != scopes.peak_operation_bytes
        || operation.active_units != scopes.terminal_operation_bytes
        || named_scopes.len() != 7
        || named_scopes.iter().any(|dimension| {
            dimension.peak_units != dimension.limit_units
                || dimension.denials != 1
                || dimension.denied_units != 1
        })
    {
        return Err("Courtroom C did not prove exact operation-scope isolation".to_owned());
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct TraceCounters {
    attempts: u64,
    admissions: u64,
    releases: u64,
    denials: u64,
    allocator_failures: u64,
    admitted_units: u64,
    released_units: u64,
    denied_units: u64,
}

fn verify_trace(
    allocation: &BoundedResidencyAllocationObservation,
    expected_store: [u8; 16],
    expected_process: u32,
    exclusive_operation_limit: u64,
) -> Result<(), String> {
    let trace = &allocation.trace;
    if trace.store != expected_store
        || trace.pool_incarnation == 0
        || trace.process != expected_process
        || usize::try_from(trace.event_count).ok() != Some(trace.events.len())
    {
        return Err("Courtroom C allocation trace lost its owner or declared extent".to_owned());
    }
    let mut attributed = BTreeSet::new();
    let mut actualizations = BTreeMap::<&str, u64>::new();
    for (index, event) in trace.events.iter().enumerate() {
        let expected_sequence = u64::try_from(index)
            .ok()
            .and_then(|index| index.checked_add(1));
        if Some(event.sequence) != expected_sequence || event.process != expected_process {
            return Err(
                "Courtroom C allocation trace lost event sequence or process identity".to_owned(),
            );
        }
        verify_boundary(event)?;
        if event.kind == "actualization" {
            *actualizations.entry(event.dimension).or_default() += 1;
        }
        if let Some(operation) = event.physical_operation {
            if operation >= exclusive_operation_limit || !attributed.insert(operation) {
                return Err(
                    "Courtroom C allocation trace carried a foreign or duplicate operation"
                        .to_owned(),
                );
            }
        }
    }
    if attributed.is_empty()
        || trace.attributed_actualizations != attributed.len() as u64
        || actualizations.get("metadata-bytes").copied().unwrap_or(0) == 0
    {
        return Err("Courtroom C allocation trace omitted materialization attribution".to_owned());
    }
    materialization_order::verify(&trace.events)?;
    for dimension in &allocation.dimensions {
        verify_trace_dimension(
            dimension,
            &trace.events,
            actualizations.get(dimension.name).copied().unwrap_or(0),
        )?;
    }
    Ok(())
}

fn verify_boundary(event: &BoundedResidencyAllocationBoundaryObservation) -> Result<(), String> {
    if !DIMENSION_LIMITS
        .iter()
        .any(|(name, _)| *name == event.dimension)
    {
        return Err(format!(
            "Courtroom C allocation trace named foreign dimension `{}`",
            event.dimension
        ));
    }
    verify_boundary_scope(event)?;
    match event.kind {
        "admission" | "release"
            if event.requested_units > 0 && event.requested_units == event.actual_units =>
        {
            Ok(())
        }
        "denial" | "allocator-failure" if event.requested_units > 0 && event.actual_units == 0 => {
            Ok(())
        }
        "actualization" => verify_materialization(event),
        _ => Err("Courtroom C allocation event misstated requested or actual units".to_owned()),
    }
}

fn verify_boundary_scope(
    event: &BoundedResidencyAllocationBoundaryObservation,
) -> Result<(), String> {
    let expected_scope = event
        .dimension
        .strip_prefix("scope-")
        .or(match event.dimension {
            "dirty-replacement-bytes" => Some("foreground-write"),
            _ => None,
        });
    if expected_scope.is_some() && event.scope != expected_scope {
        return Err("Courtroom C allocation event carried the wrong semantic scope".to_owned());
    }
    if matches!(
        event.dimension,
        "resident-bytes" | "frame-entries" | "operation-bytes"
    ) && event.scope.is_none()
    {
        return Err("Courtroom C scoped allocation event omitted its scope".to_owned());
    }
    if event.dimension == "metadata-bytes" && event.scope.is_some() {
        return Err("Courtroom C metadata allocation invented an operation scope".to_owned());
    }
    Ok(())
}

fn verify_materialization(
    event: &BoundedResidencyAllocationBoundaryObservation,
) -> Result<(), String> {
    if event.requested_units == 0 || event.actual_units == 0 {
        return Err("Courtroom C allocation materialization reported zero units".to_owned());
    }
    let unit_order_is_valid = match (event.dimension, event.scope, event.physical_operation) {
        ("metadata-bytes", None, None) => event.actual_units >= event.requested_units,
        ("resident-bytes", Some("foreground-read"), Some(_))
        | ("resident-bytes", Some("foreground-write"), _)
        | ("resident-bytes", Some("recovery"), None)
        | ("dirty-replacement-bytes", Some("foreground-write"), None) => {
            event.actual_units <= event.requested_units
        }
        _ => {
            return Err(
                "Courtroom C allocation materialization carried false attribution".to_owned(),
            )
        }
    };
    if !unit_order_is_valid {
        return Err(
            "Courtroom C allocation materialization transposed requested and actual units"
                .to_owned(),
        );
    }
    Ok(())
}

fn verify_trace_dimension(
    dimension: &BoundedResidencyAllocationDimensionObservation,
    events: &[BoundedResidencyAllocationBoundaryObservation],
    actualizations: u64,
) -> Result<(), String> {
    let mut traced = TraceCounters::default();
    for event in events
        .iter()
        .filter(|event| event.dimension == dimension.name)
    {
        apply_trace_event(&mut traced, event);
    }
    let aggregate = TraceCounters {
        attempts: dimension.attempts,
        admissions: dimension.admissions,
        releases: dimension.releases,
        denials: dimension.denials,
        allocator_failures: dimension.allocator_failures,
        admitted_units: dimension.admitted_units,
        released_units: dimension.released_units,
        denied_units: dimension.denied_units,
    };
    let materialized_dimension = matches!(
        dimension.name,
        "metadata-bytes" | "resident-bytes" | "dirty-replacement-bytes"
    );
    if traced != aggregate
        || (materialized_dimension && actualizations != dimension.admissions)
        || (!materialized_dimension && actualizations != 0)
    {
        return Err(format!(
            "Courtroom C allocation trace failed `{}` aggregate or materialization reconciliation",
            dimension.name
        ));
    }
    Ok(())
}

fn apply_trace_event(
    counters: &mut TraceCounters,
    event: &BoundedResidencyAllocationBoundaryObservation,
) {
    match event.kind {
        "admission" => {
            counters.attempts += 1;
            counters.admissions += 1;
            counters.admitted_units = counters.admitted_units.saturating_add(event.actual_units);
        }
        "release" => {
            counters.releases += 1;
            counters.released_units = counters.released_units.saturating_add(event.actual_units);
        }
        "denial" => {
            counters.attempts += 1;
            counters.denials += 1;
            counters.denied_units = counters.denied_units.saturating_add(event.requested_units);
        }
        "allocator-failure" => {
            counters.allocator_failures += 1;
            counters.releases += 1;
            counters.released_units = counters
                .released_units
                .saturating_add(event.requested_units);
        }
        "actualization" => {}
        _ => unreachable!("allocation boundary kinds are parser-closed"),
    }
}

fn verify_dimension(
    dimension: BoundedResidencyAllocationDimensionObservation,
) -> Result<(), String> {
    let expected_limit = DIMENSION_LIMITS
        .iter()
        .find_map(|(name, limit)| (*name == dimension.name).then_some(*limit))
        .ok_or_else(|| {
            format!(
                "Courtroom C reported foreign allocation dimension `{}`",
                dimension.name
            )
        })?;
    if dimension.limit_units != expected_limit
        || dimension.attempts != dimension.admissions.saturating_add(dimension.denials)
        || dimension.allocator_failures != 0
        || dimension.admitted_units < dimension.released_units
        || dimension.active_units
            != dimension
                .admitted_units
                .saturating_sub(dimension.released_units)
        || dimension.active_units != dimension.current_units
        || dimension.current_units > dimension.peak_units
        || dimension.peak_units > dimension.limit_units
        || (dimension.admissions == 0 && dimension.admitted_units != 0)
        || (dimension.releases == 0 && dimension.released_units != 0)
        || (dimension.denials == 0 && dimension.denied_units != 0)
    {
        return Err(format!(
            "Courtroom C allocation dimension `{}` failed independent reconciliation",
            dimension.name
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "allocation/tests.rs"]
mod tests;
