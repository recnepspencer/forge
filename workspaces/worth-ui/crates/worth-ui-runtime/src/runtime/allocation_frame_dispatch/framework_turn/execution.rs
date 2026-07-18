use crate::runtime::launch::runtime_instance::WorthUiRuntime;

use super::transition_planning::{
    UiFrameworkTransitionAuthorityPlan, UiFrameworkTransitionFamilyPlan,
    UiPlannedFrameworkTransition,
};

/// Borrowed proof that the ordinary framework turn closed and pumped without
/// producing a sealed source frame that must first be consumed by Phase 5.
#[derive(Debug)]
pub struct WorthUiFrameworkTurnExecution<'runtime> {
    pub(crate) _runtime: &'runtime WorthUiRuntime,
    pub(super) boundary: crate::runtime::WorthUiFrameBoundary,
    pub(super) planning_counters: super::UiFrameworkTransitionPlanningCounters,
}

impl WorthUiFrameworkTurnExecution<'_> {
    pub fn activation_boundary(&self) -> &crate::runtime::WorthUiFrameBoundary {
        &self.boundary
    }

    pub fn into_activation_boundary(self) -> crate::runtime::WorthUiFrameBoundary {
        self.boundary
    }

    pub fn planning_counters(&self) -> super::UiFrameworkTransitionPlanningCounters {
        self.planning_counters
    }
}

pub(super) fn execute_planned_framework_transition(
    runtime: &mut WorthUiRuntime,
    planned: UiPlannedFrameworkTransition,
) -> super::WorthUiFrameworkTurnCompletion<'_> {
    let (generation, predecessor_epoch, authority, planning_counters, family) =
        planned.into_parts();
    if runtime.active.generation_identity() != &generation {
        return super::WorthUiFrameworkTurnCompletion::FrameworkTransitionExecutionDenied {
            denial: super::UiFrameworkTransitionExecutionDenial::ActiveApplicationGenerationChanged,
        };
    }
    if runtime.active.frame_epoch() != predecessor_epoch {
        return super::WorthUiFrameworkTurnCompletion::FrameworkTransitionExecutionDenied {
            denial: super::UiFrameworkTransitionExecutionDenial::ActiveFrameEpochChanged,
        };
    }
    match authority {
        UiFrameworkTransitionAuthorityPlan::NoIngress => {}
        UiFrameworkTransitionAuthorityPlan::AdmittedFrame {
            frame_epoch_assignment,
            source_order_transition,
        } => {
            if (*source_order_transition)
                .commit(&mut runtime.allocation_source_order_ledger)
                .is_err()
            {
                return super::WorthUiFrameworkTurnCompletion::FrameworkTransitionExecutionDenied {
                    denial:
                        super::UiFrameworkTransitionExecutionDenial::SourceOrderAuthorityChanged,
                };
            }
            runtime
                .active
                .apply_allocation_frame_epoch_assignment(frame_epoch_assignment);
        }
    }
    execute_family(runtime, family, planning_counters)
}

/// A collection callback that unwinds may have submitted a valid partial
/// frame. Advance only the transport authorities needed to retire that frame;
/// never execute its semantic policy family.
pub(super) fn acknowledge_discarded_framework_transition(
    runtime: &mut WorthUiRuntime,
    planned: UiPlannedFrameworkTransition,
) {
    let (generation, predecessor_epoch, authority, _, family) = planned.into_parts();
    if runtime.active.generation_identity() != &generation
        || runtime.active.frame_epoch() != predecessor_epoch
    {
        return;
    }
    if let UiFrameworkTransitionAuthorityPlan::AdmittedFrame {
        frame_epoch_assignment,
        source_order_transition,
    } = authority
    {
        if (*source_order_transition)
            .commit(&mut runtime.allocation_source_order_ledger)
            .is_ok()
        {
            runtime
                .active
                .apply_allocation_frame_epoch_assignment(frame_epoch_assignment);
        }
    }
    drop(family);
}

fn execute_family(
    runtime: &mut WorthUiRuntime,
    family: UiFrameworkTransitionFamilyPlan,
    counters: super::UiFrameworkTransitionPlanningCounters,
) -> super::WorthUiFrameworkTurnCompletion<'_> {
    match family {
        UiFrameworkTransitionFamilyPlan::NoIngress { boundary } => {
            super::WorthUiFrameworkTurnCompletion::ReadyToExecute {
                execution: WorthUiFrameworkTurnExecution {
                    _runtime: runtime,
                    boundary,
                    planning_counters: counters,
                },
            }
        }
        UiFrameworkTransitionFamilyPlan::Ordinary(execution) => {
            super::policy_execution::ordinary::execute(
                &runtime.allocation_receipt_ledger,
                &runtime.allocation_invalidation_index,
                execution,
                counters,
            )
        }
        UiFrameworkTransitionFamilyPlan::Viewport(execution) => {
            super::policy_execution::viewport::execute(
                &runtime.allocation_receipt_ledger,
                &runtime.allocation_invalidation_index,
                execution,
                counters,
            )
        }
        UiFrameworkTransitionFamilyPlan::ViewportDenied(denial) => {
            super::policy_execution::viewport::deny(denial, counters)
        }
        UiFrameworkTransitionFamilyPlan::ResizePreview(outcome) => {
            super::policy_execution::resize_preview::execute(
                &runtime.allocation_receipt_ledger,
                outcome,
                counters,
            )
        }
        UiFrameworkTransitionFamilyPlan::AllocationDenied(execution) => {
            super::policy_execution::ordinary::deny(execution, counters)
        }
        UiFrameworkTransitionFamilyPlan::DurableResize(execution) => {
            super::policy_execution::durable_resize::execute(
                &runtime.allocation_receipt_ledger,
                &runtime.allocation_invalidation_index,
                execution,
                counters,
            )
        }
        UiFrameworkTransitionFamilyPlan::DragResize(execution) => {
            super::policy_execution::drag_resize::execute(
                &runtime.allocation_receipt_ledger,
                &runtime.allocation_invalidation_index,
                execution,
                counters,
            )
        }
    }
}
