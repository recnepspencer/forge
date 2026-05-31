use super::explanation::ForgeQueryRecoveryExplanation;
use super::family::{
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryBasisPosture,
    ForgeQueryRecoveryConflictPosture, ForgeQueryRecoveryEvidenceStrength,
    ForgeQueryRecoverySourceFamily,
};
use super::request::{ForgeQueryRecoveryRequest, ForgeQueryRecoveryRequestKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryStopFamily {
    Binding,
    Continuation,
    ContributionComposedOrchestration,
    DeclarationEntry,
    DeclarationReceipt,
    DeclarationRoutePlan,
    GroupedNeighborhoodOrchestration,
    SignalCompatibilityOrchestration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryStopKind {
    Ambiguous,
    AspectConflict,
    AuthorityMismatch,
    BasisMismatch,
    ContributionDenied,
    DeclarationDenied,
    Deferred,
    Failed,
    MissingRequiredAspect,
    RebindRequired,
    Refused,
    Stale,
    Unavailable,
    Unsupported,
    WrongHandle,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryAuthoritySurface {
    AdmittedOperatingWorld,
    AutomationBoundary,
    AvailabilityDiscovery,
    BoundInputContext,
    ContributionComposition,
    DeclarationMeaning,
    FailureEscalation,
    HandleIdentity,
    InputNarrowing,
    SignalCompatibility,
    SupportReadiness,
    TruthContinuationContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryAction {
    CheckSupport,
    CorrectHandle,
    CorrectWorld,
    EscalateFailure,
    GatherAvailability,
    InspectCheckedLane,
    InspectProofLane,
    NarrowInput,
    RebindContext,
    RefreshBasis,
    RepairDeclarationMeaning,
    RetryLater,
    ReviewContributionIntent,
    UseExplicitHandoff,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRecoveryBrief {
    stop_family: ForgeQueryRecoveryStopFamily,
    stop_kind: ForgeQueryRecoveryStopKind,
    authority_surface: ForgeQueryRecoveryAuthoritySurface,
    recommended_action: ForgeQueryRecoveryAction,
    reason: String,
    explanation: ForgeQueryRecoveryExplanation,
    recovery_request: ForgeQueryRecoveryRequest,
}

impl ForgeQueryRecoveryBrief {
    pub(crate) fn new(
        stop_family: ForgeQueryRecoveryStopFamily,
        stop_kind: ForgeQueryRecoveryStopKind,
        authority_surface: ForgeQueryRecoveryAuthoritySurface,
        recommended_action: ForgeQueryRecoveryAction,
        reason: impl Into<String>,
        explanation: ForgeQueryRecoveryExplanation,
    ) -> Self {
        let recovery_request =
            ForgeQueryRecoveryRequest::new(request_kind(recommended_action), explanation.clone());
        Self {
            stop_family,
            stop_kind,
            authority_surface,
            recommended_action,
            reason: reason.into(),
            explanation,
            recovery_request,
        }
    }

    pub fn stop_family(&self) -> ForgeQueryRecoveryStopFamily {
        self.stop_family
    }

    pub fn stop_kind(&self) -> ForgeQueryRecoveryStopKind {
        self.stop_kind
    }

    pub fn authority_surface(&self) -> ForgeQueryRecoveryAuthoritySurface {
        self.authority_surface
    }

    pub fn recommended_action(&self) -> ForgeQueryRecoveryAction {
        self.recommended_action
    }

    pub fn source_family(&self) -> ForgeQueryRecoverySourceFamily {
        self.explanation.source_family()
    }

    pub fn evidence_strength(&self) -> ForgeQueryRecoveryEvidenceStrength {
        self.explanation.evidence_strength()
    }

    pub fn basis_posture(&self) -> ForgeQueryRecoveryBasisPosture {
        self.explanation.basis_posture()
    }

    pub fn aspect_posture(&self) -> ForgeQueryRecoveryAspectPosture {
        self.explanation.aspect_posture()
    }

    pub fn conflict_posture(&self) -> ForgeQueryRecoveryConflictPosture {
        self.explanation.conflict_posture()
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn explanation(&self) -> &ForgeQueryRecoveryExplanation {
        &self.explanation
    }

    pub fn route_sensitive_explanation(&self) -> &ForgeQueryRecoveryExplanation {
        &self.explanation
    }

    pub fn recovery_request(&self) -> &ForgeQueryRecoveryRequest {
        &self.recovery_request
    }

    pub(crate) fn with_stop_family(mut self, stop_family: ForgeQueryRecoveryStopFamily) -> Self {
        self.stop_family = stop_family;
        self
    }

    pub(crate) fn with_explanation(mut self, explanation: ForgeQueryRecoveryExplanation) -> Self {
        self.explanation = explanation.clone();
        self.recovery_request =
            ForgeQueryRecoveryRequest::new(request_kind(self.recommended_action), explanation);
        self
    }
}

fn request_kind(action: ForgeQueryRecoveryAction) -> ForgeQueryRecoveryRequestKind {
    match action {
        ForgeQueryRecoveryAction::CheckSupport => ForgeQueryRecoveryRequestKind::CheckSupport,
        ForgeQueryRecoveryAction::CorrectHandle => ForgeQueryRecoveryRequestKind::CorrectHandle,
        ForgeQueryRecoveryAction::CorrectWorld => ForgeQueryRecoveryRequestKind::CorrectWorld,
        ForgeQueryRecoveryAction::EscalateFailure => ForgeQueryRecoveryRequestKind::EscalateFailure,
        ForgeQueryRecoveryAction::GatherAvailability => {
            ForgeQueryRecoveryRequestKind::GatherAvailability
        }
        ForgeQueryRecoveryAction::InspectCheckedLane => {
            ForgeQueryRecoveryRequestKind::InspectCheckedLane
        }
        ForgeQueryRecoveryAction::InspectProofLane => {
            ForgeQueryRecoveryRequestKind::InspectProofLane
        }
        ForgeQueryRecoveryAction::NarrowInput => ForgeQueryRecoveryRequestKind::NarrowInput,
        ForgeQueryRecoveryAction::RebindContext => ForgeQueryRecoveryRequestKind::RebindContext,
        ForgeQueryRecoveryAction::RefreshBasis => ForgeQueryRecoveryRequestKind::RefreshBasis,
        ForgeQueryRecoveryAction::RepairDeclarationMeaning => {
            ForgeQueryRecoveryRequestKind::RepairDeclarationMeaning
        }
        ForgeQueryRecoveryAction::RetryLater => ForgeQueryRecoveryRequestKind::RetryLater,
        ForgeQueryRecoveryAction::ReviewContributionIntent => {
            ForgeQueryRecoveryRequestKind::ReviewContributionIntent
        }
        ForgeQueryRecoveryAction::UseExplicitHandoff => {
            ForgeQueryRecoveryRequestKind::UseExplicitHandoff
        }
    }
}
