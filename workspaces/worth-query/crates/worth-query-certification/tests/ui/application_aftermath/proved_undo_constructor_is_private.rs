use worth_query_host::facade::provisional_aftermath::WorthQueryProvedUndo;
use worth_relational::facade::{
    history::{BranchId, CommitId, CommitReference},
    identity::VersionId,
};

fn main() {
    let _forged = WorthQueryProvedUndo {
        original_operation: [0; 32],
        undo_commit: CommitReference {
            commit_id: CommitId(7),
            version_id: VersionId(7),
            branch_id: BranchId("main".to_owned()),
            parents: vec![CommitId(6)],
        },
        principal_scope_digest: [0; 32],
        compatibility_generation: 1,
        runtime_instance: 9,
        _private: (),
    };
}
