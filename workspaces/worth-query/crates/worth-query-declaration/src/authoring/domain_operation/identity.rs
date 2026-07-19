use super::DomainGraphOperationDeclarationError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryGraphReadOperationName(String);

impl WorthQueryGraphReadOperationName {
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
pub struct WorthQueryDomainOwner(String);

impl WorthQueryDomainOwner {
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
pub struct WorthQueryGraphReadOperationVersion(u32);

impl WorthQueryGraphReadOperationVersion {
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
pub struct WorthQueryGraphReadOperationKey {
    name: WorthQueryGraphReadOperationName,
    version: WorthQueryGraphReadOperationVersion,
    owner: WorthQueryDomainOwner,
}

impl WorthQueryGraphReadOperationKey {
    pub fn new(
        name: impl Into<String>,
        version: u32,
        owner: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        Ok(Self {
            name: WorthQueryGraphReadOperationName::new(name)?,
            version: WorthQueryGraphReadOperationVersion::new(version)?,
            owner: WorthQueryDomainOwner::new(owner)?,
        })
    }

    pub fn name(&self) -> &WorthQueryGraphReadOperationName {
        &self.name
    }

    pub fn version(&self) -> &WorthQueryGraphReadOperationVersion {
        &self.version
    }

    pub fn owner(&self) -> &WorthQueryDomainOwner {
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
