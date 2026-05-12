use super::causal_envelope_mapping_support::{bridge_reference, query_observation_reference};
use super::support::*;
use crate::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEvidenceFamily, RuntimeBridge,
};

struct RetainedWritebackChain {
    admission_identity: String,
    mapper_envelope_identity: String,
    mapped_input_identity: String,
    mapper_record_identity: String,
    execution_identity: String,
    replay_identity: String,
}

#[test]
fn causal_envelope_full_writeback_chain_lookup_cost_ignores_unrelated_records() {
    let mut envelope_identities = Vec::new();

    for unrelated_records in [0, 2, 5] {
        let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
        for index in 0..unrelated_records {
            retain_writeback_chain(&runtime, &format!("noise-{index}"));
        }
        let target_chain = retain_writeback_chain(&runtime, "target");
        let routed = runtime
            .route("commit-causal-writeback-full-scale")
            .expect("route should succeed");
        let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
            crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
                "query-admission:writeback-full-scale",
                "causal-anchor:writeback-full-scale",
            )
            .expect("query admission summary should be valid"),
            vec![
                query_observation_reference("query-observation:writeback-full-scale"),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.result().result_summary().route_identity().as_str(),
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeWritebackAdmission,
                    &target_chain.admission_identity,
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeWritebackMapperEnvelope,
                    &target_chain.mapper_envelope_identity,
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeWritebackMappedFamilyInput,
                    &target_chain.mapped_input_identity,
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeWritebackMapper,
                    &target_chain.mapper_record_identity,
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeWritebackExecution,
                    &target_chain.execution_identity,
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeWritebackReplay,
                    &target_chain.replay_identity,
                ),
            ],
        )
        .expect("request should be valid");

        let envelope = runtime
            .diagnostics()
            .assemble_causal_explanation_envelope(request)
            .expect("full writeback chain should bind");

        assert_eq!(
            runtime.diagnostics().writeback_admission_records().len(),
            unrelated_records + 1
        );
        assert_eq!(
            runtime.diagnostics().writeback_mapper_envelopes().len(),
            (unrelated_records + 1) * 4
        );
        assert_eq!(
            runtime.diagnostics().writeback_mapped_family_inputs().len(),
            (unrelated_records + 1) * 3
        );
        assert_eq!(
            runtime.diagnostics().writeback_mapper_records().len(),
            unrelated_records + 1
        );
        assert_eq!(
            runtime.diagnostics().writeback_execution_records().len(),
            unrelated_records + 1
        );
        assert_eq!(
            runtime.diagnostics().writeback_replay_records().len(),
            unrelated_records + 1
        );
        assert_eq!(envelope.counters().bridge_retained_lookup_count(), 7);
        assert_eq!(envelope.counters().retained_bridge_binding_count(), 7);
        assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);
        envelope_identities.push(envelope.identity().identity_digest().to_string());
    }

    assert_eq!(envelope_identities[0], envelope_identities[1]);
    assert_eq!(envelope_identities[1], envelope_identities[2]);
}

fn retain_writeback_chain(runtime: &RuntimeBridge, suffix: &str) -> RetainedWritebackChain {
    let lowered_policy = lowered_policy(runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                &format!("writeback:causal-scale-{suffix}"),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                &format!("strategy:sha256:causal-scale-{suffix}"),
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");
    let admission = runtime
        .diagnostics()
        .last_writeback_admission_record()
        .expect("writeback admission should be retained");
    let causality = causality_basis(
        &format!("causality:causal-scale-{suffix}"),
        &format!("trigger:sha256:causal-scale-{suffix}"),
    );
    let mapper_envelope = runtime.lower_writeback_mapper_envelope(
        &contract,
        &causality,
        format!("effect:sha256:causal-scale-{suffix}"),
        format!("evidence:sha256:causal-scale-{suffix}"),
    );
    let mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        format!("effect:sha256:causal-scale-{suffix}"),
        format!("evidence:sha256:causal-scale-{suffix}"),
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new(format!("effect:causal-scale-{suffix}")),
        format!("effect:sha256:causal-scale-{suffix}"),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        format!("truth-state:sha256:causal-scale-{suffix}"),
        BridgeWritebackIdempotenceIdentity::new(format!("idempotence:causal-scale-{suffix}")),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (outcome, _) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("authority execution should succeed");
    let mapper_record = runtime
        .diagnostics()
        .last_writeback_mapper_record()
        .expect("mapper record should be retained");
    let execution = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("execution record should be retained");
    let expected_bundle =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &outcome);
    let drifted_effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new(format!("effect:causal-scale-{suffix}-drifted")),
        format!("effect:sha256:causal-scale-{suffix}-drifted"),
    );
    let drifted_idempotence = runtime.classify_writeback_idempotence(
        &drifted_effect,
        &lowered_policy,
        format!("truth-state:sha256:causal-scale-{suffix}"),
        BridgeWritebackIdempotenceIdentity::new(format!(
            "idempotence:causal-scale-{suffix}-drifted"
        )),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let replayed_bundle =
        runtime.replay_writeback_bundle(&contract, &drifted_effect, &drifted_idempotence, &outcome);
    runtime
        .validate_replayed_writeback_bundle(&expected_bundle, &replayed_bundle)
        .expect_err("drifted replay should retain replay evidence");
    let replay = runtime
        .diagnostics()
        .last_writeback_replay_record()
        .expect("replay record should be retained");

    RetainedWritebackChain {
        admission_identity: admission.record_identity().as_str().to_string(),
        mapper_envelope_identity: mapper_envelope.envelope_identity().as_str().to_string(),
        mapped_input_identity: mapped_input.mapped_input_identity().as_str().to_string(),
        mapper_record_identity: mapper_record.record_identity().as_str().to_string(),
        execution_identity: execution.record_identity().as_str().to_string(),
        replay_identity: replay.record_identity().as_str().to_string(),
    }
}
