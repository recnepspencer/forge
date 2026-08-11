//! Real commit-to-owner-observation support for C4 tests.

use worth_relational::facade::transactions::{
    CreateIntent, CreatedEntityRef, MutationIntent, RecordRef, RelationalTransaction,
    WorkerIntentBatch,
};

use super::super::{WorthQueryCommittedDispatchOutboxObservation, WorthQueryPrimaryGraphProvider};
use crate::domain_computation::application_aftermath::{
    bind_dispatch_outbox_create_intent, WorthQueryDispatchOutboxRecord,
};
use crate::domain_computation::primary_graph::{
    primary_relational_branch_id, tests::fixture::installed_authorization_world,
    WorthQueryAdmittedExternalDispatchAttempt, WorthQueryCommittedDispatchOutboxBinding,
};

pub(in crate::domain_computation) fn commit_observe_and_admit_fixture(
    record: &WorthQueryDispatchOutboxRecord,
) -> (
    WorthQueryAdmittedExternalDispatchAttempt,
    worth_relational::facade::history::CommitReference,
    worth_relational::facade::transactions::RecordRef,
    u64,
) {
    let world = installed_authorization_world(true);
    let observation = commit_and_observe_fixture(&world.application.primary_provider, record);
    let commit = observation.commit_reference().clone();
    let record_ref = observation.record_ref().clone();
    let relational_runtime_instance_id = observation.relational_runtime_instance_id();
    let admitted = world
        .application
        .admit_external_dispatch_attempt(observation)
        .expect("application runtime admits its owner observation");
    (admitted, commit, record_ref, relational_runtime_instance_id)
}

pub(in crate::domain_computation) fn commit_observe_and_admit_twice_fixture(
    record: &WorthQueryDispatchOutboxRecord,
) -> (
    WorthQueryAdmittedExternalDispatchAttempt,
    WorthQueryAdmittedExternalDispatchAttempt,
) {
    let world = installed_authorization_world(true);
    let observation = commit_and_observe_fixture(&world.application.primary_provider, record);
    let first = world
        .application
        .admit_external_dispatch_attempt(observation.clone())
        .expect("first physical attempt is admitted");
    let second = world
        .application
        .admit_external_dispatch_attempt(observation)
        .expect("second physical attempt is admitted");
    (first, second)
}

pub(in crate::domain_computation) fn commit_distinct_records_and_admit_fixture(
    record: &WorthQueryDispatchOutboxRecord,
) -> (
    WorthQueryAdmittedExternalDispatchAttempt,
    WorthQueryAdmittedExternalDispatchAttempt,
    RecordRef,
    RecordRef,
) {
    let world = installed_authorization_world(true);
    let provider = &world.application.primary_provider;
    let branch = primary_relational_branch_id();
    let (first_binding, second_binding, commit, runtime_id) =
        provider.graph.with_runtime_mut(|runtime| {
            let (first_intent, first_pending) = bind_dispatch_outbox_create_intent(
                Some(provider.graph.layout.provider_dispatch_outbox()),
                Some(record),
            )
            .expect("first outbox binds");
            let MutationIntent::Create(CreateIntent::Entity(mut second_spec)) =
                first_intent.clone()
            else {
                panic!("outbox intent creates an entity")
            };
            second_spec.client_key =
                worth_relational::facade::symbols::ClientKey::raw("same-record-second-identity");
            let second_created = CreatedEntityRef {
                partition_id: second_spec.partition_id,
                kind_id: second_spec.kind_id,
                client_key: second_spec.client_key.clone(),
            };
            let mut transaction: RelationalTransaction<'_> =
                runtime.begin_transaction(Default::default());
            transaction.push_batch(
                WorkerIntentBatch::new("same-value-distinct-record-causal-twin")
                    .push(first_intent)
                    .push(MutationIntent::Create(CreateIntent::Entity(second_spec))),
            );
            let committed = transaction.commit().expect("both outboxes commit");
            let first_binding = WorthQueryCommittedDispatchOutboxBinding::fixture_from_commit(
                provider.graph.layout.provider_dispatch_outbox(),
                Some(first_pending.record()),
                &committed,
            )
            .unwrap()
            .unwrap();
            let second_ref = RecordRef::Entity(
                committed
                    .created_entity(&second_created)
                    .expect("second create reference resolves independently"),
            );
            let second_binding =
                WorthQueryCommittedDispatchOutboxBinding::fixture(record.clone(), second_ref);
            let snapshot = runtime.snapshots().snapshot_for_branch(&branch).unwrap();
            let runtime_id = snapshot.runtime_instance_id;
            runtime.snapshots().release_snapshot(&snapshot);
            (
                first_binding,
                second_binding,
                committed.outcome().commit.clone(),
                runtime_id,
            )
        });
    let first_ref = first_binding.record_ref().clone();
    let second_ref = second_binding.record_ref().clone();
    let first = provider
        .observe_expected(&first_binding, &commit, runtime_id)
        .expect("first exact owner observation");
    let second = provider
        .observe_expected(&second_binding, &commit, runtime_id)
        .expect("second exact owner observation");
    let first = world
        .application
        .admit_external_dispatch_attempt(first)
        .expect("first record dispatch admitted");
    let second = world
        .application
        .admit_external_dispatch_attempt(second)
        .expect("second record dispatch admitted");
    (first, second, first_ref, second_ref)
}

pub(in crate::domain_computation::primary_graph) fn commit_and_observe_fixture(
    provider: &WorthQueryPrimaryGraphProvider,
    record: &WorthQueryDispatchOutboxRecord,
) -> WorthQueryCommittedDispatchOutboxObservation {
    let branch = primary_relational_branch_id();
    let (binding, commit, runtime_id) = provider.graph.with_runtime_mut(|runtime| {
        let (intent, pending) = bind_dispatch_outbox_create_intent(
            Some(provider.graph.layout.provider_dispatch_outbox()),
            Some(record),
        )
        .expect("declared fixture outbox binds a create intent");
        let mut transaction: RelationalTransaction<'_> =
            runtime.begin_transaction(Default::default());
        transaction.push_batch(WorkerIntentBatch::new("committed-outbox-real-test").push(intent));
        let committed = transaction.commit().expect("fixture outbox commits");
        let binding = WorthQueryCommittedDispatchOutboxBinding::fixture_from_commit(
            provider.graph.layout.provider_dispatch_outbox(),
            Some(pending.record()),
            &committed,
        )
        .unwrap_or_else(|denial| {
            panic!(
                "owner mapping resolves: {denial}; requested={:?}",
                pending.created_entity()
            )
        })
        .expect("declared outbox has a binding");
        let snapshot = runtime
            .snapshots()
            .snapshot_for_branch(&branch)
            .expect("fixture branch has a snapshot");
        let runtime_id = snapshot.runtime_instance_id;
        runtime.snapshots().release_snapshot(&snapshot);
        (binding, committed.outcome().commit.clone(), runtime_id)
    });
    provider
        .observe_expected(&binding, &commit, runtime_id)
        .expect("real committed outbox is owner-observable")
}
