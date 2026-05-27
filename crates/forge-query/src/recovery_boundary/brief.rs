use super::explanation::ForgeQueryRecoveryExplanation;
use super::request::{ForgeQueryRecoveryRequest, ForgeQueryRecoveryRequestKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryStopFamily {
    Binding,
    Continuation,
    ContributionComposedOrchestration,
    DeclarationEntry,
    DeclarationReceipt,
    DeclarationRoutePlan,
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

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn route_sensitive_explanation(&self) -> &ForgeQueryRecoveryExplanation {
        &self.explanation
    }

    pub fn recovery_request(&self) -> &ForgeQueryRecoveryRequest {
        &self.recovery_request
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
