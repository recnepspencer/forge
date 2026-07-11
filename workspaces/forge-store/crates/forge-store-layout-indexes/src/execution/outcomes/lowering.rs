use super::super::S8LoweredAccessReceipt;
use crate::production_transition::define_owner_outcome;

define_owner_outcome!(
    pub S8IndexedLoweringOutcome,
    pub S8IndexedLoweringView,
    S8IndexedLoweringPayload,
    AccessLowering,
    LowerSelectedAccess,
    [lowered => Lowered(S8LoweredAccessReceipt): Budgeted => Lower => Lowered]
);

define_owner_outcome!(
    pub S8DegradedLoweringOutcome,
    pub S8DegradedLoweringView,
    S8DegradedLoweringPayload,
    DegradedExactScan,
    ExecuteBudgetedDegradedExactScan,
    [lowered => DegradedLowered(S8LoweredAccessReceipt): Budgeted => Lower => Lowered]
);

#[derive(Debug, PartialEq, Eq)]
enum LoweringOwnerOutcome {
    Indexed(S8IndexedLoweringOutcome),
    Degraded(S8DegradedLoweringOutcome),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8AccessLoweringOutcome {
    owner: LoweringOwnerOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessLoweringView<'a> {
    Lowered(&'a S8LoweredAccessReceipt),
}

impl S8AccessLoweringOutcome {
    pub(crate) fn lower(receipt: S8LoweredAccessReceipt) -> Self {
        let owner = if receipt.path_kind().is_degraded_exact_scan() {
            LoweringOwnerOutcome::Degraded(S8DegradedLoweringOutcome::lowered(receipt))
        } else {
            LoweringOwnerOutcome::Indexed(S8IndexedLoweringOutcome::lowered(receipt))
        };
        Self { owner }
    }
    pub fn view(&self) -> S8AccessLoweringView<'_> {
        match &self.owner {
            LoweringOwnerOutcome::Indexed(value) => match value.view() {
                S8IndexedLoweringView::Lowered(receipt) => S8AccessLoweringView::Lowered(receipt),
            },
            LoweringOwnerOutcome::Degraded(value) => match value.view() {
                S8DegradedLoweringView::DegradedLowered(receipt) => {
                    S8AccessLoweringView::Lowered(receipt)
                }
            },
        }
    }
    pub fn into_lowered(self) -> S8LoweredAccessReceipt {
        match self.owner {
            LoweringOwnerOutcome::Indexed(value) => match value.into_owner_payload() {
                S8IndexedLoweringPayload::Lowered(receipt) => receipt,
            },
            LoweringOwnerOutcome::Degraded(value) => match value.into_owner_payload() {
                S8DegradedLoweringPayload::DegradedLowered(receipt) => receipt,
            },
        }
    }
    pub const fn production_transition(
        &self,
    ) -> crate::production_transition::S8LayoutProductionTransition {
        match &self.owner {
            LoweringOwnerOutcome::Indexed(value) => value.production_transition(),
            LoweringOwnerOutcome::Degraded(value) => value.production_transition(),
        }
    }
    pub(crate) fn indexed_contract() -> crate::production_transition::S8OwnerTransitionContract {
        S8IndexedLoweringOutcome::owner_transition_contract()
    }
    pub(crate) fn degraded_contract() -> crate::production_transition::S8OwnerTransitionContract {
        S8DegradedLoweringOutcome::owner_transition_contract()
    }
}
