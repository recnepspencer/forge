use crate::artifact_family::ArtifactFamilyDenial;

#[derive(Debug, PartialEq, Eq)]
enum S8KeyDomainAdmissionCase {
    Success(super::PhysicalKeyDomainWitness),
    Denied(ArtifactFamilyDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8KeyDomainAdmissionOutcome {
    case: S8KeyDomainAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8KeyDomainAdmissionView<'a> {
    Success(&'a super::PhysicalKeyDomainWitness),
    Denied(&'a ArtifactFamilyDenial),
}

impl S8KeyDomainAdmissionOutcome {
    pub(crate) fn admitted(value: super::PhysicalKeyDomainWitness) -> Self {
        Self::from_owner_payload(S8KeyDomainAdmissionCase::Success(value))
    }

    pub(crate) fn denied(value: ArtifactFamilyDenial) -> Self {
        Self::from_owner_payload(S8KeyDomainAdmissionCase::Denied(value))
    }

    fn from_owner_payload(case: S8KeyDomainAdmissionCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8KeyDomainAdmissionView<'_> {
        match &self.case {
            S8KeyDomainAdmissionCase::Success(value) => S8KeyDomainAdmissionView::Success(value),
            S8KeyDomainAdmissionCase::Denied(value) => S8KeyDomainAdmissionView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> S8KeyDomainAdmissionCase {
        self.case
    }
}

impl S8KeyDomainAdmissionOutcome {
    pub fn into_result(self) -> Result<super::PhysicalKeyDomainWitness, ArtifactFamilyDenial> {
        match self.into_owner_payload() {
            S8KeyDomainAdmissionCase::Success(value) => Ok(value),
            S8KeyDomainAdmissionCase::Denied(denial) => Err(denial),
        }
    }

    pub fn unwrap(self) -> super::PhysicalKeyDomainWitness {
        self.into_result().unwrap()
    }
    pub fn unwrap_err(self) -> ArtifactFamilyDenial {
        self.into_result().unwrap_err()
    }
}

impl PartialEq<Result<super::PhysicalKeyDomainWitness, ArtifactFamilyDenial>>
    for S8KeyDomainAdmissionOutcome
{
    fn eq(&self, other: &Result<super::PhysicalKeyDomainWitness, ArtifactFamilyDenial>) -> bool {
        match (self.view(), other) {
            (S8KeyDomainAdmissionView::Success(left), Ok(right)) => left == right,
            (S8KeyDomainAdmissionView::Denied(left), Err(right)) => left == right,
            _ => false,
        }
    }
}

pub(crate) fn issue_key_domain_admission(
    result: Result<super::PhysicalKeyDomainWitness, ArtifactFamilyDenial>,
) -> S8KeyDomainAdmissionOutcome {
    match result {
        Ok(domain) => S8KeyDomainAdmissionOutcome::admitted(domain),
        Err(denial) => S8KeyDomainAdmissionOutcome::denied(denial),
    }
}
