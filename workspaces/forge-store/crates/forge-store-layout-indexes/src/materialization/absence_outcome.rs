use crate::production_transition::define_owner_outcome;

define_owner_outcome!(
    pub S8PhysicalAbsenceOutcome,
    pub S8PhysicalAbsenceOutcomeView,
    S8PhysicalAbsenceCase,
    MaterializationCoverageAbsence,
    ProveExactIndexAbsence,
    [
        absence_proven => Success(super::S8PhysicalAbsenceProof): Admitted => ProveAbsence => AbsenceProven,
        partial_denied => PartialDenied(super::S8MaterializationDenial): CoveragePartial => ProveAbsence => Denied,
        denied => Denied(super::S8MaterializationDenial): Admitted => Deny => Denied,
    ]
);

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
