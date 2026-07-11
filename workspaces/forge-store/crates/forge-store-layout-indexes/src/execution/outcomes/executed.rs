use super::super::{S8AccessLoweringDenied, S8ExecutedAccessReceipt};

#[derive(Debug, PartialEq, Eq)]
enum S8IndexedExecutedEvidencePayload {
    Executed(S8ExecutedAccessReceipt),
    Denied(S8AccessLoweringDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8IndexedExecutedEvidenceOutcome {
    case: S8IndexedExecutedEvidencePayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexedExecutedEvidenceView<'a> {
    Executed(&'a S8ExecutedAccessReceipt),
    Denied(&'a S8AccessLoweringDenied),
}

impl S8IndexedExecutedEvidenceOutcome {
    pub(crate) fn executed(value: S8ExecutedAccessReceipt) -> Self {
        Self::from_owner_payload(S8IndexedExecutedEvidencePayload::Executed(value))
    }

    pub(crate) fn denied(value: S8AccessLoweringDenied) -> Self {
        Self::from_owner_payload(S8IndexedExecutedEvidencePayload::Denied(value))
    }

    fn from_owner_payload(case: S8IndexedExecutedEvidencePayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8IndexedExecutedEvidenceView<'_> {
        match &self.case {
            S8IndexedExecutedEvidencePayload::Executed(value) => {
                S8IndexedExecutedEvidenceView::Executed(value)
            }
            S8IndexedExecutedEvidencePayload::Denied(value) => {
                S8IndexedExecutedEvidenceView::Denied(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8IndexedExecutedEvidencePayload {
        self.case
    }
}

#[derive(Debug, PartialEq, Eq)]
enum S8DegradedExecutedEvidencePayload {
    DegradedExecuted(S8ExecutedAccessReceipt),
    DegradedDenied(S8AccessLoweringDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8DegradedExecutedEvidenceOutcome {
    case: S8DegradedExecutedEvidencePayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DegradedExecutedEvidenceView<'a> {
    DegradedExecuted(&'a S8ExecutedAccessReceipt),
    DegradedDenied(&'a S8AccessLoweringDenied),
}

impl S8DegradedExecutedEvidenceOutcome {
    pub(crate) fn executed(value: S8ExecutedAccessReceipt) -> Self {
        Self::from_owner_payload(S8DegradedExecutedEvidencePayload::DegradedExecuted(value))
    }

    pub(crate) fn denied(value: S8AccessLoweringDenied) -> Self {
        Self::from_owner_payload(S8DegradedExecutedEvidencePayload::DegradedDenied(value))
    }

    fn from_owner_payload(case: S8DegradedExecutedEvidencePayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8DegradedExecutedEvidenceView<'_> {
        match &self.case {
            S8DegradedExecutedEvidencePayload::DegradedExecuted(value) => {
                S8DegradedExecutedEvidenceView::DegradedExecuted(value)
            }
            S8DegradedExecutedEvidencePayload::DegradedDenied(value) => {
                S8DegradedExecutedEvidenceView::DegradedDenied(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8DegradedExecutedEvidencePayload {
        self.case
    }
}

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
}
