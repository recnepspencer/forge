use crate::domain_artifacts::core_artifact::{impl_hadwiger_artifact, HadwigerArtifactCore};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisFrontierClosureExactStatus {
    ExactChunkCertified,
    InvalidChunkScope,
    ScoutDigestMismatch,
    ScoutSourceMismatch,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
    FrontierShapeMismatch,
    ParentIdentityMismatch,
    ScoutRowMismatch,
    FloatingBoundAboveTarget,
    IncompleteLeafPartition,
    ExactReplayFailed,
}

impl G27MwisFrontierClosureExactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactChunkCertified => "exact_chunk_certified",
            Self::InvalidChunkScope => "invalid_chunk_scope",
            Self::ScoutDigestMismatch => "scout_digest_mismatch",
            Self::ScoutSourceMismatch => "scout_source_mismatch",
            Self::PrefixReplayMismatch => "prefix_replay_mismatch",
            Self::FrozenInstanceMismatch => "frozen_instance_mismatch",
            Self::FrontierShapeMismatch => "frontier_shape_mismatch",
            Self::ParentIdentityMismatch => "parent_identity_mismatch",
            Self::ScoutRowMismatch => "scout_row_mismatch",
            Self::FloatingBoundAboveTarget => "floating_bound_above_target",
            Self::IncompleteLeafPartition => "incomplete_leaf_partition",
            Self::ExactReplayFailed => "exact_replay_failed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisFrontierClosureExactLeafStatus {
    ExactLeafCertified,
    DualCoverageFailed,
    DualObjectiveMismatch,
    BoundAboveThreshold,
}

impl G27MwisFrontierClosureExactLeafStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactLeafCertified => "exact_leaf_certified",
            Self::DualCoverageFailed => "dual_coverage_failed",
            Self::DualObjectiveMismatch => "dual_objective_mismatch",
            Self::BoundAboveThreshold => "bound_above_threshold",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisFrontierClosureExactLeaf {
    pub(super) leaf_index: usize,
    pub(super) terminal_total: i128,
    pub(super) certified_total: i128,
    pub(super) explicit_rows: usize,
    pub(super) positive_dual_rows: usize,
    pub(super) max_denominator: i128,
    pub(super) min_slack_floor: i128,
    pub(super) objective_excess: i128,
    pub(super) row_digest: String,
    pub(super) dual_digest: String,
    pub(super) status: G27MwisFrontierClosureExactLeafStatus,
}

impl G27MwisFrontierClosureExactLeaf {
    pub fn summary(&self) -> (usize, i128, i128, usize, usize, i128, i128, i128) {
        (
            self.leaf_index,
            self.terminal_total,
            self.certified_total,
            self.explicit_rows,
            self.positive_dual_rows,
            self.max_denominator,
            self.min_slack_floor,
            self.objective_excess,
        )
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn dual_digest(&self) -> &str {
        &self.dual_digest
    }

    pub fn status(&self) -> G27MwisFrontierClosureExactLeafStatus {
        self.status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisFrontierClosureExactNode {
    pub(super) index: usize,
    pub(super) parent_total: i128,
    pub(super) parent_depth: usize,
    pub(super) parent_digest: String,
    pub(super) first_branch: usize,
    pub(super) first_child_totals: [i128; 2],
    pub(super) worse_child: usize,
    pub(super) second_branch: usize,
    pub(super) terminal_totals: Vec<i128>,
    pub(super) certified_leaves: usize,
    pub(super) explicit_rows: usize,
    pub(super) positive_dual_rows: usize,
    pub(super) max_denominator: i128,
    pub(super) min_slack_floor: i128,
    pub(super) max_objective_excess: i128,
    pub(super) row_digest: String,
    pub(super) leaves: Vec<G27MwisFrontierClosureExactLeaf>,
    pub(super) status: G27MwisFrontierClosureExactStatus,
}

impl G27MwisFrontierClosureExactNode {
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

    pub fn exact_summary(&self) -> (usize, usize, usize, i128, i128, i128) {
        (
            self.certified_leaves,
            self.explicit_rows,
            self.positive_dual_rows,
            self.max_denominator,
            self.min_slack_floor,
            self.max_objective_excess,
        )
    }

    pub fn parent_digest(&self) -> &str {
        &self.parent_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn terminal_totals(&self) -> &[i128] {
        &self.terminal_totals
    }

    pub fn leaves(&self) -> &[G27MwisFrontierClosureExactLeaf] {
        &self.leaves
    }

    pub fn status(&self) -> G27MwisFrontierClosureExactStatus {
        self.status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisFrontierClosureExactReplayReport {
    pub(super) core: HadwigerArtifactCore,
    pub(super) scout_digest: String,
    pub(super) selected_start: usize,
    pub(super) selected_end: usize,
    pub(super) unresolved_start: usize,
    pub(super) unresolved_end: usize,
    pub(super) checked_nodes: usize,
    pub(super) certified_nodes: usize,
    pub(super) certified_leaves: usize,
    pub(super) explicit_rows: usize,
    pub(super) positive_dual_rows: usize,
    pub(super) max_denominator: i128,
    pub(super) min_slack_floor: i128,
    pub(super) max_objective_excess: i128,
    pub(super) worst_terminal_total: i128,
    pub(super) nodes: Vec<G27MwisFrontierClosureExactNode>,
    pub(super) status: G27MwisFrontierClosureExactStatus,
}

impl G27MwisFrontierClosureExactReplayReport {
    pub fn summary(&self) -> (usize, usize, usize, usize, usize, usize, usize, i128) {
        (
            self.selected_start,
            self.selected_end,
            self.checked_nodes,
            self.certified_nodes,
            self.certified_leaves,
            self.explicit_rows,
            self.positive_dual_rows,
            self.worst_terminal_total,
        )
    }

    pub fn exact_summary(&self) -> (i128, i128, i128) {
        (
            self.max_denominator,
            self.min_slack_floor,
            self.max_objective_excess,
        )
    }

    pub fn scout_digest(&self) -> &str {
        &self.scout_digest
    }

    pub fn unresolved_suffix(&self) -> (usize, usize) {
        (self.unresolved_start, self.unresolved_end)
    }

    pub fn nodes(&self) -> &[G27MwisFrontierClosureExactNode] {
        &self.nodes
    }

    pub fn status(&self) -> G27MwisFrontierClosureExactStatus {
        self.status
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27MwisFrontierClosureExactReplayReport, core);
