use crate::runtime::{
    WorthQueryGraphObligationBudgetExceededPolicy, WorthQueryGraphObligationExecutionBudget,
    WorthQueryGraphObligationExecutionCostClass, WorthQueryGraphObligationExecutionScope,
    WorthQueryGraphObligationKind, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportStatus,
};

use super::row::WorthQueryGraphObligationSupportMatrixRow;

pub(super) fn authority_certification_surface_rows(
) -> Vec<WorthQueryGraphObligationSupportMatrixRow> {
    let mut rows = Vec::new();
    for kind in WorthQueryGraphObligationKind::ALL {
        for lane in WorthQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED {
            rows.push(authority_certification_row(kind, lane));
        }
    }
    rows
}

fn authority_certification_row(
    obligation_kind: WorthQueryGraphObligationKind,
    support_lane: WorthQueryGraphObligationSupportLane,
) -> WorthQueryGraphObligationSupportMatrixRow {
    WorthQueryGraphObligationSupportMatrixRow::with_budget(
        obligation_kind,
        support_lane,
        authority_certification_status_for(obligation_kind, support_lane),
        authority_certification_budget(),
        WorthQueryGraphObligationExecutionCostClass::SparseTopology,
        "state-load counters",
        "artifact-policy-gated diagnostics",
    )
}

fn authority_certification_budget() -> WorthQueryGraphObligationExecutionBudget {
    WorthQueryGraphObligationExecutionBudget::bounded_sparse(
        WorthQueryGraphObligationExecutionScope::TouchedCollection,
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed,
    )
}

fn authority_certification_status_for(
    kind: WorthQueryGraphObligationKind,
    lane: WorthQueryGraphObligationSupportLane,
) -> WorthQueryGraphObligationSupportStatus {
    if matches!(
        kind,
        WorthQueryGraphObligationKind::BlockingInvariant
            | WorthQueryGraphObligationKind::SchemaContractValidator
            | WorthQueryGraphObligationKind::OperatingContextGate
    ) {
        return WorthQueryGraphObligationSupportStatus::Unsupported;
    }
    match lane {
        WorthQueryGraphObligationSupportLane::GraphComposition
        | WorthQueryGraphObligationSupportLane::AuthoritativeCommandBatch
        | WorthQueryGraphObligationSupportLane::ScalarMutation
        | WorthQueryGraphObligationSupportLane::PolicyAwareGraphMutation
        | WorthQueryGraphObligationSupportLane::PrimitiveConstructionBirth => {
            WorthQueryGraphObligationSupportStatus::Supported
        }
        WorthQueryGraphObligationSupportLane::ReadFamily
        | WorthQueryGraphObligationSupportLane::LiveRead => {
            WorthQueryGraphObligationSupportStatus::DiagnosticOnly
        }
        WorthQueryGraphObligationSupportLane::EffectTriggeredWriteIntent
        | WorthQueryGraphObligationSupportLane::PreviewIntent
        | WorthQueryGraphObligationSupportLane::BranchIntent
        | WorthQueryGraphObligationSupportLane::WorthTopoOperatorCatalog
        | WorthQueryGraphObligationSupportLane::WorthKernelPhaseChain => {
            WorthQueryGraphObligationSupportStatus::DeferredToBackstop
        }
        WorthQueryGraphObligationSupportLane::DeclarationEntry => match kind {
            WorthQueryGraphObligationKind::AdvisoryObligation => {
                WorthQueryGraphObligationSupportStatus::NotApplicable
            }
            _ => WorthQueryGraphObligationSupportStatus::DeferredToBackstop,
        },
        WorthQueryGraphObligationSupportLane::ContributionOrchestration => {
            WorthQueryGraphObligationSupportStatus::Unsupported
        }
        WorthQueryGraphObligationSupportLane::PreviewMutation => match kind {
            WorthQueryGraphObligationKind::AdvisoryObligation => {
                WorthQueryGraphObligationSupportStatus::DiagnosticOnly
            }
            WorthQueryGraphObligationKind::PreflightSequencingObligation
            | WorthQueryGraphObligationKind::CapabilityGapScreen => {
                WorthQueryGraphObligationSupportStatus::DeferredToBackstop
            }
            WorthQueryGraphObligationKind::BlockingInvariant
            | WorthQueryGraphObligationKind::SchemaContractValidator
            | WorthQueryGraphObligationKind::OperatingContextGate => unreachable!(
                "ownerless obligation kinds return unsupported before lane classification"
            ),
        },
        WorthQueryGraphObligationSupportLane::AssemblyIndexSelection => {
            WorthQueryGraphObligationSupportStatus::Supported
        }
    }
}
