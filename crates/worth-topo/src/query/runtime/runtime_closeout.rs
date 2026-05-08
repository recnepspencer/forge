use super::contracts::TopologyRuntimeSupport;
use super::edit_support::{
    TopologyQueryEditLaneExecutionShape, TopologyRuntimeEditFamilySupportRow,
    TopologyRuntimeEditLaneSupportRow,
};
use super::read_support::TopologyRuntimeReadFamilySupportRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyRuntimeCloseoutFamily {
    BridgeBackedRuntimePath,
    QueryNativeTopologyReadFamilies,
    QueryNativeTopologyEditFamilies,
    QueryNativeGraphComposedEditLanes,
    MirrorReadDeletion,
}

impl TopologyRuntimeCloseoutFamily {
    pub const ALL: [Self; 5] = [
        Self::BridgeBackedRuntimePath,
        Self::QueryNativeTopologyReadFamilies,
        Self::QueryNativeTopologyEditFamilies,
        Self::QueryNativeGraphComposedEditLanes,
        Self::MirrorReadDeletion,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyRuntimeCloseoutStatus {
    Satisfied,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRuntimeCloseoutRow {
    family: TopologyRuntimeCloseoutFamily,
    status: TopologyRuntimeCloseoutStatus,
    reason: String,
    row_digest: String,
}

impl TopologyRuntimeCloseoutRow {
    pub fn family(&self) -> TopologyRuntimeCloseoutFamily {
        self.family
    }

    pub fn status(&self) -> TopologyRuntimeCloseoutStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(super) fn satisfied(
        family: TopologyRuntimeCloseoutFamily,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        Self {
            family,
            status: TopologyRuntimeCloseoutStatus::Satisfied,
            row_digest: closeout_row_digest(
                family,
                TopologyRuntimeCloseoutStatus::Satisfied,
                &reason,
            ),
            reason,
        }
    }

    pub(super) fn blocked(
        family: TopologyRuntimeCloseoutFamily,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        Self {
            family,
            status: TopologyRuntimeCloseoutStatus::Blocked,
            row_digest: closeout_row_digest(
                family,
                TopologyRuntimeCloseoutStatus::Blocked,
                &reason,
            ),
            reason,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRuntimeCloseout {
    rows: Vec<TopologyRuntimeCloseoutRow>,
}

impl TopologyRuntimeCloseout {
    pub fn rows(&self) -> &[TopologyRuntimeCloseoutRow] {
        &self.rows
    }

    pub fn status(&self, family: TopologyRuntimeCloseoutFamily) -> TopologyRuntimeCloseoutStatus {
        self.rows
            .iter()
            .find(|row| row.family == family)
            .map(TopologyRuntimeCloseoutRow::status)
            .unwrap_or_else(|| {
                panic!(" runtime closeout rows should cover every declared closeout family")
            })
    }

    pub(super) fn new(rows: Vec<TopologyRuntimeCloseoutRow>) -> Self {
        Self { rows }
    }
}

impl TopologyRuntimeSupport {
    pub fn closeout(&self) -> &TopologyRuntimeCloseout {
        &self.closeout
    }
}

pub(super) fn runtime_closeout_from_support_rows(
    read_family_support_rows: &[TopologyRuntimeReadFamilySupportRow],
    edit_family_support_rows: &[TopologyRuntimeEditFamilySupportRow],
    edit_lane_support_rows: &[TopologyRuntimeEditLaneSupportRow],
) -> TopologyRuntimeCloseout {
    let all_read_families_admitted = read_family_support_rows
        .iter()
        .all(|row| row.status().is_admitted());
    let all_edit_families_supported = edit_family_support_rows
        .iter()
        .all(|row| row.status().is_supported());
    let all_graph_composed_edit_lanes_admitted = edit_lane_support_rows
        .iter()
        .filter(|row| {
            matches!(
                row.execution_shape(),
                TopologyQueryEditLaneExecutionShape::GraphComposition
            )
        })
        .all(|row| row.status().is_admitted());
    TopologyRuntimeCloseout::new(
        TopologyRuntimeCloseoutFamily::ALL
            .into_iter()
            .map(|family| {
                runtime_closeout_row(
                    family,
                    all_read_families_admitted,
                    all_edit_families_supported,
                    all_graph_composed_edit_lanes_admitted,
                )
            })
            .collect(),
    )
}

fn runtime_closeout_row(
    family: TopologyRuntimeCloseoutFamily,
    all_read_families_admitted: bool,
    all_edit_families_supported: bool,
    all_graph_composed_edit_lanes_admitted: bool,
) -> TopologyRuntimeCloseoutRow {
    match family {
        TopologyRuntimeCloseoutFamily::BridgeBackedRuntimePath => {
            TopologyRuntimeCloseoutRow::satisfied(
                family,
                "the surviving  runtime facade is admitted only through the bridge-backed forge-query runtime path",
            )
        }
        TopologyRuntimeCloseoutFamily::QueryNativeTopologyReadFamilies => {
            if all_read_families_admitted {
                TopologyRuntimeCloseoutRow::satisfied(
                    family,
                    "the public topology-domain read families are admitted on this runtime posture",
                )
            } else {
                TopologyRuntimeCloseoutRow::blocked(
                    family,
                    "one or more public topology-domain read families remain outside the admitted runtime posture",
                )
            }
        }
        TopologyRuntimeCloseoutFamily::QueryNativeTopologyEditFamilies => {
            if all_edit_families_supported {
                TopologyRuntimeCloseoutRow::satisfied(
                    family,
                    "the public topology-domain edit families are admitted or lane-backed on this runtime posture",
                )
            } else {
                TopologyRuntimeCloseoutRow::blocked(
                    family,
                    "one or more public topology-domain edit families remain outside the admitted runtime posture",
                )
            }
        }
        TopologyRuntimeCloseoutFamily::QueryNativeGraphComposedEditLanes => {
            if all_graph_composed_edit_lanes_admitted {
                TopologyRuntimeCloseoutRow::satisfied(
                    family,
                    "the graph-composed public topology edit lanes are admitted on this runtime posture",
                )
            } else {
                TopologyRuntimeCloseoutRow::blocked(
                    family,
                    "one or more graph-composed public topology edit lanes remain outside the admitted runtime posture",
                )
            }
        }
        TopologyRuntimeCloseoutFamily::MirrorReadDeletion => {
            TopologyRuntimeCloseoutRow::satisfied(
                family,
                "the surviving public runtime contract does not expose mirror-read fallback surfaces",
            )
        }
    }
}

fn closeout_row_digest(
    family: TopologyRuntimeCloseoutFamily,
    status: TopologyRuntimeCloseoutStatus,
    reason: &str,
) -> String {
    format!("family={family:?};status={status:?};reason={reason}",)
}
