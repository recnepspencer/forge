use super::super::{S8AccessLoweringDenied, S8AdmittedExecutedCounters};

#[derive(Debug, PartialEq, Eq)]
enum S8IndexedCounterAdmissionPayload {
    Admitted(S8AdmittedExecutedCounters),
    Denied(S8AccessLoweringDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8IndexedCounterAdmissionOutcome {
    case: S8IndexedCounterAdmissionPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexedCounterAdmissionView<'a> {
    Admitted(&'a S8AdmittedExecutedCounters),
    Denied(&'a S8AccessLoweringDenied),
}

impl S8IndexedCounterAdmissionOutcome {
    pub(crate) fn admitted(value: S8AdmittedExecutedCounters) -> Self {
        Self::from_owner_payload(S8IndexedCounterAdmissionPayload::Admitted(value))
    }

    pub(crate) fn denied(value: S8AccessLoweringDenied) -> Self {
        Self::from_owner_payload(S8IndexedCounterAdmissionPayload::Denied(value))
    }

    fn from_owner_payload(case: S8IndexedCounterAdmissionPayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8IndexedCounterAdmissionView<'_> {
        match &self.case {
            S8IndexedCounterAdmissionPayload::Admitted(value) => {
                S8IndexedCounterAdmissionView::Admitted(value)
            }
            S8IndexedCounterAdmissionPayload::Denied(value) => {
                S8IndexedCounterAdmissionView::Denied(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8IndexedCounterAdmissionPayload {
        self.case
    }
}

#[derive(Debug, PartialEq, Eq)]
enum S8DegradedCounterAdmissionPayload {
    DegradedAdmitted(S8AdmittedExecutedCounters),
    DegradedDenied(S8AccessLoweringDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8DegradedCounterAdmissionOutcome {
    case: S8DegradedCounterAdmissionPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DegradedCounterAdmissionView<'a> {
    DegradedAdmitted(&'a S8AdmittedExecutedCounters),
    DegradedDenied(&'a S8AccessLoweringDenied),
}

impl S8DegradedCounterAdmissionOutcome {
    pub(crate) fn admitted(value: S8AdmittedExecutedCounters) -> Self {
        Self::from_owner_payload(S8DegradedCounterAdmissionPayload::DegradedAdmitted(value))
    }

    pub(crate) fn denied(value: S8AccessLoweringDenied) -> Self {
        Self::from_owner_payload(S8DegradedCounterAdmissionPayload::DegradedDenied(value))
    }

    fn from_owner_payload(case: S8DegradedCounterAdmissionPayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8DegradedCounterAdmissionView<'_> {
        match &self.case {
            S8DegradedCounterAdmissionPayload::DegradedAdmitted(value) => {
                S8DegradedCounterAdmissionView::DegradedAdmitted(value)
            }
            S8DegradedCounterAdmissionPayload::DegradedDenied(value) => {
                S8DegradedCounterAdmissionView::DegradedDenied(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8DegradedCounterAdmissionPayload {
        self.case
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CounterAdmissionOwnerOutcome {
    Indexed(S8IndexedCounterAdmissionOutcome),
    Degraded(S8DegradedCounterAdmissionOutcome),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8ExecutedCounterAdmissionOutcome {
    owner: CounterAdmissionOwnerOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ExecutedCounterAdmissionView<'a> {
    Admitted(&'a S8AdmittedExecutedCounters),
    Denied(&'a S8AccessLoweringDenied),
}

impl S8ExecutedCounterAdmissionOutcome {
    pub(crate) fn issue(
        degraded: bool,
        result: Result<S8AdmittedExecutedCounters, S8AccessLoweringDenied>,
    ) -> Self {
        let owner = match (degraded, result) {
            (false, Ok(value)) => CounterAdmissionOwnerOutcome::Indexed(
                S8IndexedCounterAdmissionOutcome::admitted(value),
            ),
            (false, Err(denial)) => CounterAdmissionOwnerOutcome::Indexed(
                S8IndexedCounterAdmissionOutcome::denied(denial),
            ),
            (true, Ok(value)) => CounterAdmissionOwnerOutcome::Degraded(
                S8DegradedCounterAdmissionOutcome::admitted(value),
            ),
            (true, Err(denial)) => CounterAdmissionOwnerOutcome::Degraded(
                S8DegradedCounterAdmissionOutcome::denied(denial),
            ),
        };
        Self { owner }
    }
    pub fn view(&self) -> S8ExecutedCounterAdmissionView<'_> {
        match &self.owner {
            CounterAdmissionOwnerOutcome::Indexed(value) => match value.view() {
                S8IndexedCounterAdmissionView::Admitted(value) => {
                    S8ExecutedCounterAdmissionView::Admitted(value)
                }
                S8IndexedCounterAdmissionView::Denied(denial) => {
                    S8ExecutedCounterAdmissionView::Denied(denial)
                }
            },
            CounterAdmissionOwnerOutcome::Degraded(value) => match value.view() {
                S8DegradedCounterAdmissionView::DegradedAdmitted(value) => {
                    S8ExecutedCounterAdmissionView::Admitted(value)
                }
                S8DegradedCounterAdmissionView::DegradedDenied(denial) => {
                    S8ExecutedCounterAdmissionView::Denied(denial)
                }
            },
        }
    }
    pub fn into_result(self) -> Result<S8AdmittedExecutedCounters, S8AccessLoweringDenied> {
        match self.owner {
            CounterAdmissionOwnerOutcome::Indexed(value) => match value.into_owner_payload() {
                S8IndexedCounterAdmissionPayload::Admitted(value) => Ok(value),
                S8IndexedCounterAdmissionPayload::Denied(denial) => Err(denial),
            },
            CounterAdmissionOwnerOutcome::Degraded(value) => match value.into_owner_payload() {
                S8DegradedCounterAdmissionPayload::DegradedAdmitted(value) => Ok(value),
                S8DegradedCounterAdmissionPayload::DegradedDenied(denial) => Err(denial),
            },
        }
    }
    pub fn expect(self, message: &str) -> S8AdmittedExecutedCounters {
        self.into_result().expect(message)
    }
    pub fn expect_err(self, message: &str) -> S8AccessLoweringDenied {
        self.into_result().expect_err(message)
    }
}
