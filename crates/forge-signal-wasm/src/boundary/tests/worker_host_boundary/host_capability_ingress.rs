use crate::boundary::types::SignalWorkerRuntime;
use crate::expression::model::{Expr, IdentitySpec, SignalValue};
use crate::recipe::model::{RecipeReadSpec, RecipeSpec, SourceSpec, TransactionOp};
use crate::runtime::core::RuntimeCore;
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::worker_host::{
    committed_truth_digest_for_runtime, WorkerHostCapabilityBoundaryArtifact,
    WorkerHostCapabilityIngressBatch, WorkerHostCapabilityUpdate, WorkerPortableGraphPublication,
    WorkerRuntimeShell,
};

#[test]
fn worker_host_capability_ingress_coalesces_by_capability_registration() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let report = worker_runtime
        .admit_host_capability_ingress_for_test(WorkerHostCapabilityIngressBatch {
            updates: vec![
                WorkerHostCapabilityUpdate {
                    family: "visibility".to_owned(),
                    registration_id: "documentVisibility".to_owned(),
                    semantic_value_identity: "visible".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                    runtime_source_id: None,
                    runtime_value: None,
                },
                WorkerHostCapabilityUpdate {
                    family: "visibility".to_owned(),
                    registration_id: "documentVisibility".to_owned(),
                    semantic_value_identity: "hidden".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                    runtime_source_id: None,
                    runtime_value: None,
                },
                WorkerHostCapabilityUpdate {
                    family: "online".to_owned(),
                    registration_id: "navigatorOnline".to_owned(),
                    semantic_value_identity: "online".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                    runtime_source_id: None,
                    runtime_value: None,
                },
            ],
        })
        .unwrap();

    assert_eq!(report.envelope_family, "hostCapabilityIngress");
    assert_eq!(report.submitted_update_count, 3);
    assert_eq!(report.submitted_admitted_update_count, 3);
    assert_eq!(report.submitted_stale_update_count, 0);
    assert_eq!(report.submitted_denied_update_count, 0);
    assert_eq!(report.submitted_detached_update_count, 0);
    assert_eq!(report.submitted_unavailable_update_count, 0);
    assert_eq!(report.coalesced_admitted_update_count, 2);
    assert_eq!(report.coalesced_update_count, 2);
    assert_eq!(report.coalesced_stale_update_count, 0);
    assert_eq!(report.coalesced_denied_update_count, 0);
    assert_eq!(report.coalesced_detached_update_count, 0);
    assert_eq!(report.coalesced_unavailable_update_count, 0);
    assert_eq!(report.runtime_admitted_update_count, 0);
    assert_eq!(report.runtime_mutation_breadth, 0);
    assert_eq!(report.performance.bridge_envelope_count, 1);
    assert_eq!(report.performance.submitted_item_count, 3);
    assert_eq!(report.performance.coalesced_item_count, 2);
    assert_eq!(report.performance.runtime_admitted_item_count, 0);
    assert_eq!(report.performance.runtime_mutation_breadth, 0);
    assert_eq!(report.performance.ambient_worker_read_count, 0);
    assert_eq!(report.causality.transaction_sequence, 0);
    assert_eq!(
        report.causality.ordering_basis,
        "transactionSequenceThenGeneration"
    );
    assert!(report.ambient_worker_read_denied);
    assert_digest_shape(&report.host_capability_envelope_digest);
    assert_digest_shape(&report.lifecycle_digest);
    assert_digest_shape(&report.truth_digest);
    assert_digest_shape(&report.worker_first_truth_digest);
    assert_digest_shape(&report.coalescing_digest);
    assert_digest_shape(&report.host_boundary_artifact_digest);
    assert_digest_shape(&report.performance.performance_digest);
}

#[test]
fn worker_host_capability_ingress_matches_compatibility_runtime_truth() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();
    worker_runtime
        .publish_portable_graph_for_test(host_capability_publication())
        .unwrap();
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    define_host_capability_graph(&mut compatibility_runtime);

    let report = worker_runtime
        .admit_host_capability_ingress_for_test(WorkerHostCapabilityIngressBatch {
            updates: vec![
                WorkerHostCapabilityUpdate {
                    family: "visibility".to_owned(),
                    registration_id: "documentVisibility".to_owned(),
                    semantic_value_identity: "visible".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                    runtime_source_id: Some("documentVisibility".to_owned()),
                    runtime_value: Some(SignalValue::String("visible".to_owned())),
                },
                WorkerHostCapabilityUpdate {
                    family: "visibility".to_owned(),
                    registration_id: "documentVisibility".to_owned(),
                    semantic_value_identity: "hidden".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                    runtime_source_id: Some("documentVisibility".to_owned()),
                    runtime_value: Some(SignalValue::String("hidden".to_owned())),
                },
            ],
        })
        .unwrap();
    compatibility_runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "documentVisibility".to_owned(),
            value: SignalValue::String("hidden".to_owned()),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    assert_eq!(report.submitted_update_count, 2);
    assert_eq!(report.runtime_admitted_update_count, 1);
    assert!(report.runtime_mutation_breadth >= 1);
    assert_eq!(report.performance.submitted_item_count, 2);
    assert_eq!(report.performance.coalesced_item_count, 1);
    assert_eq!(report.performance.runtime_admitted_item_count, 1);
    assert_eq!(
        report.worker_first_truth_digest,
        committed_truth_digest_for_runtime(&compatibility_runtime).unwrap()
    );
}

#[test]
fn worker_host_capability_ingress_reports_stale_detached_unavailable_artifacts() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let report = worker_runtime
        .admit_host_capability_ingress_for_test(WorkerHostCapabilityIngressBatch {
            updates: vec![
                WorkerHostCapabilityUpdate {
                    family: "visibility".to_owned(),
                    registration_id: "documentVisibility".to_owned(),
                    semantic_value_identity: "stale:hidden".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Stale,
                    runtime_source_id: None,
                    runtime_value: None,
                },
                WorkerHostCapabilityUpdate {
                    family: "viewport".to_owned(),
                    registration_id: "rootViewport".to_owned(),
                    semantic_value_identity: "detached".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Detached,
                    runtime_source_id: None,
                    runtime_value: None,
                },
                WorkerHostCapabilityUpdate {
                    family: "persistence".to_owned(),
                    registration_id: "localStorage".to_owned(),
                    semantic_value_identity: "unavailable".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Unavailable,
                    runtime_source_id: None,
                    runtime_value: None,
                },
            ],
        })
        .unwrap();

    assert_eq!(report.submitted_update_count, 3);
    assert_eq!(report.coalesced_update_count, 3);
    assert_eq!(report.submitted_admitted_update_count, 0);
    assert_eq!(report.submitted_stale_update_count, 1);
    assert_eq!(report.submitted_denied_update_count, 0);
    assert_eq!(report.submitted_detached_update_count, 1);
    assert_eq!(report.submitted_unavailable_update_count, 1);
    assert_eq!(report.coalesced_admitted_update_count, 0);
    assert_eq!(report.coalesced_stale_update_count, 1);
    assert_eq!(report.coalesced_denied_update_count, 0);
    assert_eq!(report.coalesced_detached_update_count, 1);
    assert_eq!(report.coalesced_unavailable_update_count, 1);
    assert_eq!(report.runtime_admitted_update_count, 0);
    assert_eq!(report.runtime_mutation_breadth, 0);
    assert_eq!(report.performance.coalesced_item_count, 3);
    assert_eq!(report.performance.runtime_admitted_item_count, 0);
    assert_digest_shape(&report.host_boundary_artifact_digest);
}

#[test]
fn worker_host_capability_ingress_preserves_overwritten_artifact_evidence() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let report = worker_runtime
        .admit_host_capability_ingress_for_test(WorkerHostCapabilityIngressBatch {
            updates: vec![
                WorkerHostCapabilityUpdate {
                    family: "visibility".to_owned(),
                    registration_id: "documentVisibility".to_owned(),
                    semantic_value_identity: "stale:hidden".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Stale,
                    runtime_source_id: None,
                    runtime_value: None,
                },
                WorkerHostCapabilityUpdate {
                    family: "visibility".to_owned(),
                    registration_id: "documentVisibility".to_owned(),
                    semantic_value_identity: "visible".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                    runtime_source_id: None,
                    runtime_value: None,
                },
            ],
        })
        .unwrap();

    assert_eq!(report.submitted_update_count, 2);
    assert_eq!(report.submitted_admitted_update_count, 1);
    assert_eq!(report.submitted_stale_update_count, 1);
    assert_eq!(report.coalesced_update_count, 1);
    assert_eq!(report.coalesced_admitted_update_count, 1);
    assert_eq!(report.coalesced_stale_update_count, 0);
    assert_digest_shape(&report.host_boundary_artifact_digest);
}

#[test]
fn worker_host_capability_ingress_rejects_unpaired_runtime_source() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let error = worker_shell
        .admit_host_capability_ingress(WorkerHostCapabilityIngressBatch {
            updates: vec![WorkerHostCapabilityUpdate {
                family: "visibility".to_owned(),
                registration_id: "documentVisibility".to_owned(),
                semantic_value_identity: "hidden".to_owned(),
                boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                runtime_source_id: Some("documentVisibility".to_owned()),
                runtime_value: None,
            }],
        })
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error.message.contains("paired runtime source id"));
}

#[test]
fn worker_host_capability_ingress_rejects_denied_runtime_mutation() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let error = worker_shell
        .admit_host_capability_ingress(WorkerHostCapabilityIngressBatch {
            updates: vec![WorkerHostCapabilityUpdate {
                family: "online".to_owned(),
                registration_id: "navigatorOnline".to_owned(),
                semantic_value_identity: "denied:online".to_owned(),
                boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Denied,
                runtime_source_id: Some("navigatorOnline".to_owned()),
                runtime_value: Some(SignalValue::String("online".to_owned())),
            }],
        })
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error
        .message
        .contains("non-admitted host capability artifacts"));
}

#[test]
fn worker_host_capability_ingress_rejects_malformed_artifact_before_coalescing() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let error = worker_shell
        .admit_host_capability_ingress(WorkerHostCapabilityIngressBatch {
            updates: vec![
                WorkerHostCapabilityUpdate {
                    family: "online".to_owned(),
                    registration_id: "navigatorOnline".to_owned(),
                    semantic_value_identity: "denied:online".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Denied,
                    runtime_source_id: Some("navigatorOnline".to_owned()),
                    runtime_value: Some(SignalValue::String("online".to_owned())),
                },
                WorkerHostCapabilityUpdate {
                    family: "online".to_owned(),
                    registration_id: "navigatorOnline".to_owned(),
                    semantic_value_identity: "online".to_owned(),
                    boundary_artifact: WorkerHostCapabilityBoundaryArtifact::Admitted,
                    runtime_source_id: Some("navigatorOnline".to_owned()),
                    runtime_value: Some(SignalValue::String("online".to_owned())),
                },
            ],
        })
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error
        .message
        .contains("non-admitted host capability artifacts"));
}

fn assert_digest_shape(digest: &str) {
    assert_eq!(digest.len(), 64);
    assert!(digest
        .chars()
        .all(|character| character.is_ascii_hexdigit()));
}

fn host_capability_publication() -> WorkerPortableGraphPublication {
    WorkerPortableGraphPublication {
        policy: RuntimePolicySpec::default(),
        sources: vec![SourceSpec {
            id: "documentVisibility".to_owned(),
            initial: SignalValue::String("visible".to_owned()),
            produces_aspects: None,
        }],
        recipes: vec![RecipeSpec {
            id: "visibilityStatus".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("documentVisibility".to_owned())],
            expr: Expr::Read {
                id: "documentVisibility".to_owned(),
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        }],
        output_ids: Vec::new(),
    }
}

fn define_host_capability_graph(runtime: &mut RuntimeCore) {
    runtime
        .define_source(SourceSpec {
            id: "documentVisibility".to_owned(),
            initial: SignalValue::String("visible".to_owned()),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "visibilityStatus".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("documentVisibility".to_owned())],
            expr: Expr::Read {
                id: "documentVisibility".to_owned(),
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();
}
