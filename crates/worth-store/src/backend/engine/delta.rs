use crate::delta::{
    BranchDeltaAutoCompactDisposition, BranchDeltaAutoCompactOutcome, BranchDeltaFallbackClass,
    BranchDeltaReadPlan, BranchDeltaReadRequest, BranchDeltaReadResult, BranchDeltaReadStrategy,
    BranchDeltaRebuildReceipt, BranchDeltaRewritePlan, BranchDeltaRewriteReceipt,
    BranchDeltaRewriteRecommendation, BranchDeltaRewriteRequest, BranchDeltaRewriteStrategy,
    SameBranchDescendantWitness,
};
use crate::failure::{StoreError, StoreErrorKind};
use worth_relational::facade::history::{BranchId, CommitId};

use super::{core::verify_durable_barrier, StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn plan_branch_delta_read(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<BranchDeltaReadPlan, StoreError> {
        match self.state.plan_branch_delta_read(request) {
            Ok(plan) => {
                self.counters.record_branch_delta_read(
                    plan.performance.layers_traversed,
                    plan.performance.records_decoded.max(plan.commit_ids.len()),
                    plan.performance.replay_commit_count,
                    matches!(
                        plan.performance.fallback_class,
                        BranchDeltaFallbackClass::RequiresAuthorityReplayControlLane
                    ),
                );
                Ok(plan)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::BranchDeltaTargetRequiresMergeAwareWidening
                ) {
                    self.counters.record_branch_delta_merge_path_search();
                }
                if matches!(
                    error.kind(),
                    StoreErrorKind::BranchDeltaDigestMismatch
                        | StoreErrorKind::BranchDeltaPublicationGap
                        | StoreErrorKind::BranchDeltaIntegrityFailure
                ) {
                    self.counters.record_branch_delta_integrity_failure();
                }
                Err(error)
            }
        }
    }

    pub fn admit_same_branch_descendant(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<SameBranchDescendantWitness, StoreError> {
        self.state.admit_same_branch_descendant(request)
    }

    pub fn admit_milestone_7_independent_reference(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<crate::Milestone7IndependentReference, StoreError> {
        self.state.admit_milestone_7_independent_reference(request)
    }

    pub fn read_branch_delta(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let result = self.state.read_branch_delta(witness)?;
        record_branch_delta_read_metrics(&self.counters, &result);
        Ok(result)
    }

    pub fn read_branch_delta_control(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let result = self.state.read_branch_delta_control(witness)?;
        record_branch_delta_read_metrics(&self.counters, &result);
        Ok(result)
    }

    pub fn read_branch_delta_control_from_milestone_7_reference(
        &self,
        reference: crate::Milestone7IndependentReference,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let result = self
            .state
            .read_branch_delta_control_from_milestone_7_reference(reference)?;
        record_branch_delta_read_metrics(&self.counters, &result);
        Ok(result)
    }

    pub(crate) fn milestone_5_delta_storage_report(
        &self,
        branch_id: BranchId,
        target_commit_id: CommitId,
        direct_plan: &BranchDeltaReadPlan,
        control_plan: &BranchDeltaReadPlan,
    ) -> Result<crate::Milestone5DeltaStorageReport, StoreError> {
        self.state.milestone_5_delta_storage_report(
            branch_id,
            target_commit_id,
            direct_plan,
            control_plan,
        )
    }

    pub fn plan_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewritePlan, StoreError> {
        self.state.plan_delta_rewrite(request)
    }

    pub fn recommend_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewriteRecommendation, StoreError> {
        self.state.recommend_delta_rewrite(request)
    }

    pub fn auto_compact_branch_delta(
        &mut self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaAutoCompactOutcome, StoreError> {
        let recommendation = self.state.recommend_delta_rewrite(request.clone())?;
        match recommendation.decision {
            crate::BranchDeltaRewritePolicyDecision::CompactNow => {
                let rewrite_receipt = self.rewrite_branch_delta(recommendation.plan.clone())?;
                Ok(BranchDeltaAutoCompactOutcome {
                    disposition: BranchDeltaAutoCompactDisposition::Compacted,
                    recommendation,
                    rewrite_receipt: Some(rewrite_receipt),
                })
            }
            crate::BranchDeltaRewritePolicyDecision::NoAction => {
                Ok(BranchDeltaAutoCompactOutcome {
                    disposition: BranchDeltaAutoCompactDisposition::NoAction,
                    recommendation,
                    rewrite_receipt: None,
                })
            }
            crate::BranchDeltaRewritePolicyDecision::Defer => Ok(BranchDeltaAutoCompactOutcome {
                disposition: BranchDeltaAutoCompactDisposition::Deferred,
                recommendation,
                rewrite_receipt: None,
            }),
            crate::BranchDeltaRewritePolicyDecision::RejectAsTooBroad => {
                Ok(BranchDeltaAutoCompactOutcome {
                    disposition: BranchDeltaAutoCompactDisposition::RejectedAsTooBroad,
                    recommendation,
                    rewrite_receipt: None,
                })
            }
        }
    }

    pub fn rewrite_branch_delta(
        &mut self,
        plan: BranchDeltaRewritePlan,
    ) -> Result<BranchDeltaRewriteReceipt, StoreError> {
        if !matches!(
            plan.strategy(),
            BranchDeltaRewriteStrategy::ReplaceContiguousSegment
        ) {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaRewriteTargetIllegal,
                "branch delta rewrite execution requires an admitted rewrite plan",
            ));
        }
        let rewrite_record_count = plan
            .segment()
            .map(|segment| segment.commit_ids().len())
            .unwrap_or(0);
        let replaced_layer_count = plan
            .segment()
            .map(|segment| segment.layer_ids().len())
            .unwrap_or(0);
        let (applied, receipt) = self.state.apply_delta_rewrite_plan_in_place(plan)?;
        if let Err(error) = self.state.verify_applied_delta_rewrite(&applied) {
            self.state.rollback_delta_rewrite(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_delta_rewrite(applied);
                return Err(error);
            }
        };
        if let Err(error) = verify_durable_barrier(&mut self.counters, &report) {
            self.state.rollback_delta_rewrite(applied);
            return Err(error);
        }
        self.counters
            .record_state_delta_apply(1, (replaced_layer_count + 1) as u64);
        self.counters.record_branch_delta_rewrite(
            replaced_layer_count,
            rewrite_record_count,
            false,
        );
        Ok(receipt)
    }

    pub fn rebuild_branch_delta_artifacts(
        &mut self,
        branch_id: BranchId,
    ) -> Result<BranchDeltaRebuildReceipt, StoreError> {
        let (applied, receipt) = self.state.apply_branch_delta_rebuild_in_place(branch_id)?;
        if let Err(error) = self.state.verify_applied_branch_delta_rebuild(&applied) {
            self.state.rollback_branch_delta_rebuild(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_branch_delta_rebuild(applied);
                return Err(error);
            }
        };
        if let Err(error) = verify_durable_barrier(&mut self.counters, &report) {
            self.state.rollback_branch_delta_rebuild(applied);
            return Err(error);
        }
        self.counters
            .record_state_delta_apply(1, receipt.rebuilt_layer_count as u64);
        self.counters
            .record_branch_delta_rebuild(receipt.rebuilt_layer_count);
        Ok(receipt)
    }
}

fn record_branch_delta_read_metrics(
    counters: &crate::evidence::StoreCounters,
    result: &BranchDeltaReadResult,
) {
    counters.record_branch_delta_read(
        result.plan.performance.layers_traversed,
        branch_delta_result_record_count(result),
        result.plan.performance.replay_commit_count,
        matches!(
            result.plan.strategy,
            BranchDeltaReadStrategy::AuthorityReplayControl
        ) || matches!(
            result.plan.performance.fallback_class,
            BranchDeltaFallbackClass::RequiresAuthorityReplayControlLane
        ),
    );
}

fn branch_delta_result_record_count(result: &BranchDeltaReadResult) -> usize {
    let export = result.authoritative_export();
    export.commit_envelopes.len()
        + export.commit_parent_records.len()
        + export.commit_support_summaries.len()
        + export.schema_support_records.len()
        + export.lineage_support_records.len()
        + export.durable_cursor_identity_records.len()
        + export.subscriber_checkpoint_records.len()
        + export.branch_records.len()
        + export.branch_head_records.len()
        + export.authoritative_artifact_digests.len()
}
