use worth_relational::facade::{
    mvcc::{RelationalPublicationOutcome, RelationalTransactionIntent},
    runtime::RelationalRuntimeApi,
    schema::RelationalSchemaRegistry,
    transactions::WorkerIntentBatch,
};

fn main() {
    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(RelationalSchemaRegistry::new())
        .build();

    // Choosing the main branch is explicit. Observation returns both a weak
    // transport descriptor and the owner-admitted basis used by live work.
    let identity = runtime.main_branch_identity();
    let (_descriptor, basis) = runtime
        .observe_branch(&identity)
        .expect("main branch must be observable");

    // Reads bind to the exact immutable observation carried by the basis.
    let pinned = runtime
        .snapshots()
        .snapshot_for_observation(&basis.observation())
        .expect("exact observation must open a pinned snapshot");
    runtime
        .snapshots()
        .release_snapshot(&pinned)
        .expect("pinned snapshot must release exactly once");

    // Transactions are detached and branch-bound. This empty batch keeps the
    // example focused on authority and publication rather than domain schema.
    let mut transaction = runtime
        .begin_branch_transaction(&basis, RelationalTransactionIntent::ordinary())
        .expect("exact basis must admit a branch transaction");
    transaction
        .push_batch(WorkerIntentBatch::new("branch-local-mvcc"))
        .expect("empty batch must stage within the declared transaction budget");

    let candidate = runtime
        .prepare_branch_transaction(transaction)
        .expect("preparation must validate without moving the branch");
    let performed = match runtime.publication_port().compare_and_publish(candidate) {
        RelationalPublicationOutcome::Performed(performed) => performed,
        outcome => panic!("unexpected publication outcome: {outcome:?}"),
    };

    // The owner settles durability and projections after the reference has
    // moved. Dropping performed evidence without settlement is not success.
    let committed = runtime
        .settle_performed_publication(performed)
        .expect("performed publication must settle through its owner");
    runtime
        .snapshots()
        .release_snapshot(&committed.snapshot)
        .expect("commit snapshot must release exactly once");
}
