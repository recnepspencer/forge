use crate::facade::history::{CommitId, CommitReference, HistoryShapeClassification};
use crate::facade::identity::VersionId;
use crate::tests::support::*;

#[test]
fn ordered_parent_guardrail_classifies_root_linear_and_merge_ready_shapes() {
    let root = CommitReference {
        commit_id: CommitId(1),
        version_id: VersionId(1),
        branch_id: BranchId("main".to_string()),
        parents: Vec::new(),
    };
    let linear = CommitReference {
        commit_id: CommitId(2),
        version_id: VersionId(2),
        branch_id: BranchId("main".to_string()),
        parents: vec![CommitId(1)],
    };
    let merge_ready = CommitReference {
        commit_id: CommitId(3),
        version_id: VersionId(3),
        branch_id: BranchId("main".to_string()),
        parents: vec![CommitId(1), CommitId(2)],
    };

    assert_eq!(
        root.history_shape_classification(),
        HistoryShapeClassification::Root
    );
    assert_eq!(
        linear.history_shape_classification(),
        HistoryShapeClassification::Linear
    );
    assert_eq!(
        merge_ready.history_shape_classification(),
        HistoryShapeClassification::MergeReady
    );
    assert_eq!(
        merge_ready.ordered_parents().as_slice(),
        merge_ready.parents
    );
}
