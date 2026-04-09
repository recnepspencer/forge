use crate::harness::fixtures::BridgeHarnessFixture;
use crate::routing::canonicalization::digest_string;
use serde_json::json;

pub(super) fn preview_declaration(
    declaration_identity: &str,
    truth_branch: &str,
    signal_branch: &str,
) -> crate::facade::BridgePreviewSessionDeclaration {
    crate::facade::BridgePreviewSessionDeclaration::new(
        crate::facade::BridgePreviewSessionDeclarationIdentity::new(declaration_identity),
        crate::facade::BridgeRequestKind::Preview,
        crate::facade::BridgeSpeculativeBranchBinding::new(
            crate::facade::BridgeSpeculativeBranchBindingIdentity::new(format!(
                "{declaration_identity}:binding"
            )),
            crate::facade::TruthBranchIdentity::new(truth_branch),
            crate::facade::BridgeSignalBranchIdentity::new(signal_branch),
        ),
        format!("truth-view:{declaration_identity}"),
        format!("source-capability:{declaration_identity}"),
        format!("request-shape:{declaration_identity}"),
        format!("artifact-schema:{declaration_identity}"),
    )
}

pub(super) fn authoritative_routing_digest(
    runtime_bridge: &crate::facade::RuntimeBridge,
    commit_identity: &str,
) -> Result<String, super::BridgeHarnessError> {
    let result = runtime_bridge
        .deliver_invalidation(
            runtime_bridge
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit(
                    commit_identity,
                ))
                .map_err(|error| {
                    super::BridgeHarnessError::new(format!(
                        "authoritative route planning failed during speculation certification: {error}"
                    ))
                })?,
        )
        .map_err(|error| {
            super::BridgeHarnessError::new(format!(
                "authoritative route delivery failed during speculation certification: {error}"
            ))
        })?;
    Ok(digest_string(
        "speculation-authoritative-routing-digest",
        result.result_summary().route_identity().as_str(),
    )
    .to_string())
}

pub(super) fn first_commit_routing_digest(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<Option<String>, super::BridgeHarnessError> {
    fixture
        .committed_patches()
        .first()
        .map(|patch| patch.commit_identity().as_str().to_string())
        .map(|commit_identity| authoritative_routing_digest(runtime_bridge, &commit_identity))
        .transpose()
}

pub(super) fn speculative_resource_digest(
    execution_digest: &str,
    discard_digest: Option<&str>,
    promotion_digest: Option<&str>,
) -> String {
    digest_string(
        "speculative-resource-digest",
        &format!(
            "execution={execution_digest}|discard={}|promotion={}",
            discard_digest.unwrap_or("none"),
            promotion_digest.unwrap_or("none"),
        ),
    )
    .to_string()
}

pub(super) fn speculative_commit_digest(
    promoted_execution_digest: &str,
    promotion_digest: &str,
    discarded_execution_digest: &str,
    discard_digest: &str,
) -> String {
    digest_string(
        "speculative-commit-digest",
        &format!(
            "promoted-execution={promoted_execution_digest}|promotion={promotion_digest}|discarded-execution={discarded_execution_digest}|discard={discard_digest}",
        ),
    )
    .to_string()
}

pub(super) fn replay_digest(promoted_replay_digest: &str, discarded_replay_digest: &str) -> String {
    digest_string(
        "speculation-replay-digest",
        &format!(
            "promoted-replay={promoted_replay_digest}|discarded-replay={discarded_replay_digest}",
        ),
    )
    .to_string()
}

pub(super) fn preview_lifecycle_digest(lifecycle_digests: &[String]) -> String {
    digest_string("preview-lifecycle-digest", &lifecycle_digests.join("|")).to_string()
}

pub(super) fn discard_residue_report_json(
    discard_record: &crate::facade::BridgePreviewDiscardRecord,
) -> serde_json::Value {
    json!({
        "digest": discard_record.residue_report().digest(),
        "authoritative_residue_count": discard_record.residue_report().authoritative_residue_count(),
        "destroyable_residue_count": discard_record.residue_report().destroyable_residue_count(),
        "retained_non_authoritative_count": discard_record.residue_report().retained_non_authoritative_count(),
        "classes": discard_record
            .residue_report()
            .residue_classes()
            .iter()
            .map(|class| format!("{class:?}"))
            .collect::<Vec<_>>(),
    })
}

pub(super) fn preview_vs_authoritative_matrix_json(
    promotion_record: &crate::facade::BridgePreviewPromotionRecord,
    discarded_record: &crate::facade::BridgePreviewDiscardRecord,
    routing_digest: Option<&str>,
) -> serde_json::Value {
    json!({
        "promoted_preview": {
            "preview_session_identity": promotion_record.preview_session_identity(),
            "preview_execution_record_identity": promotion_record.preview_execution_record_identity().as_str(),
            "promotion_record_identity": promotion_record.record_identity().as_str(),
            "authoritative_commit_boundary_digest": promotion_record.authoritative_commit_boundary_digest(),
            "authoritative_artifact_digest": promotion_record.authoritative_artifact_digest(),
        },
        "discarded_preview": {
            "preview_session_identity": discarded_record.preview_session_identity(),
            "preview_execution_record_identity": discarded_record.preview_execution_record_identity().as_str(),
            "discard_record_identity": discarded_record.record_identity().as_str(),
            "discard_cleanup_outcome": format!("{:?}", discarded_record.cleanup_outcome()),
            "discard_residue_report_digest": discarded_record.residue_report().digest(),
        },
        "authoritative_route_digest": routing_digest,
    })
}

pub(super) fn counter_snapshot_json(
    counters: &crate::facade::BridgeSpeculationCounters,
) -> serde_json::Value {
    json!({
        "preview_session_count_touched": counters.preview_session_count_touched(),
        "branch_binding_proof_width": counters.branch_binding_proof_width(),
        "admissibility_proof_width": counters.admissibility_proof_width(),
        "preview_artifact_count": counters.preview_artifact_count(),
        "discard_artifact_count": counters.discard_artifact_count(),
        "destroyed_artifact_count": counters.destroyed_artifact_count(),
        "retained_non_authoritative_artifact_count": counters.retained_non_authoritative_artifact_count(),
        "promotion_proof_checks": counters.promotion_proof_checks(),
        "replay_bundle_width": counters.replay_bundle_width(),
    })
}
