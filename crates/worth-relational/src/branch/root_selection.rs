use super::RelationalLegacyBranchBinding;
use super::{RelationalBranchRoot, RelationalBranchRootState, RelationalBranchTarget};
use crate::history::data::{BranchId, CommitId};
use crate::identity::data::VersionId;
use crate::storage::overlay::{PartitionAccess, PartitionState};
use crate::transactions::data::CommitPreparationError;
use worth_foundational::FoundationalBranchTarget;

#[derive(Debug, Clone)]
pub(crate) enum SelectedRelationalBranchState {
    Empty(RelationalBranchRootState),
    Committed(RelationalBranchRootState),
}

impl PartialEq for SelectedRelationalBranchState {
    fn eq(&self, other: &Self) -> bool {
        self.state().root().id() == other.state().root().id()
    }
}

impl Eq for SelectedRelationalBranchState {}

impl SelectedRelationalBranchState {
    pub(crate) fn state(&self) -> &RelationalBranchRootState {
        match self {
            Self::Empty(state) | Self::Committed(state) => state,
        }
    }

    pub(crate) fn version_id(&self) -> VersionId {
        match self {
            Self::Empty(_) => VersionId(0),
            Self::Committed(state) => state.version_id(),
        }
    }

    pub(crate) fn root(&self) -> Option<&std::sync::Arc<RelationalBranchRoot>> {
        match self {
            Self::Empty(_) => None,
            Self::Committed(state) => Some(state.root()),
        }
    }
}

impl PartitionAccess for SelectedRelationalBranchState {
    fn get_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
    ) -> Option<&PartitionState> {
        self.state().get_partition(partition_id)
    }

    fn partition_ids(&self) -> Vec<crate::identity::data::PartitionId> {
        self.state().partition_ids()
    }
}

impl crate::runtime::RelationalRuntime {
    pub(crate) fn selected_branch_state(
        &self,
        binding: &RelationalLegacyBranchBinding,
    ) -> Result<SelectedRelationalBranchState, CommitPreparationError> {
        let branch_id = binding.identity().branch_id().clone();
        let Some(cell) = self.history.branch_cell(&branch_id) else {
            return Err(reference_mismatch(&branch_id, binding));
        };
        if cell.identity() != binding.identity()
            || cell.observation() != binding.observation()
            || cell.truth_version() != binding.truth_version()
        {
            return Err(reference_mismatch(&branch_id, binding));
        }

        match binding.observation().target() {
            FoundationalBranchTarget::Empty => {
                let root = cell.root().cloned().unwrap_or_else(|| {
                    RelationalBranchRoot::empty_with_schema(
                        &self.config.schema.registry,
                        crate::schema::data::runtime_descriptor_semantics_policy()
                            .current_write_version(),
                    )
                });
                Ok(SelectedRelationalBranchState::Empty(
                    RelationalBranchRootState::new(root),
                ))
            }
            FoundationalBranchTarget::Basis(target) => {
                let expected_version = VersionId(target.version_id());
                let expected_commit = Some(target.selected_commit_id());
                let Some(root) = cell.root() else {
                    return Err(CommitPreparationError::selected_branch_root_unavailable(
                        branch_id,
                        expected_commit,
                        expected_version,
                    ));
                };
                let Some(artifact) = self
                    .history
                    .commit_catalog
                    .get(CommitId(target.selected_commit_id()))
                else {
                    return Err(CommitPreparationError::selected_branch_root_unavailable(
                        branch_id,
                        expected_commit,
                        expected_version,
                    ));
                };
                if target.runtime_instance_id() != self.runtime_instance_id()
                    || !root_matches_target(root, target)
                    || artifact.identity().commit_id().0 != target.selected_commit_id()
                    || artifact.identity().version_id().0 != target.version_id()
                    || artifact.parentage().as_slice()
                        != target
                            .parent_commit_ids()
                            .iter()
                            .copied()
                            .map(CommitId)
                            .collect::<Vec<_>>()
                            .as_slice()
                {
                    return Err(
                        CommitPreparationError::selected_branch_root_reference_mismatch(
                            branch_id,
                            expected_commit,
                            expected_version,
                        ),
                    );
                }
                Ok(SelectedRelationalBranchState::Committed(
                    RelationalBranchRootState::new(root.clone()),
                ))
            }
        }
    }
}

fn root_matches_target(root: &RelationalBranchRoot, target: &RelationalBranchTarget) -> bool {
    let Some(axes) = root.axes() else {
        return false;
    };
    let Some(descriptor) = root.descriptor() else {
        return false;
    };
    root.commit_id()
        .is_some_and(|commit_id| commit_id.0 == target.selected_commit_id())
        && axes.storage_version == target.version_id()
        && descriptor.truth_root() == target.roots().truth_root()
        && descriptor.schema_root() == target.roots().schema_root()
}

fn reference_mismatch(
    branch_id: &BranchId,
    binding: &RelationalLegacyBranchBinding,
) -> CommitPreparationError {
    let (commit_id, version_id) = match binding.observation().target() {
        FoundationalBranchTarget::Empty => (None, VersionId(0)),
        FoundationalBranchTarget::Basis(target) => (
            Some(target.selected_commit_id()),
            VersionId(target.version_id()),
        ),
    };
    CommitPreparationError::selected_branch_root_reference_mismatch(
        branch_id.clone(),
        commit_id,
        version_id,
    )
}
