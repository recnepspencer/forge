use std::sync::Arc;

use crate::authority::commit::phases::publication::{
    append_durable_commit, canonical_commit_envelope,
};
use crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan;
use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::history::data::CanonicalCommitAuthorityKind;
use crate::history::data::{BranchId, RelationalCommitReceipt};
use crate::lineage::authority::diagnostic_fields::metadata_promotion_summary_fields;
use crate::lineage::authority::phase_types::{
    ExecutionAuthorizedPromotionPlan, LoweredPromotionPlan,
};
use crate::lineage::authority::LineageAuthority;
use crate::lineage::data::{
    CorrespondenceCandidateId, CorrespondencePromotionExecutionFailureClass,
    LineageFinalizationArtifact,
};
use crate::publication::patch::data::{
    PatchOrdering, PatchPublicationMode, PatchStreamPosition, PublishedAuthoritativePatchEnvelope,
};
use crate::transactions::data::{MergedCommitPlan, TransactionId};

pub(crate) struct LineageDurableAppendAdmission {
    runtime_instance_id: u64,
    commit_id: crate::history::data::CommitId,
    branch_id: BranchId,
}

impl LineageDurableAppendAdmission {
    fn new(
        runtime: &crate::runtime::RelationalRuntime,
        commit_id: crate::history::data::CommitId,
        branch_id: &BranchId,
    ) -> Self {
        Self {
            runtime_instance_id: runtime.runtime_instance_id(),
            commit_id,
            branch_id: branch_id.clone(),
        }
    }

    pub(crate) fn into_parts(self) -> (u64, crate::history::data::CommitId, BranchId) {
        (self.runtime_instance_id, self.commit_id, self.branch_id)
    }
}

impl<'runtime> LineageAuthority<'runtime> {
    pub(super) fn authorize_promotion_execution(
        &self,
        plan: LoweredPromotionPlan,
    ) -> Result<ExecutionAuthorizedPromotionPlan, CorrespondencePromotionExecutionFailureClass>
    {
        let anchor_commit = plan.commit();
        let authoritative_anchor = self
            .runtime
            .history()
            .branch_head(&anchor_commit.branch_id)
            .cloned();
        if authoritative_anchor.as_ref().map(|head| head.commit_id) != Some(anchor_commit.commit_id)
        {
            return Err(CorrespondencePromotionExecutionFailureClass::AnchorDriftedFromBranchHead);
        }
        let Some(authoritative_anchor) = authoritative_anchor else {
            return Err(CorrespondencePromotionExecutionFailureClass::AnchorDriftedFromBranchHead);
        };
        Ok(ExecutionAuthorizedPromotionPlan {
            lowered: plan,
            authoritative_anchor,
        })
    }

    pub(super) fn publish_promotion_commit(
        &mut self,
        plan: &ExecutionAuthorizedPromotionPlan,
        artifact: &LineageFinalizationArtifact,
    ) -> Result<RelationalCommitReceipt, CorrespondencePromotionExecutionFailureClass> {
        let candidate_id = plan.candidate_id();
        let authoritative_anchor = plan.authoritative_anchor();

        let promotion_commit = RelationalCommitReceipt {
            commit_id: self.runtime.history().next_commit_id(),
            version_id: authoritative_anchor.version_id,
            branch_id: authoritative_anchor.branch_id.clone(),
            parents: vec![authoritative_anchor.commit_id],
        };
        let diagnostics_summary = promotion_diagnostics_summary(
            &promotion_commit.branch_id,
            promotion_commit.commit_id,
            candidate_id,
        );
        let envelope = canonical_commit_envelope(
            self.runtime,
            &promotion_commit,
            &promotion_commit.branch_id,
            CanonicalCommitAuthorityKind::MetadataOnlyLineage,
            None,
            None,
            &[],
            &[],
            &MergedCommitPlan {
                transaction_id: TransactionId(candidate_id.0),
                merged_intents: Vec::new(),
            },
            metadata_only_patch(self.runtime, promotion_commit.commit_id),
            diagnostics_summary.clone(),
            artifact.clone(),
            crate::indexes::data::DerivedIndexArtifacts::default(),
            &SchemaContinuityPlan::current(
                self.runtime
                    .config
                    .schema
                    .descriptor_semantics_policy
                    .current_write_version(),
            ),
        )
        .map_err(|_| CorrespondencePromotionExecutionFailureClass::AuthorityPublicationFailed)?;

        let append_authority = crate::durability::authority::DurableAppendAuthority::from_lineage(
            LineageDurableAppendAdmission::new(
                self.runtime,
                promotion_commit.commit_id,
                &promotion_commit.branch_id,
            ),
        );
        append_durable_commit(self.runtime, append_authority, &envelope).map_err(|_| {
            CorrespondencePromotionExecutionFailureClass::AuthorityPublicationFailed
        })?;

        let published_lineage = envelope.published_lineage().clone();
        let patch_position = envelope.patch.position;
        self.runtime
            .history_authority()
            .publish_metadata_artifact(
                promotion_commit.commit_id,
                promotion_commit.clone(),
                promotion_commit.branch_id.clone(),
                patch_position,
                Arc::new(envelope),
            )
            .map_err(|_| {
                CorrespondencePromotionExecutionFailureClass::AuthorityPublicationFailed
            })?;
        self.record_published_lineage_events(&published_lineage);
        self.runtime
            .publication_authority()
            .push_diagnostic_artifact(diagnostics_summary);
        self.runtime.durability_authority().compact_log_if_needed();
        Ok(promotion_commit)
    }
}

fn metadata_only_patch(
    _runtime: &crate::runtime::RelationalRuntime,
    commit_id: crate::history::data::CommitId,
) -> PublishedAuthoritativePatchEnvelope {
    PublishedAuthoritativePatchEnvelope {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(commit_id.0),
        authoritative_record_patches: Vec::new(),
    }
    .canonicalized()
}

fn promotion_diagnostics_summary(
    branch_id: &BranchId,
    commit_id: crate::history::data::CommitId,
    candidate_id: CorrespondenceCandidateId,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact::new(
        DiagnosticsScope::Lineage,
        DiagnosticsArtifactKind::MinimalSummary,
        DeterminismExpectation::Required,
        vec![RelationalDiagnosticsEntry::new(
            DiagnosticCode::LineagePromotionPublished,
            "lineage correspondence promotion published as a metadata-only commit",
            metadata_promotion_summary_fields(branch_id, commit_id, candidate_id),
        )],
    )
}
