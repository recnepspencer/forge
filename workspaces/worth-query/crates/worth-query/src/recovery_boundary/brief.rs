use super::explanation::WorthQueryRecoveryExplanation;
use super::family::{
    WorthQueryRecoveryAspectPosture, WorthQueryRecoveryBasisPosture,
    WorthQueryRecoveryConflictPosture, WorthQueryRecoveryEvidenceStrength,
    WorthQueryRecoverySourceFamily,
};
use super::request::{WorthQueryRecoveryRequest, WorthQueryRecoveryRequestKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRecoveryStopFamily {
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
pub enum WorthQueryRecoveryStopKind {
    Ambiguous,
    AsyncRequestDrift,
    AspectConflict,
    AuthorityMismatch,
    BasisMismatch,
    ContributionDenied,
    DeclarationDenied,
    Deferred,
    Failed,
    MissingRequiredAspect,
    PreviewCrossedResidue,
    RebindRequired,
    RemaskDrift,
    ReplayDrift,
    Refused,
    Stale,
    StaleCompletion,
    Unavailable,
    Unsupported,
    WrongHandle,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRecoveryAuthoritySurface {
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
pub enum WorthQueryRecoveryAction {
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
pub struct WorthQueryRecoveryBrief {
    stop_family: WorthQueryRecoveryStopFamily,
    stop_kind: WorthQueryRecoveryStopKind,
    authority_surface: WorthQueryRecoveryAuthoritySurface,
    recommended_action: WorthQueryRecoveryAction,
    reason: String,
    explanation: WorthQueryRecoveryExplanation,
    recovery_request: WorthQueryRecoveryRequest,
}

impl WorthQueryRecoveryBrief {
    pub(crate) fn new(
        stop_family: WorthQueryRecoveryStopFamily,
        stop_kind: WorthQueryRecoveryStopKind,
        authority_surface: WorthQueryRecoveryAuthoritySurface,
        recommended_action: WorthQueryRecoveryAction,
        reason: impl Into<String>,
        explanation: WorthQueryRecoveryExplanation,
    ) -> Self {
        let recovery_request =
            WorthQueryRecoveryRequest::new(request_kind(recommended_action), explanation.clone());
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

    pub fn stop_family(&self) -> WorthQueryRecoveryStopFamily {
        self.stop_family
    }

    pub fn stop_kind(&self) -> WorthQueryRecoveryStopKind {
        self.stop_kind
    }

    pub fn authority_surface(&self) -> WorthQueryRecoveryAuthoritySurface {
        self.authority_surface
    }

    pub fn recommended_action(&self) -> WorthQueryRecoveryAction {
        self.recommended_action
    }

    pub fn source_family(&self) -> WorthQueryRecoverySourceFamily {
        self.explanation.source_family()
    }

    pub fn evidence_strength(&self) -> WorthQueryRecoveryEvidenceStrength {
        self.explanation.evidence_strength()
    }

    pub fn basis_posture(&self) -> WorthQueryRecoveryBasisPosture {
        self.explanation.basis_posture()
    }

    pub fn aspect_posture(&self) -> WorthQueryRecoveryAspectPosture {
        self.explanation.aspect_posture()
    }

    pub fn conflict_posture(&self) -> WorthQueryRecoveryConflictPosture {
        self.explanation.conflict_posture()
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn explanation(&self) -> &WorthQueryRecoveryExplanation {
        &self.explanation
    }

    pub fn route_sensitive_explanation(&self) -> &WorthQueryRecoveryExplanation {
        &self.explanation
    }

    pub fn recovery_request(&self) -> &WorthQueryRecoveryRequest {
        &self.recovery_request
    }

    pub(crate) fn with_stop_family(mut self, stop_family: WorthQueryRecoveryStopFamily) -> Self {
        self.stop_family = stop_family;
        self
    }

    pub(crate) fn with_explanation(mut self, explanation: WorthQueryRecoveryExplanation) -> Self {
        self.explanation = explanation.clone();
        self.recovery_request =
            WorthQueryRecoveryRequest::new(request_kind(self.recommended_action), explanation);
        self
    }
}

fn request_kind(action: WorthQueryRecoveryAction) -> WorthQueryRecoveryRequestKind {
    match action {
        WorthQueryRecoveryAction::CheckSupport => WorthQueryRecoveryRequestKind::CheckSupport,
        WorthQueryRecoveryAction::CorrectHandle => WorthQueryRecoveryRequestKind::CorrectHandle,
        WorthQueryRecoveryAction::CorrectWorld => WorthQueryRecoveryRequestKind::CorrectWorld,
        WorthQueryRecoveryAction::EscalateFailure => WorthQueryRecoveryRequestKind::EscalateFailure,
        WorthQueryRecoveryAction::GatherAvailability => {
            WorthQueryRecoveryRequestKind::GatherAvailability
        }
        WorthQueryRecoveryAction::InspectCheckedLane => {
            WorthQueryRecoveryRequestKind::InspectCheckedLane
        }
        WorthQueryRecoveryAction::InspectProofLane => {
            WorthQueryRecoveryRequestKind::InspectProofLane
        }
        WorthQueryRecoveryAction::NarrowInput => WorthQueryRecoveryRequestKind::NarrowInput,
        WorthQueryRecoveryAction::RebindContext => WorthQueryRecoveryRequestKind::RebindContext,
        WorthQueryRecoveryAction::RefreshBasis => WorthQueryRecoveryRequestKind::RefreshBasis,
        WorthQueryRecoveryAction::RepairDeclarationMeaning => {
            WorthQueryRecoveryRequestKind::RepairDeclarationMeaning
        }
        WorthQueryRecoveryAction::RetryLater => WorthQueryRecoveryRequestKind::RetryLater,
        WorthQueryRecoveryAction::ReviewContributionIntent => {
            WorthQueryRecoveryRequestKind::ReviewContributionIntent
        }
        WorthQueryRecoveryAction::UseExplicitHandoff => {
            WorthQueryRecoveryRequestKind::UseExplicitHandoff
        }
    }
}
