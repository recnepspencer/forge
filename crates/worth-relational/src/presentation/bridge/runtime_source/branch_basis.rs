use crate::branch::{
    AdmittedRelationalBranchBasis, RelationalBranchBasisDenial, RelationalBranchBasisDescriptor,
    RelationalBranchIdentity,
};

use super::{
    RelationalBridgeBranchHeadLease, RelationalBridgeObservationLease,
    RuntimeBridgeRelationalSource,
};

impl RuntimeBridgeRelationalSource {
    /// Execute a read against one exact observation while its Bridge binding
    /// remains retained by the caller.
    #[doc(hidden)]
    pub fn with_retained_observation<T>(
        &self,
        snapshot: &worth_runtime_bridge::facade::TruthSnapshotIdentity,
        read: impl FnOnce(
            &crate::runtime::RelationalRuntime,
            &crate::mvcc::RelationalBranchObservation,
        ) -> T,
    ) -> Result<T, worth_runtime_bridge::facade::RelationalBridgeSourceError> {
        let observation = self.observation_bindings.resolve(snapshot)?;
        Ok(self
            .runtime
            .with_runtime(|runtime| read(runtime, &observation)))
    }

    /// Ask the Relational owner for one exact admitted branch basis.
    pub fn observe_branch_basis(
        &self,
        identity: &RelationalBranchIdentity,
    ) -> Result<
        (
            RelationalBranchBasisDescriptor,
            AdmittedRelationalBranchBasis,
        ),
        RelationalBranchBasisDenial,
    > {
        self.runtime
            .with_runtime(|runtime| runtime.observe_branch(identity))
    }

    /// Readmit a transported descriptor through the Relational owner.
    pub fn readmit_branch_basis(
        &self,
        descriptor: &RelationalBranchBasisDescriptor,
    ) -> Result<AdmittedRelationalBranchBasis, RelationalBranchBasisDenial> {
        self.runtime
            .with_runtime(|runtime| runtime.readmit_branch_basis(descriptor))
    }

    /// Retain a concrete admitted basis for Bridge snapshot reads.
    ///
    /// The adapter indexes the owner-issued observation; it never recreates
    /// authority from a branch name, version, snapshot identity, or commit.
    pub fn retain_branch_basis_for_bridge(
        &self,
        basis: &AdmittedRelationalBranchBasis,
    ) -> Result<RelationalBridgeObservationLease, RelationalBranchBasisDenial> {
        self.runtime
            .with_runtime(|runtime| self.retain_branch_basis_for_bridge_in_runtime(runtime, basis))
    }

    /// Retain a basis while the caller already holds this source's runtime
    /// owner closure. This avoids recursively locking a shared runtime during
    /// atomic publication.
    pub fn retain_branch_basis_for_bridge_in_runtime(
        &self,
        runtime: &crate::runtime::RelationalRuntime,
        basis: &AdmittedRelationalBranchBasis,
    ) -> Result<RelationalBridgeObservationLease, RelationalBranchBasisDenial> {
        if runtime.runtime_instance_id()
            != self.authoritative_source_profile().runtime_instance_id()
        {
            return Err(RelationalBranchBasisDenial::ForeignRuntime {
                expected_runtime_instance_id: self
                    .authoritative_source_profile()
                    .runtime_instance_id(),
                actual_runtime_instance_id: runtime.runtime_instance_id(),
            });
        }
        let retention = runtime.retain_component_basis(basis)?;
        let snapshot_id = runtime.visibility.allocate_snapshot_id();
        Ok(self
            .observation_bindings
            .insert(snapshot_id, basis.observation(), retention))
    }

    /// Bind one already-admitted basis as the exact Bridge head for its branch.
    ///
    /// The returned lease owns both the branch-head registration and the
    /// external observation retention. Dropping or releasing it removes the
    /// registration and terminates that retention exactly once.
    pub fn bind_branch_head_basis_for_bridge(
        &self,
        basis: &AdmittedRelationalBranchBasis,
    ) -> Result<RelationalBridgeBranchHeadLease, RelationalBranchBasisDenial> {
        let commit_id = basis
            .observation()
            .selected_root()
            .commit_id()
            .ok_or(RelationalBranchBasisDenial::UnavailableRetainedTarget)?;
        let branch_identity =
            worth_runtime_bridge::facade::TruthBranchIdentity::from_relational_branch_id(
                basis.identity().branch_id().0.clone(),
            );
        let observation = self.retain_branch_basis_for_bridge(basis)?;
        Ok(self
            .branch_head_bindings
            .insert(branch_identity, commit_id, observation))
    }

    /// Bind a branch head while the caller already owns the runtime closure.
    pub fn bind_branch_head_basis_for_bridge_in_runtime(
        &self,
        runtime: &crate::runtime::RelationalRuntime,
        basis: &AdmittedRelationalBranchBasis,
    ) -> Result<RelationalBridgeBranchHeadLease, RelationalBranchBasisDenial> {
        let commit_id = basis
            .observation()
            .selected_root()
            .commit_id()
            .ok_or(RelationalBranchBasisDenial::UnavailableRetainedTarget)?;
        let branch_identity =
            worth_runtime_bridge::facade::TruthBranchIdentity::from_relational_branch_id(
                basis.identity().branch_id().0.clone(),
            );
        let observation = self.retain_branch_basis_for_bridge_in_runtime(runtime, basis)?;
        Ok(self
            .branch_head_bindings
            .insert(branch_identity, commit_id, observation))
    }
}
