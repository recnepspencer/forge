use super::super::{S8AccessLoweringDenied, S8ExecutedAccessReceipt};
use crate::production_transition::define_owner_outcome;

define_owner_outcome!(
    pub S8IndexedExecutedEvidenceOutcome,
    pub S8IndexedExecutedEvidenceView,
    S8IndexedExecutedEvidencePayload,
    ExecutedEvidence,
    AdmitCountersAndExecute,
    [
        executed => Executed(S8ExecutedAccessReceipt): ExactCountersObserved => Execute => Executed,
        denied => Denied(S8AccessLoweringDenied): ExactCountersObserved => Deny => Denied,
    ]
);

define_owner_outcome!(
    pub S8DegradedExecutedEvidenceOutcome,
    pub S8DegradedExecutedEvidenceView,
    S8DegradedExecutedEvidencePayload,
    DegradedExactScan,
    ExecuteBudgetedDegradedExactScan,
    [
        executed => DegradedExecuted(S8ExecutedAccessReceipt): ExactCountersObserved => Execute => Executed,
        denied => DegradedDenied(S8AccessLoweringDenied): ExactCountersObserved => Deny => Denied,
    ]
);

#[derive(Debug, PartialEq, Eq)]
enum ExecutedOwnerOutcome {
    Indexed(S8IndexedExecutedEvidenceOutcome),
    Degraded(S8DegradedExecutedEvidenceOutcome),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8ExecutedEvidenceOutcome {
    owner: ExecutedOwnerOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ExecutedEvidenceView<'a> {
    Executed(&'a S8ExecutedAccessReceipt),
    Denied(&'a S8AccessLoweringDenied),
}

impl S8ExecutedEvidenceOutcome {
    pub(crate) fn executed(value: S8ExecutedAccessReceipt) -> Self {
        let owner = if value.basis().path_kind().is_degraded_exact_scan() {
            ExecutedOwnerOutcome::Degraded(S8DegradedExecutedEvidenceOutcome::executed(value))
        } else {
            ExecutedOwnerOutcome::Indexed(S8IndexedExecutedEvidenceOutcome::executed(value))
        };
        Self { owner }
    }
    pub(crate) fn denied(degraded: bool, denial: S8AccessLoweringDenied) -> Self {
        let owner = if degraded {
            ExecutedOwnerOutcome::Degraded(S8DegradedExecutedEvidenceOutcome::denied(denial))
        } else {
            ExecutedOwnerOutcome::Indexed(S8IndexedExecutedEvidenceOutcome::denied(denial))
        };
        Self { owner }
    }
    pub fn view(&self) -> S8ExecutedEvidenceView<'_> {
        match &self.owner {
            ExecutedOwnerOutcome::Indexed(value) => match value.view() {
                S8IndexedExecutedEvidenceView::Executed(value) => {
                    S8ExecutedEvidenceView::Executed(value)
                }
                S8IndexedExecutedEvidenceView::Denied(denial) => {
                    S8ExecutedEvidenceView::Denied(denial)
                }
            },
            ExecutedOwnerOutcome::Degraded(value) => match value.view() {
                S8DegradedExecutedEvidenceView::DegradedExecuted(value) => {
                    S8ExecutedEvidenceView::Executed(value)
                }
                S8DegradedExecutedEvidenceView::DegradedDenied(denial) => {
                    S8ExecutedEvidenceView::Denied(denial)
                }
            },
        }
    }
    pub fn into_result(self) -> Result<S8ExecutedAccessReceipt, S8AccessLoweringDenied> {
        match self.owner {
            ExecutedOwnerOutcome::Indexed(value) => match value.into_owner_payload() {
                S8IndexedExecutedEvidencePayload::Executed(value) => Ok(value),
                S8IndexedExecutedEvidencePayload::Denied(denial) => Err(denial),
            },
            ExecutedOwnerOutcome::Degraded(value) => match value.into_owner_payload() {
                S8DegradedExecutedEvidencePayload::DegradedExecuted(value) => Ok(value),
                S8DegradedExecutedEvidencePayload::DegradedDenied(denial) => Err(denial),
            },
        }
    }
    pub fn expect(self, message: &str) -> S8ExecutedAccessReceipt {
        self.into_result().expect(message)
    }
    pub fn expect_err(self, message: &str) -> S8AccessLoweringDenied {
        self.into_result().expect_err(message)
    }
    pub const fn production_transition(
        &self,
    ) -> crate::production_transition::S8LayoutProductionTransition {
        match &self.owner {
            ExecutedOwnerOutcome::Indexed(value) => value.production_transition(),
            ExecutedOwnerOutcome::Degraded(value) => value.production_transition(),
        }
    }
    pub(crate) fn indexed_contract() -> crate::production_transition::S8OwnerTransitionContract {
        S8IndexedExecutedEvidenceOutcome::owner_transition_contract()
    }
    pub(crate) fn degraded_contract() -> crate::production_transition::S8OwnerTransitionContract {
        S8DegradedExecutedEvidenceOutcome::owner_transition_contract()
    }
}
