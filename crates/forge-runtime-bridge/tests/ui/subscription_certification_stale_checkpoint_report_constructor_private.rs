use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
    BridgeSubscriptionCertificationStaleCheckpointReport,
};

fn main() {
    let _report = BridgeSubscriptionCertificationStaleCheckpointReport {
        fresh_bundle_digest: "fresh-bundle".into(),
        stale_bundle_digest: "stale-bundle".into(),
        fresh_checkpoint_digest: "fresh-checkpoint".into(),
        stale_checkpoint_digest: "stale-checkpoint".into(),
        comparison_report_digest: "comparison".into(),
        primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility,
        primary_failure_precedence_stage:
            BridgeSubscriptionCertificationFailurePrecedenceStage::CheckpointResumeOrReplay,
        checkpoint_drift_is_primary_without_replay_mismatch: true,
        suppressed_failure_boundary_count: 0,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: "basis".into(),
        digest: "digest".into(),
    };
}
