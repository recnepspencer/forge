use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotReadPolicy};
use crate::visibility::cache_state::reconstruct_state;

use super::{
    RelationalExecutionBasisCounters, RelationalExecutionBasisDenial,
    RelationalExecutionBasisDenialKind, RelationalExecutionBasisLease,
};

pub(crate) fn admit_execution_basis(
    runtime: &RelationalRuntime,
    version_id: VersionId,
) -> Result<RelationalExecutionBasisLease, RelationalExecutionBasisDenial> {
    let mut counters = RelationalExecutionBasisCounters::default();
    counters.checked_version_availability();
    if reconstruct_state(runtime, version_id, false).is_none() {
        return Err(denial(
            RelationalExecutionBasisDenialKind::VersionUnavailable,
            "Relational execution basis requires a version reconstructible by this runtime",
            &counters,
        ));
    }

    let handle = SnapshotHandle {
        runtime_instance_id: runtime.runtime_instance_id(),
        snapshot_id: runtime.visibility.allocate_snapshot_id(),
        version_id,
        read_policy: SnapshotReadPolicy::ImmutablePinned,
    };
    counters.allocated_snapshot_identity();
    let (lease_ordinal, registry) = runtime.visibility.admit_execution_basis(
        handle.snapshot_id,
        handle.version_id,
        handle.read_policy,
    );
    counters.inserted_lease_registry_entry();
    let current_at_admission = version_id == runtime.current_version_id();
    Ok(RelationalExecutionBasisLease::new(
        handle,
        current_at_admission,
        lease_ordinal,
        registry,
        counters,
    ))
}

fn denial(
    kind: RelationalExecutionBasisDenialKind,
    detail: &'static str,
    counters: &RelationalExecutionBasisCounters,
) -> RelationalExecutionBasisDenial {
    RelationalExecutionBasisDenial::new(kind, detail, counters.clone())
}
