use crate::capabilities::StorageRead;
use crate::identity::data::{RelationId, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{CommitConflict, ConflictClass, ExistingRecordTarget};

use super::super::record_lookup::{relation_exists_in_state, relation_exists_in_version_basis};

pub(super) fn validate_existing_relation_target(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    branch_basis_version_id: Option<VersionId>,
    relation_id: RelationId,
) -> Result<(), CommitConflict> {
    if relation_exists_in_commit_scope(runtime, state, branch_basis_version_id, relation_id) {
        return Ok(());
    }

    Err(CommitConflict::new(ConflictClass::StaleTarget {
        target: ExistingRecordTarget::Relation(relation_id),
        context: "relation validation".to_string(),
    }))
}

fn relation_exists_in_commit_scope(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    branch_basis_version_id: Option<VersionId>,
    relation_id: RelationId,
) -> bool {
    relation_exists_in_state(state, relation_id)
        || branch_basis_version_id.is_some_and(|version_id| {
            relation_exists_in_version_basis(runtime, version_id, relation_id)
        })
}
