pub(super) const EXPECTED_FRONTIER_TOTALS: [i128; 28] = [
    518_612, 518_543, 518_471, 518_441, 518_343, 518_123, 518_105, 517_777, 517_741, 517_649,
    517_394, 517_365, 517_311, 517_204, 517_107, 516_592, 516_221, 516_140, 516_061, 515_994,
    515_851, 515_694, 515_464, 514_844, 514_775, 514_411, 513_989, 513_289,
];

pub(super) const EXPECTED_FRONTIER_DEPTHS: [usize; 28] = [
    5, 4, 3, 3, 5, 6, 6, 6, 6, 6, 6, 6, 3, 4, 6, 6, 6, 5, 5, 7, 5, 5, 6, 5, 5, 6, 7, 5,
];

pub(super) const H39_INDICES: [usize; 2] = [2, 3];
pub(super) const H40_INDICES: [usize; 2] = [4, 5];
pub(super) const H41_INDICES: [usize; 2] = [6, 7];
pub(super) const H42_INDICES: [usize; 2] = [8, 9];
pub(super) const H43_INDICES: [usize; 2] = [10, 11];

pub(super) const H39_REMAINING_BEST_TOTAL: i128 = 518_343;
pub(super) const H40_REMAINING_BEST_TOTAL: i128 = 518_105;
pub(super) const H41_REMAINING_BEST_TOTAL: i128 = 517_741;
pub(super) const H43_REMAINING_BEST_TOTAL: i128 = 517_311;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisLpGuidedTopPrefixStatus {
    TopBandPrefixExactProgress,
    RemainingFrontierPairExactProgress,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
    FrontierShapeMismatch,
    ParentIdentityMismatch,
    NonContiguousPrefix,
    RemainingBestMismatch,
    FloatingBoundAboveTarget,
    IncompleteLeafPartition,
    ExactReplayFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisLpGuidedTopPrefixNode {
    pub index: usize,
    pub parent_total: i128,
    pub parent_depth: usize,
    pub first_branch: usize,
    pub first_child_totals: [i128; 2],
    pub second_branch: usize,
    pub terminal_totals: Vec<i128>,
    pub certified_leaves: usize,
    pub explicit_rows: usize,
    pub positive_dual_rows: usize,
    pub max_denominator: i128,
    pub min_slack_floor: i128,
    pub max_objective_excess: i128,
    pub parent_digest: String,
    pub row_digest: String,
    pub status: G27MwisLpGuidedTopPrefixStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisLpGuidedTopPrefixReport {
    pub checked_nodes: usize,
    pub certified_nodes: usize,
    pub certified_leaves: usize,
    pub remaining_best_open_total: i128,
    pub nodes: Vec<G27MwisLpGuidedTopPrefixNode>,
    pub status: G27MwisLpGuidedTopPrefixStatus,
}
