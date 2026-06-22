use crate::runtime::{
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationExecutionBudget,
    ForgeQueryGraphObligationExecutionCostClass, ForgeQueryGraphObligationExecutionScope,
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportStatus,
};

use super::row::ForgeQueryGraphObligationSupportMatrixRow;

pub(super) fn authority_certification_surface_rows(
) -> Vec<ForgeQueryGraphObligationSupportMatrixRow> {
    let mut rows = Vec::new();
    for kind in ForgeQueryGraphObligationKind::ALL {
        for lane in ForgeQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED {
            rows.push(authority_certification_row(kind, lane));
        }
    }
    rows
}

fn authority_certification_row(
    obligation_kind: ForgeQueryGraphObligationKind,
    support_lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphObligationSupportMatrixRow {
    ForgeQueryGraphObligationSupportMatrixRow::with_budget(
        obligation_kind,
        support_lane,
        authority_certification_status_for(obligation_kind, support_lane),
        authority_certification_budget(),
        ForgeQueryGraphObligationExecutionCostClass::SparseTopology,
        "state-load counters",
        "artifact-policy-gated diagnostics",
    )
}

fn authority_certification_budget() -> ForgeQueryGraphObligationExecutionBudget {
    ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
        ForgeQueryGraphObligationExecutionScope::TouchedCollection,
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
    )
}

fn authority_certification_status_for(
    kind: ForgeQueryGraphObligationKind,
    lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphObligationSupportStatus {
    match lane {
        ForgeQueryGraphObligationSupportLane::GraphComposition
        | ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch
        | ForgeQueryGraphObligationSupportLane::ScalarMutation
        | ForgeQueryGraphObligationSupportLane::PolicyAwareGraphMutation
        | ForgeQueryGraphObligationSupportLane::PrimitiveConstructionBirth => {
            ForgeQueryGraphObligationSupportStatus::Supported
        }
        ForgeQueryGraphObligationSupportLane::ReadFamily
        | ForgeQueryGraphObligationSupportLane::LiveRead => {
            ForgeQueryGraphObligationSupportStatus::DiagnosticOnly
        }
        ForgeQueryGraphObligationSupportLane::EffectTriggeredWriteIntent
        | ForgeQueryGraphObligationSupportLane::PreviewIntent
        | ForgeQueryGraphObligationSupportLane::BranchIntent
        | ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog
        | ForgeQueryGraphObligationSupportLane::WorthKernelPhaseChain => {
            ForgeQueryGraphObligationSupportStatus::DeferredToBackstop
        }
        ForgeQueryGraphObligationSupportLane::DeclarationEntry => match kind {
            ForgeQueryGraphObligationKind::AdvisoryObligation => {
                ForgeQueryGraphObligationSupportStatus::NotApplicable
            }
            _ => ForgeQueryGraphObligationSupportStatus::DeferredToBackstop,
        },
        ForgeQueryGraphObligationSupportLane::ContributionOrchestration => {
            ForgeQueryGraphObligationSupportStatus::Unsupported
        }
        ForgeQueryGraphObligationSupportLane::PreviewMutation => match kind {
            ForgeQueryGraphObligationKind::BlockingInvariant
            | ForgeQueryGraphObligationKind::SchemaContractValidator
            | ForgeQueryGraphObligationKind::OperatingContextGate => {
                ForgeQueryGraphObligationSupportStatus::Supported
            }
            ForgeQueryGraphObligationKind::AdvisoryObligation => {
                ForgeQueryGraphObligationSupportStatus::DiagnosticOnly
            }
            ForgeQueryGraphObligationKind::PreflightSequencingObligation
            | ForgeQueryGraphObligationKind::CapabilityGapScreen => {
                ForgeQueryGraphObligationSupportStatus::DeferredToBackstop
            }
        },
        ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection => {
            ForgeQueryGraphObligationSupportStatus::Supported
        }
    }
}
