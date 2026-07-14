use super::super::UiAllocationStreamPolicy;
use super::{
    UiAllocationCadenceBudget, UiAllocationCadenceKind, UiAllocationCommitTarget,
    UiAllocationEvidenceCadence, UiAllocationFamilyPairOutcome, UiAllocationPartialSettlementLaw,
    UiAllocationStreamCollapseLaw, UiAllocationStreamCompositionDenial, UiAllocationStreamFamily,
    UiResolvedAllocationStreamPolicy,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiAllocationFamilyPairContract {
    left: UiAllocationStreamFamily,
    right: UiAllocationStreamFamily,
    outcome: UiAllocationFamilyPairOutcome,
    resolved: UiResolvedAllocationStreamPolicy,
}

pub(super) fn pair_contract(
    left: UiAllocationStreamFamily,
    right: UiAllocationStreamFamily,
) -> Result<UiAllocationFamilyPairContract, UiAllocationStreamCompositionDenial> {
    use UiAllocationFamilyPairOutcome::{CoSelect, Compose, Deny};
    let pair = if left.canonical_order() <= right.canonical_order() {
        (left, right)
    } else {
        (right, left)
    };
    let outcome = match pair {
        (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::DurableResize) => Deny,
        (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::ViewportObservation)
        | (
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::DurableResize,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::ResizePreview,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::PortalAnchorObservation,
        ) => Deny,
        (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::ResizePreview)
        | (UiAllocationStreamFamily::QueryProjection, UiAllocationStreamFamily::ResizePreview)
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::ResizePreview,
        )
        | (UiAllocationStreamFamily::DurableResize, UiAllocationStreamFamily::ResizePreview)
        | (
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::PortalAnchorObservation,
        ) => CoSelect,
        (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::TextInput)
        | (UiAllocationStreamFamily::TextInput, UiAllocationStreamFamily::QueryProjection)
        | (
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (UiAllocationStreamFamily::QueryProjection, UiAllocationStreamFamily::QueryProjection)
        | (
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (UiAllocationStreamFamily::QueryProjection, UiAllocationStreamFamily::DurableResize)
        | (
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::DurableResize,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (UiAllocationStreamFamily::DurableResize, UiAllocationStreamFamily::DurableResize)
        | (
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (UiAllocationStreamFamily::ResizePreview, UiAllocationStreamFamily::ResizePreview)
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::ScrollExtentObservation,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::PortalAnchorObservation,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::PortalAnchorObservation,
        ) => Compose,
        (UiAllocationStreamFamily::QueryProjection, UiAllocationStreamFamily::TextInput)
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::TextInput,
        )
        | (
            UiAllocationStreamFamily::HostMeasurementReplacement,
            UiAllocationStreamFamily::QueryProjection,
        )
        | (UiAllocationStreamFamily::ViewportObservation, UiAllocationStreamFamily::TextInput)
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::QueryProjection,
        )
        | (
            UiAllocationStreamFamily::ViewportObservation,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (UiAllocationStreamFamily::DurableResize, UiAllocationStreamFamily::TextInput)
        | (UiAllocationStreamFamily::DurableResize, UiAllocationStreamFamily::QueryProjection)
        | (
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (UiAllocationStreamFamily::ResizePreview, UiAllocationStreamFamily::TextInput)
        | (UiAllocationStreamFamily::ResizePreview, UiAllocationStreamFamily::QueryProjection)
        | (
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (UiAllocationStreamFamily::ResizePreview, UiAllocationStreamFamily::DurableResize)
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::TextInput,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::QueryProjection,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::DurableResize,
        )
        | (
            UiAllocationStreamFamily::ScrollExtentObservation,
            UiAllocationStreamFamily::ResizePreview,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::TextInput,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::QueryProjection,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::HostMeasurementReplacement,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::ViewportObservation,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::DurableResize,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::ResizePreview,
        )
        | (
            UiAllocationStreamFamily::PortalAnchorObservation,
            UiAllocationStreamFamily::ScrollExtentObservation,
        ) => {
            unreachable!("pair normalization must produce canonical family order")
        }
    };
    if outcome == Deny {
        return Err(UiAllocationStreamCompositionDenial::IllegalFamilyPair {
            left: pair.0,
            right: pair.1,
        });
    }
    Ok(UiAllocationFamilyPairContract {
        left: pair.0,
        right: pair.1,
        outcome,
        resolved: join_contract_policies(
            resolved_family_policy(pair.0),
            resolved_family_policy(pair.1),
        ),
    })
}

impl UiAllocationFamilyPairContract {
    pub(super) fn left(self) -> UiAllocationStreamFamily {
        self.left
    }
    pub(super) fn right(self) -> UiAllocationStreamFamily {
        self.right
    }
    pub(super) fn outcome(self) -> UiAllocationFamilyPairOutcome {
        self.outcome
    }
    pub(super) fn resolved(self) -> UiResolvedAllocationStreamPolicy {
        self.resolved
    }
}

pub(super) fn resolved_family_policy(
    family: UiAllocationStreamFamily,
) -> UiResolvedAllocationStreamPolicy {
    let policy = UiAllocationStreamPolicy::for_family(family);
    UiResolvedAllocationStreamPolicy {
        commit_lane: match family {
            UiAllocationStreamFamily::ViewportObservation => {
                super::UiAllocationResolvedCommitLane::ViewportDerived
            }
            UiAllocationStreamFamily::ResizePreview => {
                super::UiAllocationResolvedCommitLane::ResizePreview
            }
            UiAllocationStreamFamily::DurableResize => {
                super::UiAllocationResolvedCommitLane::DurableResize
            }
            _ => super::UiAllocationResolvedCommitLane::Ordinary,
        },
        target: policy.target(),
        cadence: policy.cadence(),
        budget: policy.budget(),
        evidence_cadence: policy.evidence_cadence(),
        collapse_law: policy.collapse_law(),
        partial_settlement_law: policy.partial_settlement_law(),
    }
}

pub(super) fn join_contract_policies(
    left: UiResolvedAllocationStreamPolicy,
    right: UiResolvedAllocationStreamPolicy,
) -> UiResolvedAllocationStreamPolicy {
    UiResolvedAllocationStreamPolicy {
        commit_lane: join_commit_lane(left.commit_lane, right.commit_lane),
        target: join_target(left.target, right.target),
        cadence: join_cadence(left.cadence, right.cadence),
        budget: UiAllocationCadenceBudget::contract(
            left.budget
                .ingress_window()
                .min(right.budget.ingress_window()),
            left.budget
                .max_resolved_plans()
                .max(right.budget.max_resolved_plans()),
            left.budget
                .max_committed_receipts()
                .max(right.budget.max_committed_receipts()),
            left.budget
                .max_durable_mutations()
                .max(right.budget.max_durable_mutations()),
            left.budget
                .max_lag_frames()
                .min(right.budget.max_lag_frames()),
        )
        .with_max_invalidation_targets(
            left.budget
                .max_invalidation_targets()
                .max(right.budget.max_invalidation_targets()),
        ),
        evidence_cadence: join_evidence(left.evidence_cadence, right.evidence_cadence),
        collapse_law: join_collapse(left.collapse_law, right.collapse_law),
        partial_settlement_law: join_partial_settlement(
            left.partial_settlement_law,
            right.partial_settlement_law,
        ),
    }
}

fn join_commit_lane(
    left: super::UiAllocationResolvedCommitLane,
    right: super::UiAllocationResolvedCommitLane,
) -> super::UiAllocationResolvedCommitLane {
    use super::UiAllocationResolvedCommitLane::*;
    match (left, right) {
        (DurableResize, ResizePreview) | (ResizePreview, DurableResize) => DragResize,
        (DragResize, ResizePreview | DurableResize)
        | (ResizePreview | DurableResize, DragResize) => DragResize,
        (same, other) if same == other => same,
        _ => Ordinary,
    }
}

fn join_target(
    left: UiAllocationCommitTarget,
    right: UiAllocationCommitTarget,
) -> UiAllocationCommitTarget {
    use UiAllocationCommitTarget::*;
    match (left, right) {
        (PreviewOnly, PreviewOnly) => PreviewOnly,
        (SemanticAndAllocation, _) | (_, SemanticAndAllocation) => SemanticAndAllocation,
        _ => AllocationOnly,
    }
}

fn join_cadence(
    left: UiAllocationCadenceKind,
    right: UiAllocationCadenceKind,
) -> UiAllocationCadenceKind {
    use UiAllocationCadenceKind::*;
    match (left, right) {
        (Immediate, _) | (_, Immediate) => Immediate,
        (Threshold, _) | (_, Threshold) => Threshold,
        (CoalescedWindow, _) | (_, CoalescedWindow) => CoalescedWindow,
        _ => Terminal,
    }
}

fn join_evidence(
    left: UiAllocationEvidenceCadence,
    right: UiAllocationEvidenceCadence,
) -> UiAllocationEvidenceCadence {
    use UiAllocationEvidenceCadence::*;
    match (left, right) {
        (EveryInput, _) | (_, EveryInput) => EveryInput,
        (PerResolvedFrame, _) | (_, PerResolvedFrame) => PerResolvedFrame,
        _ => PerCommittedReceipt,
    }
}

fn join_collapse(
    left: UiAllocationStreamCollapseLaw,
    right: UiAllocationStreamCollapseLaw,
) -> UiAllocationStreamCollapseLaw {
    use UiAllocationStreamCollapseLaw::*;
    match (left, right) {
        (PreserveEveryInput, _) | (_, PreserveEveryInput) => PreserveEveryInput,
        (TerminalOnly, _) | (_, TerminalOnly) => TerminalOnly,
        (CoalesceWithinResolvedFrame, _) | (_, CoalesceWithinResolvedFrame) => {
            CoalesceWithinResolvedFrame
        }
        _ => LatestWinsWithinResolvedFrame,
    }
}

fn join_partial_settlement(
    left: UiAllocationPartialSettlementLaw,
    right: UiAllocationPartialSettlementLaw,
) -> UiAllocationPartialSettlementLaw {
    if left == UiAllocationPartialSettlementLaw::StaleButBounded
        || right == UiAllocationPartialSettlementLaw::StaleButBounded
    {
        UiAllocationPartialSettlementLaw::StaleButBounded
    } else {
        UiAllocationPartialSettlementLaw::NotApplicable
    }
}
