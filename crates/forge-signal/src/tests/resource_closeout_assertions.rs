use crate::facade::*;

pub(super) fn required_scenario_row(
    matrix: &ResourceMilestoneBScenarioMatrix,
    scenario: ResourceMilestoneBScenarioId,
) -> &ResourceMilestoneBScenarioRow {
    let matches = matrix
        .rows()
        .iter()
        .filter(|row| row.id() == scenario)
        .count();
    assert_eq!(
        matches,
        1,
        "scenario {} must appear exactly once",
        scenario.label()
    );
    matrix
        .rows()
        .iter()
        .find(|row| row.id() == scenario)
        .expect("required scenario row should exist after uniqueness assertion")
}

pub(super) fn required_hostile_evidence_row(
    evidence: &ResourceMilestoneBHostileScenarioEvidence,
    scenario: ResourceMilestoneBScenarioId,
) -> &ResourceMilestoneBHostileScenarioEvidenceRow {
    let matches = evidence
        .rows()
        .iter()
        .filter(|row| row.id() == scenario)
        .count();
    assert_eq!(
        matches,
        1,
        "hostile scenario {} must appear exactly once",
        scenario.label()
    );
    evidence
        .rows()
        .iter()
        .find(|row| row.id() == scenario)
        .expect("required hostile evidence row should exist after uniqueness assertion")
}

pub(super) fn required_performance_claim_row(
    closeout: &ResourceMilestoneBPerformanceCloseout,
    claim: ResourceMilestoneBPerformanceClaimId,
) -> &ResourceMilestoneBPerformanceCloseoutRow {
    let matches = closeout
        .rows()
        .iter()
        .filter(|row| row.id() == claim)
        .count();
    assert_eq!(
        matches,
        1,
        "claim {} must appear exactly once",
        claim.label()
    );
    closeout
        .rows()
        .iter()
        .find(|row| row.id() == claim)
        .expect("required performance claim should exist after uniqueness assertion")
}

fn assert_boundary_shape(
    performance: ResourceBoundaryPerformanceEnvelope,
    boundary: ResourceBoundaryKind,
    cost_contract: u64,
    cost_posture: ResourceCostPosture,
    density_strategy: ResourceDensityStrategy,
) {
    assert_eq!(performance.boundary(), boundary);
    assert_eq!(
        performance.cost_contract(),
        ResourceCostContractId::new(cost_contract)
    );
    assert_eq!(performance.cost_posture(), cost_posture);
    assert_eq!(performance.density_strategy(), density_strategy);
    assert_eq!(performance.temporal_wake_footprint(), 0);
    assert_eq!(performance.facade_report_allocation_count(), 1);
}

pub(super) fn assert_hostile_evidence_shape(row: &ResourceMilestoneBHostileScenarioEvidenceRow) {
    let expected_denial = row
        .id()
        .completion_denial_class()
        .expect("hostile scenario should declare expected denial class");
    assert_eq!(row.expected_denial_class(), expected_denial);
    assert_eq!(row.denied_completion().class(), expected_denial);
    assert!(!row.evidence_digest().is_empty());

    let performance = row.performance();
    assert_boundary_shape(
        performance,
        ResourceBoundaryKind::CompletionAdmission,
        2,
        ResourceCostPosture::Verified,
        ResourceDensityStrategy::SparseIndexedLookup,
    );
    assert_eq!(performance.input_width(), 1);
    assert_eq!(performance.admitted_count(), 0);
    assert_eq!(performance.denied_count(), 1);
    assert_eq!(performance.lifecycle_transition_count(), 0);
    assert_eq!(performance.operational_allocation_count(), 0);
    assert_eq!(performance.retained_history_allocation_count(), 1);
    assert_eq!(performance.diagnostics_allocation_count(), 0);
}

pub(super) fn assert_performance_closeout_claim_shape(
    row: &ResourceMilestoneBPerformanceCloseoutRow,
) {
    assert!(row.passed());
    assert!(!row.evidence_digest().is_empty());
    let performance = row.performance();

    match row.id() {
        ResourceMilestoneBPerformanceClaimId::LifecycleReplayParityDebtBounded => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::ReplayReconstruction,
                14,
                ResourceCostPosture::Debt,
                ResourceDensityStrategy::NotApplicable,
            );
            assert_eq!(performance.operational_allocation_count(), 0);
            assert_eq!(performance.retained_history_allocation_count(), 0);
            assert_eq!(
                performance.diagnostics_allocation_count(),
                performance.input_width()
            );
        }
        ResourceMilestoneBPerformanceClaimId::OutOfOrderSupersessionAdmissionBounded => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::RequestAdmission,
                1,
                ResourceCostPosture::Verified,
                ResourceDensityStrategy::BurstySortedDeduplicated,
            );
            assert_eq!(performance.input_width(), 1);
            assert_eq!(performance.admitted_count(), 1);
            assert_eq!(performance.denied_count(), 0);
            assert_eq!(performance.lifecycle_transition_count(), 2);
            assert_eq!(performance.operational_allocation_count(), 1);
            assert_eq!(performance.retained_history_allocation_count(), 2);
            assert_eq!(performance.diagnostics_allocation_count(), 0);
        }
        ResourceMilestoneBPerformanceClaimId::RollbackObservationRollbackBounded => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::CompletionRollback,
                11,
                ResourceCostPosture::Verified,
                ResourceDensityStrategy::NotApplicable,
            );
            assert_eq!(performance.input_width(), 1);
            assert_eq!(performance.admitted_count(), 1);
            assert_eq!(performance.denied_count(), 0);
            assert_eq!(performance.lifecycle_transition_count(), 0);
            assert_eq!(performance.operational_allocation_count(), 0);
            assert_eq!(performance.retained_history_allocation_count(), 0);
            assert_eq!(performance.diagnostics_allocation_count(), 0);
        }
        ResourceMilestoneBPerformanceClaimId::BranchRestoreReplayRestoreBounded => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::BranchRestore,
                13,
                ResourceCostPosture::Verified,
                ResourceDensityStrategy::NotApplicable,
            );
            assert_eq!(performance.denied_count(), 0);
            assert!(performance.broad_scan_denial_count() > 0);
            assert_eq!(
                performance.operational_allocation_count(),
                performance.admitted_count()
            );
            assert_eq!(
                performance.retained_history_allocation_count(),
                performance
                    .input_width()
                    .saturating_sub(performance.admitted_count())
            );
            assert_eq!(performance.diagnostics_allocation_count(), 0);
        }
        ResourceMilestoneBPerformanceClaimId::InflightBoundednessAdmissionBounded => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::RequestAdmission,
                1,
                ResourceCostPosture::Verified,
                ResourceDensityStrategy::SparseIndexedLookup,
            );
            assert_eq!(performance.input_width(), 1);
            assert_eq!(performance.admitted_count(), 1);
            assert_eq!(performance.denied_count(), 0);
            assert_eq!(performance.lifecycle_transition_count(), 1);
            assert_eq!(performance.operational_allocation_count(), 1);
            assert_eq!(performance.retained_history_allocation_count(), 1);
            assert_eq!(performance.diagnostics_allocation_count(), 0);
        }
        ResourceMilestoneBPerformanceClaimId::RuntimeSummaryReadZeroColdReconstruction => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::SummaryRead,
                15,
                ResourceCostPosture::Verified,
                ResourceDensityStrategy::NotApplicable,
            );
            assert_eq!(performance.input_width(), 1);
            assert_eq!(performance.admitted_count(), 1);
            assert_eq!(performance.denied_count(), 0);
            assert_eq!(performance.lifecycle_transition_count(), 0);
            assert_eq!(performance.operational_allocation_count(), 0);
            assert_eq!(performance.retained_history_allocation_count(), 0);
            assert_eq!(performance.diagnostics_allocation_count(), 0);
        }
        ResourceMilestoneBPerformanceClaimId::DiagnosticsExpansionBudgetedColdReconstruction => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::DiagnosticsExpansion,
                16,
                ResourceCostPosture::Debt,
                ResourceDensityStrategy::NotApplicable,
            );
            assert_eq!(performance.denied_count(), 0);
            assert_eq!(performance.operational_allocation_count(), 0);
            assert_eq!(performance.retained_history_allocation_count(), 0);
            assert_eq!(
                performance.diagnostics_allocation_count(),
                performance.broad_scan_denial_count()
            );
            assert!(performance.diagnostics_allocation_count() > 0);
        }
        ResourceMilestoneBPerformanceClaimId::DiagnosticsExpansionBudgetDenial => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::DiagnosticsExpansion,
                16,
                ResourceCostPosture::DeniedFallback,
                ResourceDensityStrategy::NotApplicable,
            );
            assert_eq!(performance.admitted_count(), 0);
            assert_eq!(performance.denied_count(), 1);
            assert!(performance.broad_scan_denial_count() > 0);
            assert_eq!(performance.operational_allocation_count(), 0);
            assert_eq!(performance.retained_history_allocation_count(), 0);
            assert_eq!(performance.diagnostics_allocation_count(), 0);
        }
        ResourceMilestoneBPerformanceClaimId::HostileCompletionDenialsScalarBounded => {
            assert_boundary_shape(
                performance,
                ResourceBoundaryKind::CompletionAdmission,
                2,
                ResourceCostPosture::Verified,
                ResourceDensityStrategy::NotApplicable,
            );
            assert_eq!(
                performance.input_width(),
                REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS.len() as u32
            );
            assert_eq!(performance.admitted_count(), 0);
            assert_eq!(
                performance.denied_count(),
                REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS.len() as u32
            );
            assert_eq!(performance.lifecycle_transition_count(), 0);
            assert_eq!(performance.operational_allocation_count(), 0);
            assert_eq!(
                performance.retained_history_allocation_count(),
                performance.denied_count()
            );
            assert_eq!(performance.diagnostics_allocation_count(), 0);
        }
    }
}
