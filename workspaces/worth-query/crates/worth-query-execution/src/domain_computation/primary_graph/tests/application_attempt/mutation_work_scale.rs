use std::collections::BTreeMap;

use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntitySpec, MutationIntent, WorkerIntentBatch,
};

use super::super::fixture::{Account, AccountIdentity, AccountLabel, AccountStatus};
use super::{
    admitted_program, authenticated_principal, idempotency, installed_authorization_world,
    live_scope, resolved_account, WorthQueryApplicationCommitOutcome,
};

#[test]
fn mutation_work_is_invariant_to_unrelated_graph_population() {
    let baseline = mutation_work(0, 71);
    let expanded = mutation_work(128, 72);

    assert_eq!(
        expanded.decision_fact_count(),
        baseline.decision_fact_count()
    );
    assert_eq!(
        expanded.proposed_fact_count(),
        baseline.proposed_fact_count()
    );
    assert_eq!(
        expanded.invariant_state_fact_count(),
        baseline.invariant_state_fact_count()
    );
    assert_eq!(
        expanded.invariant_work_units(),
        baseline.invariant_work_units()
    );
    assert_eq!(
        expanded.relational_invariant_execution_count(),
        baseline.relational_invariant_execution_count()
    );
    assert_eq!(
        expanded.relational_invariant_result_count(),
        baseline.relational_invariant_result_count()
    );
    assert!(baseline.decision_fact_count() > 0);
    assert!(baseline.proposed_fact_count() > 0);
    assert_eq!(
        baseline.invariant_state_fact_count(),
        baseline.proposed_fact_count()
    );
    assert_eq!(
        baseline.invariant_work_units(),
        baseline.proposed_fact_count() as u64
    );
    assert_eq!(baseline.relational_invariant_execution_count(), 3);
    assert!(baseline.relational_invariant_result_count() > 0);
    // C2 — commit-derived names are present. Absolute EntityIds differ across
    // separately populated worlds; the *count* of records this mutation
    // touched must not grow with unrelated graph width.
    assert!(!baseline.touched_records().is_empty());
    assert_eq!(
        expanded.touched_record_count(),
        baseline.touched_record_count()
    );
}

fn mutation_work(
    unrelated_accounts: usize,
    idempotency_key: u8,
) -> super::super::super::provider::WorthQueryPrimaryMutationWorkEvidence {
    let world = installed_authorization_world(true);
    grow_unrelated_accounts(&world, unrelated_accounts);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let program = admitted_program(&world, &principal, &account, &request, "scale-committed");
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(
                program,
                idempotency(idempotency_key, idempotency_key),
            ),
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
    world
        .application
        .completed_mutation_work()
        .expect("committed mutation records its phase-separated work")
}

fn grow_unrelated_accounts(world: &super::super::fixture::AuthorizationWorld, count: usize) {
    if count == 0 {
        return;
    }
    let graph = world.application.primary_provider.graph.clone();
    let kind = graph
        .layout
        .entity_kind(Account::reference().name())
        .expect("account kind is installed");
    let locator = |entity: &str, aspect: &str, field: &str| {
        graph
            .layout
            .field_locator(entity, aspect, field)
            .expect("scale field is installed")
            .clone()
    };
    let identity_ref = AccountIdentity::reference();
    let status_ref = AccountStatus::reference();
    let label_ref = AccountLabel::reference();
    let identity = locator(
        identity_ref.entity(),
        identity_ref.aspect(),
        identity_ref.field(),
    );
    let status = locator(status_ref.entity(), status_ref.aspect(), status_ref.field());
    let label = locator(label_ref.entity(), label_ref.aspect(), label_ref.field());
    graph.with_runtime_mut(|runtime| {
        let batch = (0..count).fold(
            WorkerIntentBatch::new("unrelated-mutation-scale-population"),
            |batch, ordinal| {
                let key = format!("unrelated-scale-{ordinal}");
                let fields = AspectFieldPatch::from(BTreeMap::from([
                    (identity.clone(), key.clone().into_foundational_value()),
                    (
                        status.clone(),
                        "unrelated".to_owned().into_foundational_value(),
                    ),
                    (
                        label.clone(),
                        "population".to_owned().into_foundational_value(),
                    ),
                ]));
                batch.push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                    partition_id: worth_relational::facade::identity::PartitionId::main(),
                    kind_id: kind,
                    client_key: worth_relational::facade::symbols::ClientKey::raw(key),
                    fields,
                })))
            },
        );
        let mut transaction = runtime.begin_transaction(Default::default());
        transaction.push_batch(batch);
        transaction.commit().expect("unrelated population commits");
        graph.ensure_primary_indexes_current(runtime).unwrap();
    });
}
