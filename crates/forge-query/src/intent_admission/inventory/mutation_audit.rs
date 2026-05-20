use super::super::ForgeQueryIntentAdmissionFamily;
use super::types::ForgeQueryIntentAdmissionCoveredEntrypoint;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionMutationAuditRow {
    public_surface: &'static str,
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    delegation_evidence: &'static str,
}

impl ForgeQueryIntentAdmissionMutationAuditRow {
    pub(crate) const fn new(
        public_surface: &'static str,
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        delegation_evidence: &'static str,
    ) -> Self {
        Self {
            public_surface,
            family,
            entrypoint,
            delegation_evidence,
        }
    }

    pub fn public_surface(&self) -> &'static str {
        self.public_surface
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn delegation_evidence(&self) -> &'static str {
        self.delegation_evidence
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionMutationAudit {
    rows: &'static [ForgeQueryIntentAdmissionMutationAuditRow],
}

impl ForgeQueryIntentAdmissionMutationAudit {
    pub(crate) const fn new(rows: &'static [ForgeQueryIntentAdmissionMutationAuditRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryIntentAdmissionMutationAuditRow] {
        self.rows
    }
}

const MUTATION_AUDIT_ROWS: [ForgeQueryIntentAdmissionMutationAuditRow; 19] = [
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write(command)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "runtime_write_calls_runtime_write_intent_execute",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write_intent(command).execute()",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "runtime_write_intent_is_canonical_scalar_authoritative_mutation_authoring",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write_batch(commands)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        "runtime_write_batch_calls_runtime_write_batch_intent_execute",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write_batch_intent(commands).execute()",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        "runtime_write_batch_intent_is_canonical_batch_authoritative_mutation_authoring",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.write(command)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_write_calls_workspace_write_intent_execute",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.write_intent(command).execute()",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_write_intent_is_canonical_scalar_authoritative_mutation_authoring",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.write_batch_intent(commands).execute()",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        "workspace_write_batch_intent_is_canonical_batch_authoritative_mutation_authoring",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.insert(collection, declaration)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_insert_builds_insert_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.update(entity_identity, declaration)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_update_builds_update_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.update_existing(binding, declaration)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_update_existing_builds_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.assert_existing(binding, declaration)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_assert_existing_builds_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.verify_existing(binding, declaration)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_verify_existing_builds_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.update_existing_verified(binding, verify, update)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_update_existing_verified_builds_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.delete(entity_identity)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_delete_builds_delete_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.delete_with(entity_identity, declaration)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_delete_with_builds_delete_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.delete_existing(binding)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_delete_existing_builds_delete_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.delete_existing_with(binding, declaration)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_delete_existing_with_builds_delete_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.delete_existing_verified(binding, verify, delete)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_delete_existing_verified_builds_delete_command_then_calls_workspace_write",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.batch(declaration)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        "workspace_batch_builds_commands_then_calls_workspace_write_batch_intent_execute",
    ),
];

pub fn forge_query_intent_admission_mutation_audit() -> ForgeQueryIntentAdmissionMutationAudit {
    ForgeQueryIntentAdmissionMutationAudit::new(&MUTATION_AUDIT_ROWS)
}
