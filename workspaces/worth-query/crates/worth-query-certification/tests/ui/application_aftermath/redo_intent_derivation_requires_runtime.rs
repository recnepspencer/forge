use worth_query_host::facade::provisional_aftermath::{WorthQueryProvedUndo, WorthQueryRedoIntent};
use worth_relational::facade::{
    history::{BranchId, CommitId, CommitReference},
    identity::VersionId,
};

fn cannot_choose_lineage_head(proved: &WorthQueryProvedUndo) {
    let caller_chosen = CommitReference {
        commit_id: CommitId(7),
        version_id: VersionId(7),
        branch_id: BranchId("main".to_owned()),
        parents: vec![CommitId(6)],
    };
    let _ = WorthQueryRedoIntent::derive(proved, caller_chosen);
}

fn main() {}
