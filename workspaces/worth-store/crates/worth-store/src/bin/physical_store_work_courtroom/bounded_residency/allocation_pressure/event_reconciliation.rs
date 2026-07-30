use worth_store::physical_runtime::{
    AdmittedPhysicalRecordResidencyPolicy, PhysicalOperationAllocationScope as Scope,
    PhysicalResidencyAllocationSnapshot, PhysicalResidencyCounterSnapshot,
    PhysicalResidencyDimension as Dimension, PhysicalSpeculativeWorkKind as Speculation,
    ServingPhysicalRuntime,
};

use super::AllocationDimensionEvidence;

pub(super) const DECLARATIONS: [DimensionDeclaration; 19] = [
    declaration("total-bytes", Dimension::TotalBytes),
    declaration("resident-bytes", Dimension::ResidentBytes),
    declaration("metadata-bytes", Dimension::MetadataBytes),
    declaration("frame-entries", Dimension::FrameEntries),
    declaration("pinned-frames", Dimension::PinnedFrames),
    declaration("pin-leases", Dimension::PinLeases),
    declaration("dirty-frames", Dimension::DirtyFrames),
    declaration("dirty-replacement-bytes", Dimension::DirtyReplacementBytes),
    declaration("operation-bytes", Dimension::OperationBytes),
    declaration(
        "scope-foreground-read",
        Dimension::OperationScope(Scope::ForegroundRead),
    ),
    declaration(
        "scope-foreground-write",
        Dimension::OperationScope(Scope::ForegroundWrite),
    ),
    declaration("scope-recovery", Dimension::OperationScope(Scope::Recovery)),
    declaration("scope-scrub", Dimension::OperationScope(Scope::Scrub)),
    declaration(
        "scope-maintenance",
        Dimension::OperationScope(Scope::Maintenance),
    ),
    declaration(
        "scope-verification",
        Dimension::OperationScope(Scope::Verification),
    ),
    declaration("scope-blob", Dimension::OperationScope(Scope::Blob)),
    declaration(
        "speculative-read-ahead",
        Dimension::SpeculativeFrames(Speculation::ReadAhead),
    ),
    declaration(
        "speculative-prefetch",
        Dimension::SpeculativeFrames(Speculation::Prefetch),
    ),
    declaration(
        "speculative-write-behind",
        Dimension::SpeculativeFrames(Speculation::WriteBehind),
    ),
];

#[derive(Clone, Copy)]
pub(super) struct DimensionDeclaration {
    pub(super) name: &'static str,
    pub(super) dimension: Dimension,
}

#[derive(Clone, Copy)]
struct ReconciliationContext {
    counters: PhysicalResidencyCounterSnapshot,
    policy: AdmittedPhysicalRecordResidencyPolicy,
    allocations: PhysicalResidencyAllocationSnapshot,
}

#[derive(Clone, Copy)]
struct DimensionBounds {
    current: u64,
    peak: u64,
    limit: u64,
}

pub(super) fn reconcile(
    serving: &ServingPhysicalRuntime,
) -> Result<[AllocationDimensionEvidence; 19], String> {
    let observation = serving.residency_observation();
    let context = ReconciliationContext {
        counters: observation.counters(),
        policy: observation.admitted_policy(),
        allocations: observation.allocations(),
    };
    DECLARATIONS
        .into_iter()
        .map(|declaration| reconcile_dimension(declaration, context))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| "C.6 allocation declaration width changed".to_owned())
}

const fn declaration(name: &'static str, dimension: Dimension) -> DimensionDeclaration {
    DimensionDeclaration { name, dimension }
}

fn reconcile_dimension(
    declaration: DimensionDeclaration,
    context: ReconciliationContext,
) -> Result<AllocationDimensionEvidence, String> {
    let bounds = dimension_bounds(declaration.dimension, context);
    let events = context.allocations.for_dimension(declaration.dimension);
    let active = events.active_units();
    if events.attempts() != events.admissions().saturating_add(events.denials())
        || events.allocator_failures() != 0
        || events.admitted_units() < events.released_units()
        || active != bounds.current
        || bounds.current > bounds.peak
        || bounds.peak > bounds.limit
    {
        return Err(format!(
            "C.6 allocation dimension `{}` did not reconcile: \
             attempts={} admissions={} denials={} allocator_failures={} \
             admitted={} released={} active={} current={} peak={} limit={}",
            declaration.name,
            events.attempts(),
            events.admissions(),
            events.denials(),
            events.allocator_failures(),
            events.admitted_units(),
            events.released_units(),
            active,
            bounds.current,
            bounds.peak,
            bounds.limit,
        ));
    }
    Ok(AllocationDimensionEvidence {
        name: declaration.name,
        attempts: events.attempts(),
        admissions: events.admissions(),
        releases: events.releases(),
        denials: events.denials(),
        allocator_failures: events.allocator_failures(),
        admitted_units: events.admitted_units(),
        released_units: events.released_units(),
        denied_units: events.denied_units(),
        active_units: active,
        current_units: bounds.current,
        peak_units: bounds.peak,
        limit_units: bounds.limit,
    })
}

fn dimension_bounds(dimension: Dimension, context: ReconciliationContext) -> DimensionBounds {
    match dimension {
        Dimension::OperationScope(scope) => scope_bounds(scope, context),
        Dimension::SpeculativeFrames(kind) => speculative_bounds(kind, context),
        other => base_bounds(other, context),
    }
}

fn scope_bounds(scope: Scope, context: ReconciliationContext) -> DimensionBounds {
    DimensionBounds {
        current: context.counters.active_operation_bytes_for(scope),
        peak: context.counters.peak_operation_bytes_for(scope),
        limit: context.policy.scope_bytes(scope),
    }
}

fn speculative_bounds(kind: Speculation, context: ReconciliationContext) -> DimensionBounds {
    DimensionBounds {
        current: u64::from(context.counters.active_speculative_frames(kind)),
        peak: u64::from(context.counters.peak_speculative_frames(kind)),
        limit: u64::from(context.policy.speculative_frames(kind)),
    }
}

fn base_bounds(dimension: Dimension, context: ReconciliationContext) -> DimensionBounds {
    let counters = context.counters;
    let policy = context.policy;
    let (current, peak, limit) = match dimension {
        Dimension::TotalBytes => (
            counters.admitted_bytes(),
            counters.peak_admitted_bytes(),
            policy.total_bytes(),
        ),
        Dimension::ResidentBytes => (
            counters.resident_bytes(),
            counters.peak_resident_bytes(),
            policy.resident_bytes(),
        ),
        Dimension::MetadataBytes => (
            counters.metadata_bytes(),
            counters.peak_metadata_bytes(),
            policy.metadata_bytes(),
        ),
        Dimension::FrameEntries => (
            u64::from(counters.frame_entries()),
            u64::from(counters.peak_frame_entries()),
            u64::from(policy.frame_entries()),
        ),
        Dimension::PinnedFrames => (
            u64::from(counters.pinned_frames()),
            u64::from(counters.peak_pinned_frames()),
            u64::from(policy.pinned_frames()),
        ),
        Dimension::PinLeases => (
            u64::from(counters.pin_leases()),
            u64::from(counters.peak_pin_leases()),
            u64::from(policy.pin_leases()),
        ),
        Dimension::DirtyFrames => (
            u64::from(counters.dirty_frames()),
            u64::from(counters.peak_dirty_frames()),
            u64::from(policy.dirty_frames()),
        ),
        Dimension::DirtyReplacementBytes => (
            counters.dirty_replacement_bytes(),
            counters.peak_dirty_replacement_bytes(),
            policy.dirty_replacement_bytes(),
        ),
        Dimension::OperationBytes => (
            counters.active_operation_bytes(),
            counters.peak_operation_bytes(),
            policy.operation_bytes(),
        ),
        Dimension::OperationScope(_) | Dimension::SpeculativeFrames(_) => {
            unreachable!("specialized allocation dimensions are routed before base bounds")
        }
    };
    DimensionBounds {
        current,
        peak,
        limit,
    }
}
