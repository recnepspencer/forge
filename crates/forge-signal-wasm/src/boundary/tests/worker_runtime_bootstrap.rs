use crate::boundary::types::SignalWorkerRuntime;
use crate::expression::model::{Expr, IdentitySpec, SignalValue};
use crate::recipe::model::{RecipeReadSpec, RecipeSpec, SourceSpec, TransactionOp};
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::worker_host::WorkerPortableGraphPublication;

#[test]
fn worker_runtime_bootstrap_exposes_worker_first_deployment_posture() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let bootstrap = worker_runtime.bootstrap_record_for_test().unwrap();

    assert_eq!(
        bootstrap.shell_lock.identity.deployment_posture,
        "workerFirst"
    );
    assert_eq!(
        bootstrap.shell_lock.identity.runtime_authority,
        "workerOwnedRuntime"
    );
    assert_eq!(
        bootstrap.shell_lock.graph_publication_admission,
        "portableDefinitionsOnly"
    );
    assert_eq!(bootstrap.boundary_surface, "workerFirstConstruction");
    assert_eq!(
        bootstrap.transport_posture,
        "inProcessBootstrapBeforeTransportBridge"
    );
    assert_eq!(bootstrap.host_capability_ingress, "deferredToHostBridge");
    assert_eq!(bootstrap.host_effect_egress, "deferredToHostBridge");
}

#[test]
fn worker_runtime_boundary_publishes_portable_graph_through_committed_envelope() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let publication = WorkerPortableGraphPublication {
        policy: RuntimePolicySpec::default(),
        sources: vec![SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        }],
        recipes: vec![RecipeSpec {
            id: "doubleCounter".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("counter".to_owned())],
            expr: Expr::Sum {
                args: vec![
                    Expr::Read {
                        id: "counter".to_owned(),
                    },
                    Expr::Read {
                        id: "counter".to_owned(),
                    },
                ],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        }],
    };

    let summary = worker_runtime
        .publish_portable_graph_for_test(publication)
        .unwrap();
    let envelope = worker_runtime
        .apply_transaction_for_test(vec![TransactionOp::Set {
            id: "counter".to_owned(),
            value: SignalValue::Number(8.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    assert_eq!(summary.published_source_count, 1);
    assert_eq!(summary.published_recipe_count, 1);
    assert_eq!(summary.denied_callback_count, 0);
    assert_eq!(envelope.deployment_posture, "workerFirst");
    assert_eq!(envelope.envelope_family, "transactionResult");
    assert_eq!(envelope.run_summary.touched_nodes, 2);
}
