use worth_query::facade::runtime::QueryPatchGroup;

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
