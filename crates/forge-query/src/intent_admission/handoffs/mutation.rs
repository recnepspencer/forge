use crate::identity::hash_parts;
use crate::intent_admission::{
    ForgeQueryAuthoritativeMutationBatchExecutionPlan, ForgeQueryAuthoritativeMutationExecutionPlan,
};
use crate::runtime::{ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWriteCommand};

use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationExecutionHandoff {
    command: ForgeQueryWriteCommand,
    verified_existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationBatchExecutionHandoff {
    commands: Vec<ForgeQueryWriteCommand>,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: ForgeQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl ForgeQueryAuthoritativeMutationExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryAuthoritativeMutationExecutionPlan) -> Self {
        Self {
            command: plan.command().clone(),
            verified_existing_truth_assertion: plan.verified_existing_truth_assertion().cloned(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest: mutation_handoff_digest(
                plan.family(),
                plan.entrypoint(),
                plan.execution_seam()
                    .expect("authoritative mutation handoff requires execution seam"),
                plan.decision_digest(),
                command_handoff_fingerprint(plan.command()),
            ),
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute
    }

    pub fn command(&self) -> &ForgeQueryWriteCommand {
        &self.command
    }

    pub fn verified_existing_truth_assertion(
        &self,
    ) -> Option<&ForgeQueryVerifiedExistingTruthAssertion> {
        self.verified_existing_truth_assertion.as_ref()
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

impl ForgeQueryAuthoritativeMutationBatchExecutionHandoff {
    pub(crate) fn from_plan(plan: ForgeQueryAuthoritativeMutationBatchExecutionPlan) -> Self {
        let commands = plan.batch_seed().commands().to_vec();
        Self {
            commands: commands.clone(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            eligibility_trace: plan.eligibility_trace().clone(),
            decision_digest: plan.decision_digest().to_string(),
            handoff_digest: mutation_handoff_digest(
                plan.family(),
                plan.entrypoint(),
                plan.execution_seam()
                    .expect("authoritative mutation batch handoff requires execution seam"),
                plan.decision_digest(),
                batch_handoff_fingerprint(&commands),
            ),
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        ForgeQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute
    }

    pub fn commands(&self) -> &[ForgeQueryWriteCommand] {
        &self.commands
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

fn mutation_handoff_digest(
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    decision_digest: &str,
    fingerprint: String,
) -> String {
    hash_parts(&[
        "forge_query_admitted_mutation_execution_handoff_v2".to_string(),
        format!("family:{}", family.as_str()),
        format!("entrypoint:{}", entrypoint.as_str()),
        format!("execution-seam:{}", execution_seam.as_str()),
        format!("decision:{decision_digest}"),
        format!("fingerprint:{fingerprint}"),
    ])
}

fn command_handoff_fingerprint(command: &ForgeQueryWriteCommand) -> String {
    format!(
        "{}:{}:{}",
        command.mutation_family().as_str(),
        command.declared_entity_identity_ref().unwrap_or("none"),
        command.declared_collection_ref().unwrap_or("none"),
    )
}

fn batch_handoff_fingerprint(commands: &[ForgeQueryWriteCommand]) -> String {
    commands
        .iter()
        .map(command_handoff_fingerprint)
        .collect::<Vec<_>>()
        .join("|")
}
