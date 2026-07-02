use crate::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget, UiSupportPosture,
};
use crate::obligations::selection::UiSelectedObligationSet;
use crate::obligations::verdict::UiObligationDispatchStopPosture;
use worth_ui_host_contract::WorthUiHostCapabilityPosture;
use worth_ui_query_binding::WorthUiQueryBasisPosture;

use super::{
    dispatch_execution::UiObligationDispatchExecution, UiObligationDispatchEntry,
    UiObligationDispatchPlan,
};

pub struct UiObligationDispatchBoundary;

impl UiObligationDispatchBoundary {
    pub(crate) const fn new() -> Self {
        Self
    }

    pub fn lower(&self, selected: &UiSelectedObligationSet) -> UiObligationDispatchPlan {
        let plan_stop_posture = match selected.support_snapshot().posture() {
            UiSupportPosture::Supported { .. } => supported_plan_stop_posture(selected),
            UiSupportPosture::Unsupported { .. } => UiObligationDispatchStopPosture::Unsupported,
            UiSupportPosture::Deferred { .. } => UiObligationDispatchStopPosture::Deferred,
            UiSupportPosture::DiagnosticOnly { .. } => {
                UiObligationDispatchStopPosture::DiagnosticOnly
            }
            UiSupportPosture::WrongWorld { .. } => UiObligationDispatchStopPosture::WrongWorld,
        };
        let entries = selected
            .obligations()
            .iter()
            .cloned()
            .map(|selected_obligation| {
                UiObligationDispatchEntry::new(
                    selected_obligation.clone(),
                    execution_for(&selected_obligation),
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        UiObligationDispatchPlan::new(selected.clone(), entries, plan_stop_posture)
    }
}

fn execution_for(
    selected: &crate::obligations::selection::UiSelectedObligation,
) -> UiObligationDispatchExecution {
    match selected.support_posture() {
        crate::obligations::selection::UiObligationSupportSelectionPosture::Unsupported => {
            return UiObligationDispatchExecution::TypedStop(
                UiObligationDispatchStopPosture::Unsupported,
            );
        }
        crate::obligations::selection::UiObligationSupportSelectionPosture::Deferred => {
            return UiObligationDispatchExecution::TypedStop(
                UiObligationDispatchStopPosture::Deferred,
            );
        }
        crate::obligations::selection::UiObligationSupportSelectionPosture::DiagnosticOnly
            if selected.family()
                != crate::obligations::catalog::UiObligationFamily::DiagnosticSurfaceRequirement =>
        {
            return UiObligationDispatchExecution::TypedStop(
                UiObligationDispatchStopPosture::DiagnosticOnly,
            );
        }
        crate::obligations::selection::UiObligationSupportSelectionPosture::Supported
        | crate::obligations::selection::UiObligationSupportSelectionPosture::DiagnosticOnly
        | crate::obligations::selection::UiObligationSupportSelectionPosture::WrongWorld => {}
    }

    match selected.family() {
        crate::obligations::catalog::UiObligationFamily::StructuralLegality
        | crate::obligations::catalog::UiObligationFamily::ParticipationLegality
        | crate::obligations::catalog::UiObligationFamily::SlotContract
        | crate::obligations::catalog::UiObligationFamily::DiagnosticSurfaceRequirement => {
            UiObligationDispatchExecution::ImmediateCheck
        }
        crate::obligations::catalog::UiObligationFamily::MeasurementRequirement
        | crate::obligations::catalog::UiObligationFamily::QueryBindingRequirement
        | crate::obligations::catalog::UiObligationFamily::IntentOperabilityRequirement
        | crate::obligations::catalog::UiObligationFamily::PortalHostRequirement
        | crate::obligations::catalog::UiObligationFamily::FocusRouteRequirement
        | crate::obligations::catalog::UiObligationFamily::MotionSupportRequirement
        | crate::obligations::catalog::UiObligationFamily::AccessibilityRequirement
        | crate::obligations::catalog::UiObligationFamily::HostCapabilityRequirement => {
            UiObligationDispatchExecution::TypedStop(UiObligationDispatchStopPosture::Deferred)
        }
    }
}

fn supported_plan_stop_posture(
    selected: &UiSelectedObligationSet,
) -> UiObligationDispatchStopPosture {
    let target = selected.support_snapshot().target();
    let ordinary_lane_cost = required_lane_cost(target.selection_budget());
    if !target
        .selection_budget()
        .admits_lane_cost(ordinary_lane_cost)
    {
        return UiObligationDispatchStopPosture::BudgetExceeded {
            budget: target.selection_budget(),
            attempted_lane_cost: ordinary_lane_cost,
        };
    }

    if selected.obligations().iter().any(|obligation| {
        obligation.family()
            == crate::obligations::catalog::UiObligationFamily::QueryBindingRequirement
    }) {
        let Some(query_prerequisites) = target.query_prerequisites() else {
            return UiObligationDispatchStopPosture::Unsupported;
        };

        return match query_prerequisites.basis_posture() {
            WorthUiQueryBasisPosture::GraphAligned => UiObligationDispatchStopPosture::None,
            WorthUiQueryBasisPosture::WrongWorldProjection
            | WorthUiQueryBasisPosture::RebindRequired => {
                UiObligationDispatchStopPosture::WrongQueryBasis {
                    required: UiAdmissionQueryBasis::GraphAligned,
                    observed: target.query_basis(),
                }
            }
            WorthUiQueryBasisPosture::StaleReceipt => UiObligationDispatchStopPosture::Stale {
                required: UiAdmissionQueryBasis::GraphAligned,
                observed: target.query_basis(),
                evidence: crate::admission::UiAdmissionStaleEvidence::QueryReceiptExpired,
            },
            WorthUiQueryBasisPosture::AmbiguousSources => {
                UiObligationDispatchStopPosture::Ambiguous {
                    required_query_basis: Some(UiAdmissionQueryBasis::GraphAligned),
                    observed_query_basis: Some(target.query_basis()),
                    required_host_capability: None,
                    observed_host_capability: None,
                }
            }
        };
    }

    if selected.obligations().iter().any(|obligation| {
        matches!(
            obligation.family(),
            crate::obligations::catalog::UiObligationFamily::PortalHostRequirement
                | crate::obligations::catalog::UiObligationFamily::FocusRouteRequirement
                | crate::obligations::catalog::UiObligationFamily::MotionSupportRequirement
                | crate::obligations::catalog::UiObligationFamily::HostCapabilityRequirement
        )
    }) {
        let Some(host_capability_report) = target.host_capability_report() else {
            return UiObligationDispatchStopPosture::Unsupported;
        };

        return match host_capability_report.posture() {
            WorthUiHostCapabilityPosture::Available => UiObligationDispatchStopPosture::None,
            WorthUiHostCapabilityPosture::Missing => {
                UiObligationDispatchStopPosture::WrongHostCapability {
                    required: UiAdmissionHostCapability::Available,
                    observed: target.host_capability(),
                }
            }
            WorthUiHostCapabilityPosture::Ambiguous => UiObligationDispatchStopPosture::Ambiguous {
                required_query_basis: None,
                observed_query_basis: None,
                required_host_capability: Some(UiAdmissionHostCapability::Available),
                observed_host_capability: Some(target.host_capability()),
            },
            WorthUiHostCapabilityPosture::DiagnosticOnly => {
                UiObligationDispatchStopPosture::DiagnosticOnly
            }
        };
    }

    UiObligationDispatchStopPosture::None
}

const fn required_lane_cost(_budget: UiAdmissionSelectionBudget) -> u8 {
    1
}
