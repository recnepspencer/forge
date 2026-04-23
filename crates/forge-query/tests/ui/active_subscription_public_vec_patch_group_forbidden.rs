use forge_query::facade::QueryPatchGroup;

fn main() {
    let _: Vec<QueryPatchGroup> = vec![
        QueryPatchGroup {
            kind: todo!(),
            source_delta_digest: "delta".to_string(),
            width: 1,
            patch_group_digest: "patch".to_string(),
        },
    ];
}
