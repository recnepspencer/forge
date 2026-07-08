use core::error::Error;
use core::fmt;
use worth_schema_core::facade::{InvalidName, Name};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidPackName {
    reason: &'static str,
}

impl InvalidPackName {
    fn new(reason: &'static str) -> Self {
        Self { reason }
    }
}

impl fmt::Display for InvalidPackName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.reason)
    }
}

impl Error for InvalidPackName {}

impl From<InvalidName> for InvalidPackName {
    fn from(_: InvalidName) -> Self {
        Self::new("pack name must be a valid schema name")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackName(Name);

impl PackName {
    pub fn new(raw: impl Into<String>) -> Result<Self, InvalidPackName> {
        let name = Name::new(raw).map_err(InvalidPackName::from)?;
        if !name.as_str().contains("-pack-") {
            return Err(InvalidPackName::new(
                "pack name must encode a pack family with '-pack-'",
            ));
        }
        Ok(Self(name))
    }

    pub fn as_name(&self) -> &Name {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
