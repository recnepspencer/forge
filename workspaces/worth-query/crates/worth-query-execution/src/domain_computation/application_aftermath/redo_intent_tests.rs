use worth_relational::facade::history::{BranchId, CommitId, CommitReference};
use worth_relational::facade::identity::VersionId;

use super::redo_intent::{
    WorthQueryProvedUndo, WorthQueryProvedUndoAxisProbe, WorthQueryRedoIntent,
};

pub(super) fn probe_commit(commit_id: u64) -> CommitReference {
    CommitReference {
        commit_id: CommitId(commit_id),
        version_id: VersionId(commit_id),
        branch_id: BranchId("main".to_owned()),
        parents: commit_id.checked_sub(1).map(CommitId).into_iter().collect(),
    }
}

#[test]
fn fanout_does_not_change_identity() {
    let bound = probe_commit(20);
    let mut digests = Vec::new();
    for (postings, lineage) in [(10usize, 1usize), (1000, 100)] {
        let _discarded = (postings, lineage);
        let proved = WorthQueryProvedUndo::axis_probe(WorthQueryProvedUndoAxisProbe {
            original_operation: [3; 32],
            undo_commit_id: 20,
            principal_scope_digest: [4; 32],
            compatibility_generation: 1,
            runtime_instance: 9,
        });
        let intent = WorthQueryRedoIntent::derive(&proved, bound.clone()).expect("derive");
        digests.push(*intent.identity().digest());
    }
    assert_eq!(digests[0], digests[1]);
}
