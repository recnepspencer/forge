use super::*;
use crate::intent_admission::WorthQueryIntentAdmissionMutationProofCase;

#[test]
fn mutation_audit_covers_closed_write_update_delete_surface_set() {
    let audit = worth_query_intent_admission_mutation_audit();
    let rows = audit
        .rows()
        .iter()
        .map(|row| {
            (
                row.public_surface(),
                row.family(),
                row.entrypoint(),
                row.proof_case(),
                row.delegation_evidence(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        rows,
        vec![
            (
                "runtime.write(command)",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                WorthQueryIntentAdmissionMutationProofCase::RuntimeWriteDelegatesToWriteIntent,
                "runtime_write_calls_runtime_write_intent_execute",
            ),
            (
                "runtime.write_intent(command).execute()",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                WorthQueryIntentAdmissionMutationProofCase::RuntimeWriteIntentCanonicalScalarAuthoring,
                "runtime_write_intent_is_canonical_scalar_authoritative_mutation_authoring",
            ),
            (
                "runtime.write_batch(commands)",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
                WorthQueryIntentAdmissionMutationProofCase::RuntimeWriteBatchDelegatesToBatchIntent,
                "runtime_write_batch_calls_runtime_write_batch_intent_execute",
            ),
            (
                "runtime.write_batch_intent(commands).execute()",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
                WorthQueryIntentAdmissionMutationProofCase::RuntimeWriteBatchIntentCanonicalBatchAuthoring,
                "runtime_write_batch_intent_is_canonical_batch_authoritative_mutation_authoring",
            ),
            (
                "workspace.write_intent(command).execute()",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                WorthQueryIntentAdmissionMutationProofCase::WorkspaceWriteIntentCanonicalScalarAuthoring,
                "workspace_write_intent_is_canonical_scalar_authoritative_mutation_authoring",
            ),
            (
                "workspace.write_batch_intent(commands).execute()",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
                WorthQueryIntentAdmissionMutationProofCase::WorkspaceWriteBatchIntentCanonicalBatchAuthoring,
                "workspace_write_batch_intent_is_canonical_batch_authoritative_mutation_authoring",
            ),
            (
                "workspace.insert(collection, declaration)",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                WorthQueryIntentAdmissionMutationProofCase::WorkspaceInsertBuildsScalarCommand,
                "workspace_insert_builds_insert_command_then_calls_workspace_write",
            ),
            (
                "workspace.update(entity_identity, declaration)",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                WorthQueryIntentAdmissionMutationProofCase::WorkspaceUpdateBuildsScalarCommand,
                "workspace_update_builds_update_command_then_calls_workspace_write",
            ),
            (
                "workspace.delete(entity_identity)",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                WorthQueryIntentAdmissionMutationProofCase::WorkspaceDeleteBuildsScalarCommand,
                "workspace_delete_builds_delete_command_then_calls_workspace_write",
            ),
            (
                "workspace.delete_with(entity_identity, declaration)",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                WorthQueryIntentAdmissionMutationProofCase::WorkspaceDeleteWithBuildsScalarCommand,
                "workspace_delete_with_builds_delete_command_then_calls_workspace_write",
            ),
            (
                "workspace.submissions()?.submit(command)",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                WorthQueryIntentAdmissionMutationProofCase::WorkspaceSubmissionSubmitDelegatesToRuntimeWrite,
                "workspace_submission_lane_submit_calls_runtime_write",
            ),
            (
                "workspace.submissions()?.submit_batch(commands)",
                WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
                WorthQueryIntentAdmissionMutationProofCase::WorkspaceSubmissionSubmitBatchDelegatesToRuntimeBatchIntent,
                "workspace_submission_lane_submit_batch_calls_runtime_write_batch_intent_execute",
            ),
        ]
    );
}

#[test]
fn mutation_audit_rows_map_to_implemented_coverage_inventory_entrypoints() {
    let audit = worth_query_intent_admission_mutation_audit();
    let inventory = worth_query_intent_admission_coverage_inventory();
    let implemented = inventory
        .rows()
        .iter()
        .filter(|row| row.status() == WorthQueryIntentAdmissionCoverageStatus::Implemented)
        .collect::<Vec<_>>();
    let mut proof_cases = std::collections::HashSet::new();

    for row in audit.rows() {
        assert_eq!(
            row.family(),
            WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent
        );
        assert!(implemented.iter().any(|coverage| {
            coverage.entrypoint() == row.entrypoint() && coverage.family() == row.family()
        }));
        assert!(
            proof_cases.insert(row.proof_case()),
            "duplicate mutation audit proof case for {}",
            row.public_surface()
        );
        assert!(!row.delegation_evidence().is_empty());
    }
}
