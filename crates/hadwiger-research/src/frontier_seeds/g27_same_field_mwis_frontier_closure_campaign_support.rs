use crate::domain_artifacts::core_artifact::{impl_hadwiger_artifact, HadwigerArtifactCore};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisFrontierCampaignStatus {
    CampaignScoutReady,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
    FrontierShapeMismatch,
}

impl G27MwisFrontierCampaignStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CampaignScoutReady => "campaign_scout_ready",
            Self::PrefixReplayMismatch => "prefix_replay_mismatch",
            Self::FrozenInstanceMismatch => "frozen_instance_mismatch",
            Self::FrontierShapeMismatch => "frontier_shape_mismatch",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisFrontierCampaignRowClass {
    ReadyForExactThreeLeafReplay,
    FloatingAboveTarget,
    LpSolveFailed,
}

impl G27MwisFrontierCampaignRowClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyForExactThreeLeafReplay => "ready_for_exact_three_leaf_replay",
            Self::FloatingAboveTarget => "floating_above_target",
            Self::LpSolveFailed => "lp_solve_failed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisFrontierCampaignNode {
    pub(super) index: usize,
    pub(super) total: i128,
    pub(super) depth: usize,
    pub(super) digest: String,
    pub(super) previously_closed: bool,
}

impl G27MwisFrontierCampaignNode {
    pub fn summary(&self) -> (usize, i128, usize, bool) {
        (self.index, self.total, self.depth, self.previously_closed)
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisFrontierCampaignRow {
    pub(super) index: usize,
    pub(super) parent_total: i128,
    pub(super) parent_depth: usize,
    pub(super) parent_digest: String,
    pub(super) first_branch: usize,
    pub(super) first_child_totals: [i128; 2],
    pub(super) worse_child: usize,
    pub(super) second_branch: usize,
    pub(super) terminal_totals: Vec<i128>,
    pub(super) row_class: G27MwisFrontierCampaignRowClass,
}

impl G27MwisFrontierCampaignRow {
    pub fn summary(&self) -> (usize, i128, usize, usize, [i128; 2], usize, usize) {
        (
            self.index,
            self.parent_total,
            self.parent_depth,
            self.first_branch,
            self.first_child_totals,
            self.worse_child,
            self.second_branch,
        )
    }

    pub fn parent_digest(&self) -> &str {
        &self.parent_digest
    }

    pub fn terminal_totals(&self) -> &[i128] {
        &self.terminal_totals
    }

    pub fn row_class(&self) -> G27MwisFrontierCampaignRowClass {
        self.row_class
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisFrontierClosureCampaignScoutReport {
    pub(super) core: HadwigerArtifactCore,
    pub(super) status: G27MwisFrontierCampaignStatus,
    pub(super) frontier_nodes: Vec<G27MwisFrontierCampaignNode>,
    pub(super) scout_rows: Vec<G27MwisFrontierCampaignRow>,
    pub(super) ready_count: usize,
    pub(super) failing_count: usize,
    pub(super) worst_terminal_total: i128,
    pub(super) continuation_max_total: i128,
}

impl G27MwisFrontierClosureCampaignScoutReport {
    pub fn summary(&self) -> (usize, usize, usize, usize, i128, i128) {
        (
            self.frontier_nodes.len(),
            self.scout_rows.len(),
            self.ready_count,
            self.failing_count,
            self.worst_terminal_total,
            self.continuation_max_total,
        )
    }

    pub fn status(&self) -> G27MwisFrontierCampaignStatus {
        self.status
    }

    pub fn frontier_nodes(&self) -> &[G27MwisFrontierCampaignNode] {
        &self.frontier_nodes
    }

    pub fn scout_rows(&self) -> &[G27MwisFrontierCampaignRow] {
        &self.scout_rows
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27MwisFrontierClosureCampaignScoutReport, core);
