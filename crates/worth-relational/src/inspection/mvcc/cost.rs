use crate::branch::RelationalBranchIdentity;
use crate::runtime::RelationalRuntime;

use super::sharing::{RelationalBranchSharingInspectionDenial, RelationalBranchSharingObservation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalMvccCostScope {
    branches: Vec<RelationalBranchIdentity>,
    baseline: crate::runtime::RelationalBranchSharingCostCounters,
    baseline_coordination_contacts: u64,
    baseline_global_population_scans: u64,
}

impl RelationalMvccCostScope {
    pub fn capture(runtime: &RelationalRuntime, branches: Vec<RelationalBranchIdentity>) -> Self {
        let branch_ids = branches
            .iter()
            .map(|identity| identity.branch_id().clone())
            .collect::<Vec<_>>();
        Self {
            branches,
            baseline: runtime.branch_sharing_cost_counters_for_branches(&branch_ids),
            baseline_global_population_scans: runtime.branch_population_scan_count(),
            baseline_coordination_contacts: branch_ids
                .iter()
                .filter_map(|branch_id| runtime.history.branch_cell(branch_id))
                .map(|cell| cell.coordination().contact_count())
                .sum(),
        }
    }

    pub fn branches(&self) -> &[RelationalBranchIdentity] {
        &self.branches
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalMvccCostObservation {
    sharing: RelationalBranchSharingObservation,
    branch_population_scans: u64,
    branch_cell_contacts: u64,
    sharing_cost_delta: crate::runtime::RelationalBranchSharingCostCounters,
}

impl RelationalMvccCostObservation {
    pub fn sharing(&self) -> &RelationalBranchSharingObservation {
        &self.sharing
    }
    pub const fn branch_population_scans(&self) -> u64 {
        self.branch_population_scans
    }
    pub const fn branch_cell_contacts(&self) -> u64 {
        self.branch_cell_contacts
    }

    pub const fn sharing_cost_delta(&self) -> crate::runtime::RelationalBranchSharingCostCounters {
        self.sharing_cost_delta
    }
}

impl RelationalRuntime {
    pub fn observe_mvcc_cost(
        &self,
        scope: &RelationalMvccCostScope,
    ) -> Result<RelationalMvccCostObservation, RelationalBranchSharingInspectionDenial> {
        let sharing = self.inspect_branch_sharing(scope.branches())?;
        let branch_cell_contacts = sharing
            .coordination_contacts()
            .saturating_sub(scope.baseline_coordination_contacts);
        let branch_ids = scope
            .branches
            .iter()
            .map(|identity| identity.branch_id().clone())
            .collect::<Vec<_>>();
        let current = self.branch_sharing_cost_counters_for_branches(&branch_ids);
        let baseline = scope.baseline;
        let branch_population_scans = self
            .branch_population_scan_count()
            .saturating_sub(scope.baseline_global_population_scans);
        Ok(RelationalMvccCostObservation {
            sharing,
            // Population traversal is a deliberately separate runtime-global
            // reconstruction lane. The selected branch counters below remain
            // owner-scoped; this field reports the explicit global delta.
            branch_population_scans,
            branch_cell_contacts,
            sharing_cost_delta: current.saturating_delta_since(baseline),
        })
    }
}
