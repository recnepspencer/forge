use crate::application::{
    WorthQueryCanonicalDeclarationArtifact, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};
use crate::runtime::{
    WorthQueryAdmittedIntentPlan, WorthQueryIntentDeclaration,
    WorthQueryLowerRuntimeBoundaryEnvelope,
};

use crate::target_binding::{
    WorthQueryBindingTarget, WorthQueryBindingTargetKind, WorthQueryBindingTargetSemantics,
    WorthQueryBindingTargetWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentDeclarationBindingTarget(WorthQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedIntentPlanBindingTarget(WorthQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget(WorthQueryBindingTarget);

impl WorthQueryIntentDeclarationBindingTarget {
    pub fn for_intent_declaration(declaration: &WorthQueryIntentDeclaration) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::IntentDeclaration,
            declaration.input_digest().to_string(),
            WorthQueryBindingTargetSemantics::for_intent_declaration(declaration),
        ))
    }

    pub fn for_canonical_declaration<
        D: WorthQueryDomainEntryMarker,
        I: WorthQueryDeclarationInput<D>,
    >(
        declaration: &WorthQueryCanonicalDeclarationArtifact<D, I>,
    ) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::IntentDeclaration,
            format!("{:?}", declaration.declaration_digest()),
            WorthQueryBindingTargetSemantics::for_canonical_declaration(declaration),
        ))
    }
}

impl WorthQueryAdmittedIntentPlanBindingTarget {
    pub fn for_admitted_intent_plan(plan: &WorthQueryAdmittedIntentPlan) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::AdmittedIntentPlan,
            plan.decision_digest().to_string(),
            WorthQueryBindingTargetSemantics::for_admitted_intent_plan(plan),
        ))
    }
}

impl WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget {
    pub fn for_lower_runtime_boundary_envelope(
        envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::LowerRuntimeBoundaryEnvelope,
            envelope
                .envelope_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            WorthQueryBindingTargetSemantics::for_lower_runtime_boundary_envelope(envelope),
        ))
    }
}

impl crate::target_binding::sealed::Sealed for WorthQueryIntentDeclarationBindingTarget {}
impl crate::target_binding::sealed::Sealed for WorthQueryAdmittedIntentPlanBindingTarget {}
impl crate::target_binding::sealed::Sealed for WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget {}

impl WorthQueryBindingTargetWitness for WorthQueryIntentDeclarationBindingTarget {
    fn erased_target(&self) -> &WorthQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> WorthQueryBindingTarget {
        self.0
    }
}

impl WorthQueryBindingTargetWitness for WorthQueryAdmittedIntentPlanBindingTarget {
    fn erased_target(&self) -> &WorthQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> WorthQueryBindingTarget {
        self.0
    }
}

impl WorthQueryBindingTargetWitness for WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget {
    fn erased_target(&self) -> &WorthQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> WorthQueryBindingTarget {
        self.0
    }
}
