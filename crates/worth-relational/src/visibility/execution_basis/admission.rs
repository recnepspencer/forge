use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotReadPolicy};
use crate::visibility::cache_state::retained_state;

use super::{
    RelationalExecutionBasisCounters, RelationalExecutionBasisDenial,
    RelationalExecutionBasisDenialKind, RelationalExecutionBasisLease,
};

pub(crate) fn admit_execution_basis(
    runtime: &RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
    version_id: VersionId,
) -> Result<RelationalExecutionBasisLease, RelationalExecutionBasisDenial> {
    let mut counters = RelationalExecutionBasisCounters::default();
    counters.checked_version_availability();
    if retained_state(runtime, version_id).is_none() {
        return Err(denial(
            RelationalExecutionBasisDenialKind::VersionUnavailable,
            "Relational execution basis requires a version reconstructible by this runtime",
            &counters,
        ));
    }
    counters.checked_branch_affinity();
    if crate::visibility::branch_scope::authoritative_branch_for_version(runtime, version_id)
        != *branch_id
    {
        return Err(denial(
            RelationalExecutionBasisDenialKind::BranchMismatch,
            "Relational execution basis branch does not own the requested version",
            &counters,
        ));
    }

    let handle = SnapshotHandle {
        runtime_instance_id: runtime.runtime_instance_id(),
        branch_id: branch_id.clone(),
        snapshot_id: runtime.visibility.allocate_snapshot_id(),
        version_id,
        read_policy: SnapshotReadPolicy::ImmutablePinned,
    };
    counters.allocated_snapshot_identity();
    let (lease_ordinal, registry) = runtime.visibility.admit_execution_basis(
        handle.snapshot_id,
        handle.branch_id.clone(),
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
