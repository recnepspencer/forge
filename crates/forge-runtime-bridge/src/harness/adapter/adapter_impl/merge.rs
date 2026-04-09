use super::*;
use crate::diagnostics::{BridgeCanonicalMergeRecord, BridgeMergeExplanation};
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::routing::canonicalization::digest_string;

pub(super) enum MergeHarnessTarget {
    Execute { declaration_identity: String },
    Replay { declaration_identity: String },
}

pub(super) enum MergeHarnessExecution {
    Execute {
        contract: crate::facade::AdmittedMergeHistoryContract,
        bundle: crate::facade::MergeReplayCertificationBundle,
        record: BridgeCanonicalMergeRecord,
        explanation: BridgeMergeExplanation,
    },
    Replay {
        contract: crate::facade::AdmittedMergeHistoryContract,
        bundle: crate::facade::MergeReplayCertificationBundle,
        record: BridgeCanonicalMergeRecord,
        explanation: BridgeMergeExplanation,
        replayed: crate::facade::BridgeMergeReplaySummary,
    },
}

impl MergeHarnessExecution {
    pub(super) fn summary_json(&self) -> serde_json::Value {
        match self {
            Self::Execute {
                contract,
                bundle,
                record,
                explanation,
            } => json!({
                "merge_declaration_identity": declaration_identity(contract),
                "merge_contract_identity": contract.contract_identity().as_str(),
                "merge_history_digest": contract.digest(),
                "result_bundle_digest": bundle.digest(),
                "continuity_digest": bundle.continuity_artifact().map(|artifact| artifact.digest()),
                "remap_digest": bundle.remap_artifact().map(|artifact| artifact.digest()),
                "explanation_digest": bundle.explanation_artifact().digest(),
                "replay_digest": serde_json::Value::Null,
                "diagnostics_digest": merge_diagnostics_digest(explanation),
                "record_identity": record.record_identity().as_str(),
                "outcome_class": format!("{:?}", bundle.reduced_routing_artifact().outcome_class()),
                "blocked_stage": bundle.lowered_packet_set().blocked_stage().map(|stage| format!("{stage:?}")),
                "denial_class": bundle.lowered_packet_set().denial_class().map(|class| format!("{class:?}")),
                "counter_snapshot": counter_snapshot_json(bundle.reduced_routing_artifact().counters()),
            }),
            Self::Replay {
                contract,
                bundle,
                record,
                explanation,
                replayed,
            } => json!({
                "merge_declaration_identity": declaration_identity(contract),
                "merge_contract_identity": contract.contract_identity().as_str(),
                "merge_history_digest": contract.digest(),
                "result_bundle_digest": bundle.digest(),
                "continuity_digest": bundle.continuity_artifact().map(|artifact| artifact.digest()),
                "remap_digest": bundle.remap_artifact().map(|artifact| artifact.digest()),
                "explanation_digest": bundle.explanation_artifact().digest(),
                "replay_digest": replayed.digest(),
                "diagnostics_digest": merge_diagnostics_digest(explanation),
                "record_identity": record.record_identity().as_str(),
                "outcome_class": format!("{:?}", bundle.reduced_routing_artifact().outcome_class()),
                "blocked_stage": bundle.lowered_packet_set().blocked_stage().map(|stage| format!("{stage:?}")),
                "denial_class": bundle.lowered_packet_set().denial_class().map(|class| format!("{class:?}")),
                "counter_snapshot": counter_snapshot_json(replayed.reduced_routing_artifact().counters()),
            }),
        }
    }

    pub(super) fn extensions_json(
        &self,
        _runtime_bridge: &crate::facade::RuntimeBridge,
    ) -> BTreeMap<String, serde_json::Value> {
        match self {
            Self::Execute {
                contract,
                bundle,
                record,
                explanation,
            } => BTreeMap::from([
                (
                    "bridge_merge_certification_bundle".to_string(),
                    certification_bundle_json(contract, bundle, record, explanation, None),
                ),
                merge_record_extension(contract, bundle, record, explanation),
            ]),
            Self::Replay {
                contract,
                bundle,
                record,
                explanation,
                replayed,
            } => BTreeMap::from([
                (
                    "bridge_merge_certification_bundle".to_string(),
                    certification_bundle_json(
                        contract,
                        bundle,
                        record,
                        explanation,
                        Some(replayed),
                    ),
                ),
                merge_record_extension(contract, bundle, record, explanation),
            ]),
        }
    }
}

pub(super) fn parse_merge_harness_target(
    target: &str,
) -> Option<Result<MergeHarnessTarget, BridgeHarnessError>> {
    if let Some(rest) = target.strip_prefix("merge-execute:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                MergeHarnessTarget::Execute {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("merge-replay:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                MergeHarnessTarget::Replay {
                    declaration_identity,
                }
            }),
        );
    }
    None
}

pub(super) fn execute_merge_request(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    target: MergeHarnessTarget,
) -> Result<MergeHarnessExecution, BridgeHarnessError> {
    match target {
        MergeHarnessTarget::Execute {
            declaration_identity,
        } => execute_merge_bundle(runtime_bridge, fixture, &declaration_identity),
        MergeHarnessTarget::Replay {
            declaration_identity,
        } => execute_merge_replay(runtime_bridge, fixture, &declaration_identity),
    }
}

fn execute_merge_bundle(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<MergeHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let bundle = runtime_bridge
        .replay_merge_history(&contract)
        .map_err(|error| BridgeHarnessError::new(format!("bridge merge replay failed: {error}")))?;
    let record = runtime_bridge.canonicalize_merge_record(&bundle);
    let explanation = runtime_bridge.diagnostics().explain_merge_record(&record);

    Ok(MergeHarnessExecution::Execute {
        contract,
        bundle,
        record,
        explanation,
    })
}

fn execute_merge_replay(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<MergeHarnessExecution, BridgeHarnessError> {
    let execution = execute_merge_bundle(runtime_bridge, fixture, declaration_identity)?;
    let MergeHarnessExecution::Execute {
        contract,
        bundle,
        record,
        explanation,
    } = execution
    else {
        unreachable!("merge bundle execution must produce an execute record");
    };
    let replayed = runtime_bridge
        .replay_canonical_merge_record(&record)
        .map_err(|error| BridgeHarnessError::new(format!("bridge merge replay failed: {error}")))?;

    Ok(MergeHarnessExecution::Replay {
        contract,
        bundle,
        record,
        explanation,
        replayed,
    })
}

fn admitted_contract(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<crate::facade::AdmittedMergeHistoryContract, BridgeHarnessError> {
    let declaration = fixture
        .merge_declarations()
        .iter()
        .find(|declaration| declaration.declaration_identity().as_str() == declaration_identity)
        .cloned()
        .ok_or_else(|| {
            BridgeHarnessError::new(format!(
                "bridge merge fixture does not declare `{declaration_identity}`"
            ))
        })?;
    runtime_bridge
        .admit_merge_history(declaration)
        .map_err(|error| BridgeHarnessError::new(format!("bridge merge admission failed: {error}")))
}

fn parse_declaration_identity(rest: &str) -> Result<String, BridgeHarnessError> {
    if rest.is_empty() {
        return Err(BridgeHarnessError::new(
            "merge harness targets require a merge declaration identity",
        ));
    }
    Ok(rest.to_string())
}

fn declaration_identity(contract: &crate::facade::AdmittedMergeHistoryContract) -> &str {
    contract
        .validated_declaration()
        .declaration()
        .declaration_identity()
        .as_str()
}

fn merge_record_extension(
    contract: &crate::facade::AdmittedMergeHistoryContract,
    bundle: &crate::facade::MergeReplayCertificationBundle,
    record: &BridgeCanonicalMergeRecord,
    explanation: &BridgeMergeExplanation,
) -> (String, serde_json::Value) {
    (
        "bridge_merge_record".to_string(),
        json!({
            "record_identity": record.record_identity().as_str(),
            "merge_contract_identity": contract.contract_identity().as_str(),
            "merge_declaration_identity": declaration_identity(contract),
            "bundle_digest": bundle.digest(),
            "lowered_digest": explanation.lowered_digest(),
            "reduced_digest": explanation.reduced_digest(),
            "continuity_digest": explanation.continuity_digest(),
            "remap_digest": explanation.remap_digest(),
            "explanation_digest": explanation.explanation_digest(),
            "outcome_class": format!("{:?}", explanation.outcome_class()),
            "blocked_stage": explanation.blocked_stage().map(|stage| format!("{stage:?}")),
            "denial_class": explanation.denial_class().map(|class| format!("{class:?}")),
        }),
    )
}

fn certification_bundle_json(
    contract: &crate::facade::AdmittedMergeHistoryContract,
    bundle: &crate::facade::MergeReplayCertificationBundle,
    record: &BridgeCanonicalMergeRecord,
    explanation: &BridgeMergeExplanation,
    replayed: Option<&crate::facade::BridgeMergeReplaySummary>,
) -> serde_json::Value {
    json!({
        "merge_history_digest": contract.digest(),
        "merge_contract_identity": contract.contract_identity().as_str(),
        "merge_ontology_mapping_report": {
            "bridge_class": format!("{:?}", contract.validated_declaration().declaration().bridge_class()),
            "ontology_mapping_digest": contract
                .validated_declaration()
                .declaration()
                .ontology_mapping()
                .digest(),
            "ontology_version": contract
                .validated_declaration()
                .declaration()
                .authority_basis()
                .ontology_version(),
            "schema_policy_descriptor_version": contract
                .validated_declaration()
                .declaration()
                .authority_basis()
                .schema_policy_descriptor_version(),
        },
        "merge_support_matrix": {
            "outcome_class": format!("{:?}", bundle.reduced_routing_artifact().outcome_class()),
            "continuity_published": bundle.continuity_artifact().is_some(),
            "remap_published": bundle.remap_artifact().is_some(),
        },
        "merge_denial_stage_report": {
            "blocked_stage": explanation.blocked_stage().map(|stage| format!("{stage:?}")),
            "denial_class": explanation.denial_class().map(|class| format!("{class:?}")),
        },
        "result_bundle_digest": bundle.digest(),
        "replay_digest": replayed.map(|bundle| bundle.digest()),
        "failure_digest": if bundle.continuity_artifact().is_none() && bundle.remap_artifact().is_none() {
            Some(bundle.explanation_artifact().digest())
        } else {
            None::<&str>
        },
        "diagnostics_digest": merge_diagnostics_digest(explanation),
        "record_identity": record.record_identity().as_str(),
        "counter_snapshot": counter_snapshot_json(bundle.reduced_routing_artifact().counters()),
    })
}

fn merge_diagnostics_digest(explanation: &BridgeMergeExplanation) -> String {
    digest_string(
        "merge-diagnostics-digest",
        &format!(
            "record={}|contract={}|lowered={}|reduced={}|continuity={}|remap={}|explanation={}|outcome={:?}|blocked_stage={:?}|denial={:?}",
            explanation.record_identity().as_str(),
            explanation.contract_identity(),
            explanation.lowered_digest(),
            explanation.reduced_digest(),
            explanation.continuity_digest().unwrap_or("none"),
            explanation.remap_digest().unwrap_or("none"),
            explanation.explanation_digest(),
            explanation.outcome_class(),
            explanation.blocked_stage(),
            explanation.denial_class(),
        ),
    )
    .to_string()
}

fn counter_snapshot_json(counters: &crate::facade::BridgeMergeCounters) -> serde_json::Value {
    json!({
        "merge_declaration_count": counters.merge_history_declaration_count(),
        "merge_contract_count": counters.merge_history_contract_count(),
        "merge_parent_count": counters.merge_parent_count(),
        "merge_supported_class_count": counters.merge_supported_class_count(),
        "merge_unsupported_class_count": counters.merge_unsupported_class_count(),
        "merge_parent_order_rejection_count": counters.merge_parent_order_rejection_count(),
        "merge_causal_frontier_count": counters.merge_causal_frontier_count(),
        "merge_policy_outcome_count": counters.merge_policy_outcome_count(),
        "merge_history_packet_count": counters.merge_packet_count(),
        "merge_routing_result_count": counters.merge_routing_result_count(),
        "merge_lineage_resolution_width": counters.merge_lineage_resolution_width(),
        "merge_candidate_cohort_width": counters.merge_candidate_cohort_width(),
        "merge_structural_consult_width": counters.merge_structural_consult_width(),
        "merge_causal_frontier_lookup_count": counters.merge_causal_frontier_lookup_count(),
        "merge_history_segment_scan_count": counters.merge_history_segment_scan_count(),
        "merge_continuity_count": counters.merge_continuity_count(),
        "merge_continuity_denial_count": counters.merge_continuity_denial_count(),
        "merge_remap_publication_count": counters.merge_remap_publication_count(),
        "merge_deletion_class_count": counters.merge_deletion_class_count(),
        "merge_topology_rewire_class_count": counters.merge_topology_rewire_class_count(),
        "merge_structural_contradiction_count": counters.merge_structural_contradiction_count(),
        "merge_explanation_request_count": counters.merge_explanation_request_count(),
        "merge_replay_request_count": counters.merge_replay_request_count(),
        "merge_replay_mismatch_count": counters.merge_replay_mismatch_count(),
        "merge_widened_scan_count": counters.merge_widened_scan_count(),
        "digest_computation_count": counters.digest_computation_count(),
        "digest_input_bytes": counters.digest_input_bytes(),
    })
}
