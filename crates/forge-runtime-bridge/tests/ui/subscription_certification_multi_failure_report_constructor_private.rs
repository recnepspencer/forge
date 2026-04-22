use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
    BridgeSubscriptionCertificationMultiFailurePrecedenceReport,
};

fn main() {
    let _report = BridgeSubscriptionCertificationMultiFailurePrecedenceReport {
        control_bundle_digest: "control".into(),
        hostile_bundle_digest: "hostile".into(),
        comparison_report_digest: "comparison".into(),
        primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary::BasisDrift,
        primary_failure_precedence_stage:
            BridgeSubscriptionCertificationFailurePrecedenceStage::BasisBinding,
        suppressed_failure_boundaries: Vec::new(),
        basis_drift_is_primary_without_registry_drift: true,
        suppressed_checkpoint_replay_and_diagnostics: true,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: "basis".into(),
        digest: "digest".into(),
    };
}
