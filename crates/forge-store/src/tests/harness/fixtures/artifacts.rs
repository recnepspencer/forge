use forge_relational::facade::history::CommitId;

use crate::ForgeStore;

use super::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};

pub fn append_two_mainline_commits(store: &mut ForgeStore) -> (CommitId, CommitId) {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    let first_id = first.commit.commit_id;
    store.append_canonical_commit(first).unwrap();

    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();

    (first_id, second_id)
}

pub fn append_three_mainline_commits(store: &mut ForgeStore) -> (CommitId, CommitId, CommitId) {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    let first_id = first.commit.commit_id;
    store.append_canonical_commit(first).unwrap();

    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();

    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let third = latest_envelope(&runtime);
    let third_id = third.commit.commit_id;
    store.append_canonical_commit(third).unwrap();

    (first_id, second_id, third_id)
}
