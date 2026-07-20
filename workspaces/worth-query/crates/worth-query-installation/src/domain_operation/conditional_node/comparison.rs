use super::{WorthQueryComparatorFamily, WorthQueryTypedFamilyIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryComparatorRequirement {
    ExactCanonicalValue,
    FoundationalContractEquivalence,
    Registered(WorthQueryTypedFamilyIdentity),
}

impl WorthQueryComparatorRequirement {
    pub fn registered<Family: WorthQueryComparatorFamily>() -> Self {
        Self::Registered(WorthQueryTypedFamilyIdentity::declared(
            Family::PORTABLE_IDENTITY,
        ))
    }

    pub(crate) fn is_portable(&self) -> bool {
        match self {
            Self::Registered(identity) => identity.is_portable(),
            Self::ExactCanonicalValue | Self::FoundationalContractEquivalence => true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOutputEquivalenceRequirement {
    ExactCanonicalValue,
    FoundationalContractEquivalence,
    OutputIdentity,
    Registered(WorthQueryTypedFamilyIdentity),
}

impl WorthQueryOutputEquivalenceRequirement {
    pub fn registered<Family: WorthQueryComparatorFamily>() -> Self {
        Self::Registered(WorthQueryTypedFamilyIdentity::declared(
            Family::PORTABLE_IDENTITY,
        ))
    }

    pub(crate) fn is_portable(&self) -> bool {
        match self {
            Self::Registered(identity) => identity.is_portable(),
            Self::ExactCanonicalValue
            | Self::FoundationalContractEquivalence
            | Self::OutputIdentity => true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactReuseEquivalence {
    NotReusable,
    DependencyAndOutputEquivalent,
    OutputEquivalent,
    Registered(WorthQueryTypedFamilyIdentity),
}

impl WorthQueryArtifactReuseEquivalence {
    pub fn registered<Family: WorthQueryComparatorFamily>() -> Self {
        Self::Registered(WorthQueryTypedFamilyIdentity::declared(
            Family::PORTABLE_IDENTITY,
        ))
    }

    pub(crate) fn is_portable(&self) -> bool {
        match self {
            Self::Registered(identity) => identity.is_portable(),
            Self::NotReusable | Self::DependencyAndOutputEquivalent | Self::OutputEquivalent => {
                true
            }
        }
    }
}

pub(crate) fn comparator_token(comparator: &WorthQueryComparatorRequirement) -> String {
    match comparator {
        WorthQueryComparatorRequirement::ExactCanonicalValue => "exact-canonical-value".to_string(),
        WorthQueryComparatorRequirement::FoundationalContractEquivalence => {
            "foundational-contract-equivalence".to_string()
        }
        WorthQueryComparatorRequirement::Registered(family) => registered_token(family),
    }
}

pub(crate) fn output_equivalence_token(
    equivalence: &WorthQueryOutputEquivalenceRequirement,
) -> String {
    match equivalence {
        WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue => {
            "exact-canonical-value".to_string()
        }
        WorthQueryOutputEquivalenceRequirement::FoundationalContractEquivalence => {
            "foundational-contract-equivalence".to_string()
        }
        WorthQueryOutputEquivalenceRequirement::OutputIdentity => "output-identity".to_string(),
        WorthQueryOutputEquivalenceRequirement::Registered(family) => registered_token(family),
    }
}

pub(crate) fn artifact_reuse_token(equivalence: &WorthQueryArtifactReuseEquivalence) -> String {
    match equivalence {
        WorthQueryArtifactReuseEquivalence::NotReusable => "not-reusable".to_string(),
        WorthQueryArtifactReuseEquivalence::DependencyAndOutputEquivalent => {
            "dependency-and-output-equivalent".to_string()
        }
        WorthQueryArtifactReuseEquivalence::OutputEquivalent => "output-equivalent".to_string(),
        WorthQueryArtifactReuseEquivalence::Registered(family) => registered_token(family),
    }
}

fn registered_token(family: &WorthQueryTypedFamilyIdentity) -> String {
    format!("registered#{}:{}", family.as_str().len(), family.as_str())
}
