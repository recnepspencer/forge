use forge_query::facade::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    SignalInvalidationRoutingReceipt,
};

fn main() {
    let receipt = ForgeQueryMutationReceipt {
        commit_identity: "commit-1".to_string(),
        snapshot_token: "snapshot-1".to_string(),
        deltas: vec![ForgeQueryMutationDelta {
            collection: "Task".to_string(),
            entity_identity: "task-1".to_string(),
            kind: ForgeQueryMutationKind::Created,
            aspect_paths: vec!["title.value".to_string()],
        }],
        bridge_authority: None,
    };
    let _ = SignalInvalidationRoutingReceipt::from_mutation_receipt(&receipt);
}
