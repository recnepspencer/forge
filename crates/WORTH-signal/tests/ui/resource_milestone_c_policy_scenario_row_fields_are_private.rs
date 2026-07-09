use worth_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceMilestoneCPolicyCertificationFamily,
    ResourceMilestoneCPolicyScenarioEvidenceKind, ResourceMilestoneCPolicyScenarioId,
    ResourceMilestoneCPolicyScenarioRow, ResourcePolicyRestoreCompatibilityDenialClass,
    ResourceRetryDenialClass, ResourceTimeoutHeartbeatExtensionDenialClass,
};

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _WORTHd = ResourceMilestoneCPolicyScenarioRow {
        id: ResourceMilestoneCPolicyScenarioId::RetryBudgetExhaustionRejected,
        evidence_kind: ResourceMilestoneCPolicyScenarioEvidenceKind::ReplayCompatibilityDenial,
        certification_family: Some(
            ResourceMilestoneCPolicyCertificationFamily::AsyncRetryBudgetAndBackoffCertification,
        ),
        policy_provenance_digest: Some(String::new()),
        retry_denial_class: Some(ResourceRetryDenialClass::RetryBudgetExhausted),
        timeout_heartbeat_denial_class: Some(
            ResourceTimeoutHeartbeatExtensionDenialClass::NonActiveRequest,
        ),
        replay_restore_denial_class: Some(
            ResourcePolicyRestoreCompatibilityDenialClass::MissingDescriptor,
        ),
        evidence_digest: String::new(),
        performance: WORTHd_performance(),
        passed: true,
    };
}
