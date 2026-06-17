use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::intent_admission::{
    ForgeQueryAuthoritativeMutationBatchExecutionPlan, ForgeQueryAuthoritativeMutationExecutionPlan,
};
use crate::runtime::{
    ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWriteCommand,
};

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
    graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
    graph_composition_program: ForgeQueryGraphCompositionProgram,
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
            graph_composition_breadth: plan.batch_seed().graph_composition_breadth().clone(),
            graph_composition_program: plan.batch_seed().graph_composition_program().clone(),
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

    pub fn graph_composition_breadth(&self) -> &ForgeQueryGraphCompositionBreadth {
        &self.graph_composition_breadth
    }

    pub fn graph_composition_program(&self) -> &ForgeQueryGraphCompositionProgram {
        &self.graph_composition_program
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
    fingerprint: ForgeQueryEvidenceIdentity,
) -> String {
    let decision_identity = forge_query_evidence_identity(
        ForgeQueryEvidenceScope::AuthoritativeMutationExecutionHandoff,
    )
    .field_shape(ForgeQueryEvidenceTag::new("role"), "decision-digest")
    .field_value(ForgeQueryEvidenceTag::new("digest"), decision_digest)
    .seal();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationExecutionHandoff)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("entrypoint"),
            entrypoint.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_seam"),
            execution_seam.as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("decision"), &decision_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("fingerprint"), &fingerprint)
        .seal()
        .as_str()
        .to_string()
}

fn command_handoff_fingerprint(command: &ForgeQueryWriteCommand) -> ForgeQueryEvidenceIdentity {
    let declared_entity_identity = command
        .declared_entity_identity_ref()
        .map(|identity| identity.evidence_identity());
    let declared_collection_identity = command.declared_collection_ref().map(|collection| {
        crate::runtime::ForgeQueryMutationTargetCollectionIdentity::new(
            "authoritative-mutation-handoff",
            collection,
        )
    });
    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationExecutionHandoff)
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            command.mutation_family().as_str(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("declared_entity_identity"),
            declared_entity_identity.as_ref(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("declared_collection"),
            declared_collection_identity
                .as_ref()
                .map(crate::runtime::ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
        )
        .seal()
}

fn batch_handoff_fingerprint(commands: &[ForgeQueryWriteCommand]) -> ForgeQueryEvidenceIdentity {
    let command_fingerprints = commands
        .iter()
        .map(command_handoff_fingerprint)
        .collect::<Vec<_>>();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationExecutionHandoff)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "batch-command-fingerprint",
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("command"),
            command_fingerprints.iter(),
        )
        .seal()
}
