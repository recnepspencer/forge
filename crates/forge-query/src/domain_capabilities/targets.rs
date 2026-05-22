use crate::identity::hash_parts;
use crate::intent_admission::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionFamily,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeCrossingClassification, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSupportPosture,
};
use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration,
    ForgeQueryLowerRuntimeBoundaryEnvelope,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainCapabilityTargetKind {
    IntentDeclaration,
    AdmittedIntentPlan,
    LowerRuntimeBoundaryEnvelope,
}

impl ForgeQueryDomainCapabilityTargetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IntentDeclaration => "intent-declaration",
            Self::AdmittedIntentPlan => "admitted-intent-plan",
            Self::LowerRuntimeBoundaryEnvelope => "lower-runtime-boundary-envelope",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityTarget {
    kind: ForgeQueryDomainCapabilityTargetKind,
    target_digest: String,
    binding_digest: String,
    semantics: ForgeQueryDomainCapabilityTargetSemantics,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainCapabilityTargetSemantics {
    IntentDeclaration {
        name: String,
        strategy_name: String,
        strategy_version: String,
        input_contract: String,
        source_lane: crate::runtime::ForgeQueryIntentSourceLane,
        target_lane: crate::runtime::ForgeQueryAuthorityLane,
    },
    AdmittedIntentPlan {
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        request_digest: String,
        eligibility_digest: String,
        decision_digest: String,
    },
    LowerRuntimeBoundaryEnvelope {
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        capability_label: &'static str,
        crossing_classification: ForgeQueryLowerRuntimeCrossingClassification,
        route_kind: ForgeQueryLowerRuntimeRouteKind,
        support_posture: ForgeQueryLowerRuntimeSupportPosture,
        envelope_digest: String,
    },
}

impl ForgeQueryDomainCapabilityTargetSemantics {
    pub fn intent_declaration(
        &self,
    ) -> Option<(
        &str,
        &str,
        &str,
        &str,
        crate::runtime::ForgeQueryIntentSourceLane,
        crate::runtime::ForgeQueryAuthorityLane,
    )> {
        match self {
            Self::IntentDeclaration {
                name,
                strategy_name,
                strategy_version,
                input_contract,
                source_lane,
                target_lane,
            } => Some((
                name.as_str(),
                strategy_name.as_str(),
                strategy_version.as_str(),
                input_contract.as_str(),
                *source_lane,
                *target_lane,
            )),
            _ => None,
        }
    }

    pub fn admitted_intent_plan(
        &self,
    ) -> Option<(
        ForgeQueryIntentAdmissionFamily,
        ForgeQueryIntentAdmissionCoveredEntrypoint,
        &str,
        &str,
        &str,
    )> {
        match self {
            Self::AdmittedIntentPlan {
                family,
                entrypoint,
                request_digest,
                eligibility_digest,
                decision_digest,
            } => Some((
                *family,
                *entrypoint,
                request_digest.as_str(),
                eligibility_digest.as_str(),
                decision_digest.as_str(),
            )),
            _ => None,
        }
    }

    pub fn lower_runtime_boundary(
        &self,
    ) -> Option<(
        ForgeQueryLowerRuntimeSeamKey,
        &'static str,
        ForgeQueryLowerRuntimeCrossingClassification,
        ForgeQueryLowerRuntimeRouteKind,
        ForgeQueryLowerRuntimeSupportPosture,
        &str,
    )> {
        match self {
            Self::LowerRuntimeBoundaryEnvelope {
                seam_key,
                capability_label,
                crossing_classification,
                route_kind,
                support_posture,
                envelope_digest,
            } => Some((
                *seam_key,
                *capability_label,
                *crossing_classification,
                *route_kind,
                *support_posture,
                envelope_digest.as_str(),
            )),
            _ => None,
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

pub trait ForgeQueryDomainCapabilityTargetBinding: Clone + sealed::Sealed {
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget;
    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget;

    fn kind(&self) -> ForgeQueryDomainCapabilityTargetKind {
        self.erased_target().kind()
    }

    fn target_digest(&self) -> &str {
        self.erased_target().target_digest()
    }

    fn binding_digest(&self) -> &str {
        self.erased_target().binding_digest()
    }

    fn semantics(&self) -> &ForgeQueryDomainCapabilityTargetSemantics {
        self.erased_target().semantics()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationBoundContributionTarget(ForgeQueryDomainCapabilityTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanBoundContributionTarget(ForgeQueryDomainCapabilityTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundaryBoundContributionTarget(ForgeQueryDomainCapabilityTarget);

impl ForgeQueryDomainCapabilityTarget {
    pub fn for_intent_declaration(declaration: &ForgeQueryIntentDeclaration) -> Self {
        let target_digest = declaration.input_digest();
        Self::new(
            ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
            target_digest,
            ForgeQueryDomainCapabilityTargetSemantics::IntentDeclaration {
                name: declaration.name().to_string(),
                strategy_name: declaration.strategy_name().to_string(),
                strategy_version: declaration.strategy_version().to_string(),
                input_contract: declaration.input_contract().to_string(),
                source_lane: declaration.source_lane(),
                target_lane: declaration.target_lane(),
            },
        )
    }

    pub fn for_admitted_intent_plan(plan: &ForgeQueryAdmittedIntentPlan) -> Self {
        let target_digest = plan.decision_digest().to_string();
        Self::new(
            ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
            target_digest,
            ForgeQueryDomainCapabilityTargetSemantics::AdmittedIntentPlan {
                family: plan.family(),
                entrypoint: plan.entrypoint(),
                request_digest: plan.request_digest().to_string(),
                eligibility_digest: plan.eligibility_digest().to_string(),
                decision_digest: plan.decision_digest().to_string(),
            },
        )
    }

    pub fn for_lower_runtime_boundary_envelope(
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        let target_digest = envelope.envelope_digest().to_string();
        Self::new(
            ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
            target_digest,
            ForgeQueryDomainCapabilityTargetSemantics::LowerRuntimeBoundaryEnvelope {
                seam_key: envelope.seam_key(),
                capability_label: envelope.capability_label(),
                crossing_classification: envelope.crossing_classification(),
                route_kind: envelope.route_kind(),
                support_posture: envelope.support_posture(),
                envelope_digest: envelope.envelope_digest().to_string(),
            },
        )
    }

    pub fn kind(&self) -> ForgeQueryDomainCapabilityTargetKind {
        self.kind
    }

    pub fn target_digest(&self) -> &str {
        &self.target_digest
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn semantics(&self) -> &ForgeQueryDomainCapabilityTargetSemantics {
        &self.semantics
    }

    #[cfg(test)]
    pub(crate) fn from_digest(
        kind: ForgeQueryDomainCapabilityTargetKind,
        target_digest: impl Into<String>,
        semantics: ForgeQueryDomainCapabilityTargetSemantics,
    ) -> Self {
        let target_digest = target_digest.into();
        Self::new(kind, target_digest, semantics)
    }

    fn new(
        kind: ForgeQueryDomainCapabilityTargetKind,
        target_digest: String,
        semantics: ForgeQueryDomainCapabilityTargetSemantics,
    ) -> Self {
        let binding_digest = hash_parts(&[
            "forge_query_domain_capability_target_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("target:{target_digest}"),
        ]);
        Self {
            kind,
            target_digest,
            binding_digest,
            semantics,
        }
    }
}

impl ForgeQueryDeclarationBoundContributionTarget {
    pub fn for_intent_declaration(declaration: &ForgeQueryIntentDeclaration) -> Self {
        Self(ForgeQueryDomainCapabilityTarget::for_intent_declaration(
            declaration,
        ))
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        Self(ForgeQueryDomainCapabilityTarget::from_digest(
            ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
            target_digest.into(),
            ForgeQueryDomainCapabilityTargetSemantics::IntentDeclaration {
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

impl ForgeQueryAdmittedPlanBoundContributionTarget {
    pub fn for_admitted_intent_plan(plan: &ForgeQueryAdmittedIntentPlan) -> Self {
        Self(ForgeQueryDomainCapabilityTarget::for_admitted_intent_plan(
            plan,
        ))
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
        Self(ForgeQueryDomainCapabilityTarget::from_digest(
            ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
            target_digest.into(),
            ForgeQueryDomainCapabilityTargetSemantics::AdmittedIntentPlan {
                family: ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
                entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
                request_digest: request_digest.into(),
                eligibility_digest: eligibility_digest.into(),
                decision_digest: decision_digest.into(),
            },
        ))
    }
}

impl ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    pub fn for_lower_runtime_boundary_envelope(
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        Self(ForgeQueryDomainCapabilityTarget::for_lower_runtime_boundary_envelope(envelope))
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        Self(ForgeQueryDomainCapabilityTarget::from_digest(
            ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
            target_digest.into(),
            ForgeQueryDomainCapabilityTargetSemantics::LowerRuntimeBoundaryEnvelope {
                seam_key: ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule,
                capability_label: "test.capability",
                crossing_classification:
                    ForgeQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse,
                route_kind: ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
                support_posture: ForgeQueryLowerRuntimeSupportPosture::Admitted,
                envelope_digest: "test.envelope".to_string(),
            },
        ))
    }
}

impl sealed::Sealed for ForgeQueryDeclarationBoundContributionTarget {}
impl sealed::Sealed for ForgeQueryAdmittedPlanBoundContributionTarget {}
impl sealed::Sealed for ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {}
impl sealed::Sealed for ForgeQueryDomainCapabilityTarget {}

impl ForgeQueryDomainCapabilityTargetBinding for ForgeQueryDeclarationBoundContributionTarget {
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.0
    }
}

impl ForgeQueryDomainCapabilityTargetBinding for ForgeQueryAdmittedPlanBoundContributionTarget {
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.0
    }
}

impl ForgeQueryDomainCapabilityTargetBinding
    for ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
{
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.0
    }
}
