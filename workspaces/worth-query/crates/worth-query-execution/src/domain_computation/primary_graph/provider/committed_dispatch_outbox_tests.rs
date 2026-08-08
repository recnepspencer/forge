use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_declaration::facade::application_schema::ApplicationExternalEffectProtocol;
use worth_query_installation::facade::InstalledExternalEffectContract;
use worth_relational::facade::history::{BranchId, CommitId};
use worth_relational::facade::identity::VersionId;
use worth_relational::facade::transactions::{
    AspectFieldPatch, EntityMutationIntent, MutationIntent, RelationalTransaction,
    TransactionOptions, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::*;
use crate::domain_computation::application_aftermath::{
    derive_external_effect_correlation_identity, dispatch_outbox_create_intent,
    ExternalEffectCorrelationBasis,
};
use crate::domain_computation::primary_graph::{
    primary_relational_branch_id, tests::fixture::installed_authorization_world,
};

#[test]
fn owner_read_denies_missing_foreign_and_every_commit_affinity_substitution() {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let (record, commit, runtime_id) = commit_record(provider, 1);
    assert_eq!(
        provider
            .observe_expected(&record, &commit, runtime_id)
            .unwrap()
            .record(),
        &record
    );

    let absent = record_for(2);
    assert_eq!(
        provider.observe_expected(&absent, &commit, runtime_id),
        Err(Denial::Missing)
    );
    assert_eq!(
        provider.observe_expected(&record, &commit, runtime_id + 1),
        Err(Denial::ForeignRuntime)
    );
    assert_commit_affinity_substitutions(provider, &record, commit, runtime_id);
}

fn assert_commit_affinity_substitutions(
    provider: &WorthQueryPrimaryGraphProvider,
    record: &WorthQueryDispatchOutboxRecord,
    commit: worth_relational::facade::history::CommitReference,
    runtime_id: u64,
) {
    let mut wrong_commit_id = commit.clone();
    wrong_commit_id.commit_id = CommitId(commit.commit_id.0.saturating_sub(1));
    assert_eq!(
        provider.observe_expected(record, &wrong_commit_id, runtime_id),
        Err(Denial::CommitMismatch)
    );
    let mut wrong_version = commit.clone();
    wrong_version.version_id = VersionId(commit.version_id.0.saturating_sub(1));
    assert_eq!(
        provider.observe_expected(record, &wrong_version, runtime_id),
        Err(Denial::ExactCommitUnavailable)
    );
    let feature = BranchId("committed-outbox-feature".to_owned());
    provider.graph.with_runtime_mut(|runtime| {
        runtime
            .history_authority()
            .create_branch(feature.clone(), &commit.branch_id)
            .unwrap();
        let feature_record = record_for(99);
        let mut transaction = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(feature.clone()),
            ..TransactionOptions::default()
        });
        transaction.push_batch(
            WorkerIntentBatch::new("feature-outbox-head").push(
                dispatch_outbox_create_intent(
                    Some(provider.graph.layout.provider_dispatch_outbox()),
                    Some(&feature_record),
                )
                .unwrap(),
            ),
        );
        transaction.commit().unwrap();
    });
    let mut wrong_branch = commit;
    wrong_branch.branch_id = feature;
    assert_eq!(
        provider.observe_expected(record, &wrong_branch, runtime_id),
        Err(Denial::CommitMismatch)
    );
}

#[test]
fn fresh_later_head_still_reports_the_rows_exact_creation_commit() {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let (first, first_commit, runtime_id) = commit_record(provider, 3);
    let (_, later_commit, _) = commit_record(provider, 4);
    assert_ne!(first_commit, later_commit);

    let observed = provider
        .observe_expected(&first, &first_commit, runtime_id)
        .expect("the current snapshot retains the older live outbox row");
    assert_eq!(observed.commit_reference(), &first_commit);
    assert_eq!(observed.record(), &first);
    assert_eq!(observed.work().exact_commit_snapshots(), 1);
    assert_eq!(observed.work().examined_index_entries(), 1);
    assert_eq!(observed.work().projected_records(), 1);
    assert_eq!(observed.work().projected_fields(), 8);
}

#[test]
fn every_later_valid_field_substitution_leaves_exact_commit_truth_unchanged() {
    for (field, replacement) in valid_later_field_substitutions() {
        let world = installed_authorization_world(true);
        let provider = &world.application.primary_provider;
        let (record, commit, runtime_id) = commit_record(provider, 31);
        let observed = provider
            .observe_expected(&record, &commit, runtime_id)
            .expect("original exact-commit row");
        let RecordRef::Entity(entity_id) = observed.record_ref() else {
            panic!("dispatch outbox is an entity record");
        };
        provider.graph.with_runtime_mut(|runtime| {
            let locator =
                outbox_field_locator(provider.graph.layout.provider_dispatch_outbox(), field);
            let mut transaction = runtime.begin_transaction(TransactionOptions::default());
            transaction.push_batch(
                WorkerIntentBatch::new("later-valid-outbox-substitution").push(
                    MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity_id,
                            fields: AspectFieldPatch::from_locator(locator, replacement),
                        },
                    )),
                ),
            );
            transaction.commit().unwrap();
        });

        let exact = provider
            .observe_expected(&record, &commit, runtime_id)
            .expect("later head cannot replace exact-commit truth");
        assert_eq!(exact.record(), &record);
        assert_eq!(exact.commit_reference(), &commit);
    }
}

fn valid_later_field_substitutions() -> Vec<(usize, AspectValue)> {
    vec![
        (0, string(hex_bytes(&[9; 32]))),
        (1, string("later-family".to_owned())),
        (2, string("later-effect".to_owned())),
        (3, string("test.owner.later".to_owned())),
        (4, AspectValue::UInt64(2)),
        (5, AspectValue::UInt64(25)),
        (6, string("ff".to_owned())),
        (7, AspectValue::UInt64(99)),
    ]
}

fn outbox_field_locator(
    layout: &WorthQueryDispatchOutboxLayout,
    field: usize,
) -> worth_foundational::facade::AspectFieldLocator {
    [
        &layout.correlation_locator,
        &layout.family_locator,
        &layout.effect_locator,
        &layout.protocol_identity_locator,
        &layout.protocol_version_locator,
        &layout.maximum_payload_bytes_locator,
        &layout.payload_locator,
        &layout.outcome_identity_locator,
    ][field]
        .clone()
}

#[test]
fn durable_field_restore_accepts_the_exact_committed_shape() {
    let restored = restore_record(valid_restored_fields()).unwrap();
    assert_eq!(restored.correlation().bytes(), &[7; 32]);
    assert_eq!(restored.correlation_family(), "family");
    assert_eq!(restored.effect(), "effect");
    assert_eq!(restored.protocol_identity().as_str(), "test.effect");
    assert_eq!(restored.protocol_version(), BoundaryProtocolVersion::new(1));
    assert_eq!(restored.maximum_payload_bytes(), 24);
    assert_eq!(restored.payload(), [1, 2]);
    assert_eq!(restored.outcome_identity(), 9);
}

#[test]
fn durable_field_restore_rejects_every_omission_and_wrong_storage_type() {
    let valid = valid_restored_fields();
    for omitted in 0..valid.len() {
        let mut values = valid.clone();
        values.remove(omitted);
        assert_eq!(restore_record(values), Err(Denial::Malformed));
    }
    for corrupted in 0..valid.len() {
        let mut values = valid.clone();
        values[corrupted] = if matches!(&values[corrupted], AspectValue::UInt64(_)) {
            string("not-an-integer".to_owned())
        } else {
            AspectValue::UInt64(17)
        };
        assert_eq!(restore_record(values), Err(Denial::Malformed));
    }
}

#[test]
fn durable_field_restore_rejects_malformed_protocol_and_encoded_bytes() {
    let odd_digest = format!("{}0", "00".repeat(32));
    for (field, invalid) in [
        (0, string("not-a-digest".to_owned())),
        (0, string(odd_digest)),
        (3, string("test.effect.v1".to_owned())),
        (4, AspectValue::UInt64(0)),
        (4, AspectValue::UInt64(u64::from(u32::MAX) + 1)),
        (6, string("not-hex".to_owned())),
        (6, string("010".to_owned())),
    ] {
        let mut values = valid_restored_fields();
        values[field] = invalid;
        assert_eq!(restore_record(values), Err(Denial::Malformed));
    }
}

fn valid_restored_fields() -> Vec<AspectValue> {
    vec![
        string(hex_bytes(&[7; 32])),
        string("family".to_owned()),
        string("effect".to_owned()),
        string("test.effect".to_owned()),
        AspectValue::UInt64(1),
        AspectValue::UInt64(24),
        string("0102".to_owned()),
        AspectValue::UInt64(9),
    ]
}

#[test]
fn duplicate_correlation_rows_deny_as_ambiguous() {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let record = record_for(5);
    let branch = primary_relational_branch_id();
    let (commit, runtime_id) = provider.graph.with_runtime_mut(|runtime| {
        let intent = || {
            dispatch_outbox_create_intent(
                Some(provider.graph.layout.provider_dispatch_outbox()),
                Some(&record),
            )
            .unwrap()
        };
        let mut transaction: RelationalTransaction<'_> =
            runtime.begin_transaction(Default::default());
        transaction.push_batch(
            WorkerIntentBatch::new("ambiguous-committed-outbox-owner-test")
                .push(intent())
                .push(intent()),
        );
        transaction.commit().unwrap();
        let commit = runtime.history().branch_head(&branch).unwrap().clone();
        publish_outbox_index(provider, runtime, &commit);
        let snapshot = runtime.snapshots().snapshot_for_branch(&branch).unwrap();
        let runtime_id = snapshot.runtime_instance_id;
        runtime.snapshots().release_snapshot(&snapshot);
        (commit, runtime_id)
    });
    assert_eq!(
        provider.observe_expected(&record, &commit, runtime_id),
        Err(Denial::Ambiguous)
    );
}

fn commit_record(
    provider: &WorthQueryPrimaryGraphProvider,
    identity: u64,
) -> (
    WorthQueryDispatchOutboxRecord,
    worth_relational::facade::history::CommitReference,
    u64,
) {
    let record = record_for(identity);
    let branch = primary_relational_branch_id();
    let (commit, runtime_id) = provider.graph.with_runtime_mut(|runtime| {
        let mut transaction: RelationalTransaction<'_> =
            runtime.begin_transaction(Default::default());
        transaction.push_batch(
            WorkerIntentBatch::new("committed-outbox-owner-test").push(
                dispatch_outbox_create_intent(
                    Some(provider.graph.layout.provider_dispatch_outbox()),
                    Some(&record),
                )
                .unwrap(),
            ),
        );
        transaction.commit().unwrap();
        let commit = runtime.history().branch_head(&branch).unwrap().clone();
        publish_outbox_index(provider, runtime, &commit);
        let snapshot = runtime.snapshots().snapshot_for_branch(&branch).unwrap();
        let runtime_id = snapshot.runtime_instance_id;
        runtime.snapshots().release_snapshot(&snapshot);
        (commit, runtime_id)
    });
    (record, commit, runtime_id)
}

fn publish_outbox_index(
    provider: &WorthQueryPrimaryGraphProvider,
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    commit: &worth_relational::facade::history::CommitReference,
) {
    let built = runtime.index_authority().build_for_commit(
        worth_relational::facade::indexes::DerivedIndexBuildRequest {
            source_commit_id: commit.commit_id,
            branch_id: commit.branch_id.clone(),
            index_ids: vec![
                provider
                    .graph
                    .layout
                    .provider_dispatch_outbox()
                    .correlation_index_id,
            ],
        },
    );
    assert!(built.failed_indexes.is_empty());
}

fn record_for(identity: u64) -> WorthQueryDispatchOutboxRecord {
    let correlation = derive_external_effect_correlation_identity(ExternalEffectCorrelationBasis {
        correlation_family: "owner-test",
        operation_slot: "notify",
        operation_version: 1,
        outcome_identity: identity,
        idempotency_key: &[identity as u8; 32],
        branch: "main",
    })
    .unwrap();
    WorthQueryDispatchOutboxRecord::from_installed_contract(
        correlation,
        &InstalledExternalEffectContract::Declared {
            correlation_family: "owner-test".to_owned(),
            effect: "OwnerTestEffect".to_owned(),
            rust_payload_type: "tests::Payload".to_owned(),
            protocol: ApplicationExternalEffectProtocol::new(
                BoundaryProtocolIdentity::new("test.owner.payload"),
                BoundaryProtocolVersion::new(1),
            ),
            maximum_payload_bytes: 24,
        },
        vec![identity as u8; 8],
        identity,
    )
    .unwrap()
}

fn string(value: String) -> AspectValue {
    AspectValue::String(InternedString::from(value))
}
