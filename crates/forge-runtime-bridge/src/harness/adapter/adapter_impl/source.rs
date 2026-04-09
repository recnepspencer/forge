use super::*;
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::routing::canonicalization::digest_string;
use crate::source::{AdmittedSourceContract, SourceFailureRecord, SourceMaterializationRecord};

pub(super) enum SourceHarnessTarget {
    Materialize { declaration_identity: String },
    MaterializeBatch { declaration_identity: String },
    Replay { declaration_identity: String },
    RejectUnregistered { declaration_identity: String },
    RejectOpenSnapshot { declaration_identity: String },
    RejectSnapshotDrift { declaration_identity: String },
}

pub(super) enum SourceHarnessExecution {
    Materialize {
        contract: AdmittedSourceContract,
        record: SourceMaterializationRecord,
    },
    Replay {
        contract: AdmittedSourceContract,
        record: SourceMaterializationRecord,
        replayed: SourceMaterializationRecord,
    },
    Rejected {
        failure: SourceFailureRecord,
    },
}

impl SourceHarnessExecution {
    pub(super) fn summary_json(&self) -> serde_json::Value {
        match self {
            Self::Materialize { contract, record } => json!({
                "source_declaration_identity": record.source_declaration_identity(),
                "source_contract_identity": contract.contract_identity().as_str(),
                "source_materialization_record_identity": record.record_identity().as_str(),
                "planned_packet_set_digest": record.planned_packet_set_digest(),
                "materialized_packet_set_digest": record.materialized_packet_set_digest(),
                "truth_view_digest": record.truth_view_digest(),
                "source_contract_digest": contract.digest(),
                "diagnostics_digest": diagnostics_digest(record),
                "failure_digest": serde_json::Value::Null,
                "counter_snapshot": counter_snapshot_json(record, false),
            }),
            Self::Replay {
                contract,
                record,
                replayed,
            } => json!({
                "source_declaration_identity": record.source_declaration_identity(),
                "source_contract_identity": contract.contract_identity().as_str(),
                "source_materialization_record_identity": record.record_identity().as_str(),
                "planned_packet_set_digest": record.planned_packet_set_digest(),
                "materialized_packet_set_digest": record.materialized_packet_set_digest(),
                "truth_view_digest": record.truth_view_digest(),
                "source_contract_digest": contract.digest(),
                "diagnostics_digest": diagnostics_digest(record),
                "replay_digest": replayed.digest(),
                "failure_digest": serde_json::Value::Null,
                "counter_snapshot": counter_snapshot_json(record, true),
            }),
            Self::Rejected { failure } => json!({
                "source_declaration_identity": failure.declaration_identity().as_str(),
                "source_contract_identity": serde_json::Value::Null,
                "source_materialization_record_identity": serde_json::Value::Null,
                "truth_view_digest": serde_json::Value::Null,
                "source_contract_digest": serde_json::Value::Null,
                "diagnostics_digest": rejection_diagnostics_digest(failure),
                "failure_digest": failure.digest(),
                "counter_snapshot": rejection_counter_snapshot_json(failure),
            }),
        }
    }

    pub(super) fn extensions_json(
        &self,
        runtime_bridge: &crate::facade::RuntimeBridge,
    ) -> BTreeMap<String, serde_json::Value> {
        match self {
            Self::Materialize { contract, record } => BTreeMap::from([
                (
                    "bridge_source_certification_bundle".to_string(),
                    certification_bundle_json(contract, record, None),
                ),
                source_record_extension(runtime_bridge, record),
            ]),
            Self::Replay {
                contract,
                record,
                replayed,
            } => BTreeMap::from([
                (
                    "bridge_source_certification_bundle".to_string(),
                    certification_bundle_json(contract, record, Some(replayed.digest())),
                ),
                source_record_extension(runtime_bridge, record),
                (
                    "bridge_source_replay_record".to_string(),
                    json!({
                        "source_materialization_record_identity": replayed.record_identity().as_str(),
                        "source_contract_identity": replayed.source_contract_identity(),
                        "source_declaration_identity": replayed.source_declaration_identity(),
                        "replay_digest": replayed.digest(),
                    }),
                ),
            ]),
            Self::Rejected { failure } => {
                let explanation = runtime_bridge
                    .diagnostics()
                    .explain_source_failure_record(failure);
                BTreeMap::from([
                    (
                        "bridge_source_certification_bundle".to_string(),
                        json!({
                            "truth_view_digest": serde_json::Value::Null,
                            "source_contract_digest": serde_json::Value::Null,
                            "routing_digest": serde_json::Value::Null,
                            "diagnostics_digest": rejection_diagnostics_digest(failure),
                            "failure_digest": failure.digest(),
                            "replay_digest": serde_json::Value::Null,
                            "counter_snapshot": rejection_counter_snapshot_json(failure),
                        }),
                    ),
                    (
                        "bridge_source_rejection".to_string(),
                        json!({
                            "source_failure_identity": failure.failure_identity().as_str(),
                            "source_declaration_identity": failure.declaration_identity().as_str(),
                            "selector_identity": failure.selector_identity(),
                            "failure_kind": format!("{:?}", failure.delivery_error_kind()),
                            "failure_class": format!("{:?}", failure.failure_class()),
                            "failure_detail": failure.detail(),
                            "source_capability_digest": failure.source_capability_digest(),
                            "digest": failure.digest(),
                            "explanation": {
                                "failure_identity": explanation.failure_identity(),
                                "declaration_identity": explanation.declaration_identity(),
                                "failure_class": format!("{:?}", explanation.failure_class()),
                                "delivery_error_kind": format!("{:?}", explanation.delivery_error_kind()),
                            },
                        }),
                    ),
                ])
            }
        }
    }
}

pub(super) fn parse_source_harness_target(
    target: &str,
) -> Option<Result<SourceHarnessTarget, BridgeHarnessError>> {
    if let Some(rest) = target.strip_prefix("source-materialize:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                SourceHarnessTarget::Materialize {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("source-materialize-batch:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                SourceHarnessTarget::MaterializeBatch {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("source-replay:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                SourceHarnessTarget::Replay {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("source-reject-unregistered:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                SourceHarnessTarget::RejectUnregistered {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("source-reject-open-snapshot:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                SourceHarnessTarget::RejectOpenSnapshot {
                    declaration_identity,
                }
            }),
        );
    }
    if let Some(rest) = target.strip_prefix("source-reject-snapshot-drift:") {
        return Some(
            parse_declaration_identity(rest).map(|declaration_identity| {
                SourceHarnessTarget::RejectSnapshotDrift {
                    declaration_identity,
                }
            }),
        );
    }
    None
}

pub(super) fn execute_source_request(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    target: SourceHarnessTarget,
) -> Result<SourceHarnessExecution, BridgeHarnessError> {
    match target {
        SourceHarnessTarget::Materialize {
            declaration_identity,
        } => {
            let (contract, record) =
                materialize_source_record(runtime_bridge, fixture, &declaration_identity)?;
            Ok(SourceHarnessExecution::Materialize { contract, record })
        }
        SourceHarnessTarget::MaterializeBatch {
            declaration_identity,
        } => {
            let (contract, record) =
                materialize_source_batch_record(runtime_bridge, fixture, &declaration_identity)?;
            Ok(SourceHarnessExecution::Materialize { contract, record })
        }
        SourceHarnessTarget::Replay {
            declaration_identity,
        } => {
            let (contract, record) =
                materialize_source_record(runtime_bridge, fixture, &declaration_identity)?;
            let replayed = runtime_bridge
                .replay_source_materialization_record(&record)
                .map_err(|error| {
                    BridgeHarnessError::new(format!("bridge source replay failed: {error}"))
                })?;
            Ok(SourceHarnessExecution::Replay {
                contract,
                record,
                replayed,
            })
        }
        SourceHarnessTarget::RejectUnregistered {
            declaration_identity,
        } => reject_unregistered_source(runtime_bridge, fixture, declaration_identity),
        SourceHarnessTarget::RejectOpenSnapshot {
            declaration_identity,
        } => reject_source_materialization(
            runtime_bridge,
            fixture,
            &declaration_identity,
            crate::error::BridgeDeliveryErrorKind::SnapshotAcquisitionFailure,
        ),
        SourceHarnessTarget::RejectSnapshotDrift {
            declaration_identity,
        } => reject_source_materialization(
            runtime_bridge,
            fixture,
            &declaration_identity,
            crate::error::BridgeDeliveryErrorKind::SnapshotIdentityMismatch,
        ),
    }
}

fn parse_declaration_identity(rest: &str) -> Result<String, BridgeHarnessError> {
    if rest.is_empty() {
        return Err(BridgeHarnessError::new(
            "source harness targets require a source declaration identity",
        ));
    }
    Ok(rest.to_string())
}

fn materialize_source_record(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<(AdmittedSourceContract, SourceMaterializationRecord), BridgeHarnessError> {
    let declaration = fixture
        .source_declarations()
        .iter()
        .find(|declaration| declaration.declaration_identity().as_str() == declaration_identity)
        .cloned()
        .ok_or_else(|| {
            BridgeHarnessError::new(format!(
                "bridge source fixture does not declare source `{declaration_identity}`"
            ))
        })?;
    let contract = runtime_bridge.admit_source(declaration).map_err(|error| {
        BridgeHarnessError::new(format!("bridge source admission failed: {error}"))
    })?;
    let observation = runtime_bridge
        .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge source materialization failed: {error}"))
        })?;
    let record = runtime_bridge
        .canonicalize_source_materialization_record(&contract, &observation)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge source materialization canonicalization failed: {error}"
            ))
        })?;
    Ok((contract, record))
}

fn materialize_source_batch_record(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
) -> Result<(AdmittedSourceContract, SourceMaterializationRecord), BridgeHarnessError> {
    let declaration = fixture
        .source_declarations()
        .iter()
        .find(|declaration| declaration.declaration_identity().as_str() == declaration_identity)
        .cloned()
        .ok_or_else(|| {
            BridgeHarnessError::new(format!(
                "bridge source fixture does not declare source `{declaration_identity}`"
            ))
        })?;
    let contract = runtime_bridge.admit_source(declaration).map_err(|error| {
        BridgeHarnessError::new(format!("bridge source admission failed: {error}"))
    })?;
    let materialized = runtime_bridge
        .materialize_source_packet_batch(
            &contract,
            vec![
                SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                    "entity-1", "profile",
                )]),
                SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                    "entity-2", "profile",
                )]),
            ],
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge source batch materialization failed: {error}"
            ))
        })?;
    let record = runtime_bridge
        .canonicalize_source_materialization_packet_set_record(&materialized)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "bridge source batch materialization canonicalization failed: {error}"
            ))
        })?;
    Ok((contract, record))
}

fn reject_unregistered_source(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: String,
) -> Result<SourceHarnessExecution, BridgeHarnessError> {
    let template = fixture.source_declarations().first().ok_or_else(|| {
        BridgeHarnessError::new("bridge source fixture requires a source declaration")
    })?;
    let declaration = crate::source::SourceDeclaration::new(
        crate::source::SourceDeclarationIdentity::new(declaration_identity.clone()),
        template.selector().clone(),
        template.required_capabilities().clone(),
    );
    let error = runtime_bridge
        .admit_source(declaration)
        .expect_err("hostile source harness request should be rejected");
    if error.kind() != crate::error::BridgeDeliveryErrorKind::SourceContractMismatch {
        return Err(BridgeHarnessError::new(format!(
            "hostile source rejection yielded unexpected error kind `{:?}`",
            error.kind()
        )));
    }
    let failure = runtime_bridge
        .diagnostics()
        .source_failure_for_declaration_identity(&declaration_identity)
        .ok_or_else(|| {
            BridgeHarnessError::new(format!(
                "bridge runtime did not retain canonical source failure for `{declaration_identity}`"
            ))
        })?;

    Ok(SourceHarnessExecution::Rejected { failure })
}

fn reject_source_materialization(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &str,
    expected_error_kind: crate::error::BridgeDeliveryErrorKind,
) -> Result<SourceHarnessExecution, BridgeHarnessError> {
    let declaration = fixture
        .source_declarations()
        .iter()
        .find(|declaration| declaration.declaration_identity().as_str() == declaration_identity)
        .cloned()
        .ok_or_else(|| {
            BridgeHarnessError::new(format!(
                "bridge source fixture does not declare source `{declaration_identity}`"
            ))
        })?;
    let contract = runtime_bridge.admit_source(declaration).map_err(|error| {
        BridgeHarnessError::new(format!("bridge source admission failed: {error}"))
    })?;
    let error = match runtime_bridge
        .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
    {
        Ok(_) => panic!("hostile source materialization should fail"),
        Err(error) => error,
    };
    if error.kind() != expected_error_kind {
        return Err(BridgeHarnessError::new(format!(
            "hostile source materialization yielded unexpected error kind `{:?}`",
            error.kind()
        )));
    }
    let failure = runtime_bridge
        .diagnostics()
        .source_failure_for_declaration_identity(declaration_identity)
        .ok_or_else(|| {
            BridgeHarnessError::new(format!(
                "bridge runtime did not retain canonical source failure for `{declaration_identity}`"
            ))
        })?;
    Ok(SourceHarnessExecution::Rejected { failure })
}

fn source_record_extension(
    runtime_bridge: &crate::facade::RuntimeBridge,
    record: &SourceMaterializationRecord,
) -> (String, serde_json::Value) {
    let explanation = runtime_bridge
        .diagnostics()
        .explain_source_materialization_record(record);
    let source_contract_digest = runtime_bridge
        .source_registry()
        .contract_for_identity(record.source_contract_identity())
        .map(|contract| contract.digest().to_string());
    (
        "bridge_source_materialization_record".to_string(),
        json!({
            "source_materialization_record_identity": record.record_identity().as_str(),
            "source_contract_identity": record.source_contract_identity(),
            "source_declaration_identity": record.source_declaration_identity(),
            "planned_packet_set_digest": record.planned_packet_set_digest(),
            "materialized_packet_set_digest": record.materialized_packet_set_digest(),
            "truth_view_digest": record.truth_view_digest(),
            "source_contract_digest": source_contract_digest,
            "source_capability_digest": record.source_capability_digest(),
            "adapter_capability_digest": record.adapter_capability_digest(),
            "planned_packet_digests": record.planned_packet_digests(),
            "read_packet_digests": record.read_packets().iter().map(crate::snapshot::SnapshotReadPacket::digest).collect::<Vec<_>>(),
            "authority_basis_digests": record.authority_basis_digests(),
            "snapshot_identities": record.snapshot_identities().iter().map(crate::snapshot::TruthSnapshotIdentity::as_str).collect::<Vec<_>>(),
            "materialization_paths": record.materialization_paths().iter().map(|path| format!("{path:?}")).collect::<Vec<_>>(),
            "digest": record.digest(),
            "explanation": {
                "record_identity": explanation.record_identity(),
                "source_contract_identity": explanation.source_contract_identity(),
                "source_declaration_identity": explanation.source_declaration_identity(),
                "truth_view_digest": explanation.truth_view_digest(),
                "planned_packet_set_digest": explanation.planned_packet_set_digest(),
                "materialized_packet_set_digest": explanation.materialized_packet_set_digest(),
                "packet_count": explanation.packet_count(),
                "snapshot_identities": explanation.snapshot_identities().iter().map(crate::snapshot::TruthSnapshotIdentity::as_str).collect::<Vec<_>>(),
                "materialization_paths": explanation.materialization_paths().iter().map(|path| format!("{path:?}")).collect::<Vec<_>>(),
            }
        }),
    )
}

fn certification_bundle_json(
    contract: &AdmittedSourceContract,
    record: &SourceMaterializationRecord,
    replay_digest: Option<&str>,
) -> serde_json::Value {
    json!({
        "truth_view_digest": record.truth_view_digest(),
        "source_contract_digest": contract.digest(),
        "routing_digest": serde_json::Value::Null,
        "diagnostics_digest": diagnostics_digest(record),
        "failure_digest": serde_json::Value::Null,
        "replay_digest": replay_digest,
        "counter_snapshot": counter_snapshot_json(record, replay_digest.is_some()),
    })
}

fn diagnostics_digest(record: &SourceMaterializationRecord) -> String {
    digest_string(
        "source-diagnostics-digest",
        &format!(
            "record={}|contract={}|truth-view={}|snapshots={}|paths={}",
            record.record_identity().as_str(),
            record.source_contract_identity(),
            record.truth_view_digest(),
            record
                .snapshot_identities()
                .iter()
                .map(crate::snapshot::TruthSnapshotIdentity::as_str)
                .collect::<Vec<_>>()
                .join(","),
            record
                .materialization_paths()
                .iter()
                .map(|path| format!("{path:?}"))
                .collect::<Vec<_>>()
                .join(","),
        ),
    )
    .to_string()
}

fn rejection_diagnostics_digest(failure: &SourceFailureRecord) -> String {
    digest_string(
        "source-rejection-diagnostics-digest",
        &format!(
            "failure={}|declaration={}|class={:?}|delivery-kind={:?}",
            failure.failure_identity().as_str(),
            failure.declaration_identity().as_str(),
            failure.failure_class(),
            failure.delivery_error_kind(),
        ),
    )
    .to_string()
}

fn counter_snapshot_json(
    record: &SourceMaterializationRecord,
    replay_requested: bool,
) -> serde_json::Value {
    let counters = record.counters();

    json!({
        "source_declaration_count": counters.source_declaration_count(),
        "source_contract_count": counters.source_contract_count(),
        "source_packet_count": counters.source_packet_count(),
        "source_packet_member_count": counters.source_packet_member_count(),
        "source_materialization_count": counters.source_materialization_count(),
        "source_snapshot_read_count": counters.source_snapshot_read_count(),
        "source_historical_read_count": counters.source_historical_read_count(),
        "source_branch_read_count": counters.source_branch_read_count(),
        "source_facet_read_count": counters.source_facet_read_count(),
        "source_capability_rejection_count": counters.source_capability_rejection_count(),
        "source_contract_mismatch_count": counters.source_contract_mismatch_count(),
        "source_adapter_fallback_count": counters.source_adapter_fallback_count(),
        "source_builder_configuration_conflict_count": counters.source_builder_configuration_conflict_count(),
        "source_replay_request_count": usize::from(replay_requested),
    })
}

fn rejection_counter_snapshot_json(failure: &SourceFailureRecord) -> serde_json::Value {
    json!({
        "source_declaration_count": 1,
        "source_contract_count": 0,
        "source_packet_count": 0,
        "source_packet_member_count": 0,
        "source_materialization_count": 0,
        "source_snapshot_read_count": 0,
        "source_historical_read_count": 0,
        "source_branch_read_count": 0,
        "source_facet_read_count": 0,
        "source_capability_rejection_count": 0,
        "source_contract_mismatch_count": usize::from(
            failure.delivery_error_kind() == crate::error::BridgeDeliveryErrorKind::SourceContractMismatch
        ),
        "source_adapter_fallback_count": 0,
        "source_builder_configuration_conflict_count": 0,
        "source_replay_request_count": 0,
        "retained_source_record_count": 0,
        "retained_failure_record_count": 0,
        "retained_source_failure_record_count": 1,
    })
}
