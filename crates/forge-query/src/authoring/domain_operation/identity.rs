use super::DomainGraphOperationDeclarationError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryGraphReadOperationName(String);

impl ForgeQueryGraphReadOperationName {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainGraphOperationDeclarationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainGraphOperationDeclarationError::EmptyOperationName);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryDomainOwner(String);

impl ForgeQueryDomainOwner {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainGraphOperationDeclarationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainGraphOperationDeclarationError::EmptyDomainOwner);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryGraphReadOperationVersion(u32);

impl ForgeQueryGraphReadOperationVersion {
    pub fn new(value: u32) -> Result<Self, DomainGraphOperationDeclarationError> {
        if value == 0 {
            return Err(DomainGraphOperationDeclarationError::ZeroOperationVersion);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryGraphReadOperationKey {
    name: ForgeQueryGraphReadOperationName,
    version: ForgeQueryGraphReadOperationVersion,
    owner: ForgeQueryDomainOwner,
}

impl ForgeQueryGraphReadOperationKey {
    pub fn new(
        name: impl Into<String>,
        version: u32,
        owner: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        Ok(Self {
            name: ForgeQueryGraphReadOperationName::new(name)?,
            version: ForgeQueryGraphReadOperationVersion::new(version)?,
            owner: ForgeQueryDomainOwner::new(owner)?,
        })
    }

    pub fn name(&self) -> &ForgeQueryGraphReadOperationName {
        &self.name
    }

    pub fn version(&self) -> &ForgeQueryGraphReadOperationVersion {
        &self.version
    }

    pub fn owner(&self) -> &ForgeQueryDomainOwner {
        &self.owner
    }

    pub fn digest_part(&self) -> String {
        format!(
            "{}@{}#{}",
            self.name.as_str(),
            self.version.value(),
            self.owner.as_str()
        )
    }
}
