use crate::historical::HistoricalMaterializationPathMetadata;
use crate::memory_workspace::{WorthQueryEntity, WorthQuerySnapshotIdentity};
use crate::WorthQueryEvidenceIdentity;

use super::WorthQueryComparisonBasisFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryComparisonCostClass {
    CurrentAndRetainedMaterialization,
    DeterministicIdentityIndexBuildAndMerge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryComparisonMaterialization {
    RuntimeCurrent,
    RetainedHistorical(HistoricalMaterializationPathMetadata),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryComparisonBasisEvidence {
    workspace_name: String,
    snapshot: WorthQuerySnapshotIdentity,
    materialization: WorthQueryComparisonMaterialization,
    branch_admission_identity: Option<WorthQueryEvidenceIdentity>,
}

impl WorthQueryComparisonBasisEvidence {
    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn snapshot(&self) -> &WorthQuerySnapshotIdentity {
        &self.snapshot
    }

    pub fn materialization(&self) -> &WorthQueryComparisonMaterialization {
        &self.materialization
    }

    pub fn branch_admission_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.branch_admission_identity.as_ref()
    }

    pub(crate) fn new(
        workspace_name: impl Into<String>,
        snapshot: WorthQuerySnapshotIdentity,
        materialization: WorthQueryComparisonMaterialization,
        branch_admission_identity: Option<WorthQueryEvidenceIdentity>,
    ) -> Self {
        Self {
            workspace_name: workspace_name.into(),
            snapshot,
            materialization,
            branch_admission_identity,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryComparisonBasisPairEvidence {
    family: WorthQueryComparisonBasisFamily,
    left: WorthQueryComparisonBasisEvidence,
    right: WorthQueryComparisonBasisEvidence,
}

impl WorthQueryComparisonBasisPairEvidence {
    pub fn family(&self) -> WorthQueryComparisonBasisFamily {
        self.family
    }

    pub fn left(&self) -> &WorthQueryComparisonBasisEvidence {
        &self.left
    }

    pub fn right(&self) -> &WorthQueryComparisonBasisEvidence {
        &self.right
    }

    pub(crate) fn new(
        family: WorthQueryComparisonBasisFamily,
        left: WorthQueryComparisonBasisEvidence,
        right: WorthQueryComparisonBasisEvidence,
    ) -> Self {
        Self {
            family,
            left,
            right,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryComparisonRowChangeFamily {
    Added,
    Removed,
    Modified,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryComparisonRowChange {
    family: WorthQueryComparisonRowChangeFamily,
    left: Option<WorthQueryEntity>,
    right: Option<WorthQueryEntity>,
}

impl WorthQueryComparisonRowChange {
    pub fn family(&self) -> WorthQueryComparisonRowChangeFamily {
        self.family
    }

    pub fn left(&self) -> Option<&WorthQueryEntity> {
        self.left.as_ref()
    }

    pub fn right(&self) -> Option<&WorthQueryEntity> {
        self.right.as_ref()
    }

    pub(crate) fn new(
        family: WorthQueryComparisonRowChangeFamily,
        left: Option<WorthQueryEntity>,
        right: Option<WorthQueryEntity>,
    ) -> Self {
        Self {
            family,
            left,
            right,
        }
    }
}
