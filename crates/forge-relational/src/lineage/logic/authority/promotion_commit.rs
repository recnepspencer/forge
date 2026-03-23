use std::sync::Arc;

use serde_json::json;

use crate::authority::commit::phases::publication::{
    append_durable_commit, canonical_commit_envelope,
};
use crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, DeterminismExpectation,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitReference};
use crate::lineage::data::{
    CorrespondenceCandidateId, CorrespondencePromotionRejectionClass, LineageFinalizationArtifact,
};
use crate::lineage::logic::authority::LineageAuthority;
use crate::publication::data::diff::RelationalPatchRecord;
use crate::publication::patch::data::{
    PatchCompatibilityClass, PatchOrdering, PatchPublicationMode, PatchStreamPosition,
};
use crate::replay::data::CanonicalCommitAuthorityKind;
use crate::transactions::data::{MergedCommitPlan, TransactionId};

impl<'runtime> LineageAuthority<'runtime> {
    pub(super) fn publish_promotion_commit(
        &mut self,
        anchor_commit: &CommitReference,
        candidate_id: CorrespondenceCandidateId,
        artifact: &LineageFinalizationArtifact,
    ) -> Result<CommitReference, CorrespondencePromotionRejectionClass> {
        let authoritative_anchor = self
            .runtime
            .history_access()
            .branch_head(&anchor_commit.branch_id)
            .cloned();
        if authoritative_anchor
            .as_ref()
            .map(|head| head.commit_id)
            != Some(anchor_commit.commit_id)
        {
            self.record_rejected_promotion_for_candidate(
                None,
                &anchor_commit.branch_id,
                candidate_id,
                CorrespondencePromotionRejectionClass::CommitNotBranchHead,
                "correspondence promotion must publish from the current branch head",
            );
            return Err(CorrespondencePromotionRejectionClass::CommitNotBranchHead);
        }
        let authoritative_anchor = authoritative_anchor
            .expect("validated branch head anchor must resolve to an authoritative commit reference");

        let promotion_commit = CommitReference {
            commit_id: self.runtime.history_access().next_commit_id(),
            version_id: authoritative_anchor.version_id,
            branch_id: authoritative_anchor.branch_id.clone(),
            parents: vec![authoritative_anchor.commit_id],
        };
        let diagnostics_summary =
            promotion_diagnostics_summary(&promotion_commit.branch_id, promotion_commit.commit_id, candidate_id);
        let envelope = canonical_commit_envelope(
            self.runtime,
            &promotion_commit,
            &promotion_commit.branch_id,
            CanonicalCommitAuthorityKind::MetadataOnlyLineage,
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
        .map_err(|_| {
            self.record_rejected_promotion_for_candidate(
                None,
                &authoritative_anchor.branch_id,
                candidate_id,
                CorrespondencePromotionRejectionClass::AuthorityPublicationFailed,
                "correspondence promotion could not assemble a canonical promotion commit envelope",
            );
            CorrespondencePromotionRejectionClass::AuthorityPublicationFailed
        })?;

        append_durable_commit(
            self.runtime,
            &envelope,
            promotion_commit.commit_id,
            &promotion_commit.branch_id,
        )
        .map_err(|_| {
            self.record_rejected_promotion_for_candidate(
                None,
                &authoritative_anchor.branch_id,
                candidate_id,
                CorrespondencePromotionRejectionClass::AuthorityPublicationFailed,
                "correspondence promotion could not append its canonical envelope durably",
            );
            CorrespondencePromotionRejectionClass::AuthorityPublicationFailed
        })?;

        let published_lineage = envelope.published_lineage().clone();
        let patch_position = envelope.patch.position;
        self.runtime.history_authority().publish_metadata_only_commit(
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
            code: DiagnosticCode::CommitPublished,
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
