use super::*;

impl CompatibilityAdmissionCounters {
    pub(crate) fn record_derived_reuse_incompatible(&mut self) {
        self.derived_reuse_incompatibility_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_rebuild_required(&mut self) {
        self.derived_rebuild_required_count += 1;
    }

    pub(crate) fn record_derived_rebuild_incompatible(&mut self) {
        self.derived_rebuild_incompatibility_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_invalidation(&mut self) {
        self.derived_invalidation_count += 1;
    }

    pub(crate) fn record_derived_stale_version_rejection(&mut self) {
        self.derived_stale_version_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_rebuild_debt(&mut self, debt_record_count: u64) {
        self.derived_rebuild_debt_count += debt_record_count;
    }

    pub(crate) fn record_maintenance_compatibility_rebuild_admission(&mut self) {
        self.maintenance_compatibility_rebuild_admission_count += 1;
    }

    pub(crate) fn record_maintenance_compatibility_rebuild_rejection(&mut self) {
        self.maintenance_compatibility_rebuild_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_lane_plan(&mut self) {
        self.derived_lane_plan_count += 1;
    }

    pub(crate) fn record_derived_lane_reuse(&mut self) {
        self.derived_lane_reuse_count += 1;
    }

    pub(crate) fn record_derived_lane_invalidation(&mut self) {
        self.derived_lane_invalidation_count += 1;
    }

    pub(crate) fn record_derived_lane_rejection(&mut self) {
        self.derived_lane_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_snapshot_reuse(&mut self) {
        self.derived_snapshot_reuse_count += 1;
    }

    pub(crate) fn record_derived_delta_reuse(&mut self) {
        self.derived_delta_reuse_count += 1;
    }

    pub(crate) fn record_derived_layout_basis_rejection(&mut self) {
        self.derived_layout_basis_rejection_count += 1;
        self.record_derived_lane_rejection();
    }

    pub(crate) fn record_derived_bulk_resume_rejection(&mut self) {
        self.derived_bulk_resume_rejection_count += 1;
        self.record_derived_lane_rejection();
    }

    pub(crate) fn record_derived_maintenance_summary_rebuild(&mut self) {
        self.derived_maintenance_summary_rebuild_count += 1;
    }

    pub(crate) fn record_tier_non_authority_preserved(&mut self) {
        self.tier_non_authority_preserved_count += 1;
    }

    pub(crate) fn record_tier_manifest_rejection(&mut self) {
        self.tier_manifest_rejection_count += 1;
        self.record_derived_lane_rejection();
    }

    pub(crate) fn record_maintenance_lane_mismatch_rejection(&mut self) {
        self.maintenance_lane_mismatch_rejection_count += 1;
        self.rejected_count += 1;
    }

}
