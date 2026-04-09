use super::*;
use crate::diagnostics::{
    BridgeCanonicalStructuralBranchComparisonRecord, BridgeCanonicalStructuralRemapRecord,
    BridgeStructuralCounters,
};
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::routing::canonicalization::digest_string;
use crate::structural::{
    AdmittedStructuralComparisonContract, PlannedStructuralMatchPacketSet,
    PublishedBranchComparisonArtifact, PublishedStructuralRemapArtifact, ReducedStructuralMatchSet,
    StructuralCandidateSearchScope, StructuralComparisonMode, StructuralMatchCandidate,
    StructuralMatchCandidateKind, StructuralMatchOutcomeClass,
};

pub(super) enum StructuralHarnessTarget {
    RemapExact { declaration_identity: String },
    RemapAmbiguous { declaration_identity: String },
    RemapNoSafeMatch { declaration_identity: String },
    RemapLineageDivergence { declaration_identity: String },
    RemapIdentityConflict { declaration_identity: String },
    RemapReplay { declaration_identity: String },
    BranchCompare { declaration_identity: String },
    BranchReplay { declaration_identity: String },
}

pub(super) enum StructuralHarnessExecution {
    Remap {
        contract: AdmittedStructuralComparisonContract,
        planned: PlannedStructuralMatchPacketSet,
        reduced: ReducedStructuralMatchSet,
        artifact: PublishedStructuralRemapArtifact,
        record: BridgeCanonicalStructuralRemapRecord,
    },
    RemapReplay {
        contract: AdmittedStructuralComparisonContract,
        planned: PlannedStructuralMatchPacketSet,
        reduced: ReducedStructuralMatchSet,
        artifact: PublishedStructuralRemapArtifact,
        record: BridgeCanonicalStructuralRemapRecord,
        replayed: PublishedStructuralRemapArtifact,
    },
    Branch {
        contract: AdmittedStructuralComparisonContract,
        planned: PlannedStructuralMatchPacketSet,
        reduced: ReducedStructuralMatchSet,
        artifact: PublishedBranchComparisonArtifact,
        record: BridgeCanonicalStructuralBranchComparisonRecord,
    },
    BranchReplay {
        contract: AdmittedStructuralComparisonContract,
        planned: PlannedStructuralMatchPacketSet,
        reduced: ReducedStructuralMatchSet,
        artifact: PublishedBranchComparisonArtifact,
        record: BridgeCanonicalStructuralBranchComparisonRecord,
        replayed: PublishedBranchComparisonArtifact,
    },
    Rejected {
        contract: AdmittedStructuralComparisonContract,
        planned: PlannedStructuralMatchPacketSet,
        reduced: ReducedStructuralMatchSet,
    },
}

impl StructuralHarnessExecution {
    pub(super) fn summary_json(&self) -> serde_json::Value {
        match self {
            Self::Remap {
                contract,
                planned,
                reduced,
                artifact,
                record,
            } => json!({
                "structural_declaration_identity": declaration_identity(contract),
                "structural_contract_identity": contract.contract_identity().as_str(),
                "structural_match_digest": planned.digest(),
                "structural_reuse_digest": artifact.digest(),
                "branch_compare_digest": serde_json::Value::Null,
                "replay_digest": serde_json::Value::Null,
                "diagnostics_digest": remap_diagnostics_digest(record),
                "failure_digest": serde_json::Value::Null,
                "outcome_class": format!("{:?}", reduced.outcome_class()),
                "counter_snapshot": counter_snapshot_json(record.counters(), false),
            }),
            Self::RemapReplay {
                contract,
                planned,
                reduced,
                artifact,
                record,
                replayed,
            } => json!({
                "structural_declaration_identity": declaration_identity(contract),
                "structural_contract_identity": contract.contract_identity().as_str(),
                "structural_match_digest": planned.digest(),
                "structural_reuse_digest": artifact.digest(),
                "branch_compare_digest": serde_json::Value::Null,
                "replay_digest": replayed.digest(),
                "diagnostics_digest": remap_diagnostics_digest(record),
                "failure_digest": serde_json::Value::Null,
                "outcome_class": format!("{:?}", reduced.outcome_class()),
                "counter_snapshot": counter_snapshot_json(record.counters(), true),
            }),
            Self::Branch {
                contract,
                reduced,
                artifact,
                record,
                ..
            } => json!({
                "structural_declaration_identity": declaration_identity(contract),
                "structural_contract_identity": contract.contract_identity().as_str(),
                "structural_match_digest": serde_json::Value::Null,
                "structural_reuse_digest": serde_json::Value::Null,
                "branch_compare_digest": artifact.digest(),
                "replay_digest": serde_json::Value::Null,
                "diagnostics_digest": branch_diagnostics_digest(record),
                "failure_digest": serde_json::Value::Null,
                "outcome_class": format!("{:?}", reduced.outcome_class()),
                "counter_snapshot": counter_snapshot_json(record.counters(), false),
            }),
            Self::BranchReplay {
                contract,
                reduced,
                artifact,
                record,
                replayed,
                ..
            } => json!({
                "structural_declaration_identity": declaration_identity(contract),
                "structural_contract_identity": contract.contract_identity().as_str(),
                "structural_match_digest": serde_json::Value::Null,
                "structural_reuse_digest": serde_json::Value::Null,
                "branch_compare_digest": artifact.digest(),
                "replay_digest": replayed.digest(),
                "diagnostics_digest": branch_diagnostics_digest(record),
                "failure_digest": serde_json::Value::Null,
                "outcome_class": format!("{:?}", reduced.outcome_class()),
                "counter_snapshot": counter_snapshot_json(record.counters(), true),
            }),
            Self::Rejected {
                contract,
                planned,
                reduced,
            } => json!({
                "structural_declaration_identity": declaration_identity(contract),
                "structural_contract_identity": contract.contract_identity().as_str(),
                "structural_match_digest": planned.digest(),
                "structural_reuse_digest": serde_json::Value::Null,
                "branch_compare_digest": serde_json::Value::Null,
                "replay_digest": serde_json::Value::Null,
                "diagnostics_digest": rejection_diagnostics_digest(contract, planned, reduced),
                "failure_digest": reduced.digest(),
                "outcome_class": format!("{:?}", reduced.outcome_class()),
                "counter_snapshot": rejection_counter_snapshot_json(contract, planned, reduced),
            }),
        }
    }

    pub(super) fn extensions_json(
        &self,
        runtime_bridge: &crate::facade::RuntimeBridge,
    ) -> BTreeMap<String, serde_json::Value> {
        match self {
            Self::Remap {
                contract,
                planned,
                reduced,
                artifact,
                record,
            } => BTreeMap::from([
                (
                    "bridge_structural_certification_bundle".to_string(),
                    json!({
                        "structural_match_digest": planned.digest(),
                        "ambiguity_report": serde_json::Value::Null,
                        "remap_artifact_digest": artifact.digest(),
                        "failure_digest": serde_json::Value::Null,
                        "structural_reuse_digest": artifact.digest(),
                        "identity_separation_report": serde_json::Value::Null,
                        "replay_digest": serde_json::Value::Null,
                        "diagnostics_digest": remap_diagnostics_digest(record),
                        "branch_compare_digest": serde_json::Value::Null,
                        "structural_diff_report": serde_json::Value::Null,
                        "counter_snapshot": counter_snapshot_json(record.counters(), false),
                    }),
                ),
                remap_record_extension(
                    runtime_bridge,
                    contract,
                    planned,
                    reduced,
                    artifact,
                    record,
                ),
            ]),
            Self::RemapReplay {
                contract,
                planned,
                reduced,
                artifact,
                record,
                replayed,
            } => BTreeMap::from([
                (
                    "bridge_structural_certification_bundle".to_string(),
                    json!({
                        "structural_match_digest": planned.digest(),
                        "ambiguity_report": serde_json::Value::Null,
                        "remap_artifact_digest": artifact.digest(),
                        "failure_digest": serde_json::Value::Null,
                        "structural_reuse_digest": artifact.digest(),
                        "identity_separation_report": serde_json::Value::Null,
                        "replay_digest": replayed.digest(),
                        "diagnostics_digest": remap_diagnostics_digest(record),
                        "branch_compare_digest": serde_json::Value::Null,
                        "structural_diff_report": serde_json::Value::Null,
                        "counter_snapshot": counter_snapshot_json(record.counters(), true),
                    }),
                ),
                remap_record_extension(
                    runtime_bridge,
                    contract,
                    planned,
                    reduced,
                    artifact,
                    record,
                ),
            ]),
            Self::Branch {
                contract,
                planned,
                reduced,
                artifact,
                record,
            } => BTreeMap::from([
                (
                    "bridge_structural_certification_bundle".to_string(),
                    json!({
                        "structural_match_digest": serde_json::Value::Null,
                        "ambiguity_report": serde_json::Value::Null,
                        "remap_artifact_digest": serde_json::Value::Null,
                        "failure_digest": serde_json::Value::Null,
                        "structural_reuse_digest": serde_json::Value::Null,
                        "identity_separation_report": serde_json::Value::Null,
                        "replay_digest": serde_json::Value::Null,
                        "diagnostics_digest": branch_diagnostics_digest(record),
                        "branch_compare_digest": artifact.digest(),
                        "structural_diff_report": structural_diff_report_json(record, reduced),
                        "counter_snapshot": counter_snapshot_json(record.counters(), false),
                    }),
                ),
                branch_record_extension(
                    runtime_bridge,
                    contract,
                    planned,
                    reduced,
                    artifact,
                    record,
                ),
            ]),
            Self::BranchReplay {
                contract,
                planned,
                reduced,
                artifact,
                record,
                replayed,
            } => BTreeMap::from([
                (
                    "bridge_structural_certification_bundle".to_string(),
                    json!({
                        "structural_match_digest": serde_json::Value::Null,
                        "ambiguity_report": serde_json::Value::Null,
                        "remap_artifact_digest": serde_json::Value::Null,
                        "failure_digest": serde_json::Value::Null,
                        "structural_reuse_digest": serde_json::Value::Null,
                        "identity_separation_report": serde_json::Value::Null,
                        "replay_digest": replayed.digest(),
                        "diagnostics_digest": branch_diagnostics_digest(record),
                        "branch_compare_digest": artifact.digest(),
                        "structural_diff_report": structural_diff_report_json(record, reduced),
                        "counter_snapshot": counter_snapshot_json(record.counters(), true),
                    }),
                ),
                branch_record_extension(
                    runtime_bridge,
                    contract,
                    planned,
                    reduced,
                    artifact,
                    record,
                ),
            ]),
            Self::Rejected {
                contract,
                planned,
                reduced,
            } => BTreeMap::from([(
                "bridge_structural_certification_bundle".to_string(),
                json!({
                    "structural_match_digest": planned.digest(),
                    "ambiguity_report": ambiguity_report_json(reduced),
                    "remap_artifact_digest": serde_json::Value::Null,
                    "failure_digest": reduced.digest(),
                    "structural_reuse_digest": serde_json::Value::Null,
                    "identity_separation_report": identity_separation_report_json(contract, reduced),
                    "replay_digest": serde_json::Value::Null,
                    "diagnostics_digest": rejection_diagnostics_digest(contract, planned, reduced),
                    "branch_compare_digest": serde_json::Value::Null,
                    "structural_diff_report": serde_json::Value::Null,
                    "counter_snapshot": rejection_counter_snapshot_json(contract, planned, reduced),
                }),
            )]),
        }
    }
}

pub(super) fn parse_structural_harness_target(
    target: &str,
) -> Option<Result<StructuralHarnessTarget, BridgeHarnessError>> {
    if let Some(rest) = target.strip_prefix("structural-remap-exact:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                StructuralHarnessTarget::RemapExact {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("structural-remap-ambiguous:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                StructuralHarnessTarget::RemapAmbiguous {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("structural-remap-no-safe-match:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                StructuralHarnessTarget::RemapNoSafeMatch {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("structural-remap-lineage-divergence:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                StructuralHarnessTarget::RemapLineageDivergence {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("structural-remap-identity-conflict:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                StructuralHarnessTarget::RemapIdentityConflict {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("structural-remap-replay:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                StructuralHarnessTarget::RemapReplay {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("structural-branch-compare:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                StructuralHarnessTarget::BranchCompare {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("structural-branch-replay:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                StructuralHarnessTarget::BranchReplay {
                    declaration_identity,
                }
            }),
        );
    }
    None
}

pub(super) fn execute_structural_request(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    target: StructuralHarnessTarget,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    match target {
        StructuralHarnessTarget::RemapExact {
            declaration_identity,
        } => execute_exact_remap(runtime_bridge, fixture, &declaration_identity),
        StructuralHarnessTarget::RemapAmbiguous {
            declaration_identity,
        } => execute_ambiguous_remap(runtime_bridge, fixture, &declaration_identity),
        StructuralHarnessTarget::RemapNoSafeMatch {
            declaration_identity,
        } => execute_no_safe_match_remap(runtime_bridge, fixture, &declaration_identity),
        StructuralHarnessTarget::RemapLineageDivergence {
            declaration_identity,
        } => execute_lineage_divergence_remap(runtime_bridge, fixture, &declaration_identity),
        StructuralHarnessTarget::RemapIdentityConflict {
            declaration_identity,
        } => execute_identity_conflict_remap(runtime_bridge, fixture, &declaration_identity),
        StructuralHarnessTarget::RemapReplay {
            declaration_identity,
        } => execute_remap_replay(runtime_bridge, fixture, &declaration_identity),
        StructuralHarnessTarget::BranchCompare {
            declaration_identity,
        } => execute_branch_compare(runtime_bridge, fixture, &declaration_identity),
        StructuralHarnessTarget::BranchReplay {
            declaration_identity,
        } => execute_branch_replay(runtime_bridge, fixture, &declaration_identity),
    }
}

fn execute_exact_remap(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let planned = runtime_bridge
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            remap_target_packet(),
            vec![remap_target_packet()],
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge structural remap planning failed: {error}"))
        })?;
    let reduced = runtime_bridge
        .reduce_structural_match_set(&planned)
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge structural remap reduction failed: {error}"))
        })?;
    let artifact = runtime_bridge
        .publish_structural_remap_artifact(&reduced)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural remap publication failed: {error}"
            ))
        })?;
    let record = runtime_bridge
        .canonicalize_structural_remap_record(&contract, &planned, &reduced, &artifact);
    Ok(StructuralHarnessExecution::Remap {
        contract,
        planned,
        reduced,
        artifact,
        record,
    })
}

fn execute_ambiguous_remap(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let fingerprint = runtime_bridge
        .materialize_structural_fingerprint(&contract, remap_target_packet())
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural ambiguity fingerprint materialization failed: {error}"
            ))
        })?;
    let planned = runtime_bridge
        .plan_structural_match_packet_set(
            &contract,
            vec![
                StructuralMatchCandidate::with_fingerprint(
                    crate::facade::StructuralCandidateIdentity::new(
                        "structural-candidate:ambiguous-a",
                    ),
                    StructuralMatchCandidateKind::ExactAdvisoryMatch,
                    Some(fingerprint.clone()),
                ),
                StructuralMatchCandidate::with_fingerprint(
                    crate::facade::StructuralCandidateIdentity::new(
                        "structural-candidate:ambiguous-b",
                    ),
                    StructuralMatchCandidateKind::AdvisoryReuseCandidate,
                    Some(fingerprint),
                ),
            ],
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural ambiguity planning failed: {error}"
            ))
        })?;
    let reduced = runtime_bridge
        .reduce_structural_match_set(&planned)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural ambiguity reduction failed: {error}"
            ))
        })?;
    Ok(StructuralHarnessExecution::Rejected {
        contract,
        planned,
        reduced,
    })
}

fn execute_no_safe_match_remap(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let planned = runtime_bridge
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            remap_target_packet(),
            vec![no_safe_match_packet()],
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural no-safe-match planning failed: {error}"
            ))
        })?;
    let reduced = runtime_bridge
        .reduce_structural_match_set(&planned)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural no-safe-match reduction failed: {error}"
            ))
        })?;
    Ok(StructuralHarnessExecution::Rejected {
        contract,
        planned,
        reduced,
    })
}

fn execute_lineage_divergence_remap(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let fingerprint = runtime_bridge
        .materialize_structural_fingerprint(&contract, remap_target_packet())
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural lineage-divergence fingerprint materialization failed: {error}"
            ))
        })?;
    let planned = runtime_bridge
        .plan_structural_match_packet_set(
            &contract,
            vec![StructuralMatchCandidate::with_fingerprint(
                crate::facade::StructuralCandidateIdentity::new(
                    "structural-candidate:lineage-divergence",
                ),
                StructuralMatchCandidateKind::LineageStructuralDivergence,
                Some(fingerprint),
            )],
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural lineage-divergence planning failed: {error}"
            ))
        })?;
    let reduced = runtime_bridge
        .reduce_structural_match_set(&planned)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural lineage-divergence reduction failed: {error}"
            ))
        })?;
    Ok(StructuralHarnessExecution::Rejected {
        contract,
        planned,
        reduced,
    })
}

fn execute_identity_conflict_remap(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let planned = runtime_bridge
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            remap_target_packet(),
            vec![identity_conflict_packet()],
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural identity-separation planning failed: {error}"
            ))
        })?;
    let reduced = runtime_bridge
        .reduce_structural_match_set(&planned)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural identity-separation reduction failed: {error}"
            ))
        })?;
    Ok(StructuralHarnessExecution::Rejected {
        contract,
        planned,
        reduced,
    })
}

fn execute_remap_replay(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    let execution = execute_exact_remap(runtime_bridge, fixture, declaration_identity)?;
    let StructuralHarnessExecution::Remap {
        contract,
        planned,
        reduced,
        artifact,
        record,
    } = execution
    else {
        unreachable!("exact remap execution must produce a remap record");
    };
    let replayed = runtime_bridge
        .replay_canonical_structural_remap_record(&record)
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge structural remap replay failed: {error}"))
        })?;
    Ok(StructuralHarnessExecution::RemapReplay {
        contract,
        planned,
        reduced,
        artifact,
        record,
        replayed,
    })
}

fn execute_branch_compare(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let planned = runtime_bridge
        .plan_structural_branch_comparison_from_read_packet(&contract, branch_packet())
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural branch comparison planning failed: {error}"
            ))
        })?;
    let reduced = runtime_bridge
        .reduce_structural_match_set(&planned)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural branch comparison reduction failed: {error}"
            ))
        })?;
    let artifact = runtime_bridge
        .publish_branch_comparison_artifact(&reduced)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural branch comparison publication failed: {error}"
            ))
        })?;
    let record = runtime_bridge
        .canonicalize_structural_branch_comparison_record(&contract, &planned, &reduced, &artifact);
    Ok(StructuralHarnessExecution::Branch {
        contract,
        planned,
        reduced,
        artifact,
        record,
    })
}

fn execute_branch_replay(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<StructuralHarnessExecution, BridgeHarnessError> {
    let execution = execute_branch_compare(runtime_bridge, fixture, declaration_identity)?;
    let StructuralHarnessExecution::Branch {
        contract,
        planned,
        reduced,
        artifact,
        record,
    } = execution
    else {
        unreachable!("branch execution must produce a branch record");
    };
    let replayed = runtime_bridge
        .replay_canonical_structural_branch_comparison_record(&record)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge structural branch comparison replay failed: {error}"
            ))
        })?;
    Ok(StructuralHarnessExecution::BranchReplay {
        contract,
        planned,
        reduced,
        artifact,
        record,
        replayed,
    })
}

fn parse_declaration_identity(rest: &str) -> Result<String, BridgeHarnessError> {
    if rest.is_empty() {
        return Err(BridgeHarnessError::new(
            "structural harness targets require a structural declaration identity",
        ));
    }
    Ok(rest.to_string())
}

fn admitted_contract(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<AdmittedStructuralComparisonContract, BridgeHarnessError> {
    let declaration = fixture
        .structural_declarations()
        .iter()
        .find(|declaration| declaration.declaration_identity().as_str() == declaration_identity)
        .cloned()
        .ok_or_else(|| {
            BridgeHarnessError::new(format!(
                "bridge structural fixture does not declare `{declaration_identity}`"
            ))
        })?;
    runtime_bridge
        .admit_structural_comparison(declaration)
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge structural admission failed: {error}"))
        })
}

fn remap_record_extension(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &AdmittedStructuralComparisonContract,
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
    artifact: &PublishedStructuralRemapArtifact,
    record: &BridgeCanonicalStructuralRemapRecord,
) -> (String, serde_json::Value) {
    let explanation = runtime_bridge
        .diagnostics()
        .explain_structural_remap_record(record);
    (
        "bridge_structural_remap_record".to_string(),
        json!({
            "record_identity": record.record_identity().as_str(),
            "structural_contract_identity": contract.contract_identity().as_str(),
            "structural_declaration_identity": declaration_identity(contract),
            "planned_digest": planned.digest(),
            "reduced_digest": reduced.digest(),
            "artifact_digest": artifact.digest(),
            "outcome_class": format!("{:?}", reduced.outcome_class()),
            "explanation": {
                "record_identity": explanation.record_identity().as_str(),
                "declaration_identity": explanation.declaration_identity(),
                "semantics_version": explanation.semantics_version(),
                "candidate_count": explanation.candidate_count(),
                "outcome_class": format!("{:?}", explanation.outcome_class()),
            },
        }),
    )
}

fn branch_record_extension(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &AdmittedStructuralComparisonContract,
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
    artifact: &PublishedBranchComparisonArtifact,
    record: &BridgeCanonicalStructuralBranchComparisonRecord,
) -> (String, serde_json::Value) {
    let explanation = runtime_bridge
        .diagnostics()
        .explain_structural_branch_comparison_record(record);
    (
        "bridge_structural_branch_comparison_record".to_string(),
        json!({
            "record_identity": record.record_identity().as_str(),
            "structural_contract_identity": contract.contract_identity().as_str(),
            "structural_declaration_identity": declaration_identity(contract),
            "planned_digest": planned.digest(),
            "reduced_digest": reduced.digest(),
            "artifact_digest": artifact.digest(),
            "branch_diff_count": reduced.branch_diff_count(),
            "explanation": {
                "record_identity": explanation.record_identity().as_str(),
                "declaration_identity": explanation.declaration_identity(),
                "semantics_version": explanation.semantics_version(),
                "candidate_count": explanation.candidate_count(),
                "branch_diff_count": explanation.branch_diff_count(),
            },
        }),
    )
}

fn remap_target_packet() -> crate::facade::SnapshotReadPacket {
    crate::facade::SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
        "entity-1", "profile",
    )])
}

fn identity_conflict_packet() -> crate::facade::SnapshotReadPacket {
    crate::facade::SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
        "entity-2", "profile",
    )])
}

fn no_safe_match_packet() -> crate::facade::SnapshotReadPacket {
    crate::facade::SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
        "entity-3", "profile",
    )])
}

fn branch_packet() -> crate::facade::SnapshotReadPacket {
    crate::facade::SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
        "entity-1", "profile",
    )])
}

fn declaration_identity(contract: &AdmittedStructuralComparisonContract) -> &str {
    contract
        .validated_declaration()
        .declaration()
        .declaration_identity()
        .as_str()
}

fn remap_diagnostics_digest(record: &BridgeCanonicalStructuralRemapRecord) -> String {
    let explanation =
        crate::diagnostics::BridgeStructuralRemapExplanation::from_canonical_record(record);
    digest_string(
        "structural-remap-diagnostics-digest",
        &format!(
            "record={}|declaration={}|family={:?}|version={}|outcome={:?}|artifact={}",
            explanation.record_identity().as_str(),
            explanation.declaration_identity(),
            explanation.fingerprint_family(),
            explanation.semantics_version(),
            explanation.outcome_class(),
            explanation.artifact_digest(),
        ),
    )
    .to_string()
}

fn branch_diagnostics_digest(record: &BridgeCanonicalStructuralBranchComparisonRecord) -> String {
    let explanation =
        crate::diagnostics::BridgeStructuralBranchComparisonExplanation::from_canonical_record(
            record,
        );
    digest_string(
        "structural-branch-diagnostics-digest",
        &format!(
            "record={}|declaration={}|family={:?}|version={}|branch-diffs={}|artifact={}",
            explanation.record_identity().as_str(),
            explanation.declaration_identity(),
            explanation.fingerprint_family(),
            explanation.semantics_version(),
            explanation.branch_diff_count(),
            explanation.artifact_digest(),
        ),
    )
    .to_string()
}

fn rejection_diagnostics_digest(
    contract: &AdmittedStructuralComparisonContract,
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
) -> String {
    digest_string(
        "structural-rejection-diagnostics-digest",
        &format!(
            "declaration={}|planned={}|reduced={}|outcome={:?}",
            declaration_identity(contract),
            planned.digest(),
            reduced.digest(),
            reduced.outcome_class(),
        ),
    )
    .to_string()
}

fn ambiguity_report_json(reduced: &ReducedStructuralMatchSet) -> serde_json::Value {
    if reduced.outcome_class() != StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch {
        return serde_json::Value::Null;
    }

    json!({
        "outcome_class": format!("{:?}", reduced.outcome_class()),
        "retained_candidates": reduced.retained_candidates(),
    })
}

fn identity_separation_report_json(
    contract: &AdmittedStructuralComparisonContract,
    reduced: &ReducedStructuralMatchSet,
) -> serde_json::Value {
    if !matches!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::RejectedIdentityAuthorityConflict
            | StructuralMatchOutcomeClass::RejectedLineageStructuralDivergence
    ) {
        return serde_json::Value::Null;
    }

    json!({
        "declaration_identity": declaration_identity(contract),
        "outcome_class": format!("{:?}", reduced.outcome_class()),
        "retained_candidates": reduced.retained_candidates(),
    })
}

fn structural_diff_report_json(
    record: &BridgeCanonicalStructuralBranchComparisonRecord,
    reduced: &ReducedStructuralMatchSet,
) -> serde_json::Value {
    json!({
        "record_identity": record.record_identity().as_str(),
        "branch_diff_count": reduced.branch_diff_count(),
        "retained_candidates": reduced.retained_candidates(),
    })
}

fn counter_snapshot_json(
    counters: &BridgeStructuralCounters,
    replay_requested: bool,
) -> serde_json::Value {
    let counters = if replay_requested {
        counters.with_replay_request()
    } else {
        *counters
    };
    json!({
        "structural_declaration_count": counters.structural_declaration_count(),
        "structural_contract_count": counters.structural_contract_count(),
        "structural_fingerprint_count": counters.structural_fingerprint_count(),
        "structural_match_packet_count": counters.structural_match_packet_count(),
        "structural_candidate_count": counters.structural_candidate_count(),
        "structural_candidate_cohort_count": counters.structural_candidate_cohort_count(),
        "structural_exact_match_count": counters.structural_exact_match_count(),
        "structural_ambiguity_count": counters.structural_ambiguity_count(),
        "structural_mismatch_count": counters.structural_mismatch_count(),
        "structural_identity_conflict_count": counters.structural_identity_conflict_count(),
        "structural_lineage_divergence_count": counters.structural_lineage_divergence_count(),
        "structural_reuse_publication_count": counters.structural_reuse_publication_count(),
        "branch_comparison_count": counters.branch_comparison_count(),
        "branch_comparison_diff_count": counters.branch_comparison_diff_count(),
        "branch_comparison_drift_rejection_count": counters.branch_comparison_drift_rejection_count(),
        "structural_widened_scan_count": counters.structural_widened_scan_count(),
        "structural_replay_request_count": counters.structural_replay_request_count(),
        "structural_replay_mismatch_count": counters.structural_replay_mismatch_count(),
    })
}

fn rejection_counter_snapshot_json(
    contract: &AdmittedStructuralComparisonContract,
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
) -> serde_json::Value {
    json!({
        "structural_declaration_count": 1,
        "structural_contract_count": 1,
        "structural_fingerprint_count": planned.target_fingerprint().iter().count()
            + planned.comparison_fingerprint().iter().count()
            + planned.candidates().iter().filter(|candidate| candidate.fingerprint().is_some()).count(),
        "structural_match_packet_count": 1,
        "structural_candidate_count": planned.candidate_count(),
        "structural_candidate_cohort_count": planned.candidate_count(),
        "structural_exact_match_count": planned.candidates().iter().filter(|candidate| matches!(candidate.candidate_kind(), StructuralMatchCandidateKind::ExactAdvisoryMatch)).count(),
        "structural_ambiguity_count": usize::from(reduced.outcome_class() == StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch),
        "structural_mismatch_count": usize::from(reduced.outcome_class().mismatch_class().is_some()),
        "structural_identity_conflict_count": planned.candidates().iter().filter(|candidate| matches!(candidate.candidate_kind(), StructuralMatchCandidateKind::IdentityAuthorityConflict)).count(),
        "structural_lineage_divergence_count": planned.candidates().iter().filter(|candidate| matches!(candidate.candidate_kind(), StructuralMatchCandidateKind::LineageStructuralDivergence)).count(),
        "structural_reuse_publication_count": 0,
        "branch_comparison_count": usize::from(contract.validated_declaration().declaration().comparison_mode() == StructuralComparisonMode::BranchComparison),
        "branch_comparison_diff_count": planned.candidates().iter().filter(|candidate| matches!(candidate.candidate_kind(), StructuralMatchCandidateKind::BranchDiff)).count(),
        "branch_comparison_drift_rejection_count": 0,
        "structural_widened_scan_count": usize::from(contract.validated_declaration().declaration().candidate_scope() == StructuralCandidateSearchScope::ExplicitWidenedDebtScan),
        "structural_replay_request_count": 0,
        "structural_replay_mismatch_count": 0,
    })
}
