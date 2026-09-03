/// Explicit posture for a component while creating a product branch from an
/// admitted composite basis. There is no omitted or ambient-current posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductBranchComponentPosture {
    ReuseExact,
    ForkExact,
    ForkAndAdvance,
}

impl ProductBranchComponentPosture {
    pub const fn is_reuse_exact(self) -> bool {
        matches!(self, Self::ReuseExact)
    }

    pub const fn requires_owner_effect(self) -> bool {
        !self.is_reuse_exact()
    }
}

/// The branch-creation posture is a complete two-component value. A branch
/// operation cannot be lowered until both owner postures are present.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductBranchComponentPostures {
    relational: ProductBranchComponentPosture,
    signal: ProductBranchComponentPosture,
}

impl ProductBranchComponentPostures {
    pub const fn new(
        relational: ProductBranchComponentPosture,
        signal: ProductBranchComponentPosture,
    ) -> Self {
        Self { relational, signal }
    }

    pub const fn relational(self) -> ProductBranchComponentPosture {
        self.relational
    }

    pub const fn signal(self) -> ProductBranchComponentPosture {
        self.signal
    }

    pub const fn is_exact_reuse(self) -> bool {
        self.relational.is_reuse_exact() && self.signal.is_reuse_exact()
    }

    pub const fn requires_owner_effect(self) -> bool {
        self.relational.requires_owner_effect() || self.signal.requires_owner_effect()
    }
}

/// Validated descriptive input. It is not a product branch identity and does
/// not select an owner or a current head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductBranchName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductBranchNameDenial {
    Empty,
    TooLong { maximum: usize, actual: usize },
}

impl ProductBranchName {
    const MAXIMUM_LENGTH: usize = 256;

    pub fn try_new(name: impl Into<String>) -> Result<Self, ProductBranchNameDenial> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ProductBranchNameDenial::Empty);
        }
        if name.len() > Self::MAXIMUM_LENGTH {
            return Err(ProductBranchNameDenial::TooLong {
                maximum: Self::MAXIMUM_LENGTH,
                actual: name.len(),
            });
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Explicit branch-creation meaning. A name is never promoted to a branch
/// identity without the Runtime World owner issuing that identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductBranchCreationIntent {
    name: ProductBranchName,
}

impl ProductBranchCreationIntent {
    pub fn named(name: impl Into<String>) -> Result<Self, ProductBranchNameDenial> {
        Ok(Self {
            name: ProductBranchName::try_new(name)?,
        })
    }

    pub fn name(&self) -> &ProductBranchName {
        &self.name
    }
}

#[cfg(test)]
#[path = "creation_tests.rs"]
mod tests;
