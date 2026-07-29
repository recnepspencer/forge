use super::{UiRebindPlanningDenial, UiRebindSubsystemKind, UiRebindSubsystemPlan};
use crate::runtime::rebind::{UiRebindBudgetInput, UiRebindLimit};

pub(super) fn require_terminal_decision_budget(
    decisions: &[super::super::UiIdentityLifecycleEntry],
    budget: UiRebindBudgetInput,
) -> Result<(), UiRebindPlanningDenial> {
    enforce(
        UiRebindLimit::TerminalDecisionRecords,
        budget.terminal_decision_records,
        decisions.len(),
    )
}

pub(super) fn require_compiled_plan_budget(
    scope: &super::super::UiResolvedAffectedScope,
    subsystems: &[UiRebindSubsystemPlan],
    budget: UiRebindBudgetInput,
) -> Result<(), UiRebindPlanningDenial> {
    enforce(
        UiRebindLimit::MeasurementAndAllocationEntries,
        budget.measurement_and_allocation_entries,
        target_count(subsystems, UiRebindSubsystemKind::Measurement)
            + target_count(subsystems, UiRebindSubsystemKind::Allocation),
    )?;
    enforce(
        UiRebindLimit::QueryBindingTransitions,
        budget.query_binding_transitions,
        target_count(subsystems, UiRebindSubsystemKind::Binding),
    )?;
    enforce(
        UiRebindLimit::Obligations,
        budget.obligations,
        target_count(subsystems, UiRebindSubsystemKind::Obligation),
    )?;
    enforce(
        UiRebindLimit::EvidenceLinkageEntries,
        budget.evidence_linkage_entries,
        evidence_linkage_count(scope),
    )
}

fn target_count(subsystems: &[UiRebindSubsystemPlan], kind: UiRebindSubsystemKind) -> usize {
    subsystems
        .binary_search_by_key(&kind, UiRebindSubsystemPlan::kind)
        .ok()
        .map(|index| subsystems[index].targets().len())
        .unwrap_or(0)
}

fn evidence_linkage_count(scope: &super::super::UiResolvedAffectedScope) -> usize {
    scope
        .lookups()
        .iter()
        .map(|lookup| lookup.predecessor().entries().len() + lookup.candidate().entries().len())
        .sum()
}

fn enforce(
    limit: UiRebindLimit,
    configured: usize,
    observed: usize,
) -> Result<(), UiRebindPlanningDenial> {
    if observed > configured {
        Err(UiRebindPlanningDenial::BudgetExceeded {
            limit,
            configured,
            observed,
        })
    } else {
        Ok(())
    }
}
