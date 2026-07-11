use crate::{StoreError, StoreErrorKind};
use forge_relational::facade::history::BranchId;
pub use forge_store_contracts::DerivedFamilyRetentionPolicy;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RetentionPolicyClass {
    Conservative(ConservativeRetentionPolicy),
    AggressiveDebt(AggressiveRetentionDebtMarker),
}

impl RetentionPolicyClass {
    pub fn require_conservative(&self) -> Result<&ConservativeRetentionPolicy, StoreError> {
        match self {
            Self::Conservative(policy) => Ok(policy),
            Self::AggressiveDebt(marker) => Err(StoreError::new(
                StoreErrorKind::RetentionPolicyUnsupported,
                format!(
                    "aggressive retention policy `{}` is explicit debt in milestone 10 phase 1",
                    marker.label()
                ),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BranchHistoryWindowPolicy {
    branch_id: BranchId,
    minimum_retained_commit_count: u64,
}

impl BranchHistoryWindowPolicy {
    pub fn new(
        branch_id: BranchId,
        minimum_retained_commit_count: u64,
    ) -> Result<Self, StoreError> {
        if minimum_retained_commit_count == 0 {
            return Err(StoreError::new(
                StoreErrorKind::RetentionPolicyUnsupported,
                format!(
                    "branch history window for `{}` must retain at least one commit",
                    branch_id.0
                ),
            ));
        }
        Ok(Self {
            branch_id,
            minimum_retained_commit_count,
        })
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn minimum_retained_commit_count(&self) -> u64 {
        self.minimum_retained_commit_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct PinnedSnapshotPolicy {
    snapshot_id: crate::SnapshotId,
}

impl PinnedSnapshotPolicy {
    pub fn new(snapshot_id: crate::SnapshotId) -> Self {
        Self { snapshot_id }
    }

    pub fn snapshot_id(&self) -> crate::SnapshotId {
        self.snapshot_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConservativeRetentionPolicy {
    retain_current_branch_heads: bool,
    branch_history_windows: Vec<BranchHistoryWindowPolicy>,
    pinned_snapshots: Vec<PinnedSnapshotPolicy>,
    reclaimable_derived_families: Vec<DerivedFamilyRetentionPolicy>,
}

impl ConservativeRetentionPolicy {
    pub fn new(
        mut branch_history_windows: Vec<BranchHistoryWindowPolicy>,
        mut pinned_snapshots: Vec<PinnedSnapshotPolicy>,
        mut reclaimable_derived_families: Vec<DerivedFamilyRetentionPolicy>,
    ) -> Self {
        branch_history_windows.sort();
        branch_history_windows.dedup();
        pinned_snapshots.sort();
        pinned_snapshots.dedup();
        reclaimable_derived_families.sort();
        reclaimable_derived_families.dedup();

        Self {
            retain_current_branch_heads: true,
            branch_history_windows,
            pinned_snapshots,
            reclaimable_derived_families,
        }
    }

    pub fn retain_current_branch_heads(&self) -> bool {
        self.retain_current_branch_heads
    }

    pub fn branch_history_windows(&self) -> &[BranchHistoryWindowPolicy] {
        &self.branch_history_windows
    }

    pub fn pinned_snapshots(&self) -> &[PinnedSnapshotPolicy] {
        &self.pinned_snapshots
    }

    pub fn reclaimable_derived_families(&self) -> &[DerivedFamilyRetentionPolicy] {
        &self.reclaimable_derived_families
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AggressiveRetentionDebtMarker {
    CrossBranchGlobalHistoryThinning,
    SelectiveContinuationSupportReclaim,
    PressureReactivePolicySwitching,
}

impl AggressiveRetentionDebtMarker {
    pub fn label(self) -> &'static str {
        match self {
            Self::CrossBranchGlobalHistoryThinning => "cross_branch_global_history_thinning",
            Self::SelectiveContinuationSupportReclaim => "selective_continuation_support_reclaim",
            Self::PressureReactivePolicySwitching => "pressure_reactive_policy_switching",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conservative_policy_normalizes_duplicates() {
        let branch = BranchId("main".to_string());
        let snapshot = crate::SnapshotId(7);
        let policy = ConservativeRetentionPolicy::new(
            vec![
                BranchHistoryWindowPolicy::new(branch.clone(), 5).unwrap(),
                BranchHistoryWindowPolicy::new(branch, 5).unwrap(),
            ],
            vec![
                PinnedSnapshotPolicy::new(snapshot),
                PinnedSnapshotPolicy::new(snapshot),
            ],
            vec![
                DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization,
                DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization,
            ],
        );

        assert!(policy.retain_current_branch_heads());
        assert_eq!(policy.branch_history_windows().len(), 1);
        assert_eq!(policy.pinned_snapshots().len(), 1);
        assert_eq!(policy.reclaimable_derived_families().len(), 1);
    }

    #[test]
    fn branch_window_rejects_zero_minimum_retained_commits() {
        let error = BranchHistoryWindowPolicy::new(BranchId("main".to_string()), 0).unwrap_err();
        assert_eq!(error.kind(), &StoreErrorKind::RetentionPolicyUnsupported);
    }

    #[test]
    fn aggressive_marker_is_not_plannable_as_conservative() {
        let policy = RetentionPolicyClass::AggressiveDebt(
            AggressiveRetentionDebtMarker::PressureReactivePolicySwitching,
        );

        let error = policy.require_conservative().unwrap_err();
        assert_eq!(error.kind(), &StoreErrorKind::RetentionPolicyUnsupported);
    }
}
