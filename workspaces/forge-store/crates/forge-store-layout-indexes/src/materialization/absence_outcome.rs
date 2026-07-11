#[derive(Debug, PartialEq, Eq)]
enum S8PhysicalAbsenceCase {
    Success(super::S8PhysicalAbsenceProof),
    PartialDenied(super::S8MaterializationDenial),
    Denied(super::S8MaterializationDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8PhysicalAbsenceOutcome {
    case: S8PhysicalAbsenceCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8PhysicalAbsenceOutcomeView<'a> {
    Success(&'a super::S8PhysicalAbsenceProof),
    PartialDenied(&'a super::S8MaterializationDenial),
    Denied(&'a super::S8MaterializationDenial),
}

impl S8PhysicalAbsenceOutcome {
    pub(crate) fn absence_proven(value: super::S8PhysicalAbsenceProof) -> Self {
        Self::from_owner_payload(S8PhysicalAbsenceCase::Success(value))
    }

    pub(crate) fn partial_denied(value: super::S8MaterializationDenial) -> Self {
        Self::from_owner_payload(S8PhysicalAbsenceCase::PartialDenied(value))
    }

    pub(crate) fn denied(value: super::S8MaterializationDenial) -> Self {
        Self::from_owner_payload(S8PhysicalAbsenceCase::Denied(value))
    }

    fn from_owner_payload(case: S8PhysicalAbsenceCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8PhysicalAbsenceOutcomeView<'_> {
        match &self.case {
            S8PhysicalAbsenceCase::Success(value) => S8PhysicalAbsenceOutcomeView::Success(value),
            S8PhysicalAbsenceCase::PartialDenied(value) => {
                S8PhysicalAbsenceOutcomeView::PartialDenied(value)
            }
            S8PhysicalAbsenceCase::Denied(value) => S8PhysicalAbsenceOutcomeView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> S8PhysicalAbsenceCase {
        self.case
    }
}

impl S8PhysicalAbsenceOutcome {
    pub fn into_result(
        self,
    ) -> Result<super::S8PhysicalAbsenceProof, super::S8MaterializationDenial> {
        match self.into_owner_payload() {
            S8PhysicalAbsenceCase::Success(proof) => Ok(proof),
            S8PhysicalAbsenceCase::PartialDenied(denial)
            | S8PhysicalAbsenceCase::Denied(denial) => Err(denial),
        }
    }

    pub fn unwrap(self) -> super::S8PhysicalAbsenceProof {
        self.into_result().unwrap()
    }
    pub fn expect(self, message: &str) -> super::S8PhysicalAbsenceProof {
        self.into_result().expect(message)
    }
    pub fn unwrap_err(self) -> super::S8MaterializationDenial {
        self.into_result().unwrap_err()
    }
    pub fn expect_err(self, message: &str) -> super::S8MaterializationDenial {
        self.into_result().expect_err(message)
    }
}

impl PartialEq<Result<super::S8PhysicalAbsenceProof, super::S8MaterializationDenial>>
    for S8PhysicalAbsenceOutcome
{
    fn eq(
        &self,
        other: &Result<super::S8PhysicalAbsenceProof, super::S8MaterializationDenial>,
    ) -> bool {
        match (self.view(), other) {
            (S8PhysicalAbsenceOutcomeView::Success(left), Ok(right)) => left == right,
            (S8PhysicalAbsenceOutcomeView::PartialDenied(left), Err(right))
            | (S8PhysicalAbsenceOutcomeView::Denied(left), Err(right)) => left == right,
            _ => false,
        }
    }
}

pub(crate) fn issue_physical_absence(
    result: Result<super::S8PhysicalAbsenceProof, super::S8MaterializationDenial>,
) -> S8PhysicalAbsenceOutcome {
    match result {
        Ok(proof) => S8PhysicalAbsenceOutcome::absence_proven(proof),
        Err(denial @ super::S8MaterializationDenial::LayoutCoverageIsPartial { .. }) => {
            S8PhysicalAbsenceOutcome::partial_denied(denial)
        }
        Err(denial) => S8PhysicalAbsenceOutcome::denied(denial),
    }
}
