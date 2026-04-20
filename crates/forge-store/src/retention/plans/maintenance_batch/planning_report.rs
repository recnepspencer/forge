use serde::Serialize;

use super::{
    lowered_declarations::LoweredRetentionMaintenanceBatch,
    lowering::{
        build_lowered_batch, lower_authoritative_reclaim, lower_compaction_declaration,
        lower_rebuild_declaration, lower_reclaim_declaration,
    },
};
use crate::retention::plans::{
    CompactionCandidateRejection, CompactionPlan, ConservativeRetentionPlan, RebuildDebtSummary,
    RetainedAuthoritativeRange, RetentionClosureSummary,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionPlanningReport {
    closure_witness: crate::RetentionClosureWitness,
    conservative_plan: ConservativeRetentionPlan,
    retained_ranges: Vec<RetainedAuthoritativeRange>,
    expired_ranges: Vec<crate::PolicyExpiredAuthorityRange>,
    compaction_plans: Vec<CompactionPlan>,
    compaction_rejections: Vec<CompactionCandidateRejection>,
    reclaim_candidates: Vec<crate::ReclaimEligibilityWitness>,
    rebuild_debts: Vec<RebuildDebtSummary>,
    basis_survival_verdicts: Vec<crate::BasisSurvivalVerdict>,
}

impl RetentionPlanningReport {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        closure_witness: crate::RetentionClosureWitness,
        conservative_plan: ConservativeRetentionPlan,
        retained_ranges: Vec<RetainedAuthoritativeRange>,
        expired_ranges: Vec<crate::PolicyExpiredAuthorityRange>,
        compaction_plans: Vec<CompactionPlan>,
        compaction_rejections: Vec<CompactionCandidateRejection>,
        reclaim_candidates: Vec<crate::ReclaimEligibilityWitness>,
        rebuild_debts: Vec<RebuildDebtSummary>,
        basis_survival_verdicts: Vec<crate::BasisSurvivalVerdict>,
    ) -> Self {
        Self {
            closure_witness,
            conservative_plan,
            retained_ranges,
            expired_ranges,
            compaction_plans,
            compaction_rejections,
            reclaim_candidates,
            rebuild_debts,
            basis_survival_verdicts,
        }
    }
    pub fn closure_witness(&self) -> &crate::RetentionClosureWitness { &self.closure_witness }
    pub fn conservative_plan(&self) -> &ConservativeRetentionPlan { &self.conservative_plan }
    pub fn retained_ranges(&self) -> &[RetainedAuthoritativeRange] { &self.retained_ranges }
    pub fn expired_ranges(&self) -> &[crate::PolicyExpiredAuthorityRange] { &self.expired_ranges }
    pub fn compaction_plans(&self) -> &[CompactionPlan] { &self.compaction_plans }
    pub fn compaction_rejections(&self) -> &[CompactionCandidateRejection] { &self.compaction_rejections }
    pub fn reclaim_candidates(&self) -> &[crate::ReclaimEligibilityWitness] { &self.reclaim_candidates }
    pub fn rebuild_debts(&self) -> &[RebuildDebtSummary] { &self.rebuild_debts }
    pub fn basis_survival_verdicts(&self) -> &[crate::BasisSurvivalVerdict] { &self.basis_survival_verdicts }

    pub fn lower_to_maintenance_batch(&self) -> LoweredRetentionMaintenanceBatch {
        let compaction_declarations = self.compaction_plans.iter().map(lower_compaction_declaration).collect();
        let reclaim_declarations = self
            .reclaim_candidates
            .iter()
            .map(lower_reclaim_declaration)
            .chain(self.expired_ranges.iter().map(lower_authoritative_reclaim))
            .collect();
        let rebuild_declarations = self.rebuild_debts.iter().map(lower_rebuild_declaration).collect();
        build_lowered_batch(
            self.closure_witness(),
            self.closure_summary(),
            compaction_declarations,
            reclaim_declarations,
            rebuild_declarations,
        )
    }

    pub fn closure_summary(&self) -> RetentionClosureSummary {
        RetentionClosureSummary::from_witness(&self.closure_witness)
    }
}
