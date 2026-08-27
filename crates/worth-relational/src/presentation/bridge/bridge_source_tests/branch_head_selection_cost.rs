use std::sync::{Arc, Mutex};

use worth_runtime_bridge::facade::{
    CommittedPatchSource, RelationalCommittedPatchRequest, TruthBranchHeadSource,
    TruthBranchIdentity, TruthCommitIdentity,
};

use crate::tests::support::{create_entity, create_entity_outcome, release_test_commit_snapshot};

use super::super::RuntimeBridgeRelationalSource;
use super::support::runtime_with_test_schema;

#[test]
fn exact_selection_is_constant_and_historical_selection_uses_exact_ancestry() {
    let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
    let ancestor = create_entity_outcome(&mut runtime.lock().unwrap(), "deep-head-ancestor");
    let ancestor_commit_id = ancestor.commit.commit_id;
    release_test_commit_snapshot(&mut runtime.lock().unwrap(), &ancestor);
    for ordinal in 1..=255 {
        create_entity(
            &mut runtime.lock().unwrap(),
            &format!("deep-head-history-{ordinal}"),
        );
    }
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .unwrap();
    let identity = runtime.lock().unwrap().main_branch_identity();
    let (_, basis) = source.observe_branch_basis(&identity).unwrap();
    let head_commit = basis
        .observation()
        .commit_id()
        .expect("deep branch head commit");
    let ancestry_work = runtime
        .lock()
        .unwrap()
        .history()
        .inspect_commit_ancestry(head_commit)
        .total_work();
    let _head_lease = source.bind_branch_head_basis_for_bridge(&basis).unwrap();
    runtime
        .lock()
        .unwrap()
        .performance_access()
        .reset_counters();
    let branch = TruthBranchIdentity::from_relational_branch_id("main");

    source
        .load_branch_head_patch(&branch)
        .expect("exact branch head publication");
    let after_head = runtime.lock().unwrap().performance_access().counters();
    assert_eq!(after_head.bridge_observation_commit_selections, 1);
    assert_eq!(after_head.bridge_observation_commit_ancestry_visits, 0);

    source
        .load_committed_patch(RelationalCommittedPatchRequest::on_branch(
            TruthCommitIdentity::from_relational_commit_id(ancestor_commit_id.0),
            branch.clone(),
        ))
        .expect("visible historical ancestor");
    let after_historical = runtime.lock().unwrap().performance_access().counters();
    assert_eq!(after_historical.bridge_observation_commit_selections, 2);
    assert_eq!(
        after_historical.bridge_observation_commit_ancestry_visits,
        ancestry_work
    );

    let future = create_entity_outcome(&mut runtime.lock().unwrap(), "future-after-bound-head");
    let future_commit_id = future.commit.commit_id;
    release_test_commit_snapshot(&mut runtime.lock().unwrap(), &future);
    let denial = source
        .load_committed_patch(RelationalCommittedPatchRequest::on_branch(
            TruthCommitIdentity::from_relational_commit_id(future_commit_id.0),
            branch,
        ))
        .expect_err("a commit beyond the bound head is unreachable");
    assert!(denial.to_string().contains("cannot see requested commit"));
    let after_denial = runtime.lock().unwrap().performance_access().counters();
    assert_eq!(after_denial.bridge_observation_commit_selections, 3);
    assert_eq!(
        after_denial.bridge_observation_commit_ancestry_visits,
        ancestry_work * 2
    );
    assert!(head_commit < future_commit_id);
}
