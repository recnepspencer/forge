use crate::ForgeStoreBuilder;

use super::super::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};

pub struct TwoCommitAuthorityScenario {
    pub first: forge_relational::facade::replay::CanonicalCommitEnvelope,
    pub second: forge_relational::facade::replay::CanonicalCommitEnvelope,
}

pub fn two_commit_authority_history() -> TwoCommitAuthorityScenario {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    TwoCommitAuthorityScenario { first, second }
}

pub fn append_two_commits_in_memory() -> (crate::ForgeStore, TwoCommitAuthorityScenario) {
    let scenario = two_commit_authority_history();
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store
        .append_canonical_commit(scenario.first.clone())
        .unwrap();
    store
        .append_canonical_commit(scenario.second.clone())
        .unwrap();
    (store, scenario)
}
