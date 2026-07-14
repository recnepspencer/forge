use forge_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_security_scope_for_certification_test;
use forge_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_background_capacity,
    admit_background_pacing, admit_secure_io_scope_for_scheduler,
    admit_security_scope_for_scheduler, admit_store_published_isolation_capability,
    BackgroundCapacityAdmissionRequest, BackgroundDebtKind, BackgroundIdleCapacityLeaseRequest,
    BackgroundIoPressureShape, BackgroundPacingOutcome, BackgroundPacingProgressionDrift,
    BackgroundPacingProgressionEvidence, BackgroundResourceBudget,
    IoSchedulerBackendCapabilityRequirement, QueueSlot, SecureIoOperation,
    SecureIoPreservationRequest,
};
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_isolation::publish_scheduler_isolation_capability_for_certification_test;
use forge_store_security::admitted_store_internal_security_scope_for_io_qos_test;

use crate::{
    certify_io_qos_background_pacing, S6BackgroundPacingCertificationDenial,
    S6BackgroundPacingOutcomeKind,
};

#[test]
fn io_qos_background_pacing_certification_preserves_all_outcomes() {
    let cases = [
        (
            producer_yield_outcome(),
            direct_yield_outcome(),
            S6BackgroundPacingOutcomeKind::Yield,
            None,
        ),
        (
            producer_deferred_outcome(),
            direct_deferred_outcome(),
            S6BackgroundPacingOutcomeKind::Deferred,
            None,
        ),
        (
            producer_denied_outcome(),
            direct_denied_outcome(),
            S6BackgroundPacingOutcomeKind::Denied,
            None,
        ),
        (
            producer_stale_outcome(),
            direct_stale_outcome(),
            S6BackgroundPacingOutcomeKind::StaleRebindRequired,
            None,
        ),
        (
            producer_throttle_outcome(),
            direct_throttle_outcome(),
            S6BackgroundPacingOutcomeKind::Throttled,
            None,
        ),
        (
            producer_admitted_with_debt_outcome(),
            direct_admitted_with_debt_outcome(),
            S6BackgroundPacingOutcomeKind::AdmittedWithDebt,
            Some(BackgroundDebtKind::RepairPressure),
        ),
        (
            producer_violation_outcome(),
            direct_violation_outcome(),
            S6BackgroundPacingOutcomeKind::Violation,
            Some(BackgroundDebtKind::RepairPressure),
        ),
    ];

    for (actual, expected, expected_kind, expected_debt) in cases {
        let expected_counters = counters_for(expected);
        let evidence = certify_io_qos_background_pacing(actual, expected)
            .expect("independently built equivalent background pacing should certify");
        assert_eq!(evidence.outcome(), expected_kind);
        assert_eq!(evidence.counters(), expected_counters);
        assert_eq!(evidence.debt().map(|debt| debt.kind()), expected_debt);
    }
}

#[test]
fn io_qos_background_pacing_certification_denies_mismatched_outcomes() {
    assert_eq!(
        certify_io_qos_background_pacing(producer_yield_outcome(), direct_throttle_outcome()),
        Err(S6BackgroundPacingCertificationDenial::OutcomeMismatch)
    );
}

mod support;
use support::*;
