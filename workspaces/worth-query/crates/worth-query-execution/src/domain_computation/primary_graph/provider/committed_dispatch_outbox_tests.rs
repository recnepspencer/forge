use worth_foundational::facade::AspectValue;
use worth_relational::facade::history::{BranchId, CommitId};
use worth_relational::facade::identity::VersionId;
use worth_relational::facade::transactions::{
    AspectFieldPatch, DeleteEntityIntent, EntityMutationIntent, MutationIntent,
    RelationalTransaction, TransactionOptions, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::owner_test_support::{commit_record, record_for, string};
use super::restoration::hex_bytes;
use super::*;
use crate::domain_computation::application_aftermath::dispatch_outbox_create_intent;
use crate::domain_computation::primary_graph::provider::WorthQueryCommittedDispatchOutboxBindingDenial;
use crate::domain_computation::primary_graph::{
    primary_relational_branch_id, tests::fixture::installed_authorization_world,
    WorthQueryCommittedDispatchOutboxBinding,
};

#[test]
fn owner_read_denies_missing_foreign_and_every_commit_affinity_substitution() {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let (binding, commit, runtime_id) = commit_record(provider, 1);
    let record = binding.record().clone();
    assert_eq!(
        provider
            .observe_expected(&binding, &commit, runtime_id)
            .unwrap()
            .record(),
        &record
    );

    let absent = WorthQueryCommittedDispatchOutboxBinding::fixture(
        record_for(2),
        RecordRef::Entity(worth_relational::facade::identity::EntityId::new(
            worth_relational::facade::identity::PartitionId::main(),
            u64::MAX,
            1,
        )),
    );
    assert_eq!(
        provider.observe_expected(&absent, &commit, runtime_id),
        Err(Denial::Missing)
    );
    assert_eq!(
        provider.observe_expected(&binding, &commit, runtime_id + 1),
        Err(Denial::ForeignRuntime)
    );
    assert_commit_affinity_substitutions(provider, &binding, commit, runtime_id);
}

fn assert_commit_affinity_substitutions(
    provider: &WorthQueryPrimaryGraphProvider,
    binding: &WorthQueryCommittedDispatchOutboxBinding,
    commit: worth_relational::facade::history::RelationalCommitReceipt,
    runtime_id: u64,
) {
    let mut wrong_commit_id = commit.clone();
    wrong_commit_id.commit_id = CommitId(commit.commit_id.0.saturating_sub(1));
    assert_eq!(
        provider.observe_expected(binding, &wrong_commit_id, runtime_id),
        Err(Denial::CommitMismatch)
    );
    let mut wrong_version = commit.clone();
    wrong_version.version_id = VersionId(commit.version_id.0.saturating_sub(1));
    assert_eq!(
        provider.observe_expected(binding, &wrong_version, runtime_id),
        Err(Denial::CommitMismatch)
    );
    let feature = BranchId("committed-outbox-feature".to_owned());
    provider.graph.with_runtime_mut(|runtime| {
        let (_, basis) = runtime.observe_fork_source(&commit.branch_id).unwrap();
        runtime.fork_branch(feature.clone(), basis).unwrap();
        let feature_record = record_for(99);
        let mut transaction = runtime.begin_transaction(
            runtime
                .owner_transaction_options_for_branch(&feature)
                .expect("feature branch binding"),
        );
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
        provider.observe_expected(binding, &wrong_branch, runtime_id),
        Err(Denial::CommitMismatch)
    );
}

#[test]
fn fresh_later_head_still_reports_the_rows_exact_creation_commit() {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let (first_binding, first_commit, runtime_id) = commit_record(provider, 3);
    let first = first_binding.record().clone();
    let (_, later_commit, _) = commit_record(provider, 4);
    assert_ne!(first_commit, later_commit);

    let observed = provider
        .observe_expected(&first_binding, &first_commit, runtime_id)
        .expect("the current snapshot retains the older live outbox row");
    assert_eq!(observed.commit_reference(), &first_commit);
    assert_eq!(observed.record(), &first);
    assert_eq!(observed.work().exact_commit_snapshots(), 1);
    assert_eq!(observed.work().canonical_version_probes(), 1);
    assert_eq!(observed.work().projection_views(), 1);
    assert_eq!(observed.work().examined_index_entries(), 0);
    assert_eq!(observed.work().direct_record_probes(), 1);
    assert_eq!(observed.work().projected_records(), 1);
    assert_eq!(observed.work().projected_fields(), 8);
    assert_eq!(observed.work().reconstruction_requests(), 0);
}

#[test]
fn every_later_valid_field_substitution_leaves_exact_commit_truth_unchanged() {
    for (field, replacement) in valid_later_field_substitutions() {
        let world = installed_authorization_world(true);
        let provider = &world.application.primary_provider;
        let (binding, commit, runtime_id) = commit_record(provider, 31);
        let record = binding.record().clone();
        let observed = provider
            .observe_expected(&binding, &commit, runtime_id)
            .expect("original exact-commit row");
        let RecordRef::Entity(entity_id) = observed.record_ref() else {
            panic!("dispatch outbox is an entity record");
        };
        provider.graph.with_runtime_mut(|runtime| {
            let locator =
                outbox_field_locator(provider.graph.layout.provider_dispatch_outbox(), field);
            let mut transaction = runtime.begin_transaction(
                runtime
                    .transaction_options_for_main()
                    .expect("main branch binding"),
            );
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
            .observe_expected(&binding, &commit, runtime_id)
            .expect("later head cannot replace exact-commit truth");
        assert_eq!(exact.record(), &record);
        assert_eq!(exact.commit_reference(), &commit);
    }
}

#[test]
fn later_deletion_cannot_erase_exact_commit_truth() {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let (binding, commit, runtime_id) = commit_record(provider, 32);
    let record = binding.record().clone();
    let observed = provider
        .observe_expected(&binding, &commit, runtime_id)
        .expect("original exact-commit row");
    let RecordRef::Entity(entity_id) = observed.record_ref() else {
        panic!("dispatch outbox is an entity record");
    };
    provider.graph.with_runtime_mut(|runtime| {
        let mut transaction = runtime.begin_transaction(
            runtime
                .transaction_options_for_main()
                .expect("main branch binding"),
        );
        transaction.push_batch(WorkerIntentBatch::new("later-outbox-deletion").push(
            MutationIntent::Entity(EntityMutationIntent::Delete(DeleteEntityIntent {
                entity_id: *entity_id,
            })),
        ));
        transaction.commit().unwrap();
    });

    let exact = provider
        .observe_expected(&binding, &commit, runtime_id)
        .expect("later deletion cannot erase retained exact-commit truth");
    assert_eq!(exact.record(), &record);
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
fn unrelated_identical_row_cannot_make_owner_mapping_ambiguous() {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let record = record_for(5);
    provider.graph.with_runtime_mut(|runtime| {
        let (intent, pending) =
            crate::domain_computation::application_aftermath::bind_dispatch_outbox_create_intent(
                Some(provider.graph.layout.provider_dispatch_outbox()),
                Some(&record),
            )
            .unwrap();
        let MutationIntent::Create(worth_relational::facade::transactions::CreateIntent::Entity(
            mut unrelated,
        )) = intent.clone()
        else {
            panic!("outbox fixture creates an entity")
        };
        unrelated.client_key = worth_relational::facade::symbols::ClientKey::raw(
            "unrelated-row-with-identical-fields",
        );
        let unrelated_created = worth_relational::facade::transactions::CreatedEntityRef {
            partition_id: unrelated.partition_id,
            kind_id: unrelated.kind_id,
            client_key: unrelated.client_key.clone(),
        };
        let mut transaction: RelationalTransaction<'_> = runtime.begin_transaction(
            runtime
                .transaction_options_for_main()
                .expect("main branch binding"),
        );
        transaction.push_batch(
            WorkerIntentBatch::new("owner-mapped-committed-outbox-test")
                .push(intent)
                .push(MutationIntent::Create(
                    worth_relational::facade::transactions::CreateIntent::Entity(unrelated),
                )),
        );
        let committed = transaction.commit().unwrap();
        assert_ne!(
            committed.created_entity(pending.created_entity()),
            committed.created_entity(&unrelated_created),
            "each exact create reference retains its own owner-minted identity"
        );
        let binding = WorthQueryCommittedDispatchOutboxBinding::fixture_from_commit(
            provider.graph.layout.provider_dispatch_outbox(),
            Some(pending.record()),
            &committed,
        )
        .unwrap()
        .unwrap();
        assert_eq!(binding.record(), &record);
        assert_eq!(
            binding.record_ref(),
            &RecordRef::Entity(committed.created_entity(pending.created_entity()).unwrap(),)
        );
        assert_eq!(
            WorthQueryCommittedDispatchOutboxBinding::fixture_from_commit(
                provider.graph.layout.provider_dispatch_outbox(),
                Some(&record_for(6)),
                &committed,
            ),
            Err(WorthQueryCommittedDispatchOutboxBindingDenial::CreatedEntityMissing)
        );
    });
}

#[test]
fn another_committed_record_ref_cannot_substitute_for_the_bound_outbox() {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let first = record_for(61);
    let second = record_for(62);
    let (first_binding, second_binding, commit, runtime_id) =
        provider.graph.with_runtime_mut(|runtime| {
            let intent = |record: &WorthQueryDispatchOutboxRecord| {
                dispatch_outbox_create_intent(
                    Some(provider.graph.layout.provider_dispatch_outbox()),
                    Some(record),
                )
                .unwrap()
            };
            let mut transaction = runtime.begin_transaction(
                runtime
                    .transaction_options_for_main()
                    .expect("main branch binding"),
            );
            transaction.push_batch(
                WorkerIntentBatch::new("record-ref-substitution-owner-test")
                    .push(intent(&first))
                    .push(intent(&second)),
            );
            let committed = transaction.commit().unwrap();
            let bind = |record| {
                WorthQueryCommittedDispatchOutboxBinding::fixture_from_commit(
                    provider.graph.layout.provider_dispatch_outbox(),
                    Some(record),
                    &committed,
                )
                .unwrap()
                .unwrap()
            };
            let snapshot = runtime
                .snapshots()
                .historical_snapshot_for_branch(&primary_relational_branch_id())
                .unwrap();
            let runtime_id = snapshot.runtime_instance_id;
            runtime.snapshots().release_snapshot(&snapshot);
            (
                bind(&first),
                bind(&second),
                committed.outcome().commit.clone(),
                runtime_id,
            )
        });
    let substituted = WorthQueryCommittedDispatchOutboxBinding::fixture(
        first_binding.record().clone(),
        second_binding.record_ref().clone(),
    );
    assert_eq!(
        provider.observe_expected(&substituted, &commit, runtime_id),
        Err(Denial::RecordMismatch)
    );
}
