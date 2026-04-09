use super::{shared, BridgeHarnessError, SpeculationHarnessExecution};
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::routing::canonicalization::digest_string;

pub(super) fn execute_promotion_certification(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<SpeculationHarnessExecution, BridgeHarnessError> {
    let admitted = runtime_bridge
        .admit_preview_session(
            crate::facade::BridgePreviewSessionIdentity::new("harness:speculation-promotion"),
            shared::preview_declaration("harness:speculation-promotion", "main", "signal:promotion"),
        )
        .map_err(|error| BridgeHarnessError::new(format!("speculation admission failed: {error}")))?;
    let (promoted_active, promoted_execution_record) =
        runtime_bridge.activate_preview_session(admitted, 3, 1, 2);
    let proof = promoted_active.promotion_admissibility_proof();
    let (_promoted, promotion_record) = runtime_bridge
        .promote_preview_session(
            promoted_active,
            &promoted_execution_record,
            &proof,
            "commit-boundary:harness",
            "authoritative-artifact:harness",
        )
        .map_err(|error| BridgeHarnessError::new(format!("speculation promotion failed: {error}")))?;
    let promoted_replay_bundle = runtime_bridge
        .replay_preview_bundle("harness:speculation-promotion")
        .map_err(|error| BridgeHarnessError::new(format!("speculation replay failed: {error}")))?;

    let discarded_admitted = runtime_bridge
        .admit_preview_session(
            crate::facade::BridgePreviewSessionIdentity::new("harness:speculation-discard-sibling"),
            shared::preview_declaration(
                "harness:speculation-discard-sibling",
                "main",
                "signal:promotion",
            ),
        )
        .map_err(|error| BridgeHarnessError::new(format!("speculation sibling admission failed: {error}")))?;
    let (discarded_active, discarded_execution_record) =
        runtime_bridge.activate_preview_session(discarded_admitted, 3, 1, 2);
    let (_discarded, discarded_record) = runtime_bridge
        .discard_preview_session(
            discarded_active,
            &discarded_execution_record,
            vec![
                crate::facade::BridgePreviewResidueClass::PreviewExecutionRetained,
                crate::facade::BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
                crate::facade::BridgePreviewResidueClass::TemporaryRoutingResidue,
                crate::facade::BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            ],
        )
        .map_err(|error| BridgeHarnessError::new(format!("speculation sibling discard failed: {error}")))?;
    let discarded_replay_bundle = runtime_bridge
        .replay_preview_bundle("harness:speculation-discard-sibling")
        .map_err(|error| BridgeHarnessError::new(format!("speculation sibling replay failed: {error}")))?;

    let routing_digest = shared::first_commit_routing_digest(runtime_bridge, fixture)?;
    let diagnostics_digest = digest_string(
        "speculation-diagnostics-digest",
        &format!(
            "promotion={}|discard={}|promoted-replay={}|discarded-replay={}|tier={:?}",
            runtime_bridge
                .diagnostics()
                .explain_preview_promotion_record(&promotion_record)
                .preview_promotion_record_identity(),
            runtime_bridge
                .diagnostics()
                .explain_preview_discard_record(&discarded_record)
                .preview_discard_record_identity(),
            runtime_bridge
                .diagnostics()
                .explain_preview_replay_bundle(&promoted_replay_bundle)
                .replay_bundle_digest(),
            runtime_bridge
                .diagnostics()
                .explain_preview_replay_bundle(&discarded_replay_bundle)
                .replay_bundle_digest(),
            runtime_bridge.policy().diagnostics_tier(),
        ),
    )
    .to_string();

    Ok(SpeculationHarnessExecution::Promotion {
        promoted_execution_record,
        promotion_record,
        promoted_replay_bundle,
        discarded_execution_record,
        discarded_record,
        discarded_replay_bundle,
        routing_digest,
        diagnostics_digest,
    })
}
