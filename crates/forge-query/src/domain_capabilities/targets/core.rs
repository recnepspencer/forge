use crate::intent_admission::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionFamily,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeCrossingClassification, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSupportPosture,
};
use crate::target_binding::{
    ForgeQueryBindingTarget, ForgeQueryBindingTargetKind, ForgeQueryBindingTargetSemantics,
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

    fn from_shared(kind: ForgeQueryBindingTargetKind) -> Option<Self> {
        match kind {
            ForgeQueryBindingTargetKind::IntentDeclaration => Some(Self::IntentDeclaration),
            ForgeQueryBindingTargetKind::AdmittedIntentPlan => Some(Self::AdmittedIntentPlan),
            ForgeQueryBindingTargetKind::LowerRuntimeBoundaryEnvelope => {
                Some(Self::LowerRuntimeBoundaryEnvelope)
            }
            _ => None,
        }
    }
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
    fn from_shared(shared: &ForgeQueryBindingTargetSemantics) -> Option<Self> {
        match shared {
            ForgeQueryBindingTargetSemantics::IntentDeclaration {
                name,
                strategy_name,
                strategy_version,
                input_contract,
                source_lane,
                target_lane,
            } => Some(Self::IntentDeclaration {
                name: name.clone(),
                strategy_name: strategy_name.clone(),
                strategy_version: strategy_version.clone(),
                input_contract: input_contract.clone(),
                source_lane: *source_lane,
                target_lane: *target_lane,
            }),
            ForgeQueryBindingTargetSemantics::AdmittedIntentPlan {
                family,
                entrypoint,
                request_digest,
                eligibility_digest,
                decision_digest,
            } => Some(Self::AdmittedIntentPlan {
                family: *family,
                entrypoint: *entrypoint,
                request_digest: request_digest.clone(),
                eligibility_digest: eligibility_digest.clone(),
                decision_digest: decision_digest.clone(),
            }),
            ForgeQueryBindingTargetSemantics::LowerRuntimeBoundaryEnvelope {
                seam_key,
                capability_label,
                crossing_classification,
                route_kind,
                support_posture,
                envelope_digest,
            } => Some(Self::LowerRuntimeBoundaryEnvelope {
                seam_key: *seam_key,
                capability_label,
                crossing_classification: *crossing_classification,
                route_kind: *route_kind,
                support_posture: *support_posture,
                envelope_digest: envelope_digest.clone(),
            }),
            _ => None,
        }
    }

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityTarget {
    shared: ForgeQueryBindingTarget,
    semantics: ForgeQueryDomainCapabilityTargetSemantics,
}

impl ForgeQueryDomainCapabilityTarget {
    pub(crate) fn from_shared(shared: ForgeQueryBindingTarget) -> Option<Self> {
        let semantics = ForgeQueryDomainCapabilityTargetSemantics::from_shared(shared.semantics())?;
        Some(Self { shared, semantics })
    }

    pub fn kind(&self) -> ForgeQueryDomainCapabilityTargetKind {
        ForgeQueryDomainCapabilityTargetKind::from_shared(self.shared.kind())
            .expect("domain-capability target must wrap a compatible shared target kind")
    }

    pub fn target_digest(&self) -> &str {
        self.shared.target_digest()
    }

    pub fn binding_digest(&self) -> &str {
        self.shared.binding_digest()
    }

    pub fn semantics(&self) -> &ForgeQueryDomainCapabilityTargetSemantics {
        &self.semantics
    }

    pub(crate) fn shared(&self) -> &ForgeQueryBindingTarget {
        &self.shared
    }

    pub(crate) fn into_shared(self) -> ForgeQueryBindingTarget {
        self.shared
    }
}

pub trait ForgeQueryDomainCapabilityTargetBinding: Clone {
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
