use serde_json::json;

use crate::capabilities::{
    DiagnosticsSink, DurabilityWrite, PublicationPolicySource, SchemaSource, SchemaVersionSource,
};
use crate::authority::commit::phases::schema_continuity::validate_schema_continuity_publication;
use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope};
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::indexes::data::DerivedIndexGeneration;
use crate::lineage::data::LineageFinalizationArtifact;
use crate::publication::data::diff::RelationalPatchRecord;
use crate::publication::data::{PublicationError, PublicationStage};
use crate::replay::data::{CanonicalCommitAuthorityKind, CanonicalCommitEnvelope};
use crate::transactions::data::{MergedCommitPlan, RecordRef, TransactionCommitError};

pub(crate) fn enforce_patch_budget(
    runtime: &mut (impl DiagnosticsSink + PublicationPolicySource),
    patch: &RelationalPatchRecord,
) -> Result<(), TransactionCommitError> {
    let max_patch_records_per_commit = runtime.max_patch_records_per_commit();
    if patch.records.len() > max_patch_records_per_commit {
        runtime.emit_diagnostic_entry(
            DiagnosticsScope::PatchPublication,
            DiagnosticCode::DiagnosticsPublicationFailure,
            "patch record budget exceeded",
            json!({
                "patch_records": patch.records.len(),
                "max_patch_records_per_commit": max_patch_records_per_commit,
            }),
        );
        return Err(TransactionCommitError::publication(PublicationError::new(
            PublicationStage::BundleAssembly,
            "patch record budget exceeded",
        )));
    }
    Ok(())
}

pub(crate) fn canonical_commit_envelope(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    commit_reference: &CommitReference,
    branch_id: &BranchId,
    authority_kind: CanonicalCommitAuthorityKind,
    merge_parent_branches: &[BranchId],
    merge_base_commits: &[CommitId],
    merged_plan: &MergedCommitPlan,
    patch: crate::publication::data::diff::RelationalPatchRecord,
    diagnostics_summary: crate::diagnostics::data::RelationalDiagnosticArtifact,
    lineage_artifact: LineageFinalizationArtifact,
    index_generation_ids: Vec<u64>,
    index_generations: Vec<DerivedIndexGeneration>,
    schema_continuity: &crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan,
) -> Result<CanonicalCommitEnvelope, TransactionCommitError> {
    let published_lineage = lineage_artifact.publish();
    runtime.performance_access().count_lineage_publication_artifact(
        published_lineage.lineage_events().len(),
        published_lineage.lineage_decision_log().len(),
    );
    if published_lineage.branch_id() != branch_id || lineage_artifact.branch_id() != branch_id {
        runtime.emit_diagnostic_entry(
            DiagnosticsScope::Lineage,
            DiagnosticCode::DiagnosticsPublicationFailure,
            "lineage artifact branch scope did not match publication branch",
            json!({
                "publication_branch_id": branch_id.0,
                "artifact_branch_id": lineage_artifact.branch_id().0,
            }),
        );
        return Err(TransactionCommitError::publication(PublicationError::new(
            PublicationStage::BundleAssembly,
            "lineage artifact branch scope mismatch",
        )));
    }
    let envelope = CanonicalCommitEnvelope::new(
        commit_reference.clone(),
        published_lineage.branch_id().clone(),
        authority_kind,
        merge_parent_branches.to_vec(),
        merge_base_commits.to_vec(),
        runtime.primary_schema_version_id(),
        runtime.schema_registry().clone(),
        merged_plan.clone(),
        patch,
        diagnostics_summary,
        index_generation_ids,
        published_lineage,
        index_generations,
        schema_continuity.schema_transition.clone(),
        schema_continuity.schema_continuation_descriptor.clone(),
        schema_continuity.schema_reconciliation_descriptor.clone(),
        schema_continuity.descriptor_semantics_version,
    );
    validate_schema_continuity_publication(runtime, branch_id, schema_continuity, &envelope)?;
    Ok(envelope)
}

pub(crate) fn append_durable_commit(
    runtime: &mut (impl DiagnosticsSink + DurabilityWrite),
    canonical_commit_envelope: &CanonicalCommitEnvelope,
    commit_id: CommitId,
    branch_id: &BranchId,
) -> Result<(), TransactionCommitError> {
    if let Err(error) = runtime.append_durable_envelope(canonical_commit_envelope) {
        runtime.emit_diagnostic_entry(
            DiagnosticsScope::History,
            DiagnosticCode::DurableAppendFailed,
            error.detail.clone(),
            json!({
                "commit_id": commit_id.0,
                "branch_id": branch_id.0,
            }),
        );
        return Err(TransactionCommitError::publication(PublicationError::new(
            PublicationStage::Visibility,
            error.detail,
        )));
    }
    Ok(())
}

pub(crate) fn canonicalize_changed_records(records: &mut Vec<RecordRef>) {
    records.sort_unstable();
    records.dedup();
}
