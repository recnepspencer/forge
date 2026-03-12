use serde_json::json;

use crate::capabilities::{
    DiagnosticsSink, DurabilityWrite, PublicationPolicySource, SchemaSource,
    SchemaVersionSource,
};
use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope};
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::publication::data::diff::RelationalPatchRecord;
use crate::publication::data::{PublicationError, PublicationStage};
use crate::replay::data::CanonicalCommitEnvelope;
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
    runtime: &(impl SchemaSource + SchemaVersionSource),
    commit_reference: &CommitReference,
    branch_id: &BranchId,
    merge_parent_branches: Vec<BranchId>,
    merge_base_commits: Vec<CommitId>,
    merged_plan: &MergedCommitPlan,
    patch: crate::publication::data::diff::RelationalPatchRecord,
    diagnostics_summary: crate::diagnostics::data::RelationalDiagnosticArtifact,
    lineage_event_ids: Vec<u64>,
) -> CanonicalCommitEnvelope {
    CanonicalCommitEnvelope {
        commit: commit_reference.clone(),
        branch_context: branch_id.clone(),
        merge_parent_branches,
        merge_base_commits,
        schema_version: runtime.primary_schema_version_id(),
        schema_registry: runtime.schema_registry().clone(),
        merged_plan: merged_plan.clone(),
        patch,
        diagnostics_summary,
        lineage_event_ids,
        index_generation_ids: Vec::new(),
    }
}

pub(crate) fn append_durable_commit(
    runtime: &mut (impl DiagnosticsSink + DurabilityWrite),
    canonical_commit_envelope: &CanonicalCommitEnvelope,
    commit_id: CommitId,
    branch_id: &BranchId,
) -> Result<(), TransactionCommitError> {
    if let Err(error) = runtime.append_durable_envelope(canonical_commit_envelope.clone()) {
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
    records.sort_by(|left, right| {
        canonical_record_sort_key(left).cmp(&canonical_record_sort_key(right))
    });
    records.dedup();
}

fn canonical_record_sort_key(
    record: &RecordRef,
) -> (u8, crate::identity::data::PartitionId, u64, u32) {
    match record {
        RecordRef::Entity(entity_id) => (
            0,
            entity_id.partition_id,
            entity_id.local_slot.0,
            entity_id.generation.0,
        ),
        RecordRef::Relation(relation_id) => (
            1,
            relation_id.partition_id,
            relation_id.local_slot.0,
            relation_id.generation.0,
        ),
    }
}
