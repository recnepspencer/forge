use super::WorthQueryArtifactOccurrenceIdentityPolicy;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryArtifactSubstitutionPurpose {
    ComputationalReuse,
    EvidentiarySubstitution,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactOccurrenceContract {
    identity_policy: WorthQueryArtifactOccurrenceIdentityPolicy,
    permitted_substitutions: Vec<WorthQueryArtifactSubstitutionPurpose>,
}

impl WorthQueryArtifactOccurrenceContract {
    pub fn independent_per_execution() -> Self {
        Self {
            identity_policy: WorthQueryArtifactOccurrenceIdentityPolicy::IndependentPerExecution,
            permitted_substitutions: Vec::new(),
        }
    }

    pub fn domain_minted_independent() -> Self {
        Self {
            identity_policy: WorthQueryArtifactOccurrenceIdentityPolicy::DomainMintedIndependent,
            permitted_substitutions: Vec::new(),
        }
    }

    pub fn permit(mut self, purpose: WorthQueryArtifactSubstitutionPurpose) -> Self {
        self.permitted_substitutions.push(purpose);
        self.canonicalize();
        self
    }

    pub const fn identity_policy(&self) -> WorthQueryArtifactOccurrenceIdentityPolicy {
        self.identity_policy
    }

    pub fn permitted_substitutions(&self) -> &[WorthQueryArtifactSubstitutionPurpose] {
        &self.permitted_substitutions
    }

    pub fn permits_substitution(&self, purpose: WorthQueryArtifactSubstitutionPurpose) -> bool {
        self.permitted_substitutions.contains(&purpose)
    }

    pub(crate) fn canonicalize(&mut self) {
        self.permitted_substitutions.sort();
        self.permitted_substitutions.dedup();
    }
}
