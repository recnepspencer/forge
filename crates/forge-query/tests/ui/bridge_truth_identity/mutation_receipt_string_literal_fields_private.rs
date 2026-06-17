use forge_query::facade::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
};

fn main() {
    let _receipt = ForgeQueryMutationReceipt {
        commit_identity: "commit-1".to_string(),
        snapshot_token: "snapshot-1".to_string(),
        deltas: vec![ForgeQueryMutationDelta {
            collection: "employees".to_string(),
            entity_identity: "entity:employee-1".to_string(),
            kind: ForgeQueryMutationKind::Created,
            aspect_paths: Vec::new(),
        }],
        bridge_authority: None,
    };
}
