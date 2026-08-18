use std::sync::{Arc, Mutex};

use crate::history::data::RelationalCommitReceipt;
use crate::runtime::RelationalRuntime;
use crate::visibility::exact_commit_snapshot::{
    open_retained_commit_snapshot, RelationalRetainedCommitSnapshotDenial,
};
use crate::visibility::runtime_authority::RelationalVisibilityRuntimeAuthority;

use super::{admit_execution_basis, RelationalExecutionBasisDenial, RelationalExecutionBasisLease};

#[derive(Clone, Debug)]
pub struct RelationalApplicationCommitBasisSource {
    runtime: RelationalVisibilityRuntimeAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationalApplicationCommitBasisDenial {
    RetainedCommit(RelationalRetainedCommitSnapshotDenial),
    ExecutionBasis(RelationalExecutionBasisDenial),
}

impl RelationalApplicationCommitBasisSource {
    pub fn for_runtime(runtime: Arc<RelationalRuntime>) -> Self {
        Self {
            runtime: RelationalVisibilityRuntimeAuthority::immutable(runtime),
        }
    }

    pub fn for_shared_runtime(runtime: Arc<Mutex<RelationalRuntime>>) -> Self {
        Self {
            runtime: RelationalVisibilityRuntimeAuthority::shared(runtime),
        }
    }

    /// Validates one whole retained application commit and mints its execution
    /// lease in the same Relational runtime observation.
    pub fn admit_application_commit(
        &self,
        expected_runtime_instance_id: u64,
        commit: &RelationalCommitReceipt,
    ) -> Result<RelationalExecutionBasisLease, RelationalApplicationCommitBasisDenial> {
        self.admit_application_commit_observing(expected_runtime_instance_id, commit, || {})
    }

    fn admit_application_commit_observing(
        &self,
        expected_runtime_instance_id: u64,
        commit: &RelationalCommitReceipt,
        observe_validated_commit: impl FnOnce(),
    ) -> Result<RelationalExecutionBasisLease, RelationalApplicationCommitBasisDenial> {
        self.runtime.with_runtime(|runtime| {
            open_retained_commit_snapshot(runtime, expected_runtime_instance_id, commit)
                .map_err(RelationalApplicationCommitBasisDenial::RetainedCommit)?;
            observe_validated_commit();
            admit_execution_basis(runtime, &commit.branch_id, commit.version_id)
                .map_err(RelationalApplicationCommitBasisDenial::ExecutionBasis)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support::*;

    #[test]
    fn exact_validation_and_lease_mint_hold_one_shared_runtime_observation() {
        let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
        let (runtime_instance_id, commit) = {
            let mut locked = runtime.lock().expect("test runtime lock remains available");
            let outcome = create_entity_outcome(&mut locked, "atomic");
            (locked.runtime_instance_id(), outcome.commit.clone())
        };
        let source =
            RelationalApplicationCommitBasisSource::for_shared_runtime(Arc::clone(&runtime));

        let lease = source
            .admit_application_commit_observing(runtime_instance_id, &commit, || {
                assert!(
                    runtime.try_lock().is_err(),
                    "exact validation and lease mint must retain the same runtime lock"
                );
            })
            .expect("the exact retained commit admits one execution basis");

        assert_eq!(lease.identity().runtime_instance_id(), runtime_instance_id);
        assert_eq!(lease.version_id(), commit.version_id);
    }
}
