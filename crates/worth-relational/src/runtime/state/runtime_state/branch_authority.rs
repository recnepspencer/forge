use super::RelationalRuntime;
use crate::branch::{
    RelationalBranchIdentity, RelationalLegacyBranchBinding, RelationalLegacyBranchBindingDenial,
};
use crate::history::data::BranchId;
use crate::transactions::data::TransactionOptions;

impl RelationalRuntime {
    /// Observe every exact axis of one owner branch-reference cell. This is a
    /// read-only evidence surface; the returned state cannot mint authority or
    /// select a transaction target.
    pub fn branch_reference_state(
        &self,
        branch_id: &BranchId,
    ) -> Option<crate::branch::RelationalBranchReferenceState> {
        self.history
            .branch_cell(branch_id)
            .map(|cell| cell.evidence_state())
    }

    /// Return the owner-issued identity for one registered branch.
    ///
    /// The identity is descriptive; it carries runtime affinity so a value
    /// copied from another runtime cannot silently route a transaction here.
    pub fn branch_identity(
        &self,
        branch_id: &BranchId,
    ) -> Result<RelationalBranchIdentity, RelationalLegacyBranchBindingDenial> {
        let cell = self
            .history
            .branch_cell(branch_id)
            .ok_or_else(|| RelationalLegacyBranchBindingDenial::UnknownBranch(branch_id.clone()))?;
        if cell.identity().runtime_instance_id() != self.runtime_instance_id() {
            return Err(RelationalLegacyBranchBindingDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: cell.identity().runtime_instance_id(),
            });
        }
        Ok(cell.identity().clone())
    }

    /// Return the configured main-branch identity through the same owner door
    /// used by named branches.
    pub fn main_branch_identity(&self) -> RelationalBranchIdentity {
        self.branch_identity(&self.config.history.main_branch)
            .expect("configured main branch must remain registered")
    }

    /// Issue transaction options bound to the exact runtime-owned branch cell.
    /// The binding contains the concrete `worth-proof` proof and the branch's
    /// exact Foundational observation/version; callers cannot construct it.
    pub fn transaction_options_for(
        &self,
        identity: &RelationalBranchIdentity,
    ) -> Result<TransactionOptions, RelationalLegacyBranchBindingDenial> {
        let binding = self.legacy_branch_binding_for_identity(identity)?;
        Ok(TransactionOptions::from_owner_binding(binding))
    }

    /// Issue options for the explicitly named configured main branch through
    /// the same owner binding door used by every other branch.
    pub fn transaction_options_for_main(
        &self,
    ) -> Result<TransactionOptions, RelationalLegacyBranchBindingDenial> {
        let identity = self.main_branch_identity();
        self.transaction_options_for(&identity)
    }

    /// Resolve a descriptive branch name through the owner and issue the
    /// same runtime-affine options as `transaction_options_for`.
    pub fn owner_transaction_options_for_branch(
        &self,
        branch_id: &BranchId,
    ) -> Result<TransactionOptions, RelationalLegacyBranchBindingDenial> {
        let identity = self.branch_identity(branch_id)?;
        self.transaction_options_for(&identity)
    }

    pub(crate) fn legacy_branch_binding_for_identity(
        &self,
        identity: &RelationalBranchIdentity,
    ) -> Result<RelationalLegacyBranchBinding, RelationalLegacyBranchBindingDenial> {
        if identity.runtime_instance_id() != self.runtime_instance_id() {
            return Err(RelationalLegacyBranchBindingDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: identity.runtime_instance_id(),
            });
        }
        let cell = self
            .history
            .branch_cell(identity.branch_id())
            .ok_or_else(|| {
                RelationalLegacyBranchBindingDenial::UnknownBranch(identity.branch_id().clone())
            })?;
        if cell.identity() != identity {
            return Err(RelationalLegacyBranchBindingDenial::IdentityMismatch);
        }
        Ok(RelationalLegacyBranchBinding::new(
            cell.identity().clone(),
            cell.observation().clone(),
            cell.truth_version(),
        ))
    }

    pub(crate) fn legacy_branch_binding_for_merge_branch(
        &self,
        branch_id: &BranchId,
    ) -> Result<
        RelationalLegacyBranchBinding,
        crate::merge::data::RelationalMergeRequestBindingDenial,
    > {
        let identity = self
            .branch_identity(branch_id)
            .map_err(map_binding_denial)?;
        self.legacy_branch_binding_for_identity(&identity)
            .map_err(map_binding_denial)
    }

    pub(crate) fn legacy_branch_binding_is_current(
        &self,
        binding: &RelationalLegacyBranchBinding,
    ) -> bool {
        self.history
            .branch_cell(binding.identity().branch_id())
            .is_some_and(|cell| {
                cell.identity() == binding.identity()
                    && cell.observation() == binding.observation()
                    && cell.truth_version() == binding.truth_version()
            })
    }

    pub(crate) fn legacy_branch_binding_commit(
        &self,
        binding: &RelationalLegacyBranchBinding,
    ) -> Option<crate::history::RelationalCommitIdentity> {
        let cell = self.history.branch_cell(binding.identity().branch_id())?;
        if cell.identity() != binding.identity()
            || cell.observation() != binding.observation()
            || cell.truth_version() != binding.truth_version()
        {
            return None;
        }
        match binding.observation().target() {
            worth_foundational::FoundationalBranchTarget::Empty => None,
            worth_foundational::FoundationalBranchTarget::Basis(target) => self
                .history
                .commit_catalog
                .get(crate::history::data::CommitId(target.commit_id()))
                .map(|artifact| artifact.identity().clone()),
        }
    }

    /// Returns the exact branch-local version represented by an owner-issued
    /// binding. An empty branch has a real local basis (version zero); it must
    /// never borrow the runtime-wide current version as a fallback.
    pub(crate) fn legacy_branch_binding_version(
        &self,
        binding: &RelationalLegacyBranchBinding,
    ) -> Option<crate::identity::data::VersionId> {
        let cell = self.history.branch_cell(binding.identity().branch_id())?;
        if cell.identity() != binding.identity()
            || cell.observation() != binding.observation()
            || cell.truth_version() != binding.truth_version()
        {
            return None;
        }
        match binding.observation().target() {
            worth_foundational::FoundationalBranchTarget::Empty => {
                Some(crate::identity::data::VersionId(0))
            }
            worth_foundational::FoundationalBranchTarget::Basis(target) => self
                .history
                .commit_catalog
                .get(crate::history::data::CommitId(target.commit_id()))
                .map(|artifact| artifact.identity().version_id()),
        }
    }

    pub(crate) fn runtime_instance_id(&self) -> u64 {
        self.services.runtime_instance_id()
    }
}

fn map_binding_denial(
    denial: RelationalLegacyBranchBindingDenial,
) -> crate::merge::data::RelationalMergeRequestBindingDenial {
    match denial {
        RelationalLegacyBranchBindingDenial::ForeignRuntime {
            expected_runtime_instance_id,
            actual_runtime_instance_id,
        } => crate::merge::data::RelationalMergeRequestBindingDenial::ForeignRuntime {
            expected_runtime_instance_id,
            actual_runtime_instance_id,
        },
        RelationalLegacyBranchBindingDenial::UnknownBranch(branch) => {
            crate::merge::data::RelationalMergeRequestBindingDenial::UnknownBranch(branch)
        }
        RelationalLegacyBranchBindingDenial::IdentityMismatch => {
            crate::merge::data::RelationalMergeRequestBindingDenial::IdentityMismatch
        }
    }
}
