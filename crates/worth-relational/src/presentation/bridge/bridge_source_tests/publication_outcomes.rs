use worth_proof::TransitionOutcome;

use crate::history::data::CommitId;

use super::super::{RelationalBridgePublicationDeferred, RelationalBridgePublicationStale};
use super::support::runtime_with_test_schema;

#[test]
fn missing_commit_outcome_uses_the_real_runtime_commit_horizon() {
    let runtime = runtime_with_test_schema();

    assert!(matches!(
        runtime.publish_commit_for_bridge(CommitId(1), "model"),
        TransitionOutcome::Deferred(RelationalBridgePublicationDeferred::CommitVisibilityPending)
    ));
    assert!(matches!(
        runtime.publish_commit_for_bridge(CommitId(0), "model"),
        TransitionOutcome::Stale(RelationalBridgePublicationStale::CommitNotRetained)
    ));
}
