use super::types::ForgeQueryIntentAdmissionCoveredEntrypoint;
use super::super::ForgeQueryIntentAdmissionFamily;

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

const MUTATION_AUDIT_ROWS: [ForgeQueryIntentAdmissionMutationAuditRow; 7] = [
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write_intent(command).execute()",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "scalar_write_delegates_to_canonical_admission_and_execution_handoff",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.write_intent(command).execute()",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "scalar_write_delegates_to_canonical_admission_and_execution_handoff",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write_batch_intent(commands).execute()",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        "batch_write_delegates_to_canonical_admission_and_execution_handoff",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.write_batch_intent(commands).execute()",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        "batch_write_delegates_to_canonical_admission_and_execution_handoff",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.verify_existing(binding, paths)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_verify_existing_delegates_to_authoritative_mutation_intent_execution",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.update_existing_verified(binding, paths, update)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_update_existing_verified_delegates_to_authoritative_mutation_intent_execution",
    ),
    ForgeQueryIntentAdmissionMutationAuditRow::new(
        "workspace.delete_existing_verified(binding, paths)",
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        "workspace_delete_existing_verified_delegates_to_authoritative_mutation_intent_execution",
    ),
];

pub fn forge_query_intent_admission_mutation_audit() -> ForgeQueryIntentAdmissionMutationAudit {
    ForgeQueryIntentAdmissionMutationAudit::new(&MUTATION_AUDIT_ROWS)
}
