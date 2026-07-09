use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::execution::execute_selected_graph_obligations_with_context;
use super::{
    WorthQueryGraphMutationPolicyGateEvidence, WorthQueryGraphObligationDispatchContext,
    WorthQueryGraphObligationDispatchContextKind, WorthQueryGraphObligationDispatchEnvelope,
    WorthQueryGraphObligationDispatchError, WorthQueryGraphObligationDispatchPlan,
    WorthQueryGraphObligationExecutionContext, WorthQueryGraphObligationExecutionInput,
    WorthQueryGraphObligationExecutionResultEnvelope, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationSelection,
    WorthQueryGraphObligationStateAccessPolicy, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportStatus, WorthQueryGraphObligationVerdict,
};
use crate::runtime::WorthQueryGraphObligationDenialProjection;

use super::WorthQueryGraphObligationAttachmentEvidence;

mod projection;
pub use projection::{
    WorthQueryAuthoritativeMutationObligationDispatchProjection,
    WorthQueryAuthoritativeMutationObligationDispatchProjectionRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAuthoritativeMutationObligationDispatch {
    selection: WorthQueryGraphObligationSelection,
    envelope: Option<WorthQueryGraphObligationDispatchEnvelope>,
    execution_inputs: Vec<WorthQueryGraphObligationExecutionInput>,
    execution_results: Option<WorthQueryGraphObligationExecutionResultEnvelope>,
    policy_gate: Option<WorthQueryGraphMutationPolicyGateEvidence>,
    dispatch_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryAuthoritativeMutationObligationDispatch {
    pub fn from_selection(
        context: WorthQueryGraphObligationDispatchContext,
        selection: WorthQueryGraphObligationSelection,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::from_selection_with_execution_context(
            context,
            selection,
            WorthQueryGraphObligationExecutionContext::default(),
        )
    }

    pub fn from_selection_with_execution_context(
        context: WorthQueryGraphObligationDispatchContext,
        selection: WorthQueryGraphObligationSelection,
        execution_context: WorthQueryGraphObligationExecutionContext,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
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
        policy_gate: WorthQueryGraphMutationPolicyGateEvidence,
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

    pub fn selection(&self) -> &WorthQueryGraphObligationSelection {
        &self.selection
    }

    pub fn envelope(&self) -> Option<&WorthQueryGraphObligationDispatchEnvelope> {
        self.envelope.as_ref()
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.envelope
            .as_ref()
            .map(WorthQueryGraphObligationDispatchEnvelope::envelope_digest)
    }

    pub fn execution_inputs(&self) -> &[WorthQueryGraphObligationExecutionInput] {
        &self.execution_inputs
    }

    pub fn execution_results(&self) -> Option<&WorthQueryGraphObligationExecutionResultEnvelope> {
        self.execution_results.as_ref()
    }

    pub fn policy_gate(&self) -> Option<&WorthQueryGraphMutationPolicyGateEvidence> {
        self.policy_gate.as_ref()
    }

    pub fn blocking_denial_projection(&self) -> Option<WorthQueryGraphObligationDenialProjection> {
        WorthQueryGraphObligationDenialProjection::from_reduction(
            &self.execution_results.as_ref()?.reduce(),
        )
    }

    pub fn attachment_evidence(&self) -> WorthQueryGraphObligationAttachmentEvidence {
        WorthQueryGraphObligationAttachmentEvidence::from_dispatch(self)
    }

    pub fn evidence_projection(
        &self,
    ) -> WorthQueryAuthoritativeMutationObligationDispatchProjection {
        WorthQueryAuthoritativeMutationObligationDispatchProjection::from_dispatch(self)
    }

    pub fn dispatch_digest(&self) -> &str {
        self.dispatch_digest.as_str()
    }

    fn execution_input_for_plan(
        &self,
        plan: &WorthQueryGraphObligationDispatchPlan,
    ) -> Option<WorthQueryGraphObligationExecutionInput> {
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
        input: &WorthQueryGraphObligationExecutionInput,
    ) -> Option<&super::WorthQueryGraphObligationExecutionResultRow> {
        self.execution_results
            .as_ref()?
            .rows()
            .iter()
            .find(|row| row.input().input_digest() == input.input_digest())
    }
}

fn dispatch_envelope_from_selection(
    context: WorthQueryGraphObligationDispatchContext,
    selection: &WorthQueryGraphObligationSelection,
) -> Result<Option<WorthQueryGraphObligationDispatchEnvelope>, WorthQueryGraphObligationDispatchError>
{
    if selection.matched_registrations().is_empty() {
        return Ok(None);
    }
    let mut builder = WorthQueryGraphObligationDispatchEnvelope::builder(context);
    for registration in selection.matched_registrations() {
        builder = builder.record(dispatch_plan_for_selected_registration(registration)?);
    }
    builder.seal().map(Some)
}

fn dispatch_plan_for_selected_registration(
    registration: &WorthQueryGraphObligationRegistration,
) -> Result<WorthQueryGraphObligationDispatchPlan, WorthQueryGraphObligationDispatchError> {
    dispatch_plan_draft_for_registration(registration)
        .with_rule_identity(registration.rule_identity().clone())
        .with_execution_budget(registration.execution_budget().clone())
        .verdict(selection_verdict_for_registration(registration)?)
}

fn execution_inputs_from_selection(
    selection: &WorthQueryGraphObligationSelection,
    execution_context: &WorthQueryGraphObligationExecutionContext,
) -> Vec<WorthQueryGraphObligationExecutionInput> {
    selection
        .matched_registrations()
        .iter()
        .cloned()
        .map(|registration| {
            WorthQueryGraphObligationExecutionInput::from_selected_registration_with_context(
                selection.selection_digest(),
                registration,
                execution_context.clone(),
            )
        })
        .collect()
}

fn dispatch_plan_draft_for_registration(
    registration: &WorthQueryGraphObligationRegistration,
) -> super::WorthQueryGraphObligationDispatchPlanDraft {
    match registration.kind() {
        super::WorthQueryGraphObligationKind::BlockingInvariant => {
            WorthQueryGraphObligationDispatchPlan::blocking_invariant(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::WorthQueryGraphObligationKind::SchemaContractValidator => {
            WorthQueryGraphObligationDispatchPlan::schema_contract_validator(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::WorthQueryGraphObligationKind::AdvisoryObligation => {
            WorthQueryGraphObligationDispatchPlan::advisory(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::WorthQueryGraphObligationKind::PreflightSequencingObligation => {
            WorthQueryGraphObligationDispatchPlan::preflight_sequencing(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::WorthQueryGraphObligationKind::CapabilityGapScreen => {
            WorthQueryGraphObligationDispatchPlan::capability_gap_screen(
                registration.rule_identity().domain_invariant_family(),
            )
        }
        super::WorthQueryGraphObligationKind::OperatingContextGate => {
            WorthQueryGraphObligationDispatchPlan::operating_context_gate(
                registration.rule_identity().domain_invariant_family(),
            )
        }
    }
}

fn selection_verdict_for_registration(
    registration: &WorthQueryGraphObligationRegistration,
) -> Result<WorthQueryGraphObligationVerdict, WorthQueryGraphObligationDispatchError> {
    match registration.support_posture().status() {
        WorthQueryGraphObligationSupportStatus::Supported => {
            WorthQueryGraphObligationVerdict::allow_with_context("selected-for-execution")
        }
        WorthQueryGraphObligationSupportStatus::Unsupported => {
            WorthQueryGraphObligationVerdict::block("selected-obligation-unsupported")
        }
        WorthQueryGraphObligationSupportStatus::NotApplicable => {
            WorthQueryGraphObligationVerdict::allow_with_context("not-applicable-at-selection")
        }
        WorthQueryGraphObligationSupportStatus::DiagnosticOnly => {
            WorthQueryGraphObligationVerdict::advise("selected-diagnostic-only")
        }
        WorthQueryGraphObligationSupportStatus::DeferredToBackstop => {
            WorthQueryGraphObligationVerdict::allow_with_context("selected-deferred-to-backstop")
        }
    }
}

fn authoritative_mutation_dispatch_digest(
    selection: &WorthQueryGraphObligationSelection,
    envelope: Option<&WorthQueryGraphObligationDispatchEnvelope>,
    execution_inputs: &[WorthQueryGraphObligationExecutionInput],
    execution_results: Option<&WorthQueryGraphObligationExecutionResultEnvelope>,
    policy_gate: Option<&WorthQueryGraphMutationPolicyGateEvidence>,
) -> WorthQueryEvidenceIdentity {
    let input_digests = execution_inputs
        .iter()
        .map(WorthQueryGraphObligationExecutionInput::input_evidence_digest)
        .collect::<Vec<_>>();
    worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationMaterializedDispatch)
        .field_value(
            WorthQueryEvidenceTag::new("selection"),
            selection.selection_digest(),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("envelope"),
            envelope.map(WorthQueryGraphObligationDispatchEnvelope::envelope_digest),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("policy_gate"),
            policy_gate.map(WorthQueryGraphMutationPolicyGateEvidence::evidence_digest),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("execution_results"),
            execution_results
                .map(WorthQueryGraphObligationExecutionResultEnvelope::envelope_digest),
        )
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("input"), input_digests)
        .seal()
}
