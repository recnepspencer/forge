use std::collections::BTreeSet;

use worth_store::physical_runtime::{
    PhysicalOperationAllocationScope as Scope, PhysicalResidencyAllocationBoundaryEvent,
    PhysicalResidencyAllocationBoundaryKind as Kind, PhysicalResidencyDimension as Dimension,
    PhysicalSpeculativeWorkKind as Speculation, ServingPhysicalRuntime,
};

use super::{
    event_reconciliation::DECLARATIONS, AllocationBoundaryEventEvidence,
    AllocationBoundaryTraceEvidence,
};

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

pub(super) fn reconcile(
    serving: &ServingPhysicalRuntime,
) -> Result<AllocationBoundaryTraceEvidence, String> {
    let aggregate = serving.residency_observation().allocations();
    let trace = serving
        .certification_physical_residency()
        .allocation_trace();
    if trace.store_identity() != serving.store_identity()
        || aggregate.store_identity() != serving.store_identity()
        || trace.pool_incarnation() != aggregate.pool_incarnation()
    {
        return Err("C.6 allocation trace identity did not match its aggregate owner".to_owned());
    }
    let process = std::process::id();
    let causal = serving.physical_work_observer().causal().records();
    if serving.physical_work_observer().causal().overflow() != 0 {
        return Err("C.6 allocation attribution outgrew the causal observer".to_owned());
    }
    let mut attributed = BTreeSet::new();
    let mut evidence = Vec::with_capacity(trace.event_count());
    for (index, event) in trace.events().enumerate() {
        let expected_sequence = u64::try_from(index)
            .map_err(|_| "C.6 allocation trace index exceeded u64".to_owned())?
            .checked_add(1)
            .ok_or_else(|| "C.6 allocation trace sequence exceeded u64".to_owned())?;
        if event.sequence() != expected_sequence || event.process() != process {
            return Err("C.6 allocation trace lost sequence or process identity".to_owned());
        }
        verify_event_semantics(event)?;
        if let Some(operation) = event.physical_operation() {
            if !attributed.insert(operation) {
                return Err(format!(
                    "C.6 allocation operation {operation} was actualized more than once"
                ));
            }
            verify_physical_operation(serving, &causal, operation)?;
        }
        evidence.push(lower_event(event));
    }
    for declaration in DECLARATIONS {
        reconcile_dimension(
            declaration.name,
            declaration.dimension,
            &evidence,
            aggregate.for_dimension(declaration.dimension),
        )?;
    }
    if attributed.is_empty() {
        return Err(
            "C.6 allocation trace attributed no materialization to physical work".to_owned(),
        );
    }
    Ok(AllocationBoundaryTraceEvidence {
        store: trace.store_identity().bytes(),
        pool_incarnation: trace.pool_incarnation(),
        event_count: evidence.len() as u64,
        process,
        attributed_actualizations: attributed.len() as u64,
        events: evidence,
    })
}

fn verify_event_semantics(event: PhysicalResidencyAllocationBoundaryEvent) -> Result<(), String> {
    let requested = event.requested_units();
    let actual = event.actual_units();
    match event.kind() {
        Kind::Admission | Kind::Release if requested == 0 || requested != actual => Err(format!(
            "C.6 allocation admission/release misstated requested or actual units: \
                 kind={:?} dimension={:?} scope={:?} requested={requested} actual={actual}",
            event.kind(),
            event.dimension(),
            event.scope(),
        )),
        Kind::Denial | Kind::AllocatorFailure if requested == 0 || actual != 0 => Err(format!(
            "C.6 allocation failure misstated requested or actual units: \
                 kind={:?} dimension={:?} scope={:?} requested={requested} actual={actual}",
            event.kind(),
            event.dimension(),
            event.scope(),
        )),
        Kind::Actualization => verify_actualization(event),
        Kind::Admission | Kind::Release | Kind::Denial | Kind::AllocatorFailure => Ok(()),
    }
}

fn verify_actualization(event: PhysicalResidencyAllocationBoundaryEvent) -> Result<(), String> {
    let requested = event.requested_units();
    let actual = event.actual_units();
    if requested == 0 || actual == 0 {
        return Err("C.6 allocation actualization omitted requested or actual units".to_owned());
    }
    if honest_actualization(
        event.dimension(),
        event.scope(),
        event.physical_operation().is_some(),
        requested,
        actual,
    ) {
        return Ok(());
    }
    Err(format!(
        "C.6 allocation actualization carried dishonest attribution: \
         dimension={:?} scope={:?} requested={} actual={} operation={:?}",
        event.dimension(),
        event.scope(),
        requested,
        actual,
        event.physical_operation(),
    ))
}

fn honest_actualization(
    dimension: Dimension,
    scope: Option<Scope>,
    has_operation: bool,
    requested: u64,
    actual: u64,
) -> bool {
    match (dimension, scope, has_operation) {
        (Dimension::MetadataBytes, None, false) => actual >= requested,
        (Dimension::ResidentBytes, Some(Scope::ForegroundRead), true)
        | (Dimension::ResidentBytes, Some(Scope::ForegroundWrite), _)
        | (Dimension::ResidentBytes, Some(Scope::Recovery), false)
        | (Dimension::DirtyReplacementBytes, Some(Scope::ForegroundWrite), false) => {
            actual <= requested
        }
        _ => false,
    }
}

fn verify_physical_operation(
    serving: &ServingPhysicalRuntime,
    causal: &[worth_store::physical_runtime::PhysicalWorkCausalRecord],
    operation: u64,
) -> Result<(), String> {
    let expected_store = serving.store_identity();
    let expected_runtime = serving.runtime_identity();
    let expected_generation = serving.residency_observation().store_generation();
    let matches = causal
        .iter()
        .filter(|record| {
            let identity = record.identity();
            identity.operation().get() == operation
                && identity.store() == expected_store
                && identity.runtime() == expected_runtime
                && identity.generation().lifecycle() == expected_generation
        })
        .count();
    if matches != 1 {
        return Err(format!(
            "C.6 allocation operation {operation} matched {matches} causal records"
        ));
    }
    Ok(())
}

fn reconcile_dimension(
    name: &'static str,
    dimension: Dimension,
    events: &[AllocationBoundaryEventEvidence],
    aggregate: worth_store::physical_runtime::PhysicalResidencyAllocationEventSnapshot,
) -> Result<(), String> {
    let mut traced = TraceCounters::default();
    for event in events
        .iter()
        .filter(|event| event.dimension == dimension_name(dimension))
    {
        apply_event(&mut traced, event);
    }
    let expected = TraceCounters {
        attempts: aggregate.attempts(),
        admissions: aggregate.admissions(),
        releases: aggregate.releases(),
        denials: aggregate.denials(),
        allocator_failures: aggregate.allocator_failures(),
        admitted_units: aggregate.admitted_units(),
        released_units: aggregate.released_units(),
        denied_units: aggregate.denied_units(),
    };
    if traced != expected {
        return Err(format!(
            "C.6 allocation trace dimension `{name}` did not reconcile with aggregate evidence"
        ));
    }
    Ok(())
}

fn apply_event(counters: &mut TraceCounters, event: &AllocationBoundaryEventEvidence) {
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
        _ => unreachable!("courtroom allocation event kinds are closed"),
    }
}

fn lower_event(event: PhysicalResidencyAllocationBoundaryEvent) -> AllocationBoundaryEventEvidence {
    AllocationBoundaryEventEvidence {
        sequence: event.sequence(),
        kind: kind_name(event.kind()),
        dimension: dimension_name(event.dimension()),
        scope: event.scope().map(scope_name),
        requested_units: event.requested_units(),
        actual_units: event.actual_units(),
        process: event.process(),
        physical_operation: event.physical_operation(),
    }
}

const fn kind_name(kind: Kind) -> &'static str {
    match kind {
        Kind::Admission => "admission",
        Kind::Release => "release",
        Kind::Denial => "denial",
        Kind::AllocatorFailure => "allocator-failure",
        Kind::Actualization => "actualization",
    }
}

const fn dimension_name(dimension: Dimension) -> &'static str {
    match dimension {
        Dimension::TotalBytes => "total-bytes",
        Dimension::ResidentBytes => "resident-bytes",
        Dimension::MetadataBytes => "metadata-bytes",
        Dimension::FrameEntries => "frame-entries",
        Dimension::PinnedFrames => "pinned-frames",
        Dimension::PinLeases => "pin-leases",
        Dimension::DirtyFrames => "dirty-frames",
        Dimension::DirtyReplacementBytes => "dirty-replacement-bytes",
        Dimension::OperationBytes => "operation-bytes",
        Dimension::OperationScope(scope) => scope_dimension_name(scope),
        Dimension::SpeculativeFrames(kind) => speculation_name(kind),
    }
}

const fn scope_dimension_name(scope: Scope) -> &'static str {
    match scope {
        Scope::ForegroundRead => "scope-foreground-read",
        Scope::ForegroundWrite => "scope-foreground-write",
        Scope::Recovery => "scope-recovery",
        Scope::Scrub => "scope-scrub",
        Scope::Maintenance => "scope-maintenance",
        Scope::Verification => "scope-verification",
        Scope::Blob => "scope-blob",
    }
}

const fn scope_name(scope: Scope) -> &'static str {
    match scope {
        Scope::ForegroundRead => "foreground-read",
        Scope::ForegroundWrite => "foreground-write",
        Scope::Recovery => "recovery",
        Scope::Scrub => "scrub",
        Scope::Maintenance => "maintenance",
        Scope::Verification => "verification",
        Scope::Blob => "blob",
    }
}

const fn speculation_name(kind: Speculation) -> &'static str {
    match kind {
        Speculation::ReadAhead => "speculative-read-ahead",
        Speculation::Prefetch => "speculative-prefetch",
        Speculation::WriteBehind => "speculative-write-behind",
    }
}

#[cfg(test)]
mod tests {
    use super::{honest_actualization, Dimension, Scope};

    #[test]
    fn foreground_write_accepts_causally_attributed_partial_residency() {
        assert!(honest_actualization(
            Dimension::ResidentBytes,
            Some(Scope::ForegroundWrite),
            true,
            16_384,
            2_600,
        ));
    }

    #[test]
    fn actualization_rejects_overuse_and_foreign_attribution() {
        assert!(!honest_actualization(
            Dimension::ResidentBytes,
            Some(Scope::ForegroundWrite),
            true,
            16_384,
            16_385,
        ));
        assert!(!honest_actualization(
            Dimension::ResidentBytes,
            Some(Scope::Blob),
            true,
            16_384,
            2_600,
        ));
        assert!(!honest_actualization(
            Dimension::DirtyReplacementBytes,
            Some(Scope::ForegroundWrite),
            true,
            16_384,
            2_600,
        ));
    }
}
