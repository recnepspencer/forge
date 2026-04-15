use crate::{
    evidence::{AbsentModeLaneEvidence, PersistedModeLaneEvidence},
    AbsentRuntimeWitness, CheckpointAuthorityReport, DurableMutationRequest,
    EmbeddedCheckpointClassification, ExternalRuntimeCheckpointEnvelope,
    ExternalRuntimeCommitEnvelope, ForgeStoreBuilder,
};
use serde_json::json;

use super::super::fixtures::runtime::{
    create_entity_commit, latest_envelope, runtime_with_demo_schema,
};

pub fn create_alpha_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "alpha"))
}

pub struct ModeParityScenarioResult {
    pub durable_lane: PersistedModeLaneEvidence,
    pub embedded_lane: PersistedModeLaneEvidence,
    pub absent_lane: AbsentModeLaneEvidence,
    pub checkpoint_authority_report: CheckpointAuthorityReport,
}

pub fn mode_contract_parity() -> ModeParityScenarioResult {
    let mut embedded_runtime = runtime_with_demo_schema();
    create_entity_commit(&mut embedded_runtime, "alpha");
    let embedded_envelope = latest_envelope(&embedded_runtime);

    let mut embedded = ForgeStoreBuilder::new()
        .in_memory()
        .embedded_mode()
        .build()
        .unwrap();
    embedded
        .persist_external_commit(ExternalRuntimeCommitEnvelope::new(
            "embedded-runtime",
            embedded_envelope,
        ))
        .unwrap();

    let before_checkpoint_artifact_digest = embedded.milestone_2_lane_evidence().artifact_digest;
    let checkpoint_receipt = embedded
        .persist_external_checkpoint(
            ExternalRuntimeCheckpointEnvelope::new(
                "checkpoint-certified",
                "embedded-runtime",
                EmbeddedCheckpointClassification::DerivedDurable,
            )
            .with_metadata(json!({"kind":"certified-checkpoint"})),
        )
        .unwrap();
    let embedded_lane = embedded.milestone_2_lane_evidence();
    let checkpoint_authority_report = CheckpointAuthorityReport::from_checkpoint(
        before_checkpoint_artifact_digest,
        embedded_lane.artifact_digest.clone(),
        &checkpoint_receipt,
    );

    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
        .in_memory()
        .durable_mode(durable_runtime)
        .build()
        .unwrap();
    durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    let durable_lane = durable.milestone_2_lane_evidence();

    let absent_runtime = {
        let mut runtime = runtime_with_demo_schema();
        create_entity_commit(&mut runtime, "alpha");
        runtime
    };
    let absent_lane = AbsentRuntimeWitness::new(absent_runtime).milestone_2_lane_evidence();

    ModeParityScenarioResult {
        durable_lane,
        embedded_lane,
        absent_lane,
        checkpoint_authority_report,
    }
}
