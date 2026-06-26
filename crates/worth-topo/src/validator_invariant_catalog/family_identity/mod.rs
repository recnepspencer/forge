mod identity_digest;
mod invariant_family_identity;
mod validator_family_identity;

pub(super) use identity_digest::legality_family_identity_digest;
pub use invariant_family_identity::WorthTopologyInvariantFamilyIdentity;
pub use validator_family_identity::WorthTopologyValidatorFamilyIdentity;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthTopologyLegalityFamilyIdentity {
    Validator(WorthTopologyValidatorFamilyIdentity),
    Invariant(WorthTopologyInvariantFamilyIdentity),
}

impl WorthTopologyLegalityFamilyIdentity {
    pub fn stable_key(&self) -> &str {
        match self {
            Self::Validator(identity) => identity.stable_key(),
            Self::Invariant(identity) => identity.stable_key(),
        }
    }

    pub fn identity_digest(&self) -> &str {
        match self {
            Self::Validator(identity) => identity.identity_digest(),
            Self::Invariant(identity) => identity.identity_digest(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Validator(identity) => identity.name(),
            Self::Invariant(identity) => identity.name(),
        }
    }
}
