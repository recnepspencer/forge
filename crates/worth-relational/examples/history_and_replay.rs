mod support;

use worth_relational::facade::{history::BranchId, runtime::RelationalRuntimeApi};

fn main() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(support::demo_schema_registry())
        .build();

    let (seed, entity_id) = support::create_entity(&mut runtime, "main-seed");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("create branch");

    let feature_commit = support::update_entity_on_branch(
        &mut runtime,
        entity_id,
        "feature-name",
        Some(BranchId("feature".to_string())),
    );

    let branches = runtime.history().branches();
    let replay = runtime.replay();
    let envelope = replay
        .canonical_commit_envelope(feature_commit.commit.commit_id)
        .expect("replay envelope");

    println!("branches={}", branches.len());
    println!(
        "seed_commit={} feature_commit={} authoritative_record_patches={}",
        seed.commit.commit_id.0,
        feature_commit.commit.commit_id.0,
        envelope.patch.authoritative_record_patches.len()
    );
}
