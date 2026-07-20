use super::super::WorthQueryIntentAdmissionFamily;
use super::types::WorthQueryIntentAdmissionCoveredEntrypoint;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WorthQueryIntentAdmissionMutationProofCase {
    RuntimeWriteDelegatesToWriteIntent,
    RuntimeWriteIntentCanonicalScalarAuthoring,
    RuntimeWriteBatchDelegatesToBatchIntent,
    RuntimeWriteBatchIntentCanonicalBatchAuthoring,
    WorkspaceWriteIntentCanonicalScalarAuthoring,
    WorkspaceWriteBatchIntentCanonicalBatchAuthoring,
    WorkspaceInsertBuildsScalarCommand,
    WorkspaceUpdateBuildsScalarCommand,
    WorkspaceDeleteBuildsScalarCommand,
    WorkspaceDeleteWithBuildsScalarCommand,
    WorkspaceSubmissionSubmitDelegatesToRuntimeWrite,
    WorkspaceSubmissionSubmitBatchDelegatesToRuntimeBatchIntent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionMutationAuditRow {
    public_surface: &'static str,
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    proof_case: WorthQueryIntentAdmissionMutationProofCase,
    delegation_evidence: &'static str,
}

impl WorthQueryIntentAdmissionMutationAuditRow {
    pub(crate) const fn new(
        public_surface: &'static str,
        family: WorthQueryIntentAdmissionFamily,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
        proof_case: WorthQueryIntentAdmissionMutationProofCase,
        delegation_evidence: &'static str,
    ) -> Self {
        Self {
            public_surface,
            family,
            entrypoint,
            proof_case,
            delegation_evidence,
        }
    }

    pub fn public_surface(&self) -> &'static str {
        self.public_surface
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn proof_case(&self) -> WorthQueryIntentAdmissionMutationProofCase {
        self.proof_case
    }

    pub fn delegation_evidence(&self) -> &'static str {
        self.delegation_evidence
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionMutationAudit {
    rows: &'static [WorthQueryIntentAdmissionMutationAuditRow],
}

impl WorthQueryIntentAdmissionMutationAudit {
    pub(crate) const fn new(rows: &'static [WorthQueryIntentAdmissionMutationAuditRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [WorthQueryIntentAdmissionMutationAuditRow] {
        self.rows
    }
}

const MUTATION_AUDIT_ROWS: [WorthQueryIntentAdmissionMutationAuditRow; 12] = [
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write(command)",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionMutationProofCase::RuntimeWriteDelegatesToWriteIntent,
        "runtime_write_calls_runtime_write_intent_execute",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write_intent(command).execute()",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionMutationProofCase::RuntimeWriteIntentCanonicalScalarAuthoring,
        "runtime_write_intent_is_canonical_scalar_authoritative_mutation_authoring",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write_batch(commands)",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        WorthQueryIntentAdmissionMutationProofCase::RuntimeWriteBatchDelegatesToBatchIntent,
        "runtime_write_batch_calls_runtime_write_batch_intent_execute",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "runtime.write_batch_intent(commands).execute()",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        WorthQueryIntentAdmissionMutationProofCase::RuntimeWriteBatchIntentCanonicalBatchAuthoring,
        "runtime_write_batch_intent_is_canonical_batch_authoritative_mutation_authoring",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "workspace.write_intent(command).execute()",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionMutationProofCase::WorkspaceWriteIntentCanonicalScalarAuthoring,
        "workspace_write_intent_is_canonical_scalar_authoritative_mutation_authoring",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "workspace.write_batch_intent(commands).execute()",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        WorthQueryIntentAdmissionMutationProofCase::WorkspaceWriteBatchIntentCanonicalBatchAuthoring,
        "workspace_write_batch_intent_is_canonical_batch_authoritative_mutation_authoring",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "workspace.insert(collection, declaration)",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionMutationProofCase::WorkspaceInsertBuildsScalarCommand,
        "workspace_insert_builds_insert_command_then_calls_workspace_write",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "workspace.update(entity_identity, declaration)",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionMutationProofCase::WorkspaceUpdateBuildsScalarCommand,
        "workspace_update_builds_update_command_then_calls_workspace_write",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "workspace.delete(entity_identity)",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionMutationProofCase::WorkspaceDeleteBuildsScalarCommand,
        "workspace_delete_builds_delete_command_then_calls_workspace_write",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "workspace.delete_with(entity_identity, declaration)",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionMutationProofCase::WorkspaceDeleteWithBuildsScalarCommand,
        "workspace_delete_with_builds_delete_command_then_calls_workspace_write",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "workspace.submissions()?.submit(command)",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionMutationProofCase::WorkspaceSubmissionSubmitDelegatesToRuntimeWrite,
        "workspace_submission_lane_submit_calls_runtime_write",
    ),
    WorthQueryIntentAdmissionMutationAuditRow::new(
        "workspace.submissions()?.submit_batch(commands)",
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        WorthQueryIntentAdmissionMutationProofCase::WorkspaceSubmissionSubmitBatchDelegatesToRuntimeBatchIntent,
        "workspace_submission_lane_submit_batch_calls_runtime_write_batch_intent_execute",
    ),
];

pub fn worth_query_intent_admission_mutation_audit() -> WorthQueryIntentAdmissionMutationAudit {
    WorthQueryIntentAdmissionMutationAudit::new(&MUTATION_AUDIT_ROWS)
}
