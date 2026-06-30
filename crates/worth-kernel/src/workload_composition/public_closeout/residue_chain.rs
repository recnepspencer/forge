use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::worth_workload::{
    WorthWorkloadOrdinaryConsumerCutoverPosture, WorthWorkloadOrdinaryConsumerCutoverRow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictResidueBoundaryPosture {
    QueryProofAccompanimentOnly,
    ReplayUndoCloseoutOnly,
    CoveredOrdinaryConsumerDependency,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictResidueRow {
    surface_name: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    boundary_posture: WorthTouchedGraphConflictResidueBoundaryPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictResidueChain {
    rows: Vec<WorthTouchedGraphConflictResidueRow>,
    residue_digest: String,
}

impl WorthTouchedGraphConflictResidueChain {
    pub(crate) fn from_cutover_rows(rows: &[WorthWorkloadOrdinaryConsumerCutoverRow]) -> Self {
        let lowered = rows
            .iter()
            .filter_map(WorthTouchedGraphConflictResidueRow::from_cutover_row)
            .collect::<Vec<_>>();
        Self::from_rows(lowered)
    }

    pub(crate) fn from_rows(mut rows: Vec<WorthTouchedGraphConflictResidueRow>) -> Self {
        rows.sort_by(|left, right| left.surface_name.cmp(&right.surface_name));
        let residue_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}:{}:{}:{}:{}",
                        row.surface_name,
                        row.owner,
                        row.blocker,
                        row.removal_trigger,
                        row.boundary_posture.as_str()
                    )
                })
                .chain(std::iter::once(
                    "worth-kernel:touched-graph-conflict-residue-chain:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            residue_digest,
        }
    }

    pub fn rows(&self) -> &[WorthTouchedGraphConflictResidueRow] {
        &self.rows
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }

    pub fn ordinary_dependency_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                row.boundary_posture
                    == WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
            })
            .count()
    }
}

impl WorthTouchedGraphConflictResidueRow {
    pub(crate) fn new(
        surface_name: impl Into<String>,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
        boundary_posture: WorthTouchedGraphConflictResidueBoundaryPosture,
    ) -> Self {
        Self {
            surface_name: surface_name.into(),
            owner: owner.into(),
            blocker: blocker.into(),
            removal_trigger: removal_trigger.into(),
            boundary_posture,
        }
    }

    fn from_cutover_row(row: &WorthWorkloadOrdinaryConsumerCutoverRow) -> Option<Self> {
        let boundary_posture = match row.posture() {
            WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer => {
                return None;
            }
            WorthWorkloadOrdinaryConsumerCutoverPosture::QueryProofAccompanimentOnly => {
                WorthTouchedGraphConflictResidueBoundaryPosture::QueryProofAccompanimentOnly
            }
            WorthWorkloadOrdinaryConsumerCutoverPosture::ReplayUndoCloseoutOnly => {
                WorthTouchedGraphConflictResidueBoundaryPosture::ReplayUndoCloseoutOnly
            }
            WorthWorkloadOrdinaryConsumerCutoverPosture::CoveredOrdinaryConsumerDependency => {
                WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
            }
        };
        Some(Self::new(
            row.surface_name(),
            row.owner(),
            row.blocker(),
            row.removal_trigger(),
            boundary_posture,
        ))
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn boundary_posture(&self) -> WorthTouchedGraphConflictResidueBoundaryPosture {
        self.boundary_posture
    }
}

impl WorthTouchedGraphConflictResidueBoundaryPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryProofAccompanimentOnly => "query-proof-accompaniment-only",
            Self::ReplayUndoCloseoutOnly => "replay-undo-closeout-only",
            Self::CoveredOrdinaryConsumerDependency => "covered-ordinary-consumer-dependency",
        }
    }
}
