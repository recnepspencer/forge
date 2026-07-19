use super::evidence::{
    IdentityEvolutionCertificationDenialEvidence, IdentityEvolutionCertificationResultEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityEvolutionCertificationEvidence {
    Result(IdentityEvolutionCertificationResultEvidence),
    Denial(IdentityEvolutionCertificationDenialEvidence),
}

impl IdentityEvolutionCertificationEvidence {
    pub fn as_result(&self) -> Option<&IdentityEvolutionCertificationResultEvidence> {
        match self {
            Self::Result(evidence) => Some(evidence),
            Self::Denial(_) => None,
        }
    }

    pub fn as_denial(&self) -> Option<&IdentityEvolutionCertificationDenialEvidence> {
        match self {
            Self::Result(_) => None,
            Self::Denial(evidence) => Some(evidence),
        }
    }
}
