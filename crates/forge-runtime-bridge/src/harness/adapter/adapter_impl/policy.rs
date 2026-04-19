use super::*;
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::routing::canonicalization::digest_string;

pub(super) enum PolicyHarnessTarget {
    ProvenanceCertification,
    RejectionCertification,
    AmbientLeakCertification,
}

pub(super) enum PolicyHarnessExecution {
    Provenance {
        policy_digest: String,
        policy_matrix: serde_json::Value,
        policy_provenance_report: crate::facade::BridgePolicyProvenanceReport,
        request_policy_matrix: serde_json::Value,
        route_policy_matrix: serde_json::Value,
        routing_digest: Option<String>,
        replay_digest: String,
        diagnostics_digest: String,
        counter_snapshot: crate::facade::BridgePolicyCounters,
    },
    Rejection {
        policy_matrix: serde_json::Value,
        failure_digest: String,
        diagnostics_digest: String,
        counter_snapshot: crate::facade::BridgePolicyCounters,
    },
    AmbientLeak {
        policy_digest: String,
        policy_matrix: serde_json::Value,
        policy_provenance_report: crate::facade::BridgePolicyProvenanceReport,
        request_policy_matrix: serde_json::Value,
        replay_digest: String,
        diagnostics_digest: String,
        counter_snapshot: crate::facade::BridgePolicyCounters,
    },
}

impl PolicyHarnessExecution {
    pub(super) fn summary_json(&self) -> serde_json::Value {
        match self {
            Self::Provenance {
                policy_digest,
                policy_matrix,
                policy_provenance_report,
                request_policy_matrix,
                route_policy_matrix,
                routing_digest,
                replay_digest,
                diagnostics_digest,
                counter_snapshot,
            } => json!({
                "policy_digest": policy_digest,
                "policy_matrix": policy_matrix,
                "policy_provenance_report": provenance_report_json(policy_provenance_report),
                "request_policy_matrix": request_policy_matrix,
                "route_policy_matrix": route_policy_matrix,
                "routing_digest": routing_digest,
                "replay_digest": replay_digest,
                "diagnostics_digest": diagnostics_digest,
                "failure_digest": serde_json::Value::Null,
                "counter_snapshot": counter_snapshot_json(counter_snapshot),
            }),
            Self::Rejection {
                policy_matrix,
                failure_digest,
                diagnostics_digest,
                counter_snapshot,
            } => json!({
                "policy_digest": serde_json::Value::Null,
                "policy_matrix": policy_matrix,
                "policy_provenance_report": empty_provenance_report_json(),
                "request_policy_matrix": empty_matrix_json(),
                "routing_digest": serde_json::Value::Null,
                "replay_digest": serde_json::Value::Null,
                "failure_digest": failure_digest,
                "diagnostics_digest": diagnostics_digest,
                "counter_snapshot": counter_snapshot_json(counter_snapshot),
            }),
            Self::AmbientLeak {
                policy_digest,
                policy_matrix,
                policy_provenance_report,
                request_policy_matrix,
                replay_digest,
                diagnostics_digest,
                counter_snapshot,
            } => json!({
                "policy_digest": policy_digest,
                "policy_matrix": policy_matrix,
                "policy_provenance_report": provenance_report_json(policy_provenance_report),
                "request_policy_matrix": request_policy_matrix,
                "routing_digest": serde_json::Value::Null,
                "replay_digest": replay_digest,
                "diagnostics_digest": diagnostics_digest,
                "counter_snapshot": counter_snapshot_json(counter_snapshot),
                "failure_digest": serde_json::Value::Null,
            }),
        }
    }

    pub(super) fn extensions_json(
        &self,
        _runtime_bridge: &crate::facade::RuntimeBridge,
    ) -> BTreeMap<String, serde_json::Value> {
        match self {
            Self::Provenance {
                policy_digest,
                policy_matrix,
                policy_provenance_report,
                request_policy_matrix,
                route_policy_matrix,
                routing_digest,
                replay_digest,
                diagnostics_digest,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_policy_certification_bundle".to_string(),
                json!({
                    "policy_digest": policy_digest,
                    "policy_matrix": policy_matrix,
                    "policy_provenance_report": provenance_report_json(policy_provenance_report),
                    "request_policy_matrix": request_policy_matrix,
                    "route_policy_matrix": route_policy_matrix,
                    "routing_digest": routing_digest,
                    "replay_digest": replay_digest,
                    "diagnostics_digest": diagnostics_digest,
                    "counter_snapshot": counter_snapshot_json(counter_snapshot),
                }),
            )]),
            Self::Rejection {
                policy_matrix,
                failure_digest,
                diagnostics_digest,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_policy_certification_bundle".to_string(),
                json!({
                    "policy_digest": serde_json::Value::Null,
                    "policy_matrix": policy_matrix,
                    "policy_provenance_report": empty_provenance_report_json(),
                    "request_policy_matrix": empty_matrix_json(),
                    "routing_digest": serde_json::Value::Null,
                    "replay_digest": serde_json::Value::Null,
                    "failure_digest": failure_digest,
                    "diagnostics_digest": diagnostics_digest,
                    "counter_snapshot": counter_snapshot_json(counter_snapshot),
                }),
            )]),
            Self::AmbientLeak {
                policy_digest,
                policy_matrix,
                policy_provenance_report,
                request_policy_matrix,
                replay_digest,
                diagnostics_digest,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_policy_certification_bundle".to_string(),
                json!({
                    "policy_digest": policy_digest,
                    "policy_matrix": policy_matrix,
                    "policy_provenance_report": provenance_report_json(policy_provenance_report),
                    "request_policy_matrix": request_policy_matrix,
                    "routing_digest": serde_json::Value::Null,
                    "replay_digest": replay_digest,
                    "diagnostics_digest": diagnostics_digest,
                    "counter_snapshot": counter_snapshot_json(counter_snapshot),
                }),
            )]),
        }
    }
}

pub(super) fn parse_policy_harness_target(
    target: &str,
) -> Option<Result<PolicyHarnessTarget, BridgeHarnessError>> {
    match target {
        "policy-provenance-certify" => Some(Ok(PolicyHarnessTarget::ProvenanceCertification)),
        "policy-rejection-certify" => Some(Ok(PolicyHarnessTarget::RejectionCertification)),
        "policy-ambient-leak-certify" => Some(Ok(PolicyHarnessTarget::AmbientLeakCertification)),
        _ => None,
    }
}

pub(super) fn execute_policy_request(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    target: PolicyHarnessTarget,
) -> Result<PolicyHarnessExecution, BridgeHarnessError> {
    match target {
        PolicyHarnessTarget::ProvenanceCertification => {
            execute_provenance_certification(runtime_bridge, fixture)
        }
        PolicyHarnessTarget::RejectionCertification => {
            execute_rejection_certification(runtime_bridge)
        }
        PolicyHarnessTarget::AmbientLeakCertification => {
            execute_ambient_leak_certification(runtime_bridge, fixture)
        }
    }
}

fn execute_provenance_certification(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<PolicyHarnessExecution, BridgeHarnessError> {
    let deterministic = admitted_policy_bundle(
        runtime_bridge,
        crate::facade::BridgePolicyDeclaration::new(
            crate::facade::BridgePolicyDeclarationIdentity::new(
                "policy-cert:deterministic-authoritative",
            ),
            crate::facade::BridgeRequestKind::Authoritative,
            crate::facade::BridgeExecutionPolicyClass::DeterministicCanonical,
            crate::facade::BridgeDiagnosticsTier::Standard,
            true,
            true,
        ),
    )?;
    let optimized = admitted_policy_bundle(
        runtime_bridge,
        crate::facade::BridgePolicyDeclaration::new(
            crate::facade::BridgePolicyDeclarationIdentity::new("policy-cert:optimized-preview"),
            crate::facade::BridgeRequestKind::Preview,
            crate::facade::BridgeExecutionPolicyClass::Optimized,
            crate::facade::BridgeDiagnosticsTier::Exhaustive,
            false,
            false,
        ),
    )?;
    let policy_digest = combined_digest(
        "policy-provenance-equivalence",
        &[deterministic.contract.digest(), optimized.contract.digest()],
    );
    let replay_digest = combined_digest(
        "policy-provenance-replay",
        &[
            deterministic.replay_bundle.digest(),
            optimized.replay_bundle.digest(),
        ],
    );
    let diagnostics_digest = combined_digest(
        "policy-provenance-diagnostics",
        &[
            deterministic.provenance.digest(),
            optimized.provenance.digest(),
        ],
    );
    let routing_digest =
        first_commit_routing_digest(runtime_bridge, fixture, &deterministic.route_policy)?;
    let policy_matrix = json!({
        "rows": [
            admitted_policy_row_json("deterministic_authoritative", &deterministic),
            admitted_policy_row_json("optimized_preview", &optimized),
        ],
    });
    let policy_provenance_report = runtime_bridge.summarize_policy_provenance_report(vec![
        runtime_bridge.summarize_policy_provenance_row(
            "deterministic_authoritative",
            &deterministic.contract,
            &deterministic.lowered,
            &deterministic.provenance,
            &deterministic.replay_bundle,
        ),
        runtime_bridge.summarize_policy_provenance_row(
            "optimized_preview",
            &optimized.contract,
            &optimized.lowered,
            &optimized.provenance,
            &optimized.replay_bundle,
        ),
    ]);
    let route_policy_matrix = json!({
        "rows": [
            route_policy_row_json("deterministic_authoritative", &deterministic),
            route_policy_row_json("optimized_preview", &optimized),
        ],
    });
    let request_policy_matrix = json!({
        "rows": [
            request_policy_row_json(
                &runtime_bridge.summarize_policy_provenance_row(
                    "deterministic_authoritative",
                    &deterministic.contract,
                    &deterministic.lowered,
                    &deterministic.provenance,
                    &deterministic.replay_bundle,
                ),
                &deterministic,
            ),
            request_policy_row_json(
                &runtime_bridge.summarize_policy_provenance_row(
                    "optimized_preview",
                    &optimized.contract,
                    &optimized.lowered,
                    &optimized.provenance,
                    &optimized.replay_bundle,
                ),
                &optimized,
            ),
        ],
    });
    let counter_snapshot = combined_counter_snapshot([&deterministic, &optimized], 0, 0, 0, 0, 0);

    Ok(PolicyHarnessExecution::Provenance {
        policy_digest,
        policy_matrix,
        policy_provenance_report,
        request_policy_matrix,
        route_policy_matrix,
        routing_digest,
        replay_digest,
        diagnostics_digest,
        counter_snapshot,
    })
}

fn execute_rejection_certification(
    runtime_bridge: &crate::facade::RuntimeBridge,
) -> Result<PolicyHarnessExecution, BridgeHarnessError> {
    let optimized_authoritative_declaration = crate::facade::BridgePolicyDeclaration::new(
        crate::facade::BridgePolicyDeclarationIdentity::new(
            "policy-cert:rejection-optimized-authoritative",
        ),
        crate::facade::BridgeRequestKind::Authoritative,
        crate::facade::BridgeExecutionPolicyClass::Optimized,
        crate::facade::BridgeDiagnosticsTier::Standard,
        false,
        false,
    );
    let optimized_authoritative =
        rejected_policy_bundle(runtime_bridge, optimized_authoritative_declaration.clone())?;
    let replay_forbidden_source =
        crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    let replay_forbidden_runtime = crate::facade::RuntimeBridge::builder()
        .with_relational_source(replay_forbidden_source.clone())
        .with_truth_branch_head_source(replay_forbidden_source)
        .with_signal_sink(crate::harness::fixtures::RecordingSignalBridgeSink::default())
        .with_policy(crate::facade::BridgeRuntimePolicy::operational().with_replay_artifacts(false))
        .register_mapping(crate::facade::BridgeMappingRegistration::new(
            crate::facade::BridgeMappingId::new("policy-cert-registration"),
            crate::facade::TruthPatchScope::new(
                crate::facade::MappingSelector::exact("user"),
                crate::facade::MappingSelector::exact("profile"),
                crate::facade::MappingSelector::exact("name"),
            ),
            crate::facade::SignalInvalidationScope::new("signal.policy"),
            crate::facade::CoarseRoutingMode::Direct,
        ))
        .build()
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "policy rejection certification runtime build failed: {error}"
            ))
        })?;
    let replay_conflict_declaration = crate::facade::BridgePolicyDeclaration::new(
        crate::facade::BridgePolicyDeclarationIdentity::new(
            "policy-cert:rejection-replay-conflict",
        ),
        crate::facade::BridgeRequestKind::Preview,
        crate::facade::BridgeExecutionPolicyClass::Optimized,
        crate::facade::BridgeDiagnosticsTier::Standard,
        true,
        true,
    );
    let replay_conflict = rejected_policy_bundle(
        &replay_forbidden_runtime,
        replay_conflict_declaration.clone(),
    )?;
    let failure_digest = combined_digest(
        "policy-rejection-certification",
        &[optimized_authoritative.digest(), replay_conflict.digest()],
    );
    let diagnostics_digest = combined_digest(
        "policy-rejection-diagnostics",
        &[optimized_authoritative.detail(), replay_conflict.detail()],
    );
    let policy_matrix = json!({
        "rows": [
            rejection_row_json("optimized_authoritative", &optimized_authoritative),
            rejection_row_json("replay_conflict", &replay_conflict),
        ]
    });
    let counter_snapshot = crate::facade::BridgePolicyCounters::from_rejections(
        &[
            &optimized_authoritative_declaration,
            &replay_conflict_declaration,
        ],
        &[&optimized_authoritative, &replay_conflict],
        0,
    );

    Ok(PolicyHarnessExecution::Rejection {
        policy_matrix,
        failure_digest,
        diagnostics_digest,
        counter_snapshot,
    })
}

fn execute_ambient_leak_certification(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<PolicyHarnessExecution, BridgeHarnessError> {
    let preview_before = admitted_policy_bundle(
        runtime_bridge,
        crate::facade::BridgePolicyDeclaration::new(
            crate::facade::BridgePolicyDeclarationIdentity::new("policy-cert:preview-before"),
            crate::facade::BridgeRequestKind::Preview,
            crate::facade::BridgeExecutionPolicyClass::Optimized,
            crate::facade::BridgeDiagnosticsTier::Minimal,
            false,
            false,
        ),
    )?;
    let branch_local_resolution = runtime_bridge.resolve_truth_view_policy(
        &crate::facade::HistoricalEvaluationDeclaration::new(
            crate::facade::BridgeTruthViewSelector::branch_snapshot(
                crate::facade::TruthBranchIdentity::new("analysis"),
                first_snapshot_identity(fixture),
            ),
            crate::facade::BridgeReplayMode::Enabled,
            crate::facade::BridgeDiagnosticsTier::Standard,
            crate::facade::BridgeDeliveryIntent::PrepareSignalEvaluation,
        ),
    );
    let authoritative_middle = admitted_policy_bundle(
        runtime_bridge,
        crate::facade::BridgePolicyDeclaration::new(
            crate::facade::BridgePolicyDeclarationIdentity::new("policy-cert:authoritative-middle"),
            crate::facade::BridgeRequestKind::Authoritative,
            crate::facade::BridgeExecutionPolicyClass::DeterministicCanonical,
            crate::facade::BridgeDiagnosticsTier::Standard,
            true,
            true,
        ),
    )?;
    let historical_resolution = runtime_bridge.resolve_truth_view_policy(
        &crate::facade::HistoricalEvaluationDeclaration::new(
            crate::facade::BridgeTruthViewSelector::branch_snapshot(
                crate::facade::TruthBranchIdentity::new("history"),
                first_snapshot_identity(fixture),
            ),
            crate::facade::BridgeReplayMode::Enabled,
            crate::facade::BridgeDiagnosticsTier::Standard,
            crate::facade::BridgeDeliveryIntent::PrepareSignalEvaluation,
        ),
    );
    let preview_after = admitted_policy_bundle(
        runtime_bridge,
        crate::facade::BridgePolicyDeclaration::new(
            crate::facade::BridgePolicyDeclarationIdentity::new("policy-cert:preview-after"),
            crate::facade::BridgeRequestKind::Preview,
            crate::facade::BridgeExecutionPolicyClass::Optimized,
            crate::facade::BridgeDiagnosticsTier::Minimal,
            false,
            false,
        ),
    )?;

    let request_policy_matrix = json!({
        "branch_local_resolution": truth_view_resolution_label(&branch_local_resolution),
        "historical_resolution": truth_view_resolution_label(&historical_resolution),
        "rows": [
            request_policy_row_json(&runtime_bridge.summarize_policy_provenance_row(
                "preview_before",
                &preview_before.contract,
                &preview_before.lowered,
                &preview_before.provenance,
                &preview_before.replay_bundle,
            ), &preview_before),
            request_policy_row_json(&runtime_bridge.summarize_policy_provenance_row(
                "authoritative_middle",
                &authoritative_middle.contract,
                &authoritative_middle.lowered,
                &authoritative_middle.provenance,
                &authoritative_middle.replay_bundle,
            ), &authoritative_middle),
            request_policy_row_json(&runtime_bridge.summarize_policy_provenance_row(
                "preview_after",
                &preview_after.contract,
                &preview_after.lowered,
                &preview_after.provenance,
                &preview_after.replay_bundle,
            ), &preview_after),
        ],
    });
    let policy_provenance_report = runtime_bridge.summarize_policy_provenance_report(vec![
        runtime_bridge.summarize_policy_provenance_row(
            "preview_before",
            &preview_before.contract,
            &preview_before.lowered,
            &preview_before.provenance,
            &preview_before.replay_bundle,
        ),
        runtime_bridge.summarize_policy_provenance_row(
            "authoritative_middle",
            &authoritative_middle.contract,
            &authoritative_middle.lowered,
            &authoritative_middle.provenance,
            &authoritative_middle.replay_bundle,
        ),
        runtime_bridge.summarize_policy_provenance_row(
            "preview_after",
            &preview_after.contract,
            &preview_after.lowered,
            &preview_after.provenance,
            &preview_after.replay_bundle,
        ),
    ]);
    let policy_matrix = json!({
        "rows": [
            admitted_policy_row_json("preview_before", &preview_before),
            admitted_policy_row_json("authoritative_middle", &authoritative_middle),
            admitted_policy_row_json("preview_after", &preview_after),
        ],
    });
    let policy_digest = combined_digest(
        "policy-ambient-leak-certification",
        &[
            preview_before.contract.digest(),
            authoritative_middle.contract.digest(),
            preview_after.contract.digest(),
        ],
    );
    let replay_digest = combined_digest(
        "policy-ambient-leak-replay",
        &[
            preview_before.replay_bundle.digest(),
            authoritative_middle.replay_bundle.digest(),
            preview_after.replay_bundle.digest(),
        ],
    );
    let diagnostics_digest = combined_digest(
        "policy-ambient-leak-diagnostics",
        &[
            preview_before.provenance.digest(),
            authoritative_middle.provenance.digest(),
            preview_after.provenance.digest(),
        ],
    );
    let counter_snapshot = combined_counter_snapshot(
        [&preview_before, &authoritative_middle, &preview_after],
        3,
        2,
        1,
        0,
        0,
    );

    Ok(PolicyHarnessExecution::AmbientLeak {
        policy_digest,
        policy_matrix,
        policy_provenance_report,
        request_policy_matrix,
        replay_digest,
        diagnostics_digest,
        counter_snapshot,
    })
}

struct AdmittedPolicyBundle {
    contract: crate::facade::AdmittedBridgePolicyContract,
    lowered: crate::facade::LoweredBridgeExecutionPolicy,
    provenance: crate::facade::BridgePolicyProvenanceRecord,
    replay_bundle: crate::facade::BridgePolicyReplayBundle,
    route_policy: crate::facade::BridgeRoutePlanningPolicy,
}

fn admitted_policy_bundle(
    runtime_bridge: &crate::facade::RuntimeBridge,
    declaration: crate::facade::BridgePolicyDeclaration,
) -> Result<AdmittedPolicyBundle, BridgeHarnessError> {
    let contract = runtime_bridge
        .admit_policy_declaration(declaration)
        .map_err(|rejection| {
            BridgeHarnessError::new(format!("policy admission failed: {rejection:?}"))
        })?;
    let lowered = runtime_bridge.lower_admitted_policy(&contract);
    let provenance = runtime_bridge.canonicalize_policy_provenance(&contract, &lowered);
    let replay_bundle = runtime_bridge.replay_policy_bundle(&contract, &lowered, &provenance);
    let route_policy = runtime_bridge
        .project_route_planning_policy(&lowered)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "route planning policy projection failed during certification: {error}"
            ))
        })?;
    Ok(AdmittedPolicyBundle {
        contract,
        lowered,
        provenance,
        replay_bundle,
        route_policy,
    })
}

fn rejected_policy_bundle(
    runtime_bridge: &crate::facade::RuntimeBridge,
    declaration: crate::facade::BridgePolicyDeclaration,
) -> Result<crate::facade::BridgePolicyRejection, BridgeHarnessError> {
    match runtime_bridge.admit_policy_declaration(declaration) {
        Ok(_) => Err(BridgeHarnessError::new(
            "policy rejection certification unexpectedly admitted declaration",
        )),
        Err(rejection) => Ok(rejection),
    }
}

fn rejection_row_json(
    label: &str,
    rejection: &crate::facade::BridgePolicyRejection,
) -> serde_json::Value {
    json!({
        "label": label,
        "declaration_identity": rejection.declaration_identity().as_str(),
        "failure_kind": format!("{:?}", rejection.kind()),
        "stage": format!("{:?}", rejection.stage()),
        "field_kind": format!("{:?}", rejection.field_kind()),
        "primary_source": format!("{:?}", rejection.primary_source()),
        "secondary_source": format!("{:?}", rejection.conflicting_source()),
        "digest": rejection.digest(),
    })
}

fn admitted_policy_row_json(label: &str, bundle: &AdmittedPolicyBundle) -> serde_json::Value {
    json!({
        "label": label,
        "declaration_identity": bundle.contract.validated_declaration().declaration().declaration_identity().as_str(),
        "request_kind": format!("{:?}", bundle.contract.validated_declaration().declaration().request_kind()),
        "execution_class": format!("{:?}", bundle.contract.resolved_execution_class()),
        "diagnostics_tier": format!("{:?}", bundle.contract.resolved_diagnostics_tier()),
        "route_artifacts": bundle.contract.resolved_route_artifacts(),
        "replay_artifacts": bundle.contract.resolved_replay_artifacts(),
        "policy_digest": bundle.contract.digest(),
        "lowered_policy_digest": bundle.lowered.digest(),
        "provenance_digest": bundle.provenance.digest(),
        "replay_digest": bundle.replay_bundle.digest(),
    })
}

fn combined_digest(label: &str, values: &[&str]) -> String {
    digest_string(label, &values.join("|")).to_string()
}

fn empty_matrix_json() -> serde_json::Value {
    json!({ "rows": [] })
}

fn empty_provenance_report_json() -> serde_json::Value {
    json!({
        "digest": digest_string("policy-empty-provenance-report", "").to_string(),
        "rows": [],
    })
}

fn provenance_report_json(
    report: &crate::facade::BridgePolicyProvenanceReport,
) -> serde_json::Value {
    json!({
        "digest": report.digest(),
        "rows": report.rows().iter().map(provenance_row_json).collect::<Vec<_>>(),
    })
}

fn provenance_row_json(row: &crate::facade::BridgePolicyProvenanceReportRow) -> serde_json::Value {
    json!({
        "label": row.label(),
        "request_kind": format!("{:?}", row.request_kind()),
        "execution_class": format!("{:?}", row.execution_class()),
        "diagnostics_tier": format!("{:?}", row.diagnostics_tier()),
        "route_artifacts": row.route_artifacts(),
        "replay_artifacts": row.replay_artifacts(),
        "policy_digest": row.policy_digest(),
        "semantic_policy_digest": row.semantic_policy_digest(),
        "lowered_policy_digest": row.lowered_policy_digest(),
        "provenance_digest": row.provenance_digest(),
        "replay_digest": row.replay_digest(),
        "provenance_entries": row.provenance_entries().iter().map(|entry| json!({
            "field_kind": format!("{:?}", entry.field_kind()),
            "declared_source": format!("{:?}", entry.declared_source()),
            "operative_source": format!("{:?}", entry.operative_source()),
            "resolution": format!("{:?}", entry.resolution()),
        })).collect::<Vec<_>>(),
    })
}

fn request_policy_row_json(
    row: &crate::facade::BridgePolicyProvenanceReportRow,
    bundle: &AdmittedPolicyBundle,
) -> serde_json::Value {
    let mut value = provenance_row_json(row);
    if let Some(object) = value.as_object_mut() {
        object.insert(
            "route_planning_policy_digest".to_string(),
            json!(bundle.route_policy.digest()),
        );
        object.insert(
            "semantic_route_planning_policy_digest".to_string(),
            json!(semantic_route_policy_digest(&bundle.route_policy)),
        );
    }
    value
}

fn route_policy_row_json(label: &str, bundle: &AdmittedPolicyBundle) -> serde_json::Value {
    json!({
        "label": label,
        "route_planning_policy_digest": bundle.route_policy.digest(),
        "semantic_route_planning_policy_digest": semantic_route_policy_digest(&bundle.route_policy),
        "lowered_policy_identity": bundle.route_policy.lowered_policy_identity().as_str(),
        "execution_class": format!("{:?}", bundle.route_policy.execution_class()),
        "diagnostics_tier": format!("{:?}", bundle.route_policy.diagnostics_tier()),
        "route_artifacts": bundle.route_policy.route_artifacts(),
        "replay_artifacts": bundle.route_policy.replay_artifacts(),
    })
}

fn semantic_route_policy_digest(route_policy: &crate::facade::BridgeRoutePlanningPolicy) -> String {
    combined_digest(
        "semantic-route-planning-policy",
        &[
            &format!("{:?}", route_policy.execution_class()),
            &format!("{:?}", route_policy.diagnostics_tier()),
            &route_policy.route_artifacts().to_string(),
            &route_policy.replay_artifacts().to_string(),
        ],
    )
}

fn combined_counter_snapshot<const N: usize>(
    bundles: [&AdmittedPolicyBundle; N],
    policy_request_count: usize,
    truth_view_interleave_count: usize,
    preview_equivalence_preserved_count: usize,
    ambient_policy_leak_count: usize,
    replay_mismatch_count: usize,
) -> crate::facade::BridgePolicyCounters {
    let declarations = bundles
        .iter()
        .map(|bundle| bundle.contract.validated_declaration().declaration())
        .collect::<Vec<_>>();
    let contracts = bundles
        .iter()
        .map(|bundle| &bundle.contract)
        .collect::<Vec<_>>();
    let provenances = bundles
        .iter()
        .map(|bundle| &bundle.provenance)
        .collect::<Vec<_>>();
    let replay_bundles = bundles
        .iter()
        .map(|bundle| &bundle.replay_bundle)
        .collect::<Vec<_>>();

    crate::facade::BridgePolicyCounters::from_admitted_artifacts(
        declarations.as_slice(),
        contracts.as_slice(),
        provenances.as_slice(),
        replay_bundles.as_slice(),
        0,
        replay_mismatch_count,
        ambient_policy_leak_count,
        policy_request_count,
        truth_view_interleave_count,
        preview_equivalence_preserved_count,
        0,
        0,
        0,
    )
}

fn counter_snapshot_json(counters: &crate::facade::BridgePolicyCounters) -> serde_json::Value {
    json!({
        "declaration_count": counters.declaration_count(),
        "declaration_width_count": counters.declaration_width_count(),
        "admitted_contract_count": counters.admitted_contract_count(),
        "admission_width_count": counters.admission_width_count(),
        "rejected_contract_count": counters.rejected_contract_count(),
        "provenance_entry_count": counters.provenance_entry_count(),
        "provenance_width_count": counters.provenance_width_count(),
        "narrowed_field_count": counters.narrowed_field_count(),
        "inherited_field_count": counters.inherited_field_count(),
        "override_count": counters.override_count(),
        "ignored_field_count": counters.ignored_field_count(),
        "replay_bundle_count": counters.replay_bundle_count(),
        "replay_mismatch_count": counters.replay_mismatch_count(),
        "ambient_policy_leak_count": counters.ambient_policy_leak_count(),
        "policy_request_count": counters.policy_request_count(),
        "truth_view_interleave_count": counters.truth_view_interleave_count(),
        "preview_equivalence_preserved_count": counters.preview_equivalence_preserved_count(),
        "policy_source_ambiguity_count": counters.policy_source_ambiguity_count(),
        "substantive_illegality_count": counters.substantive_illegality_count(),
        "fallback_success_count": counters.fallback_success_count(),
    })
}

fn first_commit_routing_digest(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    route_policy: &crate::facade::BridgeRoutePlanningPolicy,
) -> Result<Option<String>, BridgeHarnessError> {
    fixture
        .committed_patches()
        .first()
        .map(|patch| {
            runtime_bridge
                .deliver_invalidation(
                    runtime_bridge
                        .plan_committed_patch_with_route_policy(
                            crate::facade::BridgeRouteRequest::for_commit(
                                patch.commit_identity().as_str(),
                            ),
                            route_policy,
                        )
                        .map_err(|error| {
                            BridgeHarnessError::new(format!(
                                "policy certification route planning failed: {error}"
                            ))
                        })?,
                )
                .map_err(|error| {
                    BridgeHarnessError::new(format!(
                        "policy certification route delivery failed: {error}"
                    ))
                })
                .map(|result| {
                    digest_string(
                        "policy-certification-routing",
                        result.result_summary().route_identity().as_str(),
                    )
                    .to_string()
                })
        })
        .transpose()
}

fn first_snapshot_identity(fixture: &BridgeHarnessFixture) -> crate::facade::TruthSnapshotIdentity {
    fixture
        .snapshots()
        .first()
        .map(|snapshot| snapshot.identity().clone())
        .unwrap_or_else(|| crate::facade::TruthSnapshotIdentity::new("snapshot-a"))
}

fn truth_view_resolution_label(
    resolution: &crate::facade::BridgeTruthViewPolicyResolution,
) -> &'static str {
    match resolution {
        crate::facade::BridgeTruthViewPolicyResolution::Admitted(_) => "Admitted",
        crate::facade::BridgeTruthViewPolicyResolution::Rejected(_) => "Rejected",
    }
}
