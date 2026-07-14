use worth_query::facade::foundation::{WorthQueryMutationDelta, WorthQueryMutationKind, WorthQueryMutationReceipt};

fn main() {
    let _receipt = WorthQueryMutationReceipt {
        commit_identity: "commit-1".to_string(),
        snapshot_token: "snapshot-1".to_string(),
        deltas: vec![WorthQueryMutationDelta {
            collection: "employees".to_string(),
            entity_identity: "entity:employee-1".to_string(),
            kind: WorthQueryMutationKind::Created,
            aspect_paths: Vec::new(),
        }],
        bridge_authority: None,
    };
}
