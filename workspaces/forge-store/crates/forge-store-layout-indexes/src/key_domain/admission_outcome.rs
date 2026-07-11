use crate::artifact_family::ArtifactFamilyDenial;
use crate::production_transition::define_owner_outcome;

define_owner_outcome!(
    pub S8KeyDomainAdmissionOutcome,
    pub S8KeyDomainAdmissionView,
    S8KeyDomainAdmissionCase,
    KeyDomainAdmission,
    AdmitKeyDomain,
    [
        admitted => Success(super::PhysicalKeyDomainWitness): Declared => Admit => CanonicalKeysAdmitted,
        denied => Denied(ArtifactFamilyDenial): Declared => Deny => Denied,
    ]
);

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
