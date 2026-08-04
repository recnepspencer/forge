use super::super::catalog::{ResourceMilestoneBPerformanceClaimId, ResourceMilestoneBScenarioId};
use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryKind;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceCostPosture;
use crate::data::resource::ResourceDensityStrategy;

pub(super) fn require_performance(
    id: ResourceMilestoneBPerformanceClaimId,
    performance: ResourceBoundaryPerformanceEnvelope,
    expected_boundary: ResourceBoundaryKind,
    expected_cost_posture: ResourceCostPosture,
) -> Result<(), SignalError> {
    if performance.boundary() != expected_boundary {
        return Err(SignalError::invalid_input(format!(
            "resource milestone B performance claim {} requires {expected_boundary:?} boundary evidence, got {:?}",
            id.label(),
            performance.boundary()
        )));
    }
    if performance.cost_posture() != expected_cost_posture {
        return Err(SignalError::invalid_input(format!(
            "resource milestone B performance claim {} requires {expected_cost_posture:?} cost posture, got {:?}",
            id.label(),
            performance.cost_posture()
        )));
    }
    Ok(())
}

pub(super) fn validate_certification_family_performance(
    id: ResourceMilestoneBPerformanceClaimId,
    scenario: ResourceMilestoneBScenarioId,
    performance: ResourceBoundaryPerformanceEnvelope,
) -> Result<(), SignalError> {
    match scenario {
        ResourceMilestoneBScenarioId::LifecycleReplayParity => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::ReplayReconstruction,
                ResourceCostPosture::Debt,
            )?;
            if performance.operational_allocation_count() != 0
                || performance.retained_history_allocation_count() != 0
                || performance.diagnostics_allocation_count() != performance.input_width()
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy() != ResourceDensityStrategy::NotApplicable
            {
                return Err(performance_claim_error(
                    id,
                    "replay parity must expose diagnostics-only cold reconstruction debt",
                ));
            }
        }
        ResourceMilestoneBScenarioId::OutOfOrderSupersession => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::RequestAdmission,
                ResourceCostPosture::Verified,
            )?;
            if performance.input_width() != 1
                || performance.admitted_count() != 1
                || performance.denied_count() != 0
                || performance.lifecycle_transition_count() != 2
                || performance.operational_allocation_count() != 1
                || performance.retained_history_allocation_count() != 2
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy()
                    != ResourceDensityStrategy::BurstySortedDeduplicated
            {
                return Err(performance_claim_error(
                    id,
                    "supersession admission must stay one admitted request with explicit two-transition lineage",
                ));
            }
        }
        ResourceMilestoneBScenarioId::RollbackObservationEquivalence => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::CompletionRollback,
                ResourceCostPosture::Verified,
            )?;
            if performance.input_width() != 1
                || performance.admitted_count() != 1
                || performance.denied_count() != 0
                || performance.lifecycle_transition_count() != 0
                || performance.operational_allocation_count() != 0
                || performance.retained_history_allocation_count() != 0
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy() != ResourceDensityStrategy::NotApplicable
            {
                return Err(performance_claim_error(
                    id,
                    "rollback observation proof must not perform lifecycle or retained-history work",
                ));
            }
        }
        ResourceMilestoneBScenarioId::BranchRestoreReplayEquivalence => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::BranchRestore,
                ResourceCostPosture::Verified,
            )?;
            if performance.denied_count() != 0
                || performance.broad_scan_denial_count() == 0
                || performance.operational_allocation_count() != performance.admitted_count()
                || performance.retained_history_allocation_count()
                    != performance
                        .input_width()
                        .saturating_sub(performance.admitted_count())
                || performance.diagnostics_allocation_count() != 0
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy() != ResourceDensityStrategy::NotApplicable
            {
                return Err(performance_claim_error(
                    id,
                    "branch restore must bind retained summaries and broad rebuild denial without diagnostics work",
                ));
            }
        }
        ResourceMilestoneBScenarioId::InflightBoundedness => {
            require_performance(
                id,
                performance,
                ResourceBoundaryKind::CompletionBatchAdmission,
                ResourceCostPosture::Verified,
            )?;
            if performance.input_width() != 4
                || performance.admitted_count() != 1
                || performance.denied_count() != 3
                || performance.lifecycle_transition_count() != 1
                || performance.operational_allocation_count() != 3
                || performance.retained_history_allocation_count() != 0
                || performance.diagnostics_allocation_count() != 4
                || performance.facade_report_allocation_count() != 1
                || performance.density_strategy()
                    != ResourceDensityStrategy::BurstySortedDeduplicated
            {
                return Err(performance_claim_error(
                    id,
                    "inflight boundedness must stay a bursty inflight-local completion boundary with explicit mixed denial pressure and attributable per-envelope diagnostics",
                ));
            }
        }
        ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected
        | ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected
        | ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected
        | ResourceMilestoneBScenarioId::MalformedCompletionRejected
        | ResourceMilestoneBScenarioId::DuplicateCompletionRejected
        | ResourceMilestoneBScenarioId::ContradictoryCompletionRejected
        | ResourceMilestoneBScenarioId::UnknownRequestCompletionRejected => {
            return Err(performance_claim_error(
                id,
                "hostile completion scenarios are certified by the hostile closeout claim",
            ));
        }
    }
    Ok(())
}

pub(super) fn performance_claim_error(
    id: ResourceMilestoneBPerformanceClaimId,
    reason: &'static str,
) -> SignalError {
    SignalError::invalid_input(format!(
        "resource milestone B performance claim {} failed: {reason}",
        id.label()
    ))
}
