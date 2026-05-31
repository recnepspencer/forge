use crate::tests::support::*;

#[test]
fn durable_log_compaction_respects_checkpoint_policy() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .durable_log_policy(DurableLogPolicy {
            retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
            max_in_memory_envelopes: 1,
            compact_after_checkpoint: true,
        })
        .build();

    create_entity(&mut runtime, "first");
    runtime.durability_authority().checkpoint().unwrap();
    create_entity(&mut runtime, "second");
    create_entity(&mut runtime, "third");

    assert!(runtime.durability().durable_log().len() <= 1);
}
