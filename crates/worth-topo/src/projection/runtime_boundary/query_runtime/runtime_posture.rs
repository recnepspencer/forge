use super::contracts::TopologyRuntimeSupport;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyRuntimePostureCapability {
    CurrentHeadLiveReads,
    CurrentHeadMaterialization,
    PostWriteMaterialization,
    HistoricalBasis,
    BranchPreviewBasis,
    BranchLocalIntentStaging,
    BranchLocalDeclarationExecution,
    AuthoritativeWrites,
}

impl TopologyRuntimePostureCapability {
    pub const ALL: [Self; 8] = [
        Self::CurrentHeadLiveReads,
        Self::CurrentHeadMaterialization,
        Self::PostWriteMaterialization,
        Self::HistoricalBasis,
        Self::BranchPreviewBasis,
        Self::BranchLocalIntentStaging,
        Self::BranchLocalDeclarationExecution,
        Self::AuthoritativeWrites,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyRuntimePostureStatus {
    Denied,
    Admitted,
}

impl TopologyRuntimePostureStatus {
    pub fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRuntimePostureRow {
    capability: TopologyRuntimePostureCapability,
    status: TopologyRuntimePostureStatus,
    reason: String,
    row_digest: String,
}

impl TopologyRuntimePostureRow {
    pub fn capability(&self) -> TopologyRuntimePostureCapability {
        self.capability
    }

    pub fn status(&self) -> TopologyRuntimePostureStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(super) fn admitted(
        capability: TopologyRuntimePostureCapability,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        Self {
            capability,
            status: TopologyRuntimePostureStatus::Admitted,
            row_digest: posture_row_digest(
                capability,
                TopologyRuntimePostureStatus::Admitted,
                &reason,
            ),
            reason,
        }
    }

    pub(super) fn denied(
        capability: TopologyRuntimePostureCapability,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        Self {
            capability,
            status: TopologyRuntimePostureStatus::Denied,
            row_digest: posture_row_digest(
                capability,
                TopologyRuntimePostureStatus::Denied,
                &reason,
            ),
            reason,
        }
    }
}

impl TopologyRuntimeSupport {
    pub fn runtime_posture_rows(&self) -> &[TopologyRuntimePostureRow] {
        &self.runtime_posture_rows
    }

    pub fn runtime_posture_status(
        &self,
        capability: TopologyRuntimePostureCapability,
    ) -> TopologyRuntimePostureStatus {
        self.runtime_posture_rows
            .iter()
            .find(|row| row.capability == capability)
            .map(TopologyRuntimePostureRow::status)
            .unwrap_or_else(|| {
                panic!(" runtime posture rows should cover every declared capability")
            })
    }
}

pub(super) fn current_head_runtime_posture_rows() -> Vec<TopologyRuntimePostureRow> {
    TopologyRuntimePostureCapability::ALL
        .into_iter()
        .map(|capability| match capability {
            TopologyRuntimePostureCapability::CurrentHeadLiveReads => {
                TopologyRuntimePostureRow::admitted(
                    capability,
                    "current-head runtime admits bridge-backed live reads over canonical topology truth",
                )
            }
            TopologyRuntimePostureCapability::CurrentHeadMaterialization => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "current-head runtime does not admit a dedicated current-head materialization posture separate from the query-native live/computed path",
                )
            }
            TopologyRuntimePostureCapability::PostWriteMaterialization => {
                TopologyRuntimePostureRow::admitted(
                    capability,
                    "current-head runtime admits post-write materialization through the query-native derived surfaces",
                )
            }
            TopologyRuntimePostureCapability::HistoricalBasis => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "current-head runtime posture does not admit historical snapshot basis selection",
                )
            }
            TopologyRuntimePostureCapability::BranchPreviewBasis => {
                TopologyRuntimePostureRow::admitted(
                    capability,
                    "current-head runtime admits preview and branch-local basis selection over retained topology truth",
                )
            }
            TopologyRuntimePostureCapability::BranchLocalIntentStaging => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "current-head runtime admits branch sessions but does not admit branch-local intent staging because the Query intent family is not admitted on this topology runtime",
                )
            }
            TopologyRuntimePostureCapability::BranchLocalDeclarationExecution => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "current-head runtime does not yet admit branch-local topology declaration execution; branch-local authoring still crosses the schema-owned branch commit lane",
                )
            }
            TopologyRuntimePostureCapability::AuthoritativeWrites => {
                TopologyRuntimePostureRow::admitted(
                    capability,
                    "current-head runtime admits authoritative bridge-backed topology writes",
                )
            }
        })
        .collect()
}

pub(super) fn snapshot_runtime_posture_rows() -> Vec<TopologyRuntimePostureRow> {
    TopologyRuntimePostureCapability::ALL
        .into_iter()
        .map(|capability| match capability {
            TopologyRuntimePostureCapability::CurrentHeadLiveReads => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "snapshot read-only runtime does not admit current-head live reads",
                )
            }
            TopologyRuntimePostureCapability::CurrentHeadMaterialization => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "snapshot read-only runtime does not admit current-head materialization",
                )
            }
            TopologyRuntimePostureCapability::PostWriteMaterialization => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "snapshot read-only runtime does not admit post-write materialization because authoritative writes are denied",
                )
            }
            TopologyRuntimePostureCapability::HistoricalBasis => {
                TopologyRuntimePostureRow::admitted(
                    capability,
                    "snapshot read-only runtime admits historical snapshot basis reads",
                )
            }
            TopologyRuntimePostureCapability::BranchPreviewBasis => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "snapshot read-only runtime is already fixed to one historical basis and does not admit preview or branch-local basis selection",
                )
            }
            TopologyRuntimePostureCapability::BranchLocalIntentStaging => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "snapshot read-only runtime does not admit branch-local intent staging because preview and branch sessions are denied on historical-basis posture",
                )
            }
            TopologyRuntimePostureCapability::BranchLocalDeclarationExecution => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "snapshot read-only runtime does not admit branch-local topology declaration execution because authoritative writes are denied and branch-local authoring is unavailable",
                )
            }
            TopologyRuntimePostureCapability::AuthoritativeWrites => {
                TopologyRuntimePostureRow::denied(
                    capability,
                    "snapshot read-only runtime is read-only and does not admit authoritative writes",
                )
            }
        })
        .collect()
}

fn posture_row_digest(
    capability: TopologyRuntimePostureCapability,
    status: TopologyRuntimePostureStatus,
    reason: &str,
) -> String {
    format!("capability={capability:?};status={status:?};reason={reason}")
}
