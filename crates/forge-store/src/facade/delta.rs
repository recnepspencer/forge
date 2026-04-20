use crate::{
    authority::{FetchedLineageSupportArtifact, FetchedSchemaBoundaryArtifact, FetchedSchemaSupportArtifact, HistoricalIdentityRequest, HistoricalIdentityResolution},
    delta::{BranchDeltaAutoCompactOutcome, BranchDeltaRebuildReceipt, BranchDeltaRewritePlan, BranchDeltaRewriteReceipt, BranchDeltaRewriteRecommendation, BranchDeltaRewriteRequest},
    failure::StoreError,
};
use forge_relational::facade::history::{BranchId, CommitId};

use super::ForgeStore;

impl ForgeStore {
    pub fn plan_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewritePlan, StoreError> {
        self.backend.plan_delta_rewrite(request)
    }

    pub fn recommend_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewriteRecommendation, StoreError> {
        self.backend.recommend_delta_rewrite(request)
    }

    pub fn auto_compact_branch_delta(
        &mut self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaAutoCompactOutcome, StoreError> {
        self.backend.auto_compact_branch_delta(request)
    }

    pub fn rewrite_branch_delta(
        &mut self,
        plan: BranchDeltaRewritePlan,
    ) -> Result<BranchDeltaRewriteReceipt, StoreError> {
        self.backend.rewrite_branch_delta(plan)
    }

    pub fn rebuild_branch_delta_artifacts(
        &mut self,
        branch_id: BranchId,
    ) -> Result<BranchDeltaRebuildReceipt, StoreError> {
        self.backend.rebuild_branch_delta_artifacts(branch_id)
    }

    pub fn fetch_schema_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedSchemaSupportArtifact, StoreError> {
        self.backend.fetch_schema_support(commit_id)
    }

    pub fn fetch_schema_boundary(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedSchemaBoundaryArtifact, StoreError> {
        self.backend.fetch_schema_support(commit_id)
    }

    pub fn fetch_lineage_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedLineageSupportArtifact, StoreError> {
        self.backend.fetch_lineage_support(commit_id)
    }

    pub fn fetch_lineage_history(
        &self,
        request: HistoricalIdentityRequest,
    ) -> Result<HistoricalIdentityResolution, StoreError> {
        self.backend.fetch_lineage_history(request)
    }
}
