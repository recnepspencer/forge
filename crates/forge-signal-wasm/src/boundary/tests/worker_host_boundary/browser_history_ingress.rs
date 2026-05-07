use crate::boundary::types::SignalWorkerRuntime;
use crate::expression::model::{Expr, IdentitySpec, SignalValue};
use crate::recipe::model::{RecipeReadSpec, RecipeSpec, SetValue, SourceSpec, TransactionOp};
use crate::runtime::core::RuntimeCore;
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::worker_host::{
    committed_truth_digest_for_runtime, WorkerBrowserHistoryIngress,
    WorkerPortableGraphPublication, WorkerRuntimeShell,
};

#[test]
fn worker_browser_history_ingress_preserves_typed_route_boundary() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let report = worker_runtime
        .admit_browser_history_ingress_for_test(WorkerBrowserHistoryIngress {
            navigation_kind: "popstate".to_owned(),
            raw_location: "/inventory?tab=stock".to_owned(),
            route_identity: "inventoryRoute".to_owned(),
            runtime_route_source_id: None,
            route_value: None,
            runtime_continuity_source_id: None,
            continuity_value: None,
        })
        .unwrap();

    assert_eq!(report.envelope_family, "browserHistoryIngress");
    assert_eq!(report.causality.transaction_sequence, 0);
    assert_eq!(report.performance.bridge_envelope_count, 1);
    assert_eq!(report.performance.submitted_item_count, 1);
    assert_eq!(report.runtime_admitted_route_count, 0);
    assert_eq!(report.runtime_admitted_continuity_count, 0);
    assert_eq!(report.runtime_mutation_breadth, 0);
    assert_eq!(report.performance.runtime_mutation_breadth, 0);
    assert_eq!(report.performance.ambient_worker_read_count, 0);
    assert!(report.performance.payload_identity_byte_count > 0);
    assert!(report.ambient_location_read_denied);
    assert_digest_shape(&report.browser_history_envelope_digest);
    assert_digest_shape(&report.route_truth_digest);
    assert_digest_shape(&report.continuity_digest);
    assert_digest_shape(&report.replay_restore_digest);
    assert_digest_shape(&report.worker_first_truth_digest);
    assert_digest_shape(&report.performance.performance_digest);
}

#[test]
fn worker_browser_history_ingress_matches_compatibility_route_truth() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();
    worker_runtime
        .publish_portable_graph_for_test(route_publication())
        .unwrap();
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    define_route_graph(&mut compatibility_runtime);

    let report = worker_runtime
        .admit_browser_history_ingress_for_test(WorkerBrowserHistoryIngress {
            navigation_kind: "push".to_owned(),
            raw_location: "/inventory/42".to_owned(),
            route_identity: "inventoryDetailRoute".to_owned(),
            runtime_route_source_id: Some("currentRoute".to_owned()),
            route_value: Some(SignalValue::String("/inventory/42".to_owned())),
            runtime_continuity_source_id: None,
            continuity_value: None,
        })
        .unwrap();
    compatibility_runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "currentRoute".to_owned(),
            value: SignalValue::String("/inventory/42".to_owned()),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    assert_eq!(report.runtime_admitted_route_count, 1);
    assert_eq!(report.runtime_admitted_continuity_count, 0);
    assert!(report.runtime_mutation_breadth >= 1);
    assert_eq!(report.performance.runtime_admitted_item_count, 1);
    assert_eq!(
        report.worker_first_truth_digest,
        committed_truth_digest_for_runtime(&compatibility_runtime).unwrap()
    );
}

#[test]
fn worker_browser_history_ingress_preserves_route_continuity_after_restore() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    worker_shell.publish_graph(route_publication()).unwrap();
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    define_route_graph(&mut compatibility_runtime);

    let worker_main = worker_shell.branch_truth_envelope().unwrap();
    let compatibility_main = compatibility_runtime.current_branch();
    let worker_feature = worker_shell
        .create_branch("route-continuity".to_owned())
        .unwrap();
    let compatibility_feature = compatibility_runtime
        .create_branch("route-continuity".to_owned())
        .unwrap();

    worker_shell.switch_branch(worker_feature.id.0).unwrap();
    compatibility_runtime
        .switch_branch(compatibility_feature.id.0)
        .unwrap();
    let feature_report = worker_shell
        .admit_browser_history_ingress(route_continuity_ingress(
            "push",
            "/inventory/42",
            "inventoryDetailRoute",
            "detailResource:42",
        ))
        .unwrap();
    apply_route_continuity_transaction(
        &mut compatibility_runtime,
        "/inventory/42",
        "detailResource:42",
    );
    let worker_feature_snapshot = worker_shell.branch_snapshot(worker_feature.id.0).unwrap();
    let compatibility_feature_snapshot = compatibility_runtime
        .branch_snapshot(compatibility_feature.id.0)
        .unwrap();

    worker_shell.switch_branch(worker_main.branch_id).unwrap();
    compatibility_runtime
        .switch_branch(compatibility_main.id.0)
        .unwrap();
    worker_shell
        .admit_browser_history_ingress(route_continuity_ingress(
            "replace",
            "/inventory",
            "inventoryListRoute",
            "listResource",
        ))
        .unwrap();
    apply_route_continuity_transaction(&mut compatibility_runtime, "/inventory", "listResource");

    let restored_feature = worker_shell
        .restore_branch_snapshot(worker_feature.id.0, worker_feature_snapshot)
        .unwrap();
    compatibility_runtime
        .restore_branch_snapshot(compatibility_feature.id.0, compatibility_feature_snapshot)
        .unwrap();
    assert_eq!(
        restored_feature.committed_truth_digest,
        compatibility_runtime
            .branch_state_proof(compatibility_feature.id.0)
            .unwrap()
            .state_digest
    );

    worker_shell.switch_branch(worker_feature.id.0).unwrap();
    compatibility_runtime
        .switch_branch(compatibility_feature.id.0)
        .unwrap();
    let post_restore_report = worker_shell
        .admit_browser_history_ingress(route_continuity_ingress(
            "popstate",
            "/inventory/43",
            "inventoryDetailRoute",
            "detailResource:43",
        ))
        .unwrap();
    apply_route_continuity_transaction(
        &mut compatibility_runtime,
        "/inventory/43",
        "detailResource:43",
    );

    assert_eq!(feature_report.runtime_admitted_route_count, 1);
    assert_eq!(feature_report.runtime_admitted_continuity_count, 1);
    assert_eq!(feature_report.performance.runtime_admitted_item_count, 2);
    assert_digest_shape(&feature_report.replay_restore_digest);
    assert_eq!(post_restore_report.runtime_admitted_route_count, 1);
    assert_eq!(post_restore_report.runtime_admitted_continuity_count, 1);
    assert_eq!(
        post_restore_report.performance.runtime_admitted_item_count,
        2
    );
    assert_digest_shape(&post_restore_report.replay_restore_digest);
    assert_eq!(
        post_restore_report.worker_first_truth_digest,
        committed_truth_digest_for_runtime(&compatibility_runtime).unwrap()
    );
    assert_eq!(
        worker_shell.read_value("routeContinuityToken").unwrap(),
        SignalValue::String("detailResource:43".to_owned())
    );
}

#[test]
fn worker_browser_history_ingress_rejects_unpaired_runtime_route_source() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let error = worker_shell
        .admit_browser_history_ingress(WorkerBrowserHistoryIngress {
            navigation_kind: "replace".to_owned(),
            raw_location: "/inventory".to_owned(),
            route_identity: "inventoryRoute".to_owned(),
            runtime_route_source_id: Some("currentRoute".to_owned()),
            route_value: None,
            runtime_continuity_source_id: None,
            continuity_value: None,
        })
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error.message.contains("paired runtime route source id"));
}

#[test]
fn worker_browser_history_ingress_rejects_unpaired_route_continuity() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let error = worker_shell
        .admit_browser_history_ingress(WorkerBrowserHistoryIngress {
            navigation_kind: "popstate".to_owned(),
            raw_location: "/inventory/42".to_owned(),
            route_identity: "inventoryDetailRoute".to_owned(),
            runtime_route_source_id: Some("currentRoute".to_owned()),
            route_value: Some(SignalValue::String("/inventory/42".to_owned())),
            runtime_continuity_source_id: Some("routeContinuityToken".to_owned()),
            continuity_value: None,
        })
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error
        .message
        .contains("paired runtime continuity source id"));
}

fn assert_digest_shape(digest: &str) {
    assert_eq!(digest.len(), 64);
    assert!(digest
        .chars()
        .all(|character| character.is_ascii_hexdigit()));
}

fn route_publication() -> WorkerPortableGraphPublication {
    WorkerPortableGraphPublication {
        policy: RuntimePolicySpec::default(),
        sources: vec![
            SourceSpec {
                id: "currentRoute".to_owned(),
                initial: SignalValue::String("/".to_owned()),
                produces_aspects: None,
            },
            SourceSpec {
                id: "routeContinuityToken".to_owned(),
                initial: SignalValue::String("listResource".to_owned()),
                produces_aspects: None,
            },
        ],
        recipes: vec![
            RecipeSpec {
                id: "routeProjection".to_owned(),
                reads: vec![RecipeReadSpec::LegacyId("currentRoute".to_owned())],
                expr: Expr::Read {
                    id: "currentRoute".to_owned(),
                },
                when: None,
                identity: Some(IdentitySpec::Exact),
                produces_aspects: None,
            },
            RecipeSpec {
                id: "routeContinuityProjection".to_owned(),
                reads: vec![RecipeReadSpec::LegacyId("routeContinuityToken".to_owned())],
                expr: Expr::Read {
                    id: "routeContinuityToken".to_owned(),
                },
                when: None,
                identity: Some(IdentitySpec::Exact),
                produces_aspects: None,
            },
        ],
    }
}

fn define_route_graph(runtime: &mut RuntimeCore) {
    runtime
        .define_source(SourceSpec {
            id: "currentRoute".to_owned(),
            initial: SignalValue::String("/".to_owned()),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "routeContinuityToken".to_owned(),
            initial: SignalValue::String("listResource".to_owned()),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "routeProjection".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("currentRoute".to_owned())],
            expr: Expr::Read {
                id: "currentRoute".to_owned(),
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "routeContinuityProjection".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("routeContinuityToken".to_owned())],
            expr: Expr::Read {
                id: "routeContinuityToken".to_owned(),
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();
}

fn route_continuity_ingress(
    navigation_kind: &str,
    raw_location: &str,
    route_identity: &str,
    continuity_token: &str,
) -> WorkerBrowserHistoryIngress {
    WorkerBrowserHistoryIngress {
        navigation_kind: navigation_kind.to_owned(),
        raw_location: raw_location.to_owned(),
        route_identity: route_identity.to_owned(),
        runtime_route_source_id: Some("currentRoute".to_owned()),
        route_value: Some(SignalValue::String(raw_location.to_owned())),
        runtime_continuity_source_id: Some("routeContinuityToken".to_owned()),
        continuity_value: Some(SignalValue::String(continuity_token.to_owned())),
    }
}

fn apply_route_continuity_transaction(
    runtime: &mut RuntimeCore,
    raw_location: &str,
    continuity_token: &str,
) {
    runtime
        .apply_transaction(vec![TransactionOp::SetMany {
            values: vec![
                SetValue {
                    id: "currentRoute".to_owned(),
                    value: SignalValue::String(raw_location.to_owned()),
                    aspect: None,
                    aspects: None,
                },
                SetValue {
                    id: "routeContinuityToken".to_owned(),
                    value: SignalValue::String(continuity_token.to_owned()),
                    aspect: None,
                    aspects: None,
                },
            ],
        }])
        .unwrap();
}
