use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration,
    ForgeQueryLowerRuntimeBoundaryEnvelope,
};
use crate::target_binding::{
    ForgeQueryAdmittedIntentPlanBindingTarget, ForgeQueryBindingTargetWitness,
    ForgeQueryIntentDeclarationBindingTarget, ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget,
};

use super::core::{
    ForgeQueryDomainCapabilityTarget, ForgeQueryDomainCapabilityTargetBinding,
    ForgeQueryDomainCapabilityTargetKind, ForgeQueryDomainCapabilityTargetSemantics,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationBoundContributionTarget {
    shared: ForgeQueryIntentDeclarationBindingTarget,
    erased: ForgeQueryDomainCapabilityTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanBoundContributionTarget {
    shared: ForgeQueryAdmittedIntentPlanBindingTarget,
    erased: ForgeQueryDomainCapabilityTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    shared: ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget,
    erased: ForgeQueryDomainCapabilityTarget,
}

impl ForgeQueryDeclarationBoundContributionTarget {
    pub fn for_intent_declaration(declaration: &ForgeQueryIntentDeclaration) -> Self {
        let shared = ForgeQueryIntentDeclarationBindingTarget::for_intent_declaration(declaration);
        let erased = ForgeQueryDomainCapabilityTarget::new(
            shared.clone().into_erased_target(),
            ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
            ForgeQueryDomainCapabilityTargetSemantics::IntentDeclaration {
                name: declaration.name().to_string(),
                strategy_name: declaration.strategy_name().to_string(),
                strategy_version: declaration.strategy_version().to_string(),
                input_contract: declaration.input_contract().to_string(),
                source_lane: declaration.source_lane(),
                target_lane: declaration.target_lane(),
            },
        );
        Self { shared, erased }
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        let shared = ForgeQueryIntentDeclarationBindingTarget::from_digest(target_digest);
        let erased = ForgeQueryDomainCapabilityTarget::new(
            shared.clone().into_erased_target(),
            ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
            ForgeQueryDomainCapabilityTargetSemantics::IntentDeclaration {
                name: "test.intent".to_string(),
                strategy_name: "test.strategy".to_string(),
                strategy_version: "1".to_string(),
                input_contract: "test.contract".to_string(),
                source_lane: crate::runtime::ForgeQueryIntentSourceLane::UserAuthored,
                target_lane: crate::runtime::ForgeQueryAuthorityLane::AuthoritativeTruth,
            },
        );
        Self { shared, erased }
    }
}

impl ForgeQueryAdmittedPlanBoundContributionTarget {
    pub fn for_admitted_intent_plan(plan: &ForgeQueryAdmittedIntentPlan) -> Self {
        let shared = ForgeQueryAdmittedIntentPlanBindingTarget::for_admitted_intent_plan(plan);
        let erased = ForgeQueryDomainCapabilityTarget::new(
            shared.clone().into_erased_target(),
            ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
            ForgeQueryDomainCapabilityTargetSemantics::AdmittedIntentPlan {
                family: plan.family(),
                entrypoint: plan.entrypoint(),
                request_digest: plan.request_digest().to_string(),
                eligibility_digest: plan.eligibility_digest().to_string(),
                decision_digest: plan.decision_digest().to_string(),
            },
        );
        Self { shared, erased }
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        Self::from_digest_parts(
            target_digest,
            "test.request",
            "test.eligibility",
            "test.decision",
        )
    }

    #[cfg(test)]
    pub(crate) fn from_digest_parts(
        target_digest: impl Into<String>,
        request_digest: impl Into<String>,
        eligibility_digest: impl Into<String>,
        decision_digest: impl Into<String>,
    ) -> Self {
        let family =
            crate::intent_admission::ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent;
        let entrypoint =
            crate::intent_admission::ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent;
        let shared = ForgeQueryAdmittedIntentPlanBindingTarget::from_digest_parts(
            target_digest,
            request_digest.into(),
            eligibility_digest.into(),
            decision_digest.into(),
        );
        let (_, _, request_digest, eligibility_digest, decision_digest) = shared
            .semantics()
            .admitted_intent_plan()
            .expect("shared admitted-intent-plan target must carry admitted-plan semantics");
        let erased = ForgeQueryDomainCapabilityTarget::new(
            shared.clone().into_erased_target(),
            ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
            ForgeQueryDomainCapabilityTargetSemantics::AdmittedIntentPlan {
                family,
                entrypoint,
                request_digest: request_digest.to_string(),
                eligibility_digest: eligibility_digest.to_string(),
                decision_digest: decision_digest.to_string(),
            },
        );
        Self { shared, erased }
    }
}

impl ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    pub fn for_lower_runtime_boundary_envelope(
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        let shared =
            ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget::for_lower_runtime_boundary_envelope(
                envelope,
            );
        let erased = ForgeQueryDomainCapabilityTarget::new(
            shared.clone().into_erased_target(),
            ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
            ForgeQueryDomainCapabilityTargetSemantics::LowerRuntimeBoundaryEnvelope {
                seam_key: envelope.seam_key(),
                capability_label: envelope.capability_label(),
                crossing_classification: envelope.crossing_classification(),
                route_kind: envelope.route_kind(),
                support_posture: envelope.support_posture(),
                envelope_digest: envelope.envelope_digest().to_string(),
            },
        );
        Self { shared, erased }
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        let shared =
            ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget::from_digest(target_digest);
        let (
            _,
            capability_label,
            crossing_classification,
            route_kind,
            support_posture,
            envelope_digest,
        ) = shared
            .semantics()
            .lower_runtime_boundary()
            .expect("shared lower-runtime target must carry lower-runtime semantics");
        let erased = ForgeQueryDomainCapabilityTarget::new(
            shared.clone().into_erased_target(),
            ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
            ForgeQueryDomainCapabilityTargetSemantics::LowerRuntimeBoundaryEnvelope {
                seam_key:
                    crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule,
                capability_label,
                crossing_classification,
                route_kind,
                support_posture,
                envelope_digest: envelope_digest.to_string(),
            },
        );
        Self { shared, erased }
    }
}

impl ForgeQueryDomainCapabilityTargetBinding for ForgeQueryDeclarationBoundContributionTarget {
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.erased
    }
}

impl ForgeQueryDomainCapabilityTargetBinding for ForgeQueryAdmittedPlanBoundContributionTarget {
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.erased
    }
}

impl ForgeQueryDomainCapabilityTargetBinding
    for ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
{
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.erased
    }
}

impl crate::target_binding::sealed::Sealed for ForgeQueryDeclarationBoundContributionTarget {}
impl crate::target_binding::sealed::Sealed for ForgeQueryAdmittedPlanBoundContributionTarget {}
impl crate::target_binding::sealed::Sealed
    for ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
{
}

impl ForgeQueryBindingTargetWitness for ForgeQueryDeclarationBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::ForgeQueryBindingTarget {
        self.shared.erased_target()
    }

    fn into_erased_target(self) -> crate::target_binding::ForgeQueryBindingTarget {
        self.shared.into_erased_target()
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryAdmittedPlanBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::ForgeQueryBindingTarget {
        self.shared.erased_target()
    }

    fn into_erased_target(self) -> crate::target_binding::ForgeQueryBindingTarget {
        self.shared.into_erased_target()
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::ForgeQueryBindingTarget {
        self.shared.erased_target()
    }

    fn into_erased_target(self) -> crate::target_binding::ForgeQueryBindingTarget {
        self.shared.into_erased_target()
    }
}
