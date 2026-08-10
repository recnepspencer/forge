use super::artifact_validation::S0ArtifactBuildRejection;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct S0ArtifactRowId(String);

impl S0ArtifactRowId {
    pub fn new(value: impl Into<String>) -> Result<Self, S0ArtifactBuildRejection> {
        let value = value.into();
        let unstable_marker = value.contains(':')
            || value.contains('/')
            || value.contains('\\')
            || value.contains('#')
            || value.to_ascii_lowercase().contains("line");
        if value.trim().is_empty() {
            return Err(S0ArtifactBuildRejection::EmptyRowId);
        }
        if unstable_marker {
            return Err(S0ArtifactBuildRejection::UnstableRowId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum S0ArtifactSubjectKind {
    Backend,
    EvidenceLane,
    Milestone,
    ClaimSurface,
    TestSuite,
    Harness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum S0ArtifactRowStatus {
    Present,
    AbsentWithInventoryEvidence,
    Deferred,
    NotApplicable,
    Admitted,
}
