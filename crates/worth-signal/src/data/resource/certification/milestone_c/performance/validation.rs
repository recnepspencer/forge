use super::super::catalog::ResourceMilestoneCPolicyPerformanceClaimId;
use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryKind;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceCostContractId;
use crate::data::resource::ResourceCostPosture;

pub(super) fn validate_milestone_c_policy_performance(
    id: ResourceMilestoneCPolicyPerformanceClaimId,
    performance: ResourceBoundaryPerformanceEnvelope,
) -> Result<(), SignalError> {
    match id {
        ResourceMilestoneCPolicyPerformanceClaimId::RegistryFreezeOrderBounded => {
            if performance.boundary() != ResourceBoundaryKind::PolicyCompatibility
                || performance.cost_posture() != ResourceCostPosture::Verified
                || performance.cost_contract() != ResourceCostContractId::new(18)
                || performance.denied_count() != 0
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires verified registry freeze compatibility evidence",
                    id.label()
                )));
            }
        }
        ResourceMilestoneCPolicyPerformanceClaimId::RetryBudgetDenialZeroWake => {
            if performance.boundary() != ResourceBoundaryKind::RetrySchedule
                || performance.cost_posture() != ResourceCostPosture::Verified
                || performance.cost_contract() != ResourceCostContractId::new(5)
                || performance.admitted_count() != 0
                || performance.denied_count() != 1
                || performance.temporal_wake_footprint() != 0
                || performance.retry_budget_scope_touch_count() == 0
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires zero-wake retry budget denial evidence",
                    id.label()
                )));
            }
        }
        ResourceMilestoneCPolicyPerformanceClaimId::RetentionCompactionAvailabilityBounded => {
            if performance.boundary() != ResourceBoundaryKind::LifecycleRetentionCompaction
                || performance.cost_posture() != ResourceCostPosture::Verified
                || performance.cost_contract() != ResourceCostContractId::new(17)
                || performance.denied_count() != 0
                || performance.retained_history_allocation_count() == 0
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires retention compaction availability evidence",
                    id.label()
                )));
            }
        }
        ResourceMilestoneCPolicyPerformanceClaimId::DiagnosticsBudgetDenialZeroCold => {
            if performance.boundary() != ResourceBoundaryKind::DiagnosticsExpansion
                || performance.cost_posture() != ResourceCostPosture::DeniedFallback
                || performance.cost_contract() != ResourceCostContractId::new(16)
                || performance.admitted_count() != 0
                || performance.denied_count() != 1
                || performance.diagnostics_allocation_count() != 0
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires zero-cold diagnostics denial evidence",
                    id.label()
                )));
            }
        }
        ResourceMilestoneCPolicyPerformanceClaimId::ReplayCompatibilityDescriptorBounded => {
            if performance.boundary() != ResourceBoundaryKind::PolicyCompatibility
                || performance.cost_posture() != ResourceCostPosture::Verified
                || performance.cost_contract() != ResourceCostContractId::new(18)
                || performance.input_width() < 3
                || performance.denied_count() < 2
            {
                return Err(SignalError::invalid_input(format!(
                    "resource milestone C policy performance claim {} requires descriptor-bounded replay compatibility evidence",
                    id.label()
                )));
            }
        }
    }
    Ok(())
}
