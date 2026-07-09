use worth_query::facade::{WorthQueryLivePatch, WorthQueryMutationKind};

fn main() {
    let _patch = WorthQueryLivePatch {
        view_name: "todo-list".to_string(),
        commit_identity: "commit-1".to_string(),
        entity_identity: "entity:todo-1".to_string(),
        mutation_kind: WorthQueryMutationKind::Updated,
        aspect_paths: Vec::new(),
        envelope: panic!("private-field proof is compile-time only"),
    };
}
