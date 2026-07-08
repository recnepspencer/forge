use forge_store_compatibility::{ArtifactFormatVersion, ArtifactSemanticVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutVersion {
    format_version: ArtifactFormatVersion,
    semantic_version: ArtifactSemanticVersion,
}

impl LayoutVersion {
    pub const fn new(
        format_version: ArtifactFormatVersion,
        semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            format_version,
            semantic_version,
        }
    }

    pub const fn format_version(self) -> ArtifactFormatVersion {
        self.format_version
    }

    pub const fn semantic_version(self) -> ArtifactSemanticVersion {
        self.semantic_version
    }
}
