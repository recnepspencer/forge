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

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        Self(ForgeQueryBindingTarget::from_digest(
            ForgeQueryBindingTargetKind::IntentDeclaration,
            target_digest.into(),
            ForgeQueryBindingTargetSemantics::IntentDeclaration {
                name: "test.intent".to_string(),
                strategy_name: "test.strategy".to_string(),
                strategy_version: "1".to_string(),
                input_contract: "test.contract".to_string(),
                source_lane: crate::runtime::ForgeQueryIntentSourceLane::UserAuthored,
                target_lane: crate::runtime::ForgeQueryAuthorityLane::AuthoritativeTruth,
            },
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

    #[cfg(test)]
    pub(crate) fn from_digest_parts(
        target_digest: impl Into<String>,
        request_digest: impl Into<String>,
        eligibility_digest: impl Into<String>,
        decision_digest: impl Into<String>,
    ) -> Self {
        Self(ForgeQueryBindingTarget::from_digest(
            ForgeQueryBindingTargetKind::AdmittedIntentPlan,
            target_digest.into(),
            ForgeQueryBindingTargetSemantics::AdmittedIntentPlan {
                family: crate::intent_admission::ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
                entrypoint: crate::intent_admission::ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
                request_digest: request_digest.into(),
                eligibility_digest: eligibility_digest.into(),
                decision_digest: decision_digest.into(),
            },
        ))
    }
}

impl ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget {
    pub fn for_lower_runtime_boundary_envelope(
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::LowerRuntimeBoundaryEnvelope,
            envelope.envelope_digest().to_string(),
            ForgeQueryBindingTargetSemantics::for_lower_runtime_boundary_envelope(envelope),
        ))
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        Self(ForgeQueryBindingTarget::from_digest(
            ForgeQueryBindingTargetKind::LowerRuntimeBoundaryEnvelope,
            target_digest.into(),
            ForgeQueryBindingTargetSemantics::LowerRuntimeBoundaryEnvelope {
                seam_key: crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule,
                capability_label: "test.capability",
                crossing_classification:
                    crate::lower_runtime_routing::ForgeQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse,
                route_kind: crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
                support_posture: crate::lower_runtime_routing::ForgeQueryLowerRuntimeSupportPosture::Admitted,
                envelope_digest: "test.envelope".to_string(),
            },
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
