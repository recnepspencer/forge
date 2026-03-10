use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::runtime::AdapterSupport;

use super::record::{ArtifactSurface, CheckpointSemantics};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowArtifactSurfaceCapability {
    pub surface: ArtifactSurface,
    pub profiles: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DifferentialMatrixCapability {
    pub matrix_name: String,
    pub profiles: BTreeSet<String>,
    pub guaranteed_surfaces: BTreeSet<ArtifactSurface>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedWorkflowComparison {
    pub surface: ArtifactSurface,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileConditionalGuarantee {
    pub profile_name: String,
    pub guarantee: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowCertificationCapabilities {
    pub artifact_surfaces: Vec<WorkflowArtifactSurfaceCapability>,
    pub checkpoint_semantics: BTreeSet<CheckpointSemantics>,
    pub replay_recovery_support: BTreeSet<ArtifactSurface>,
    pub differential_matrices: Vec<DifferentialMatrixCapability>,
    pub unsupported_comparisons: Vec<UnsupportedWorkflowComparison>,
    pub profile_guarantees: Vec<ProfileConditionalGuarantee>,
    pub budget_artifacts: AdapterSupport,
}

impl Default for WorkflowCertificationCapabilities {
    fn default() -> Self {
        Self {
            artifact_surfaces: Vec::new(),
            checkpoint_semantics: BTreeSet::new(),
            replay_recovery_support: BTreeSet::new(),
            differential_matrices: Vec::new(),
            unsupported_comparisons: Vec::new(),
            profile_guarantees: Vec::new(),
            budget_artifacts: AdapterSupport::Unsupported,
        }
    }
}
