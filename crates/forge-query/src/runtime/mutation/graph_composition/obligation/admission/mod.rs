use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::execution::execute_selected_graph_obligations_with_context;
use super::{
    ForgeQueryGraphMutationPolicyGateEvidence, ForgeQueryGraphObligationDispatchContext,
    ForgeQueryGraphObligationDispatchContextKind, ForgeQueryGraphObligationDispatchEnvelope,
    ForgeQueryGraphObligationDispatchError, ForgeQueryGraphObligationDispatchPlan,
    ForgeQueryGraphObligationExecutionContext, ForgeQueryGraphObligationExecutionInput,
    ForgeQueryGraphObligationExecutionResultEnvelope, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationSelection,
    ForgeQueryGraphObligationStateAccessPolicy, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportStatus, ForgeQueryGraphObligationVerdict,
};
use crate::runtime::ForgeQueryGraphObligationDenialProjection;

use super::ForgeQueryGraphObligationAttachmentEvidence;

mod projection;
pub use projection::{
    ForgeQueryAuthoritativeMutationObligationDispatchProjection,
    ForgeQueryAuthoritativeMutationObligationDispatchProjectionRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationObligationDispatch {
    selection: ForgeQueryGraphObligationSelection,
    envelope: Option<ForgeQueryGraphObligationDispatchEnvelope>,
    execution_inputs: Vec<ForgeQueryGraphObligationExecutionInput>,
    execution_results: Option<ForgeQueryGraphObligationExecutionResultEnvelope>,
    policy_gate: Option<ForgeQueryGraphMutationPolicyGateEvidence>,
    dispatch_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryAuthoritativeMutationObligationDispatch {
    pub fn from_selection(
        context: ForgeQueryGraphObligationDispatchContext,
        selection: ForgeQueryGraphObligationSelection,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::from_selection_with_execution_context(
            context,
            selection,
            ForgeQueryGraphObligationExecutionContext::default(),
        )
    }

    pub fn from_selection_with_execution_context(
        context: ForgeQueryGraphObligationDispatchContext,
        selection: ForgeQueryGraphObligationSelection,
        execution_context: ForgeQueryGraphObligationExecutionContext,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        let envelope = dispatch_envelope_from_selection(context, &selection)?;
        let execution_inputs = execution_inputs_from_selection(&selection, &execution_context);
        let execution_results =
            execute_selected_graph_obligations_with_context(&selection, execution_context);
        let dispatch_digest = authoritative_mutation_dispatch_digest(
            &selection,
            envelope.as_ref(),
            &execution_inputs,
            execution_results.as_ref(),
            None,
        );
        Ok(Self {
            selection,
            envelope,
            execution_inputs,
            execution_results,
            policy_gate: None,
            dispatch_digest,
        })
    }

    pub(crate) fn with_policy_gate(
        mut self,
        policy_gate: ForgeQueryGraphMutationPolicyGateEvidence,
    ) -> Self {
        self.dispatch_digest = authoritative_mutation_dispatch_digest(
            &self.selection,
            self.envelope.as_ref(),
            &self.execution_inputs,
            self.execution_results.as_ref(),
            Some(&policy_gate),
        );
        self.policy_gate = Some(policy_gate);
        self
    }

    pub fn selection(&self) -> &ForgeQueryGraphObligationSelection {
        &self.selection
    }

    pub fn envelope(&self) -> Option<&ForgeQueryGraphObligationDispatchEnvelope> {
        self.envelope.as_ref()
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.envelope
            .as_ref()
            .map(ForgeQueryGraphObligationDispatchEnvelope::envelope_digest)
    }

    pub fn execution_inputs(&self) -> &[ForgeQueryGraphObligationExecutionInput] {
        &self.execution_inputs
    }

    pub fn execution_results(&self) -> Option<&ForgeQueryGraphObligationExecutionResultEnvelope> {
        self.execution_results.as_ref()
    }

    pub fn policy_gate(&self) -> Option<&ForgeQueryGraphMutationPolicyGateEvidence> {
        self.policy_gate.as_ref()
    }

    pub fn blocking_denial_projection(&self) -> Option<ForgeQueryGraphObligationDenialProjection> {
        ForgeQueryGraphObligationDenialProjection::from_reduction(
            &self.execution_results.as_ref()?.reduce(),
        )
    }

    pub fn attachment_evidence(&self) -> ForgeQueryGraphObligationAttachmentEvidence {
        ForgeQueryGraphObligationAttachmentEvidence::from_dispatch(self)
    }

    pub fn evidence_projection(
        &self,
    ) -> ForgeQueryAuthoritativeMutationObligationDispatchProjection {
        ForgeQueryAuthoritativeMutationObligationDispatchProjection::from_dispatch(self)
    }

    pub fn dispatch_digest(&self) -> &str {
        self.dispatch_digest.as_str()
    }

    fn execution_input_for_plan(
        &self,
        plan: &ForgeQueryGraphObligationDispatchPlan,
    ) -> Option<ForgeQueryGraphObligationExecutionInput> {
        self.execution_inputs
            .iter()
            .find(|input| {
                input
                    .selected_registration()
                    .rule_identity()
                    .identity_digest()
                    == plan.rule_identity().identity_digest()
            })
            .cloned()
    }

    fn execution_result_for_input(
        &self,
        input: &ForgeQueryGraphObligationExecutionInput,
    ) -> Option<&super::ForgeQueryGraphObligationExecutionResultRow> {
        self.execution_results
            .as_ref()?
            .rows()
            .iter()
            .find(|row| row.input().input_digest() == input.input_digest())
    }
}

fn dispatch_envelope_from_selection(
    context: ForgeQueryGraphObligationDispatchContext,
    selection: &ForgeQueryGraphObligationSelection,
) -> Result<Option<ForgeQueryGraphObligationDispatchEnvelope>, ForgeQueryGraphObligationDispatchError>
{
    if selection.matched_registrations().is_empty() {
        return Ok(None);
    }
    let mut builder = ForgeQueryGraphObligationDispatchEnvelope::builder(context);
    for registration in selection.matched_registrations() {
        builder = builder.record(dispatch_plan_for_selected_registration(registration)?);
    }
    builder.seal().map(Some)
}

fn dispatch_plan_for_selected_registration(
    registration: &ForgeQueryGraphObligationRegistration,
) -> Result<ForgeQueryGraphObligationDispatchPlan, ForgeQueryGraphObligationDispatchError> {
    dispatch_plan_draft_for_registration(registration)
        .with_rule_identity(registration.rule_identity().clone())
        .with_execution_budget(registration.execution_budget().clone())
        .verdict(selection_verdict_for_registration(registration)?)
}

fn execution_inputs_from_selection(
    selection: &ForgeQueryGraphObligationSelection,
    execution_context: &ForgeQueryGraphObligationExecutionContext,
) -> Vec<ForgeQueryGraphObligationExecutionInput> {
    selection
        .matched_registrations()
        .iter()
        .cloned()
        .map(|registration| {
            ForgeQueryGraphObligationExecutionInput::from_selected_registration_with_context(
                selection.selection_digest(),
                registration,
                execution_context.clone(),
            )
        })
        .collect()
}

fn dispatch_plan_draft_for_registration(
    registration: &ForgeQueryGraphObligationRegistration,
) -> super::ForgeQueryGraphObligationDispatchPlanDraft {
    match registration.kind() {
        super::ForgeQueryGraphObligationKind::BlockingInvariant => {
            ForgeQueryGraphObligationDispatchPlan::blocking_invariant(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::ForgeQueryGraphObligationKind::SchemaContractValidator => {
            ForgeQueryGraphObligationDispatchPlan::schema_contract_validator(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::ForgeQueryGraphObligationKind::AdvisoryObligation => {
            ForgeQueryGraphObligationDispatchPlan::advisory(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::ForgeQueryGraphObligationKind::PreflightSequencingObligation => {
            ForgeQueryGraphObligationDispatchPlan::preflight_sequencing(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::ForgeQueryGraphObligationKind::CapabilityGapScreen => {
            ForgeQueryGraphObligationDispatchPlan::capability_gap_screen(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::ForgeQueryGraphObligationKind::OperatingContextGate => {
            ForgeQueryGraphObligationDispatchPlan::operating_context_gate(
                registration.rule_identity().domain_invariant_family(),
            )
        }
    }
}

fn selection_verdict_for_registration(
    registration: &ForgeQueryGraphObligationRegistration,
) -> Result<ForgeQueryGraphObligationVerdict, ForgeQueryGraphObligationDispatchError> {
    match registration.support_posture().status() {
        ForgeQueryGraphObligationSupportStatus::Supported => {
            ForgeQueryGraphObligationVerdict::allow_with_context("selected-for-execution")
        }
        ForgeQueryGraphObligationSupportStatus::Unsupported => {
            ForgeQueryGraphObligationVerdict::block("selected-obligation-unsupported")
        }
        ForgeQueryGraphObligationSupportStatus::NotApplicable => {
            ForgeQueryGraphObligationVerdict::allow_with_context("not-applicable-at-selection")
        }
        ForgeQueryGraphObligationSupportStatus::DiagnosticOnly => {
            ForgeQueryGraphObligationVerdict::advise("selected-diagnostic-only")
        }
        ForgeQueryGraphObligationSupportStatus::DeferredToBackstop => {
            ForgeQueryGraphObligationVerdict::allow_with_context("selected-deferred-to-backstop")
        }
    }
}

fn authoritative_mutation_dispatch_digest(
    selection: &ForgeQueryGraphObligationSelection,
    envelope: Option<&ForgeQueryGraphObligationDispatchEnvelope>,
    execution_inputs: &[ForgeQueryGraphObligationExecutionInput],
    execution_results: Option<&ForgeQueryGraphObligationExecutionResultEnvelope>,
    policy_gate: Option<&ForgeQueryGraphMutationPolicyGateEvidence>,
) -> ForgeQueryEvidenceIdentity {
    let input_digests = execution_inputs
        .iter()
        .map(ForgeQueryGraphObligationExecutionInput::input_evidence_digest)
        .collect::<Vec<_>>();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationMaterializedDispatch)
        .field_value(
            ForgeQueryEvidenceTag::new("selection"),
            selection.selection_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("envelope"),
            envelope.map(ForgeQueryGraphObligationDispatchEnvelope::envelope_digest),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("policy_gate"),
            policy_gate.map(ForgeQueryGraphMutationPolicyGateEvidence::evidence_digest),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("execution_results"),
            execution_results
                .map(ForgeQueryGraphObligationExecutionResultEnvelope::envelope_digest),
        )
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("input"), input_digests)
        .seal()
}
