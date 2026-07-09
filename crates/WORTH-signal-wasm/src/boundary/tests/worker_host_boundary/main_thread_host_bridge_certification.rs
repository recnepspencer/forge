use crate::boundary::types::SignalWorkerRuntime;
use crate::expression::model::{Expr, IdentitySpec, SignalValue};
use crate::recipe::model::{RecipeReadSpec, RecipeSpec, SourceSpec, TransactionOp};
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::worker_host::{
    WorkerBrowserHistoryIngress, WorkerHostCapabilityBoundaryArtifact,
    WorkerHostCapabilityIngressBatch, WorkerHostCapabilityUpdate, WorkerHostEffectAcknowledgement,
    WorkerHostEffectOutcome, WorkerHostEffectRequest, WorkerHostEffectRequestEnvelope,
    WorkerPortableGraphPublication, WorkerRuntimeShell,
};

#[test]
fn worker_main_thread_host_bridge_certification_packages_suite_evidence() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();
    worker_runtime
        .publish_portable_graph_for_test(main_thread_host_bridge_publication())
        .unwrap();

    let host_capability_report = worker_runtime
        .admit_host_capability_ingress_for_test(WorkerHostCapabilityIngressBatch {
            updates: vec![
                WorkerHostCapabilityUpdate {
                    family: "visibility".to_owned(),
                    registration_id: "documentVisibility".to_owned(),
                    semantic_value_identity: "hidden".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                    runtime_source_id: Some("documentVisibility".to_owned()),
                    runtime_value: Some(SignalValue::String("hidden".to_owned())),
                },
                WorkerHostCapabilityUpdate {
                    family: "online".to_owned(),
                    registration_id: "navigatorOnline".to_owned(),
                    semantic_value_identity: "unavailable".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Unavailable,
                    runtime_source_id: None,
                    runtime_value: None,
                },
            ],
        })
        .unwrap();
    let browser_history_report = worker_runtime
        .admit_browser_history_ingress_for_test(WorkerBrowserHistoryIngress {
            navigation_kind: "popstate".to_owned(),
            raw_location: "/search?q=WORTH".to_owned(),
            route_identity: "searchRoute:WORTH".to_owned(),
            runtime_route_source_id: Some("routeIdentity".to_owned()),
            route_value: Some(SignalValue::String("searchRoute:WORTH".to_owned())),
            runtime_continuity_source_id: Some("routeContinuity".to_owned()),
            continuity_value: Some(SignalValue::String("restored".to_owned())),
        })
        .unwrap();
    let host_effect_request = worker_runtime
        .issue_host_effect_request_for_test(WorkerHostEffectRequest {
            effect_id: "focusSearchInput".to_owned(),
            host_capability_family: "domFocus".to_owned(),
            closed_payload_identity: "focusSearchInputPayload".to_owned(),
        })
        .unwrap();
    let host_effect_acknowledgement = worker_runtime
        .admit_host_effect_acknowledgement_for_test(WorkerHostEffectAcknowledgement {
            request_digest: host_effect_request.request_digest.clone(),
            outcome: WorkerHostEffectOutcome::Unavailable,
            artifact_identity: "searchInputUnavailable".to_owned(),
            runtime_lifecycle_source_id: None,
            lifecycle_value: None,
        })
        .unwrap();

    let package = worker_runtime
        .certify_main_thread_host_bridge_for_test()
        .unwrap();

    assert_eq!(
        package.certification_family,
        "mainThreadHostBridgeCertification"
    );
    assert_eq!(package.covered_suite_count, 3);
    assert_eq!(
        package.host_capability_envelope_digest,
        host_capability_report.host_capability_envelope_digest
    );
    assert_eq!(
        package.browser_history_replay_restore_digest,
        browser_history_report.replay_restore_digest
    );
    assert_eq!(
        package.host_effect_lifecycle_artifact,
        "hostEffectUnavailable"
    );
    assert_eq!(
        package.worth_proof_readmission_digest,
        host_effect_acknowledgement.worth_proof_readmission_digest
    );
    assert_eq!(
        package.host_effect_acknowledged_request_digest,
        host_effect_request.request_digest
    );
    assert!(package.ambient_host_read_denied);
    assert!(!package.host_acknowledgement_is_authoritative);
    assert_digest_shape(&package.host_boundary_causality_digest);
    assert_digest_shape(&package.boundary_performance_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn worker_main_thread_host_bridge_certification_rejects_mismatched_host_effect_acknowledgement() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    worker_shell
        .publish_graph(main_thread_host_bridge_publication())
        .unwrap();

    admit_hidden_visibility_host_fact(&mut worker_shell);
    admit_search_route_history_fact(&mut worker_shell);
    issue_search_focus_host_effect_request(&mut worker_shell);
    admit_denied_search_focus_acknowledgement(
        &mut worker_shell,
        "differentHostEffectRequest".to_owned(),
    );

    let error = worker_shell.certify_main_thread_host_bridge().unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error.message.contains("acknowledgement request digest"));
}

#[test]
fn worker_main_thread_host_bridge_certification_rejects_stale_runtime_truth_evidence() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    worker_shell
        .publish_graph(main_thread_host_bridge_publication())
        .unwrap();

    admit_hidden_visibility_host_fact(&mut worker_shell);
    admit_search_route_history_fact(&mut worker_shell);
    let host_effect_request = issue_search_focus_host_effect_request(&mut worker_shell);
    admit_denied_search_focus_acknowledgement(
        &mut worker_shell,
        host_effect_request.request_digest,
    );
    worker_shell.certify_main_thread_host_bridge().unwrap();

    worker_shell
        .apply_committed_transaction(vec![TransactionOp::Set {
            id: "documentVisibility".to_owned(),
            value: SignalValue::String("visible".to_owned()),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    let error = worker_shell.certify_main_thread_host_bridge().unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error.message.contains("host capability ingress evidence"));
}

#[test]
fn worker_main_thread_host_bridge_certification_rejects_unordered_evidence() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    worker_shell
        .publish_graph(main_thread_host_bridge_publication())
        .unwrap();

    let _host_capability_report = worker_shell
        .admit_host_capability_ingress(WorkerHostCapabilityIngressBatch {
            updates: vec![WorkerHostCapabilityUpdate {
                family: "visibility".to_owned(),
                registration_id: "documentVisibility".to_owned(),
                semantic_value_identity: "hidden".to_owned(),
                boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                runtime_source_id: Some("documentVisibility".to_owned()),
                runtime_value: Some(SignalValue::String("hidden".to_owned())),
            }],
        })
        .unwrap();
    let host_effect_request = worker_shell
        .issue_host_effect_request(WorkerHostEffectRequest {
            effect_id: "focusSearchInput".to_owned(),
            host_capability_family: "domFocus".to_owned(),
            closed_payload_identity: "focusSearchInputPayload".to_owned(),
        })
        .unwrap();
    let _browser_history_report = worker_shell
        .admit_browser_history_ingress(WorkerBrowserHistoryIngress {
            navigation_kind: "push".to_owned(),
            raw_location: "/search".to_owned(),
            route_identity: "searchRoute".to_owned(),
            runtime_route_source_id: Some("routeIdentity".to_owned()),
            route_value: Some(SignalValue::String("searchRoute".to_owned())),
            runtime_continuity_source_id: None,
            continuity_value: None,
        })
        .unwrap();
    let _host_effect_acknowledgement = worker_shell
        .admit_host_effect_acknowledgement(WorkerHostEffectAcknowledgement {
            request_digest: host_effect_request.request_digest.clone(),
            outcome: WorkerHostEffectOutcome::Denied,
            artifact_identity: "searchInputDenied".to_owned(),
            runtime_lifecycle_source_id: None,
            lifecycle_value: None,
        })
        .unwrap();

    let error = worker_shell.certify_main_thread_host_bridge().unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error.message.contains("monotonically ordered"));
}

fn main_thread_host_bridge_publication() -> WorkerPortableGraphPublication {
    WorkerPortableGraphPublication {
        policy: RuntimePolicySpec::default(),
        sources: vec![
            SourceSpec {
                id: "documentVisibility".to_owned(),
                initial: SignalValue::String("visible".to_owned()),
                produces_aspects: None,
            },
            SourceSpec {
                id: "routeIdentity".to_owned(),
                initial: SignalValue::String("homeRoute".to_owned()),
                produces_aspects: None,
            },
            SourceSpec {
                id: "routeContinuity".to_owned(),
                initial: SignalValue::String("fresh".to_owned()),
                produces_aspects: None,
            },
        ],
        recipes: vec![
            RecipeSpec {
                id: "visibilityStatus".to_owned(),
                reads: vec![RecipeReadSpec::LegacyId("documentVisibility".to_owned())],
                expr: Expr::Read {
                    id: "documentVisibility".to_owned(),
                },
                when: None,
                identity: Some(IdentitySpec::Exact),
                produces_aspects: None,
            },
            RecipeSpec {
                id: "activeRouteProjection".to_owned(),
                reads: vec![
                    RecipeReadSpec::LegacyId("routeIdentity".to_owned()),
                    RecipeReadSpec::LegacyId("routeContinuity".to_owned()),
                ],
                expr: Expr::Read {
                    id: "routeIdentity".to_owned(),
                },
                when: None,
                identity: Some(IdentitySpec::Exact),
                produces_aspects: None,
            },
        ],
        output_ids: Vec::new(),
    }
}

fn admit_hidden_visibility_host_fact(worker_shell: &mut WorkerRuntimeShell) {
    worker_shell
        .admit_host_capability_ingress(WorkerHostCapabilityIngressBatch {
            updates: vec![WorkerHostCapabilityUpdate {
                family: "visibility".to_owned(),
                registration_id: "documentVisibility".to_owned(),
                semantic_value_identity: "hidden".to_owned(),
                boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                runtime_source_id: Some("documentVisibility".to_owned()),
                runtime_value: Some(SignalValue::String("hidden".to_owned())),
            }],
        })
        .unwrap();
}

fn admit_search_route_history_fact(worker_shell: &mut WorkerRuntimeShell) {
    worker_shell
        .admit_browser_history_ingress(WorkerBrowserHistoryIngress {
            navigation_kind: "push".to_owned(),
            raw_location: "/search".to_owned(),
            route_identity: "searchRoute".to_owned(),
            runtime_route_source_id: Some("routeIdentity".to_owned()),
            route_value: Some(SignalValue::String("searchRoute".to_owned())),
            runtime_continuity_source_id: None,
            continuity_value: None,
        })
        .unwrap();
}

fn issue_search_focus_host_effect_request(
    worker_shell: &mut WorkerRuntimeShell,
) -> WorkerHostEffectRequestEnvelope {
    worker_shell
        .issue_host_effect_request(WorkerHostEffectRequest {
            effect_id: "focusSearchInput".to_owned(),
            host_capability_family: "domFocus".to_owned(),
            closed_payload_identity: "focusSearchInputPayload".to_owned(),
        })
        .unwrap()
}

fn admit_denied_search_focus_acknowledgement(
    worker_shell: &mut WorkerRuntimeShell,
    request_digest: String,
) {
    worker_shell
        .admit_host_effect_acknowledgement(WorkerHostEffectAcknowledgement {
            request_digest,
            outcome: WorkerHostEffectOutcome::Denied,
            artifact_identity: "searchInputDenied".to_owned(),
            runtime_lifecycle_source_id: None,
            lifecycle_value: None,
        })
        .unwrap();
}

fn assert_digest_shape(digest: &str) {
    assert_eq!(digest.len(), 64);
    assert!(digest
        .chars()
        .all(|character| character.is_ascii_hexdigit()));
}
