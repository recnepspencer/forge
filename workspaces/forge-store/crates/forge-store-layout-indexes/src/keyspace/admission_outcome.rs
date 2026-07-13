use crate::catalog::ArtifactFamilyDenial;

#[derive(Debug, PartialEq, Eq)]
enum KeyDomainAdmissionCase {
    Success(super::PhysicalKeyDomainWitness),
    Denied(ArtifactFamilyDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct KeyDomainAdmissionOutcome {
    case: KeyDomainAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyDomainAdmissionView<'a> {
    Success(&'a super::PhysicalKeyDomainWitness),
    Denied(&'a ArtifactFamilyDenial),
}

impl KeyDomainAdmissionOutcome {
    pub(crate) fn admitted(value: super::PhysicalKeyDomainWitness) -> Self {
        Self::from_owner_payload(KeyDomainAdmissionCase::Success(value))
    }

    pub(crate) fn denied(value: ArtifactFamilyDenial) -> Self {
        Self::from_owner_payload(KeyDomainAdmissionCase::Denied(value))
    }

    fn from_owner_payload(case: KeyDomainAdmissionCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> KeyDomainAdmissionView<'_> {
        match &self.case {
            KeyDomainAdmissionCase::Success(value) => KeyDomainAdmissionView::Success(value),
            KeyDomainAdmissionCase::Denied(value) => KeyDomainAdmissionView::Denied(value),
        }
    }

    fn into_owner_payload(self) -> KeyDomainAdmissionCase {
        self.case
    }
}

impl KeyDomainAdmissionOutcome {
    pub fn into_result(self) -> Result<super::PhysicalKeyDomainWitness, ArtifactFamilyDenial> {
        match self.into_owner_payload() {
            KeyDomainAdmissionCase::Success(value) => Ok(value),
            KeyDomainAdmissionCase::Denied(denial) => Err(denial),
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
    for KeyDomainAdmissionOutcome
{
    fn eq(&self, other: &Result<super::PhysicalKeyDomainWitness, ArtifactFamilyDenial>) -> bool {
        match (self.view(), other) {
            (KeyDomainAdmissionView::Success(left), Ok(right)) => left == right,
            (KeyDomainAdmissionView::Denied(left), Err(right)) => left == right,
            _ => false,
        }
    }
}

pub(crate) fn issue_key_domain_admission(
    result: Result<super::PhysicalKeyDomainWitness, ArtifactFamilyDenial>,
) -> KeyDomainAdmissionOutcome {
    match result {
        Ok(domain) => KeyDomainAdmissionOutcome::admitted(domain),
        Err(denial) => KeyDomainAdmissionOutcome::denied(denial),
    }
}
