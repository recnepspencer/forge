use crate::facade::transactions::MutationIntent;
use crate::tests::support::*;

#[test]
fn client_key_symbol_policy_interns_client_keys_before_merge() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .client_key_symbol_policy(ClientKeySymbolPolicy::RequireInterned)
        .build();
    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(batch_create("intern-me"));
    let plan = txn.merged_plan().unwrap().clone();

    match &plan.merged_intents[0] {
        MutationIntent::Create(CreateIntent::Entity(spec)) => {
            assert!(spec.client_key.as_symbol().is_some());
        }
        other => panic!("expected create entity intent, got {other:?}"),
    }
    assert!(!runtime.config().identity.symbol_table.entries.is_empty());
}

#[test]
fn client_key_symbol_policy_skips_symbol_table_snapshot_refresh_when_no_raw_keys_are_present() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .client_key_symbol_policy(ClientKeySymbolPolicy::RequireInterned)
        .build();
    let entity = create_entity(&mut runtime, "stable-key");
    let before = runtime.config().identity.symbol_table.clone();

    let _ = update_entity(&mut runtime, entity, "updated-aspect");

    assert_eq!(runtime.config().identity.symbol_table, before);
}

#[test]
fn client_key_symbol_policy_incrementally_merges_new_snapshot_entries_in_sorted_order() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .client_key_symbol_policy(ClientKeySymbolPolicy::RequireInterned)
        .build();

    let _ = create_entity(&mut runtime, "beta");
    let _ = create_entity(&mut runtime, "alpha");

    assert_eq!(
        runtime
            .config()
            .identity
            .symbol_table
            .entries
            .iter()
            .map(|(_, value)| value.as_str())
            .collect::<Vec<_>>(),
        vec!["alpha", "beta"]
    );
}
