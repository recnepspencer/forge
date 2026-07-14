use worth_query::facade::foundation::{WorthQueryMutationDelta, WorthQueryMutationKind, WorthQueryMutationReceipt};
use worth_query::facade::runtime::SignalInvalidationRoutingReceipt;

fn main() {
    let receipt = WorthQueryMutationReceipt {
        commit_identity: "commit-1".to_string(),
        snapshot_token: "snapshot-1".to_string(),
        deltas: vec![WorthQueryMutationDelta {
            collection: "Task".to_string(),
            entity_identity: "task-1".to_string(),
            kind: WorthQueryMutationKind::Created,
            touched_aspects: Vec::new(),
        }],
        bridge_authority: None,
    };
    let _ = SignalInvalidationRoutingReceipt::from_mutation_receipt(&receipt);
}
