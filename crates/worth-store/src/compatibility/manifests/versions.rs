use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ArtifactFormatVersion(u32);

impl ArtifactFormatVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ArtifactSemanticVersion(u32);

impl ArtifactSemanticVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactCompatibilityWindow {
    minimum_format: ArtifactFormatVersion,
    maximum_format: ArtifactFormatVersion,
    minimum_semantic: ArtifactSemanticVersion,
    maximum_semantic: ArtifactSemanticVersion,
}

impl ArtifactCompatibilityWindow {
    pub fn new(
        minimum_format: ArtifactFormatVersion,
        maximum_format: ArtifactFormatVersion,
        minimum_semantic: ArtifactSemanticVersion,
        maximum_semantic: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            minimum_format,
            maximum_format,
            minimum_semantic,
            maximum_semantic,
        }
    }

    pub fn native(version: u32) -> Self {
        Self::new(
            ArtifactFormatVersion::new(version),
            ArtifactFormatVersion::new(version),
            ArtifactSemanticVersion::new(version),
            ArtifactSemanticVersion::new(version),
        )
    }

    pub fn minimum_format(&self) -> ArtifactFormatVersion {
        self.minimum_format
    }

    pub fn maximum_format(&self) -> ArtifactFormatVersion {
        self.maximum_format
    }

    pub fn minimum_semantic(&self) -> ArtifactSemanticVersion {
        self.minimum_semantic
    }

    pub fn maximum_semantic(&self) -> ArtifactSemanticVersion {
        self.maximum_semantic
    }
}

impl ArtifactCompatibilityWindow {
    pub fn contains_format(&self, version: ArtifactFormatVersion) -> bool {
        self.minimum_format <= version && version <= self.maximum_format
    }

    pub fn contains_semantic(&self, version: ArtifactSemanticVersion) -> bool {
        self.minimum_semantic <= version && version <= self.maximum_semantic
    }
}
