use crate::application::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration,
    ForgeQueryLowerRuntimeBoundaryEnvelope,
};

use crate::target_binding::{
    ForgeQueryBindingTarget, ForgeQueryBindingTargetKind, ForgeQueryBindingTargetSemantics,
    ForgeQueryBindingTargetWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentDeclarationBindingTarget(ForgeQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedIntentPlanBindingTarget(ForgeQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget(ForgeQueryBindingTarget);

impl ForgeQueryIntentDeclarationBindingTarget {
    pub fn for_intent_declaration(declaration: &ForgeQueryIntentDeclaration) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::IntentDeclaration,
            declaration.input_digest().to_string(),
            ForgeQueryBindingTargetSemantics::for_intent_declaration(declaration),
        ))
    }

    pub fn for_canonical_declaration<
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        declaration: &ForgeQueryCanonicalDeclarationArtifact<D, I>,
    ) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::IntentDeclaration,
            format!("{:?}", declaration.declaration_digest()),
            ForgeQueryBindingTargetSemantics::for_canonical_declaration(declaration),
        ))
    }
}

impl ForgeQueryAdmittedIntentPlanBindingTarget {
    pub fn for_admitted_intent_plan(plan: &ForgeQueryAdmittedIntentPlan) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::AdmittedIntentPlan,
            plan.decision_digest().to_string(),
            ForgeQueryBindingTargetSemantics::for_admitted_intent_plan(plan),
        ))
    }
}

impl ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget {
    pub fn for_lower_runtime_boundary_envelope(
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::LowerRuntimeBoundaryEnvelope,
            envelope.envelope_identity().as_ref().to_string(),
            ForgeQueryBindingTargetSemantics::for_lower_runtime_boundary_envelope(envelope),
        ))
    }
}

impl crate::target_binding::sealed::Sealed for ForgeQueryIntentDeclarationBindingTarget {}
impl crate::target_binding::sealed::Sealed for ForgeQueryAdmittedIntentPlanBindingTarget {}
impl crate::target_binding::sealed::Sealed for ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget {}

impl ForgeQueryBindingTargetWitness for ForgeQueryIntentDeclarationBindingTarget {
    fn erased_target(&self) -> &ForgeQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryBindingTarget {
        self.0
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryAdmittedIntentPlanBindingTarget {
    fn erased_target(&self) -> &ForgeQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryBindingTarget {
        self.0
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget {
    fn erased_target(&self) -> &ForgeQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryBindingTarget {
        self.0
    }
}
