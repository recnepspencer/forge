use std::borrow::Borrow;
use std::fmt;

use super::AuthoringError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AspectName(String);

impl AspectName {
    pub fn new(name: impl Into<String>) -> Result<Self, AuthoringError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(AuthoringError::EmptyProjectionSelector);
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for AspectName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AspectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FieldName(String);

impl FieldName {
    pub fn new(name: impl Into<String>) -> Result<Self, AuthoringError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(AuthoringError::EmptyProjectionSelector);
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for FieldName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeliveredFieldName(String);

impl DeliveredFieldName {
    pub fn new(name: impl Into<String>) -> Result<Self, AuthoringError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(AuthoringError::EmptyDeliveredFieldName);
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for DeliveredFieldName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DeliveredFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelationName(String);

impl RelationName {
    pub fn new(name: impl Into<String>) -> Result<Self, AuthoringError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(AuthoringError::EmptyTraversalRelation);
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for RelationName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RelationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AspectFieldKey {
    aspect: AspectName,
    field: FieldName,
}

impl AspectFieldKey {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        Ok(Self {
            aspect: AspectName::new(aspect)?,
            field: FieldName::new(field)?,
        })
    }

    pub fn from_parts(aspect: AspectName, field: FieldName) -> Self {
        Self { aspect, field }
    }

    pub fn aspect(&self) -> &AspectName {
        &self.aspect
    }

    pub fn field(&self) -> &FieldName {
        &self.field
    }
}
