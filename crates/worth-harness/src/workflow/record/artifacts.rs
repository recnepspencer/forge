use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::artifact::AttachmentRecord;

use super::super::capability::UnsupportedWorkflowComparison;
use super::planning::{WorkflowRuntimeProfile, WorkflowState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArtifactClass {
    Truth,
    Observability,
    Forensic,
    Performance,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArtifactSurface {
    SnapshotVisibleTruth,
    BranchHeadState,
    ReplayRecoveryTruthState,
    Diagnostics,
    PatchChangeSurface,
    StepTrace,
    CheckpointTrace,
    FailureInjectionTrace,
    ReproductionMetadata,
    ComplexityCounters,
    BudgetOutcome,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactBundle {
    pub artifact_class: ArtifactClass,
    pub surface: ArtifactSurface,
    pub name: String,
    pub boundary: WorkflowState,
    pub payload: Value,
    pub attachments: Vec<AttachmentRecord>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCheck {
    pub check_id: String,
    pub description: String,
    pub boundary: WorkflowState,
    pub required: bool,
}

impl InvariantCheck {
    pub fn new(
        check_id: impl Into<String>,
        description: impl Into<String>,
        boundary: WorkflowState,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            description: description.into(),
            boundary,
            required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvariantReport {
    pub check_id: String,
    pub boundary: WorkflowState,
    pub passed: bool,
    pub detail: String,
    pub fields: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DifferentialComparison {
    pub comparison_name: String,
    pub left_label: String,
    pub right_label: String,
    pub active_profile: WorkflowRuntimeProfile,
    pub guaranteed_overlap: BTreeSet<ArtifactSurface>,
    pub skipped_surfaces: Vec<UnsupportedWorkflowComparison>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentialOutcome {
    pub matched: bool,
    pub compared_surfaces: BTreeSet<ArtifactSurface>,
    pub mismatches: Vec<String>,
    pub skipped_surfaces: Vec<UnsupportedWorkflowComparison>,
}
