pub(super) mod mounted_identity;
mod observation_ingress;
mod presentation_semantic_subscriber;

pub use observation_ingress::UiNativeClientObservationIngressObservation;
pub use presentation_semantic_subscriber::UiNativeClientPresentationSemanticSubscriberObservation;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UiNativeClientShutdownObservation {
    managed_semantic_resources_closed: u64,
    managed_semantic_resources_complete: bool,
    presentation_transitions: Box<[UiNativeClientPresentationTransitionObservation]>,
    presentation_transition_trace_complete: bool,
    presentation_semantic_frontiers: Box<[UiNativeClientPresentationSemanticFrontierObservation]>,
    presentation_semantic_frontier_trace_complete: bool,
    text_presentation_work: Box<[UiNativeClientTextPresentationWorkObservation]>,
    text_presentation_work_trace_complete: bool,
    authored_mounted_instances: mounted_identity::Observations,
    derived_state_reconstruction:
        Option<super::UiNativeClientDerivedStateReconstructionObservation>,
    resources: super::UiNativeClientResourceObservation,
    observation_ingress: UiNativeClientObservationIngressObservation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeClientPresentationSemanticChange {
    Content,
    Width,
    PaintValue,
    PaintBoundary,
    Dpi,
    UploadCompletion,
    PinRelease,
    Currentness,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeClientConditionalOutcome {
    ComputedChanged,
    ComputedRevertedClean,
    DependencyUnchanged,
    Suppressed,
    DeferredByCondition,
    DeferredTemporal,
    DeferredOnDemand,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeClientPresentationSemanticFrontierObservation {
    change: UiNativeClientPresentationSemanticChange,
    subscribers: Box<[UiNativeClientPresentationSemanticSubscriberObservation]>,
    source_deliveries: u32,
    outcomes: Box<[UiNativeClientConditionalOutcome]>,
    performed_counter_rows: Box<[[u64; 24]]>,
    scope_rejections: [u64; 4],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeClientPresentationTransitionKind {
    Pending,
    Superseded,
    StaleCompletionRejected,
    Completed,
    DuplicateCompletionRejected,
    Cancelled,
    Unresolved,
    RecoveryRequired,
    ReconstructionCurrent,
    TerminalClosed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeClientPresentationTransitionObservation {
    kind: UiNativeClientPresentationTransitionKind,
    attempt: u64,
    binding: u64,
}

impl UiNativeClientShutdownObservation {
    pub fn from_client(
        managed_semantic_resources_closed: u64,
        managed_semantic_resources_complete: bool,
    ) -> Self {
        Self {
            managed_semantic_resources_closed,
            managed_semantic_resources_complete,
            presentation_transitions: Box::new([]),
            presentation_transition_trace_complete: true,
            presentation_semantic_frontiers: Box::new([]),
            presentation_semantic_frontier_trace_complete: true,
            text_presentation_work: Box::new([]),
            text_presentation_work_trace_complete: true,
            authored_mounted_instances: Box::new([]),
            derived_state_reconstruction: None,
            resources: Default::default(),
            observation_ingress: Default::default(),
        }
    }

    pub fn from_client_with_presentation_transitions(
        managed_semantic_resources_closed: u64,
        managed_semantic_resources_complete: bool,
        presentation_transitions: Box<[UiNativeClientPresentationTransitionObservation]>,
        presentation_transition_trace_complete: bool,
    ) -> Self {
        Self::from_client_with_presentation_evidence(
            managed_semantic_resources_closed,
            managed_semantic_resources_complete,
            presentation_transitions,
            presentation_transition_trace_complete,
            Box::new([]),
            true,
        )
    }

    pub fn from_client_with_presentation_evidence(
        managed_semantic_resources_closed: u64,
        managed_semantic_resources_complete: bool,
        presentation_transitions: Box<[UiNativeClientPresentationTransitionObservation]>,
        presentation_transition_trace_complete: bool,
        presentation_semantic_frontiers: Box<
            [UiNativeClientPresentationSemanticFrontierObservation],
        >,
        presentation_semantic_frontier_trace_complete: bool,
    ) -> Self {
        Self {
            managed_semantic_resources_closed,
            managed_semantic_resources_complete,
            presentation_transitions,
            presentation_transition_trace_complete,
            presentation_semantic_frontiers,
            presentation_semantic_frontier_trace_complete,
            text_presentation_work: Box::new([]),
            text_presentation_work_trace_complete: true,
            authored_mounted_instances: Box::new([]),
            derived_state_reconstruction: None,
            resources: Default::default(),
            observation_ingress: Default::default(),
        }
    }

    pub fn with_text_presentation_work(
        mut self,
        observations: Box<[UiNativeClientTextPresentationWorkObservation]>,
        trace_complete: bool,
    ) -> Self {
        self.text_presentation_work = observations;
        self.text_presentation_work_trace_complete = trace_complete;
        self
    }

    pub fn with_derived_state_reconstruction(
        mut self,
        observation: Option<super::UiNativeClientDerivedStateReconstructionObservation>,
    ) -> Self {
        self.derived_state_reconstruction = observation;
        self
    }

    pub const fn with_resources(
        mut self,
        resources: super::UiNativeClientResourceObservation,
    ) -> Self {
        self.resources = resources;
        self
    }

    pub const fn with_observation_ingress(
        mut self,
        observation_ingress: UiNativeClientObservationIngressObservation,
    ) -> Self {
        self.observation_ingress = observation_ingress;
        self
    }

    pub const fn managed_semantic_resources_closed(&self) -> u64 {
        self.managed_semantic_resources_closed
    }

    pub const fn managed_semantic_resources_complete(&self) -> bool {
        self.managed_semantic_resources_complete
    }

    pub fn presentation_transitions(&self) -> &[UiNativeClientPresentationTransitionObservation] {
        &self.presentation_transitions
    }

    pub const fn presentation_transition_trace_complete(&self) -> bool {
        self.presentation_transition_trace_complete
    }

    pub fn presentation_semantic_frontiers(
        &self,
    ) -> &[UiNativeClientPresentationSemanticFrontierObservation] {
        &self.presentation_semantic_frontiers
    }

    pub const fn presentation_semantic_frontier_trace_complete(&self) -> bool {
        self.presentation_semantic_frontier_trace_complete
    }

    pub fn text_presentation_work(&self) -> &[UiNativeClientTextPresentationWorkObservation] {
        &self.text_presentation_work
    }

    pub const fn text_presentation_work_trace_complete(&self) -> bool {
        self.text_presentation_work_trace_complete
    }

    pub const fn derived_state_reconstruction(
        &self,
    ) -> Option<super::UiNativeClientDerivedStateReconstructionObservation> {
        self.derived_state_reconstruction
    }

    pub const fn resources(&self) -> super::UiNativeClientResourceObservation {
        self.resources
    }

    pub const fn observation_ingress(&self) -> UiNativeClientObservationIngressObservation {
        self.observation_ingress
    }
}

impl UiNativeClientPresentationSemanticFrontierObservation {
    pub fn reported(
        change: UiNativeClientPresentationSemanticChange,
        subscribers: impl IntoIterator<Item = UiNativeClientPresentationSemanticSubscriberObservation>,
        source_deliveries: u32,
        outcomes: impl IntoIterator<Item = UiNativeClientConditionalOutcome>,
        performed_counter_rows: impl IntoIterator<Item = [u64; 24]>,
        scope_rejections: [u64; 4],
    ) -> Self {
        Self {
            change,
            subscribers: subscribers.into_iter().collect(),
            source_deliveries,
            outcomes: outcomes.into_iter().collect(),
            performed_counter_rows: performed_counter_rows.into_iter().collect(),
            scope_rejections,
        }
    }

    pub const fn change(&self) -> UiNativeClientPresentationSemanticChange {
        self.change
    }

    pub fn outcomes(&self) -> &[UiNativeClientConditionalOutcome] {
        &self.outcomes
    }

    pub fn subscribers(&self) -> &[UiNativeClientPresentationSemanticSubscriberObservation] {
        &self.subscribers
    }

    pub const fn source_deliveries(&self) -> u32 {
        self.source_deliveries
    }

    pub fn performed_counter_rows(&self) -> &[[u64; 24]] {
        &self.performed_counter_rows
    }

    pub const fn scope_rejections(&self) -> [u64; 4] {
        self.scope_rejections
    }
}

impl UiNativeClientPresentationTransitionObservation {
    pub const fn reported(
        kind: UiNativeClientPresentationTransitionKind,
        attempt: u64,
        binding: u64,
    ) -> Self {
        Self {
            kind,
            attempt,
            binding,
        }
    }

    pub const fn kind(self) -> UiNativeClientPresentationTransitionKind {
        self.kind
    }

    pub const fn attempt(self) -> u64 {
        self.attempt
    }

    pub const fn binding(self) -> u64 {
        self.binding
    }
}
mod text_presentation_work;

pub use text_presentation_work::{
    UiNativeClientPresentationMechanicIdentityObservation,
    UiNativeClientTextPresentationWorkObservation,
};
