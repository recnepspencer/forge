use std::sync::Arc;

use serde_json::json;

use crate::authority::commit::phases::publication::{
    append_durable_commit, canonical_commit_envelope,
};
use crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan;
use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitReference};
use crate::lineage::data::{
    CorrespondenceCandidateId, CorrespondencePromotionExecutionFailureClass,
    LineageFinalizationArtifact,
};
use crate::lineage::logic::authority::phase_types::{
    ExecutionAuthorizedPromotionPlan, LoweredPromotionPlan,
};
use crate::lineage::logic::authority::LineageAuthority;
use crate::publication::data::diff::RelationalPatchRecord;
use crate::publication::patch::data::{
    PatchCompatibilityClass, PatchOrdering, PatchPublicationMode, PatchStreamPosition,
};
use crate::replay::data::CanonicalCommitAuthorityKind;
use crate::transactions::data::{MergedCommitPlan, TransactionId};

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
    ) -> Result<CommitReference, CorrespondencePromotionExecutionFailureClass> {
        let candidate_id = plan.candidate_id();
        let authoritative_anchor = plan.authoritative_anchor();

        let promotion_commit = CommitReference {
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
            &[],
            &[],
            &MergedCommitPlan {
                transaction_id: TransactionId(candidate_id.0),
                merged_intents: Vec::new(),
            },
            metadata_only_patch(self.runtime, promotion_commit.commit_id),
            diagnostics_summary.clone(),
            artifact.clone(),
            Vec::new(),
            Vec::new(),
            &SchemaContinuityPlan::current(
                self.runtime
                    .config
                    .schema
                    .descriptor_semantics_policy
                    .current_write_version(),
            ),
        )
        .map_err(|_| CorrespondencePromotionExecutionFailureClass::AuthorityPublicationFailed)?;

        append_durable_commit(
            self.runtime,
            &envelope,
            promotion_commit.commit_id,
            &promotion_commit.branch_id,
        )
        .map_err(|_| CorrespondencePromotionExecutionFailureClass::AuthorityPublicationFailed)?;

        let published_lineage = envelope.published_lineage().clone();
        let patch_position = envelope.patch.position;
        self.runtime
            .history_authority()
            .publish_metadata_only_commit(
                promotion_commit.commit_id,
                promotion_commit.clone(),
                promotion_commit.branch_id.clone(),
                patch_position,
                Arc::new(envelope),
            );
        self.record_published_lineage_events(&published_lineage);
        self.runtime
            .publication_authority()
            .push_diagnostic_artifact(diagnostics_summary);
        self.runtime.durability_authority().compact_log_if_needed();
        Ok(promotion_commit)
    }
}

fn metadata_only_patch(
    runtime: &crate::logic::runtime::RelationalRuntime,
    commit_id: crate::history::data::CommitId,
) -> RelationalPatchRecord {
    RelationalPatchRecord {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(commit_id.0),
        compatibility: match runtime.config.publication.policy.patch_surface_policy {
            crate::config::data::PatchSurfacePolicy::StructuredPatchSurface => {
                PatchCompatibilityClass::StructuredCompatible
            }
            crate::config::data::PatchSurfacePolicy::DensePatchSurface => {
                PatchCompatibilityClass::DenseCompatible
            }
        },
        records: Vec::new(),
    }
    .canonicalized()
}

fn promotion_diagnostics_summary(
    branch_id: &BranchId,
    commit_id: crate::history::data::CommitId,
    candidate_id: CorrespondenceCandidateId,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Lineage,
        kind: DiagnosticsArtifactKind::MinimalSummary,
        determinism: DeterminismExpectation::Required,
        entries: vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::LineagePromotionPublished,
            message: "lineage correspondence promotion published as a metadata-only commit"
                .to_string(),
            fields: json!({
                "branch_id": branch_id.0,
                "commit_id": commit_id.0,
                "candidate_id": candidate_id.0,
            }),
        }],
    }
}
