use super::*;

#[test]
fn mutation_audit_covers_closed_write_update_delete_surface_set() {
    let audit = forge_query_intent_admission_mutation_audit();
    let rows = audit
        .rows()
        .iter()
        .map(|row| {
            (
                row.public_surface(),
                row.family(),
                row.entrypoint(),
                row.delegation_evidence(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        rows,
        vec![
            (
                "runtime.write(command)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "runtime_write_calls_runtime_write_intent_execute",
            ),
            (
                "runtime.write_intent(command).execute()",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "runtime_write_intent_is_canonical_scalar_authoritative_mutation_authoring",
            ),
            (
                "runtime.write_batch(commands)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
                "runtime_write_batch_calls_runtime_write_batch_intent_execute",
            ),
            (
                "runtime.write_batch_intent(commands).execute()",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
                "runtime_write_batch_intent_is_canonical_batch_authoritative_mutation_authoring",
            ),
            (
                "workspace.write(command)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_write_calls_workspace_write_intent_execute",
            ),
            (
                "workspace.write_intent(command).execute()",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_write_intent_is_canonical_scalar_authoritative_mutation_authoring",
            ),
            (
                "workspace.write_batch_intent(commands).execute()",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
                "workspace_write_batch_intent_is_canonical_batch_authoritative_mutation_authoring",
            ),
            (
                "workspace.insert(collection, declaration)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_insert_builds_insert_command_then_calls_workspace_write",
            ),
            (
                "workspace.update(entity_identity, declaration)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_update_builds_update_command_then_calls_workspace_write",
            ),
            (
                "workspace.update_existing(binding, declaration)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_update_existing_builds_command_then_calls_workspace_write",
            ),
            (
                "workspace.assert_existing(binding, declaration)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_assert_existing_builds_command_then_calls_workspace_write",
            ),
            (
                "workspace.verify_existing(binding, declaration)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_verify_existing_builds_command_then_calls_workspace_write",
            ),
            (
                "workspace.update_existing_verified(binding, verify, update)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_update_existing_verified_builds_command_then_calls_workspace_write",
            ),
            (
                "workspace.delete(entity_identity)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_delete_builds_delete_command_then_calls_workspace_write",
            ),
            (
                "workspace.delete_with(entity_identity, declaration)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_delete_with_builds_delete_command_then_calls_workspace_write",
            ),
            (
                "workspace.delete_existing(binding)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_delete_existing_builds_delete_command_then_calls_workspace_write",
            ),
            (
                "workspace.delete_existing_with(binding, declaration)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_delete_existing_with_builds_delete_command_then_calls_workspace_write",
            ),
            (
                "workspace.delete_existing_verified(binding, verify, delete)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
                "workspace_delete_existing_verified_builds_delete_command_then_calls_workspace_write",
            ),
            (
                "workspace.batch(declaration)",
                ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
                "workspace_batch_builds_commands_then_calls_workspace_write_batch_intent_execute",
            ),
        ]
    );
}

#[test]
fn mutation_audit_rows_map_to_implemented_coverage_inventory_entrypoints() {
    let audit = forge_query_intent_admission_mutation_audit();
    let inventory = forge_query_intent_admission_coverage_inventory();
    let implemented = inventory
        .rows()
        .iter()
        .filter(|row| row.status() == ForgeQueryIntentAdmissionCoverageStatus::Implemented)
        .collect::<Vec<_>>();

    for row in audit.rows() {
        assert_eq!(
            row.family(),
            ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent
        );
        assert!(implemented.iter().any(|coverage| {
            coverage.entrypoint() == row.entrypoint() && coverage.family() == row.family()
        }));
        assert!(!row.delegation_evidence().is_empty());
    }
}
