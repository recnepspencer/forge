use super::readmission_direct::{yielded_direct, yielded_direct_with_provider};
use super::readmission_workflow::yielded_workflow;
use super::yield_fixture::YieldProvider;
use super::*;
use worth_foundational::facade::RetentionDeliveryProfile;
use worth_query_installation::facade::{
    WorthQueryArtifactClassification, WorthQueryArtifactDeletionPosture,
    WorthQueryArtifactLegalHoldPosture, WorthQueryArtifactRedactionPosture,
};

#[test]
fn direct_export_binds_durable_evidence_without_consuming_resume_authority() {
    let (yielded, bridge, runtime) = yielded_direct();
    let expected = ExpectedDirectHandoff::from_yielded(&yielded);
    let exported = match yielded.export_checkpoint() {
        crate::domain_computation::WorthQueryDirectCheckpointExportOutcome::Exported(exported) => {
            exported
        }
        _ => panic!("export-capable checkpoint should produce a handoff"),
    };
    assert_direct_handoff(exported.handoff(), &expected);
    let cloned_handoff = exported.handoff().clone();
    assert_eq!(cloned_handoff, *exported.handoff());

    let (_, yielded) = exported.into_parts();
    assert_eq!(yielded.checkpoint().identity(), expected.checkpoint);
    let active = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Readmitted(active) => active,
        _ => panic!("export must leave the in-memory yielded authority readmittable"),
    };
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("restored provider should complete"),
    };
    assert!(completion
        .into_running()
        .completed()
        .unwrap()
        .cleanup()
        .is_ok());
}

#[test]
fn unsupported_direct_export_returns_the_exact_yielded_authority() {
    let (yielded, _bridge, _runtime) =
        yielded_direct_with_provider(YieldProvider::checkpoint_restore_failure(7));
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let resource_attempt = yielded.resource_attempt_identity().to_owned();
    let failed = match yielded.export_checkpoint() {
        crate::domain_computation::WorthQueryDirectCheckpointExportOutcome::Failed(failed) => {
            failed
        }
        _ => panic!("unsupported export should be an ordinary failure"),
    };
    assert!(failed.detail().contains("does not support durable export"));
    let yielded = failed.into_yielded();
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    assert_eq!(yielded.resource_attempt_identity(), resource_attempt);
    assert_eq!(
        complete_direct_yield_cleanup(yielded)
            .checkpoint()
            .unwrap()
            .identity(),
        checkpoint
    );
}

#[test]
fn panicking_direct_export_returns_typed_recovery_with_cleanup_authority() {
    let (yielded, _bridge, _runtime) =
        yielded_direct_with_provider(YieldProvider::checkpoint_restore_panic(7));
    let recovery = match yielded.export_checkpoint() {
        crate::domain_computation::WorthQueryDirectCheckpointExportOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("provider export panic must remain typed recovery"),
    };
    assert_eq!(recovery.detail(), "provider checkpoint export panicked");
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryCheckpointExportRecoveryKind::ProviderExportPanicked
    );
    assert_eq!(
        recovery.posture(),
        crate::domain_computation::WorthQueryCheckpointExportRecoveryPosture::
            TerminalCleanupRequired
    );
    assert!(!recovery.checkpoint().identity().is_empty());
    match recovery.cleanup() {
        crate::domain_computation::WorthQueryDirectYieldCleanupOutcome::Complete(_) => {}
        _ => panic!("export recovery should retain direct cleanup authority"),
    }
}

#[test]
fn workflow_export_binds_artifact_generation_and_preserves_cleanup_authority() {
    let (yielded, _bridge, _runtime, _old_producer) = yielded_workflow(YieldProvider::installed(9));
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let artifact_run = yielded.artifact_run_identity().to_owned();
    let artifact_evidence = yielded.artifact_evidence();
    let generation = artifact_evidence.production_generation();
    let exported = match yielded.export_checkpoint() {
        crate::domain_computation::WorthQueryWorkflowCheckpointExportOutcome::Exported(
            exported,
        ) => exported,
        _ => panic!("workflow checkpoint should export"),
    };
    assert_eq!(
        exported.handoff().checkpoint_occurrence_identity(),
        checkpoint
    );
    assert_eq!(
        exported.handoff().artifact_run_identity(),
        Some(artifact_run.as_str())
    );
    assert_eq!(
        exported.handoff().artifact_production_generation(),
        Some(generation)
    );
    assert_eq!(
        exported.handoff().artifact_evidence(),
        Some(artifact_evidence)
    );
    let (_, yielded) = exported.into_parts();
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    match yielded.cleanup() {
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Complete(_) => {}
        _ => panic!("export must preserve workflow cleanup authority"),
    }
}

struct ExpectedDirectHandoff {
    logical_run: String,
    yielded_attempt: String,
    operation_binding: String,
    installed_operation: String,
    installation_generation: u64,
    semantic_basis: String,
    provider_generation: u64,
    checkpoint: String,
    resource_evidence: String,
    bridge_basis: String,
    bridge_request: String,
    relational_basis: worth_relational::facade::runtime::RelationalExecutionBasisIdentity,
}

impl ExpectedDirectHandoff {
    fn from_yielded(yielded: &crate::domain_computation::WorthQueryYieldedDirectRun) -> Self {
        Self {
            logical_run: yielded.logical_run_identity().to_owned(),
            yielded_attempt: yielded.yielded_attempt_identity().to_owned(),
            operation_binding: yielded.operation_binding_identity().to_owned(),
            installed_operation: yielded.installed_operation_identity().to_owned(),
            installation_generation: yielded.installation_generation().ordinal(),
            semantic_basis: yielded.semantic_basis_identity().to_owned(),
            provider_generation: yielded.checkpoint().provider_generation(),
            checkpoint: yielded.checkpoint().identity().to_owned(),
            resource_evidence: yielded.resource_attempt_evidence().identity().to_owned(),
            bridge_basis: yielded.bridge().basis_identity().as_str().to_owned(),
            bridge_request: yielded.bridge_request_identity().to_owned(),
            relational_basis: yielded.relational_basis_identity().clone(),
        }
    }
}

fn assert_direct_handoff(
    handoff: &crate::domain_computation::WorthQueryCheckpointExportHandoff,
    expected: &ExpectedDirectHandoff,
) {
    assert_eq!(
        handoff.protocol_identity(),
        crate::domain_computation::WORTH_QUERY_CHECKPOINT_EXPORT_PROTOCOL_IDENTITY
    );
    assert_eq!(
        handoff.protocol_version(),
        crate::domain_computation::WORTH_QUERY_CHECKPOINT_EXPORT_PROTOCOL_VERSION
    );
    assert_eq!(handoff.binding_digest().len(), 64);
    assert_eq!(handoff.logical_run_identity(), expected.logical_run);
    assert_eq!(handoff.yielded_attempt_identity(), expected.yielded_attempt);
    assert_eq!(
        handoff.operation_binding_identity(),
        expected.operation_binding
    );
    assert_eq!(
        handoff.installed_operation_identity(),
        expected.installed_operation
    );
    assert_eq!(
        handoff.installation_generation(),
        expected.installation_generation
    );
    assert_eq!(handoff.semantic_basis_identity(), expected.semantic_basis);
    assert_eq!(handoff.provider_generation(), expected.provider_generation);
    assert_eq!(
        handoff.resource_attempt_evidence().identity(),
        expected.resource_evidence
    );
    assert_eq!(handoff.bridge_basis_identity(), expected.bridge_basis);
    assert_eq!(handoff.bridge_request_identity(), expected.bridge_request);
    assert_eq!(
        handoff.relational_basis_identity(),
        &expected.relational_basis
    );
    assert_eq!(
        handoff.checkpoint_occurrence_identity(),
        expected.checkpoint
    );
    assert_eq!(handoff.artifact_run_identity(), None);
    assert_eq!(handoff.artifact_production_generation(), None);
    assert_eq!(
        handoff.provider_export().format_identity(),
        "worth-query-tests-yield"
    );
    assert_eq!(handoff.provider_export().format_version(), 1);
    assert_eq!(
        handoff.provider_export().compatibility_identity(),
        "worth-query-tests-yield-v1"
    );
    assert_eq!(handoff.provider_export().payload(), b"retained-bytes:5");
    assert_eq!(
        handoff.provider_export().payload_bytes(),
        b"retained-bytes:5".len()
    );
    assert_eq!(handoff.provider_export().payload_digest().len(), 64);
    assert_eq!(handoff.provider_export().contract_digest().len(), 64);
    assert_eq!(
        handoff.governance().classification(),
        WorthQueryArtifactClassification::Confidential
    );
    assert_eq!(
        handoff.governance().redaction(),
        WorthQueryArtifactRedactionPosture::DomainRedactorRequired
    );
    assert_eq!(
        handoff.governance().retention(),
        RetentionDeliveryProfile::Durable
    );
    assert_eq!(
        handoff.governance().deletion(),
        WorthQueryArtifactDeletionPosture::ExternallyControlled
    );
    assert_eq!(
        handoff.governance().legal_hold(),
        WorthQueryArtifactLegalHoldPosture::DomainControlled
    );
    assert_eq!(
        handoff.governance().audiences(),
        ["store-checkpoint-ingestion"]
    );
    assert_eq!(
        handoff.cost().provider_payload_bytes(),
        handoff.provider_export().payload_bytes()
    );
    assert!(handoff.cost().binding_material_bytes() > 0);
    assert_eq!(
        handoff.cost().total_bound_bytes(),
        handoff
            .cost()
            .binding_material_bytes()
            .saturating_add(handoff.cost().provider_payload_bytes())
    );
}
