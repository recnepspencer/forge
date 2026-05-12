use crate::boundary::types::SignalWorkerRuntime;
use crate::expression::model::{Expr, IdentitySpec, SignalValue};
use crate::recipe::model::{RecipeReadSpec, RecipeSpec, SourceSpec, TransactionOp};
use crate::runtime::core::RuntimeCore;
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::worker_host::{
    committed_truth_digest_for_runtime, WorkerHostEffectAcknowledgement, WorkerHostEffectOutcome,
    WorkerHostEffectRequest, WorkerPortableGraphPublication, WorkerRuntimeShell,
};

#[test]
fn worker_host_effect_acknowledgement_remains_non_authoritative_until_readmission() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let request = worker_runtime
        .issue_host_effect_request_for_test(WorkerHostEffectRequest {
            effect_id: "focusSearchInput".to_owned(),
            host_capability_family: "domFocus".to_owned(),
            closed_payload_identity: "focusSearchInputPayload".to_owned(),
        })
        .unwrap();
    let acknowledgement = worker_runtime
        .admit_host_effect_acknowledgement_for_test(WorkerHostEffectAcknowledgement {
            request_digest: request.request_digest.clone(),
            outcome: WorkerHostEffectOutcome::Unavailable,
            artifact_identity: "detachedSearchInput".to_owned(),
            runtime_lifecycle_source_id: None,
            lifecycle_value: None,
        })
        .unwrap();

    assert_eq!(request.envelope_family, "hostEffectEgress");
    assert_eq!(request.host_execution_boundary, "mainThreadHostEffect");
    assert_eq!(request.causality.transaction_sequence, 0);
    assert_eq!(request.performance.bridge_envelope_count, 1);
    assert_eq!(request.performance.submitted_item_count, 1);
    assert_eq!(request.performance.runtime_mutation_breadth, 0);
    assert_digest_shape(&request.request_digest);
    assert_digest_shape(&request.performance.performance_digest);
    assert_eq!(acknowledgement.envelope_family, "hostEffectEgress");
    assert_eq!(acknowledgement.causality.transaction_sequence, 1);
    assert_eq!(acknowledgement.performance.bridge_envelope_count, 1);
    assert_eq!(acknowledgement.performance.submitted_item_count, 1);
    assert_eq!(acknowledgement.runtime_admitted_lifecycle_count, 0);
    assert_eq!(acknowledgement.performance.runtime_mutation_breadth, 0);
    assert_eq!(acknowledgement.performance.runtime_admitted_item_count, 0);
    assert_eq!(
        acknowledgement.host_effect_lifecycle_artifact,
        "hostEffectUnavailable"
    );
    assert!(!acknowledgement.host_acknowledgement_is_authoritative);
    assert!(acknowledgement.worker_readmission_required);
    assert_digest_shape(&acknowledgement.acknowledgement_digest);
    assert_digest_shape(&acknowledgement.lifecycle_integrity_digest);
    assert_digest_shape(&acknowledgement.forge_proof_readmission_digest);
    assert_digest_shape(&acknowledgement.worker_first_truth_digest);
    assert_digest_shape(&acknowledgement.performance.performance_digest);
}

#[test]
fn worker_host_effect_acknowledgement_readmits_lifecycle_truth_through_forge_proof() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();
    worker_runtime
        .publish_portable_graph_for_test(effect_lifecycle_publication())
        .unwrap();
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    define_effect_lifecycle_graph(&mut compatibility_runtime);

    let request = worker_runtime
        .issue_host_effect_request_for_test(WorkerHostEffectRequest {
            effect_id: "focusSearchInput".to_owned(),
            host_capability_family: "domFocus".to_owned(),
            closed_payload_identity: "focusSearchInputPayload".to_owned(),
        })
        .unwrap();
    let acknowledgement = worker_runtime
        .admit_host_effect_acknowledgement_for_test(WorkerHostEffectAcknowledgement {
            request_digest: request.request_digest,
            outcome: WorkerHostEffectOutcome::Completed,
            artifact_identity: "focusedSearchInput".to_owned(),
            runtime_lifecycle_source_id: Some("searchFocusLifecycle".to_owned()),
            lifecycle_value: Some(SignalValue::String("completed".to_owned())),
        })
        .unwrap();
    compatibility_runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "searchFocusLifecycle".to_owned(),
            value: SignalValue::String("completed".to_owned()),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    assert_eq!(
        acknowledgement.host_effect_lifecycle_artifact,
        "hostEffectCompleted"
    );
    assert_eq!(acknowledgement.runtime_admitted_lifecycle_count, 1);
    assert!(acknowledgement.runtime_mutation_breadth >= 1);
    assert_eq!(acknowledgement.performance.runtime_admitted_item_count, 1);
    assert_eq!(
        acknowledgement.performance.runtime_mutation_breadth,
        acknowledgement.runtime_mutation_breadth
    );
    assert!(!acknowledgement.host_acknowledgement_is_authoritative);
    assert!(!acknowledgement.worker_readmission_required);
    assert_eq!(
        acknowledgement.worker_first_truth_digest,
        committed_truth_digest_for_runtime(&compatibility_runtime).unwrap()
    );
    assert_digest_shape(&acknowledgement.forge_proof_readmission_digest);
}

#[test]
fn worker_host_effect_acknowledgement_rejects_unpaired_lifecycle_readmission() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let error = worker_shell
        .admit_host_effect_acknowledgement(WorkerHostEffectAcknowledgement {
            request_digest: "hostRequestDigest".to_owned(),
            outcome: WorkerHostEffectOutcome::Failed,
            artifact_identity: "focusFailure".to_owned(),
            runtime_lifecycle_source_id: Some("searchFocusLifecycle".to_owned()),
            lifecycle_value: None,
        })
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error.message.contains("paired runtime lifecycle source id"));
}

#[test]
fn worker_host_effect_acknowledgement_rejects_detached_lifecycle_readmission() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let error = worker_shell
        .admit_host_effect_acknowledgement(WorkerHostEffectAcknowledgement {
            request_digest: "hostRequestDigest".to_owned(),
            outcome: WorkerHostEffectOutcome::Detached,
            artifact_identity: "searchInputDetached".to_owned(),
            runtime_lifecycle_source_id: Some("searchFocusLifecycle".to_owned()),
            lifecycle_value: Some(SignalValue::String("detached".to_owned())),
        })
        .unwrap_err();

    assert_eq!(error.code, "invalidInput");
    assert!(error.message.contains("cannot mutate worker runtime truth"));
}

#[test]
fn worker_host_effect_acknowledgement_emits_detached_lifecycle_artifact() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let acknowledgement = worker_runtime
        .admit_host_effect_acknowledgement_for_test(WorkerHostEffectAcknowledgement {
            request_digest: "hostRequestDigest".to_owned(),
            outcome: WorkerHostEffectOutcome::Detached,
            artifact_identity: "searchInputDetached".to_owned(),
            runtime_lifecycle_source_id: None,
            lifecycle_value: None,
        })
        .unwrap();

    assert_eq!(
        acknowledgement.host_effect_lifecycle_artifact,
        "hostEffectDetached"
    );
    assert!(acknowledgement.worker_readmission_required);
    assert_eq!(acknowledgement.runtime_admitted_lifecycle_count, 0);
    assert_digest_shape(&acknowledgement.lifecycle_integrity_digest);
}

#[test]
fn worker_host_effect_acknowledgement_emits_denied_lifecycle_artifact() {
    let worker_runtime = SignalWorkerRuntime::new().unwrap();

    let acknowledgement = worker_runtime
        .admit_host_effect_acknowledgement_for_test(WorkerHostEffectAcknowledgement {
            request_digest: "hostRequestDigest".to_owned(),
            outcome: WorkerHostEffectOutcome::Denied,
            artifact_identity: "searchInputDenied".to_owned(),
            runtime_lifecycle_source_id: None,
            lifecycle_value: None,
        })
        .unwrap();

    assert_eq!(
        acknowledgement.host_effect_lifecycle_artifact,
        "hostEffectDenied"
    );
    assert!(acknowledgement.worker_readmission_required);
    assert_eq!(acknowledgement.runtime_admitted_lifecycle_count, 0);
    assert_digest_shape(&acknowledgement.lifecycle_integrity_digest);
}

fn assert_digest_shape(digest: &str) {
    assert_eq!(digest.len(), 64);
    assert!(digest
        .chars()
        .all(|character| character.is_ascii_hexdigit()));
}

fn effect_lifecycle_publication() -> WorkerPortableGraphPublication {
    WorkerPortableGraphPublication {
        policy: RuntimePolicySpec::default(),
        sources: vec![SourceSpec {
            id: "searchFocusLifecycle".to_owned(),
            initial: SignalValue::String("pending".to_owned()),
            produces_aspects: None,
        }],
        recipes: vec![RecipeSpec {
            id: "searchFocusProjection".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("searchFocusLifecycle".to_owned())],
            expr: Expr::Read {
                id: "searchFocusLifecycle".to_owned(),
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        }],
        output_ids: Vec::new(),
    }
}

fn define_effect_lifecycle_graph(runtime: &mut RuntimeCore) {
    runtime
        .define_source(SourceSpec {
            id: "searchFocusLifecycle".to_owned(),
            initial: SignalValue::String("pending".to_owned()),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "searchFocusProjection".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("searchFocusLifecycle".to_owned())],
            expr: Expr::Read {
                id: "searchFocusLifecycle".to_owned(),
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();
}
