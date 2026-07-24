use worth_ui_host_contract::UiMountedPresentationAttemptIdentity;

mod outcome;
mod pending;
mod presentation;

pub struct WorthUiPendingMountedPreview<'session> {
    generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    graph: crate::graph::UiGraphAuthority<'session>,
    plan_digest: u64,
    transition: crate::runtime::UiPendingMountedPreviewTransition<'session>,
    planning_counters: crate::runtime::UiFrameworkTransitionPlanningCounters,
    ports: WorthUiMountedPreviewPorts<'session>,
}

pub struct WorthUiPreparedMountedPreview<'session> {
    frame: crate::mounting::UiPreparedMountedFrame,
    transition: crate::runtime::UiPendingMountedPreviewTransition<'session>,
    planning_counters: crate::runtime::UiFrameworkTransitionPlanningCounters,
    ports: WorthUiMountedPreviewPorts<'session>,
}

pub struct WorthUiMountedPreviewPreparationRejection<'session> {
    denial: WorthUiMountedPreviewPreparationDenial,
    pending: Box<WorthUiPendingMountedPreview<'session>>,
}

pub struct WorthUiMountedPreviewAdmissionRejection<'session> {
    denial: crate::mounting::UiMountedPresentationAdmissionDenial,
    preview: WorthUiPreparedMountedPreview<'session>,
}

pub struct WorthUiMountedPreviewInFlight<'session> {
    handle: crate::mounting::UiMountedPresentationInFlight,
    before: crate::runtime::UiAllocationTruthRevision,
    transition: crate::runtime::UiPendingMountedPreviewTransition<'session>,
    planning_counters: crate::runtime::UiFrameworkTransitionPlanningCounters,
    ports: WorthUiMountedPreviewPorts<'session>,
}

pub struct WorthUiMountedPreviewCompletionRejection<'session> {
    denial: crate::mounting::UiMountedPresentationCompletionDenial,
    in_flight: WorthUiMountedPreviewInFlight<'session>,
}

struct WorthUiMountedPreviewPorts<'session> {
    host_session: &'session crate::facade::WorthUiHostSessionAuthority,
    identity: &'session mut crate::mounting::UiMountedIdentityState,
    presentation: &'session mut crate::mounting::UiMountedPresentationCoordinator,
    reservations: &'session mut std::collections::BTreeMap<
        UiMountedPresentationAttemptIdentity,
        crate::mounting::UiMountedFramePublicationCandidate,
    >,
    observations: &'session mut crate::host_exchange::observation_report_validation::UiHostObservationReportValidation,
}

#[derive(Debug, PartialEq)]
pub enum WorthUiMountedPreviewPreparationDenial {
    UnknownMountedInstance,
    PreviewTargetMismatch,
    MissingSurfaceBinding,
    Frame(crate::mounting::UiMountedFramePreparationDenial),
}

pub enum WorthUiMountedPreviewDisposition {
    Published(crate::mounting::UiMountedFramePublicationReceipt),
    RejectedBeforeEffects(crate::mounting::UiMountedRejectedFrame),
    PresentationIndeterminate(crate::mounting::UiMountedIndeterminateFrame),
    Superseded,
}

pub struct WorthUiResolvedMountedPreview {
    disposition: WorthUiMountedPreviewDisposition,
    isolation: crate::runtime::UiPreviewPaintIsolationOutcome,
    follow_on: crate::runtime::WorthUiMountedPreviewFollowOn,
    planning_counters: crate::runtime::UiFrameworkTransitionPlanningCounters,
}

pub enum WorthUiMountedPreviewOutcome<'session> {
    Resolved(WorthUiResolvedMountedPreview),
    InFlight(WorthUiMountedPreviewInFlight<'session>),
    AdmissionDenied(WorthUiMountedPreviewAdmissionRejection<'session>),
    CompletionDenied(WorthUiMountedPreviewCompletionRejection<'session>),
}
