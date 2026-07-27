use super::ApplicationSchemaIdentity;

/// Descriptive identity of the installed schema against which an intent was
/// authored.
///
/// This value is not installation authority. Execution must validate it
/// against the opaque installed schema handle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationSchemaBindingIdentity {
    runtime_ordinal: u64,
    generation: u64,
    package_identity: String,
    schema_identity: ApplicationSchemaIdentity,
}

impl ApplicationSchemaBindingIdentity {
    #[doc(hidden)]
    pub fn from_installed_parts(
        runtime_ordinal: u64,
        generation: u64,
        package_identity: impl Into<String>,
        schema_identity: ApplicationSchemaIdentity,
    ) -> Self {
        Self {
            runtime_ordinal,
            generation,
            package_identity: package_identity.into(),
            schema_identity,
        }
    }

    pub const fn runtime_ordinal(&self) -> u64 {
        self.runtime_ordinal
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub fn package_identity(&self) -> &str {
        &self.package_identity
    }

    pub fn schema_identity(&self) -> &ApplicationSchemaIdentity {
        &self.schema_identity
    }
}
