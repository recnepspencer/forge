use crate::denial::WorthQueryPackageArchiveDenial as Denial;

use super::validate_descriptive_text;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageBuildMetadata {
    compiler_identity: String,
    compiler_version: String,
    toolchain_identity: String,
    toolchain_version: String,
    target_triple: String,
}

impl WorthQueryPackageBuildMetadata {
    pub fn new(
        compiler_identity: impl Into<String>,
        compiler_version: impl Into<String>,
        toolchain_identity: impl Into<String>,
        toolchain_version: impl Into<String>,
        target_triple: impl Into<String>,
    ) -> Result<Self, Denial> {
        let value = Self {
            compiler_identity: compiler_identity.into(),
            compiler_version: compiler_version.into(),
            toolchain_identity: toolchain_identity.into(),
            toolchain_version: toolchain_version.into(),
            target_triple: target_triple.into(),
        };
        value.validate()?;
        Ok(value)
    }

    pub fn compiler_identity(&self) -> &str {
        &self.compiler_identity
    }
    pub fn compiler_version(&self) -> &str {
        &self.compiler_version
    }
    pub fn toolchain_identity(&self) -> &str {
        &self.toolchain_identity
    }
    pub fn toolchain_version(&self) -> &str {
        &self.toolchain_version
    }
    pub fn target_triple(&self) -> &str {
        &self.target_triple
    }

    fn validate(&self) -> Result<(), Denial> {
        validate_descriptive_text(&self.compiler_identity)?;
        validate_descriptive_text(&self.compiler_version)?;
        validate_descriptive_text(&self.toolchain_identity)?;
        validate_descriptive_text(&self.toolchain_version)?;
        validate_descriptive_text(&self.target_triple)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageReleaseMetadata {
    release_name: String,
    release_version: String,
}

impl WorthQueryPackageReleaseMetadata {
    pub fn new(
        release_name: impl Into<String>,
        release_version: impl Into<String>,
    ) -> Result<Self, Denial> {
        let value = Self {
            release_name: release_name.into(),
            release_version: release_version.into(),
        };
        validate_descriptive_text(&value.release_name)?;
        validate_descriptive_text(&value.release_version)?;
        Ok(value)
    }

    pub fn release_name(&self) -> &str {
        &self.release_name
    }
    pub fn release_version(&self) -> &str {
        &self.release_version
    }
}
