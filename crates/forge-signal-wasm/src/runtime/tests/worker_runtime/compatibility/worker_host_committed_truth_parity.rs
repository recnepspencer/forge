use crate::runtime::worker_host::{WorkerCompatibilityTruthReport, WorkerRuntimeShell};

use crate::runtime::tests::support::*;
use crate::runtime::tests::worker_runtime::fixtures::portable_counter_graph::{
    define_portable_counter_graph, portable_counter_publication,
};

#[test]
fn worker_runtime_shell_matches_compatibility_committed_truth_for_portable_graph() {
    let publication = portable_counter_publication();
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();

    let worker_publication = worker_shell.publish_graph(publication.clone()).unwrap();
    define_portable_counter_graph(&mut compatibility_runtime);

    assert_eq!(worker_publication.published_source_count, 1);
    assert_eq!(worker_publication.published_recipe_count, 1);

    let transaction = vec![TransactionOp::Set {
        id: "counter".to_owned(),
        value: SignalValue::Number(7.0),
        aspect: None,
        aspects: None,
    }];
    let worker_envelope = worker_shell
        .apply_committed_transaction(transaction.clone())
        .unwrap();
    compatibility_runtime
        .apply_transaction(transaction)
        .unwrap();
    let compatibility_digest = compatibility_runtime
        .branch_state_proof(compatibility_runtime.current_branch().id.0)
        .unwrap()
        .state_digest;

    let parity = WorkerCompatibilityTruthReport::compare(&worker_envelope, compatibility_digest);

    assert!(parity.committed_truth_matches);
    assert_eq!(parity.worker_envelope_family, "transactionResult");
    assert_eq!(
        compatibility_runtime.read_value("doubleCounter").unwrap(),
        SignalValue::Number(14.0)
    );
}
