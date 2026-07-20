#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct InstalledDeclarationFamilyKey(String);

impl InstalledDeclarationFamilyKey {
    pub(super) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for InstalledDeclarationFamilyKey {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct InstalledDeclarationFamilySlot {
    owner: String,
    family: String,
}

impl InstalledDeclarationFamilySlot {
    pub(super) fn new(owner: impl Into<String>, family: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            family: family.into(),
        }
    }

    pub(super) fn terminal_projection(&self) -> String {
        format!("{}:{}", self.owner, self.family)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct InstalledInvariantSlot {
    owner: String,
    invariant: String,
}

impl InstalledInvariantSlot {
    pub(super) fn new(owner: impl Into<String>, invariant: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            invariant: invariant.into(),
        }
    }

    pub(super) fn terminal_projection(&self) -> String {
        format!("{}:{}", self.owner, self.invariant)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct InstalledDomainOwner(String);

impl InstalledDomainOwner {
    pub(super) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}
