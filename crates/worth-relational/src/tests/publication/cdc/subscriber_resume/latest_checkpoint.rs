use super::fixtures::{install_schema_version, transaction_options_for_subscriber_impact};
use crate::publication::cdc::data::{
    SubscriberContinuationClassSet, SubscriberContractDeclaration,
};
use crate::schema::data::{
    SchemaContinuationClassification, SchemaSubscriberImpact, SchemaVersionId,
};
use crate::tests::support::*;

#[test]
fn latest_available_checkpoint_reflects_head_continuation_state_for_subscriber_contract() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");
    let contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.geometry.v3".to_string(),
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([
            SchemaContinuationClassification::ContinueWithContractUpgrade,
        ]),
        ..SubscriberContractDeclaration::default()
    };

    let first_batch = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::from_head(1).with_subscriber_contract(contract.clone()),
        )
        .unwrap();
    let checkpoint = first_batch.next_checkpoint.unwrap();

    install_schema_version(&mut runtime, SchemaVersionId(2));
    let mut visible_txn = runtime.begin_transaction(transaction_options_for_subscriber_impact(
        &runtime,
        SchemaVersionId(2),
        SchemaSubscriberImpact::ConsumableSurfaceChanged,
    ));
    visible_txn.push_batch(batch_create("b"));
    visible_txn.commit().unwrap();

    install_schema_version(&mut runtime, SchemaVersionId(3));
    let mut upgrade_txn = runtime.begin_transaction(transaction_options_for_subscriber_impact(
        &runtime,
        SchemaVersionId(3),
        SchemaSubscriberImpact::ContractUpgradeRequired,
    ));
    upgrade_txn.push_batch(batch_create("c"));
    upgrade_txn.commit().unwrap();

    let resumed = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::resume_after(checkpoint, 1).with_subscriber_contract(contract),
        )
        .unwrap();

    let latest_available_checkpoint = resumed.latest_available_checkpoint.unwrap();
    assert_eq!(
        latest_available_checkpoint
            .continuation_summary()
            .continuation_outcome,
        SchemaContinuationClassification::ContinueWithContractUpgrade
    );
    assert_eq!(
        latest_available_checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        2
    );
}
