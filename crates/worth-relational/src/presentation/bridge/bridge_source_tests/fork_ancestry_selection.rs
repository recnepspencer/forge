use std::sync::{Arc, Mutex};

use worth_runtime_bridge::facade::{
    BridgeTruthViewEvaluationRequest, CommittedPatchSource, RelationalCommittedPatchRequest,
    TruthBranchIdentity, TruthCommitIdentity,
};

use crate::history::data::BranchId;
use crate::tests::support::{create_entity_outcome, create_entity_outcome_on_branch};

use super::super::RuntimeBridgeRelationalSource;
use super::support::{runtime_bridge_for_envelope, runtime_with_test_schema};

#[test]
fn advanced_fork_selects_inherited_ancestor_but_not_post_fork_source_sibling() {
    let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
    let inherited = create_entity_outcome(&runtime.lock().unwrap(), "fork-ancestor");
    let feature = BranchId("feature".to_owned());
    runtime
        .lock()
        .unwrap()
        .history_authority()
        .fork_branch_from(feature.clone(), &BranchId("main".to_owned()))
        .unwrap();
    let source_sibling = create_entity_outcome(&runtime.lock().unwrap(), "source-sibling");
    let feature_head =
        create_entity_outcome_on_branch(&runtime.lock().unwrap(), "feature-head", feature.clone());
    assert!(source_sibling.commit.commit_id < feature_head.commit.commit_id);

    let feature_identity = runtime
        .lock()
        .unwrap()
        .branch_identity(&feature)
        .expect("feature branch identity");
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .unwrap();
    let (_, basis) = source.observe_branch_basis(&feature_identity).unwrap();
    let _head_lease = source.bind_branch_head_basis_for_bridge(&basis).unwrap();
    let lease = source.retain_branch_basis_for_bridge(&basis).unwrap();
    let snapshot = lease.snapshot_identity().clone();
    let branch = TruthBranchIdentity::from_relational_branch_id("feature");
    let ancestor_commit =
        TruthCommitIdentity::from_relational_commit_id(inherited.commit.commit_id.0);

    runtime
        .lock()
        .unwrap()
        .performance_access()
        .reset_counters();
    let inherited_envelope = source
        .load_committed_patch(RelationalCommittedPatchRequest::at_snapshot(
            ancestor_commit.clone(),
            snapshot.clone(),
        ))
        .expect("an advanced fork must retain its source-branch ancestor");
    assert_eq!(
        inherited_envelope.branch_identity().relational_branch_id(),
        Some("feature")
    );
    let source_basis = inherited_envelope
        .producer_metadata()
        .authoritative_source()
        .expect("selected fork provenance")
        .source_basis();
    assert!(source_basis.contains("selected-branch=feature"));
    assert!(source_basis.contains("authoring-branch=main"));
    runtime_bridge_for_envelope(source.clone(), &inherited_envelope)
        .evaluate(BridgeTruthViewEvaluationRequest::for_historical_commit(
            branch,
            ancestor_commit,
        ))
        .expect("historical evaluation must retain the advanced fork's ancestor");

    let denial = source
        .load_committed_patch(RelationalCommittedPatchRequest::at_snapshot(
            TruthCommitIdentity::from_relational_commit_id(source_sibling.commit.commit_id.0),
            snapshot,
        ))
        .expect_err("a post-fork source sibling is not beneath the feature head");
    assert!(denial.to_string().contains("cannot see requested commit"));
    let counters = runtime.lock().unwrap().performance_access().counters();
    assert_eq!(counters.bridge_observation_commit_selections, 3);
    assert!(counters.bridge_observation_commit_ancestry_visits > 0);
}
