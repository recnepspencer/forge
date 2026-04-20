mod compaction;
mod maintenance_batch;
mod planning;
mod verification;

pub use compaction::{
    CompactionCutoverReport, CompactionPlan, CompactionPublicationReport,
    PublishedCompactionProduct, SupersededPhysicalFamily,
};
pub use maintenance_batch::{
    LoweredCompactionDeclaration, LoweredRebuildDeclaration, LoweredReclaimDeclaration,
    LoweredRetentionMaintenanceBatch, RetentionPlanningReport,
};
pub use planning::{
    CompactionBackedRetentionPlan, ConservativeRetentionPlan, RebuildRequiredRetentionPlan,
    RetainedAuthoritativeRange, RetentionCandidatePlan, RetentionClosureSummary,
};
pub use verification::{
    AuthoritativeReclaimReport, CompactionCandidateRejection, RebuildDebtSummary,
    ReclaimExecutionReport, RetainedRangeRebuildReport, RetentionMaintenanceVerification,
    RetentionTargetStateVerification,
};

#[cfg(test)]
mod tests {
    use super::RetentionClosureSummary;
    use forge_relational::facade::history::{BranchId, CommitId};

    #[test]
    fn closure_summary_derives_from_synthetic_witness() {
        let witness = crate::RetentionClosureWitness::new(
            crate::RetainedHeadSet::new(vec![
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
            ]),
            crate::StableBasisSet::new(vec!["basis-a".to_string()]),
            vec![CommitId(1), CommitId(2), CommitId(3)],
            vec![CommitId(3), CommitId(2)],
        );

        let summary = RetentionClosureSummary::from_witness(&witness);
        assert_eq!(summary.retained_head_count(), 2);
        assert_eq!(summary.stable_basis_count(), 1);
        assert_eq!(summary.closure_commit_count(), 3);
        assert_eq!(summary.closure_frontier_count(), 2);
    }
}
