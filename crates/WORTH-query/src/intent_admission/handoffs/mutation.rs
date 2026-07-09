use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::intent_admission::{
    WorthQueryAuthoritativeMutationBatchExecutionPlan, WorthQueryAuthoritativeMutationExecutionPlan,
};
use crate::runtime::{
    WorthQueryAuthoritativeMutationObligationDispatch, WorthQueryGraphCompositionBreadth,
    WorthQueryGraphCompositionProgram, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchDescriptorDenial, WorthQueryVerifiedExistingTruthAssertion,
    WorthQueryWriteCommand,
};

use super::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentEligibilityTraceEvidence,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeMutationExecutionHandoff {
    command: WorthQueryWriteCommand,
    verified_existing_truth_assertion: Option<WorthQueryVerifiedExistingTruthAssertion>,
    obligation_dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeMutationBatchExecutionHandoff {
    commands: Vec<WorthQueryWriteCommand>,
    graph_composition_breadth: WorthQueryGraphCompositionBreadth,
    graph_composition_program: WorthQueryGraphCompositionProgram,
    obligation_dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
    request_digest: String,
    eligibility_digest: String,
    eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    decision_digest: String,
    handoff_digest: String,
}

impl WorthQueryAuthoritativeMutationExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryAuthoritativeMutationExecutionPlan) -> Self {
        Self {
            command: plan.command().clone(),
            verified_existing_truth_assertion: plan.verified_existing_truth_assertion().cloned(),
            obligation_dispatch: None,
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
                None,
            ),
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute
    }

    pub(crate) fn with_obligation_dispatch(
        mut self,
        obligation_dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
    ) -> Self {
        self.obligation_dispatch = obligation_dispatch;
        self.handoff_digest = mutation_handoff_digest(
            self.family(),
            self.entrypoint(),
            self.execution_seam(),
            &self.decision_digest,
            command_handoff_fingerprint(&self.command),
            self.obligation_dispatch
                .as_ref()
                .map(WorthQueryAuthoritativeMutationObligationDispatch::dispatch_digest),
        );
        self
    }

    pub fn command(&self) -> &WorthQueryWriteCommand {
        &self.command
    }

    pub fn verified_existing_truth_assertion(
        &self,
    ) -> Option<&WorthQueryVerifiedExistingTruthAssertion> {
        self.verified_existing_truth_assertion.as_ref()
    }

    pub fn obligation_dispatch(
        &self,
    ) -> Option<&WorthQueryAuthoritativeMutationObligationDispatch> {
        self.obligation_dispatch.as_ref()
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

impl WorthQueryAuthoritativeMutationBatchExecutionHandoff {
    pub(crate) fn from_plan(plan: WorthQueryAuthoritativeMutationBatchExecutionPlan) -> Self {
        let commands = plan.batch_seed().commands().to_vec();
        Self {
            commands: commands.clone(),
            graph_composition_breadth: plan.batch_seed().graph_composition_breadth().clone(),
            graph_composition_program: plan.batch_seed().graph_composition_program().clone(),
            obligation_dispatch: plan.obligation_dispatch().cloned(),
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
                None,
            ),
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite
    }

    pub fn execution_seam(&self) -> WorthQueryIntentAdmissionExecutionSeam {
        WorthQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute
    }

    pub(crate) fn with_obligation_dispatch(
        mut self,
        obligation_dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
    ) -> Self {
        self.obligation_dispatch = obligation_dispatch;
        self.handoff_digest = mutation_handoff_digest(
            self.family(),
            self.entrypoint(),
            self.execution_seam(),
            &self.decision_digest,
            batch_handoff_fingerprint(&self.commands),
            self.obligation_dispatch
                .as_ref()
                .map(WorthQueryAuthoritativeMutationObligationDispatch::dispatch_digest),
        );
        self
    }

    pub fn commands(&self) -> &[WorthQueryWriteCommand] {
        &self.commands
    }

    pub fn graph_composition_breadth(&self) -> &WorthQueryGraphCompositionBreadth {
        &self.graph_composition_breadth
    }

    pub fn graph_composition_program(&self) -> &WorthQueryGraphCompositionProgram {
        &self.graph_composition_program
    }

    pub fn graph_touch_descriptor(
        &self,
    ) -> Result<WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial> {
        WorthQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
            &self.graph_composition_program,
            &self.graph_composition_breadth,
            &self.commands,
        )
    }

    pub fn obligation_dispatch(
        &self,
    ) -> Option<&WorthQueryAuthoritativeMutationObligationDispatch> {
        self.obligation_dispatch.as_ref()
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
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
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    decision_digest: &str,
    fingerprint: WorthQueryEvidenceIdentity,
    obligation_dispatch_digest: Option<&str>,
) -> String {
    let decision_identity = worth_query_evidence_identity(
        WorthQueryEvidenceScope::AuthoritativeMutationExecutionHandoff,
    )
    .field_shape(WorthQueryEvidenceTag::new("role"), "decision-digest")
    .field_value(WorthQueryEvidenceTag::new("digest"), decision_digest)
    .seal();
    worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationExecutionHandoff)
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("entrypoint"),
            entrypoint.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("execution_seam"),
            execution_seam.as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("decision"), &decision_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("fingerprint"), &fingerprint)
        .optional_value(
            WorthQueryEvidenceTag::new("obligation_dispatch"),
            obligation_dispatch_digest,
        )
        .seal()
        .as_str()
        .to_string()
}

fn command_handoff_fingerprint(command: &WorthQueryWriteCommand) -> WorthQueryEvidenceIdentity {
    let declared_entity_identity = command
        .declared_entity_identity_ref()
        .map(|identity| identity.evidence_identity());
    let declared_collection_identity = command.declared_collection_identity();
    worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationExecutionHandoff)
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            command.mutation_family().as_str(),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("declared_entity_identity"),
            declared_entity_identity.as_ref(),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("declared_collection"),
            declared_collection_identity
                .as_ref()
                .map(crate::runtime::WorthQueryMutationTargetCollectionIdentity::evidence_identity),
        )
        .seal()
}

fn batch_handoff_fingerprint(commands: &[WorthQueryWriteCommand]) -> WorthQueryEvidenceIdentity {
    let command_fingerprints = commands
        .iter()
        .map(command_handoff_fingerprint)
        .collect::<Vec<_>>();
    worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationExecutionHandoff)
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "batch-command-fingerprint",
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("command"),
            command_fingerprints.iter(),
        )
        .seal()
}
