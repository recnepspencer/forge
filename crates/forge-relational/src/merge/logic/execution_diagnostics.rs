use serde_json::json;

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::merge::data::{
    ExecutedMergeAspectDiagnosticRow, ExecutedMergeAspectClass, ExecutedMergeRecordClass,
    ExecutedMergeRecordDiagnosticRow, MergeExecutionDiagnosticsPlan, PreparedMergeExecution,
};
use crate::transactions::data::{MergeExecutionStructuralSummary, MergeExecutionSummary};

pub(crate) fn merge_execution_summary_entry(
    summary: &MergeExecutionSummary,
    structural_summary: &MergeExecutionStructuralSummary,
    commit_id: crate::history::data::CommitId,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: DiagnosticCode::MergeExecutionPublished,
        message: "prepared merge execution published through the authoritative commit lifecycle"
            .to_string(),
        fields: json!({
            "target_branch": summary.request.target_branch.0.clone(),
            "source_branch": summary.request.source_branch.0.clone(),
            "merge_intent": format!("{:?}", summary.request.merge_intent),
            "target_head_commit_id": summary.target_head_commit_id.0.clone(),
            "source_head_commit_id": summary.source_head_commit_id.0.clone(),
            "merge_base_commit_id": summary.merge_base_commit_id.0.clone(),
            "commit_id": commit_id.0,
            "executed_record_count": structural_summary.executed_record_count,
            "adopted_source_record_count": structural_summary.adopted_source_record_count,
            "preserved_shared_record_count": structural_summary.preserved_shared_record_count,
            "reconciled_record_count": structural_summary.reconciled_record_count,
            "converged_deleted_on_both_sides_count": structural_summary.converged_deleted_on_both_sides_count,
            "deleted_on_both_sides_lineage_unchanged_count": structural_summary.deleted_on_both_sides_lineage_unchanged_count,
            "emitted_mutation_intent_count": structural_summary.emitted_mutation_intent_count,
            "emitted_entity_create_count": structural_summary.emitted_entity_create_count,
            "emitted_relation_create_count": structural_summary.emitted_relation_create_count,
            "emitted_entity_update_count": structural_summary.emitted_entity_update_count,
            "execution_digest": summary.execution_digest,
            "diagnostics_digest": summary.diagnostics_digest,
        }),
    }
}

pub(crate) fn merge_execution_success_artifact(
    summary: &MergeExecutionSummary,
    plan: &MergeExecutionDiagnosticsPlan,
    commit_id: crate::history::data::CommitId,
) -> RelationalDiagnosticArtifact {
    let mut entries = Vec::with_capacity(plan.executed_records.len() + 1);
    entries.push(RelationalDiagnosticsEntry {
        code: DiagnosticCode::MergeExecutionPublished,
        message: "prepared merge execution artifact records executed rows only".to_string(),
        fields: json!({
            "commit_id": commit_id.0,
            "target_branch": summary.request.target_branch.0.clone(),
            "source_branch": summary.request.source_branch.0.clone(),
            "execution_digest": summary.execution_digest,
            "diagnostics_digest": plan.digest,
            "executed_record_count": plan.executed_records.len(),
        }),
    });
    entries.extend(
        plan.executed_records
            .iter()
            .map(executed_record_diagnostics_entry),
    );
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::History,
        kind: DiagnosticsArtifactKind::DetailedTrace,
        determinism: DeterminismExpectation::Required,
        entries,
    }
}

pub(crate) fn merge_execution_failure_artifact(
    prepared: &PreparedMergeExecution,
    error: &crate::merge::data::MergeExecutionError,
) -> RelationalDiagnosticArtifact {
    let binding = &prepared.bound_executable_plan().authority_binding;
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::History,
        kind: DiagnosticsArtifactKind::Failure,
        determinism: DeterminismExpectation::Required,
        entries: vec![RelationalDiagnosticsEntry {
            code: merge_execution_failure_code(error),
            message: "prepared merge execution failed before authoritative merge publication"
                .to_string(),
            fields: json!({
                "target_branch": prepared.request().target_branch.0.clone(),
                "source_branch": prepared.request().source_branch.0.clone(),
                "merge_intent": format!("{:?}", prepared.request().merge_intent),
                "target_head_commit_id": binding.target_head_commit_id.0.clone(),
                "source_head_commit_id": binding.source_head_commit_id.0.clone(),
                "merge_base_commit_id": binding.merge_base_commit_id.0.clone(),
                "schema_snapshot_digest": binding.schema_snapshot_digest.clone(),
                "executable_plan_digest": binding.executable_plan_digest.clone(),
                "diagnostics_digest": prepared.bound_executable_plan().diagnostics_plan.digest,
                "error": format!("{error:?}"),
            }),
        }],
    }
}

fn executed_record_diagnostics_entry(
    row: &ExecutedMergeRecordDiagnosticRow,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: DiagnosticCode::MergeExecutionPublished,
        message: "merge execution record row".to_string(),
        fields: json!({
            "record_class": executed_record_class_label(row.class),
            "source_record": row.source_record,
            "target_record": row.target_record,
            "record": row.record,
            "classification": format!("{:?}", row.provenance.classification),
            "causal_disposition": format!("{:?}", row.provenance.causal_disposition),
            "equality_witness_digest": row.equality_witness.as_ref().map(|witness| witness.witness_digest.clone()),
            "deletion_semantics": row.deletion_semantics.map(|semantics| format!("{:?}", semantics)),
            "lineage_continuity": row.lineage_continuity.map(|verdict| format!("{:?}", verdict)),
            "policy_proof_boundary": policy_proof_boundary_json(row.provenance.policy_proof_boundary),
            "applied_policies": row.provenance.applied_policies,
            "aspect_rows": row.aspect_rows.iter().map(executed_aspect_row_json).collect::<Vec<_>>(),
        }),
    }
}

fn policy_proof_boundary_json(
    boundary: crate::merge::data::MergePolicyProofBoundary,
) -> serde_json::Value {
    let decision_boundary = match boundary.decision_boundary {
        crate::merge::data::MergePolicyDecisionBoundary::AutoResolved => json!({
            "kind": "auto_resolved",
        }),
        crate::merge::data::MergePolicyDecisionBoundary::RequiresManualResolution { class } => json!({
            "kind": "requires_manual_resolution",
            "class": format!("{:?}", class),
        }),
        crate::merge::data::MergePolicyDecisionBoundary::Reject { class } => json!({
            "kind": "reject",
            "class": format!("{:?}", class),
        }),
    };

    json!({
        "ownership_surface": format!("{:?}", boundary.ownership_surface),
        "decision_boundary": decision_boundary,
    })
}

fn executed_aspect_row_json(row: &ExecutedMergeAspectDiagnosticRow) -> serde_json::Value {
    json!({
        "aspect_key": row.aspect_key,
        "class": executed_aspect_class_label(row.class),
        "source_value": row.source_value,
        "target_value": row.target_value,
        "base_value": row.base_value,
        "shared_value": row.shared_value,
        "resolved_value": row.resolved_value,
    })
}

fn executed_record_class_label(class: ExecutedMergeRecordClass) -> &'static str {
    match class {
        ExecutedMergeRecordClass::AdoptSource => "adopt_source",
        ExecutedMergeRecordClass::PreserveShared => "preserve_shared",
        ExecutedMergeRecordClass::Reconcile => "reconcile",
        ExecutedMergeRecordClass::ConvergeDeletedOnBothSides => "converge_deleted_on_both_sides",
    }
}

fn executed_aspect_class_label(class: ExecutedMergeAspectClass) -> &'static str {
    match class {
        ExecutedMergeAspectClass::AdoptSourceValue => "adopt_source_value",
        ExecutedMergeAspectClass::PreserveSharedValue => "preserve_shared_value",
        ExecutedMergeAspectClass::ReconcileValue => "reconcile_value",
    }
}

fn merge_execution_failure_code(
    error: &crate::merge::data::MergeExecutionError,
) -> DiagnosticCode {
    match error {
        crate::merge::data::MergeExecutionError::MergeBaseDrift { .. } => {
            DiagnosticCode::MissingMergeBase
        }
        crate::merge::data::MergeExecutionError::RuntimeInstanceMismatch { .. }
        | crate::merge::data::MergeExecutionError::StaleBranchHead { .. }
        | crate::merge::data::MergeExecutionError::SchemaSemanticDrift { .. }
        | crate::merge::data::MergeExecutionError::Compilation(_)
        | crate::merge::data::MergeExecutionError::MutationPlan(_)
        | crate::merge::data::MergeExecutionError::Commit(_) => {
            DiagnosticCode::DeterministicMergeViolation
        }
    }
}
