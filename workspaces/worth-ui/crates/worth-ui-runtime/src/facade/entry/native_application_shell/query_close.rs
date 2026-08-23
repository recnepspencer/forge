pub(crate) struct UiNativeApplicationQueryCloseObservation {
    closed_query_resources: u64,
    transitions: Box<[worth_ui_query_binding::WorthUiPresentationTransitionObservation]>,
    semantic_frontiers:
        Box<[worth_ui_query_binding::WorthUiPresentationSemanticFrontierObservation]>,
    semantic_frontier_trace_complete: bool,
    text_presentation_work:
        Box<[crate::native_platform::text_presentation::UiNativeTextPresentationWorkObservation]>,
    text_presentation_work_trace_complete: bool,
    authored_mounted_instances:
        Box<[worth_ui_host_native::UiNativeClientAuthoredMountedInstanceObservation]>,
    client_resource_peaks: [usize; 2],
    complete: bool,
}

pub(super) struct UiNativeApplicationQueryCloseInput {
    pub(super) closed_resources: u64,
    pub(super) transitions: Box<[worth_ui_query_binding::WorthUiPresentationTransitionObservation]>,
    pub(super) semantic_frontiers:
        Box<[worth_ui_query_binding::WorthUiPresentationSemanticFrontierObservation]>,
    pub(super) semantic_frontier_trace_complete: bool,
    pub(super) text_work:
        Box<[crate::native_platform::text_presentation::UiNativeTextPresentationWorkObservation]>,
    pub(super) text_work_trace_complete: bool,
    pub(super) authored_mounted_instances:
        Box<[worth_ui_host_native::UiNativeClientAuthoredMountedInstanceObservation]>,
    pub(super) client_resource_peaks: [usize; 2],
    pub(super) complete: bool,
}

impl UiNativeApplicationQueryCloseObservation {
    pub(super) fn from_runtime(input: UiNativeApplicationQueryCloseInput) -> Self {
        Self {
            closed_query_resources: input.closed_resources,
            transitions: input.transitions,
            semantic_frontiers: input.semantic_frontiers,
            semantic_frontier_trace_complete: input.semantic_frontier_trace_complete,
            text_presentation_work: input.text_work,
            text_presentation_work_trace_complete: input.text_work_trace_complete,
            authored_mounted_instances: input.authored_mounted_instances,
            client_resource_peaks: input.client_resource_peaks,
            complete: input.complete,
        }
    }

    pub(crate) fn empty_complete() -> Self {
        Self::from_runtime(UiNativeApplicationQueryCloseInput {
            closed_resources: 0,
            transitions: Box::new([]),
            semantic_frontiers: Box::new([]),
            semantic_frontier_trace_complete: true,
            text_work: Box::new([]),
            text_work_trace_complete: true,
            authored_mounted_instances: Box::new([]),
            client_resource_peaks: [0, 0],
            complete: true,
        })
    }

    pub(crate) const fn closed_query_resources(&self) -> u64 {
        self.closed_query_resources
    }

    pub(crate) fn transitions(
        &self,
    ) -> &[worth_ui_query_binding::WorthUiPresentationTransitionObservation] {
        &self.transitions
    }

    pub(crate) const fn complete(&self) -> bool {
        self.complete
    }

    pub(crate) fn semantic_frontiers(
        &self,
    ) -> &[worth_ui_query_binding::WorthUiPresentationSemanticFrontierObservation] {
        &self.semantic_frontiers
    }

    pub(crate) const fn semantic_frontier_trace_complete(&self) -> bool {
        self.semantic_frontier_trace_complete
    }

    pub(crate) fn text_presentation_work(
        &self,
    ) -> &[crate::native_platform::text_presentation::UiNativeTextPresentationWorkObservation] {
        &self.text_presentation_work
    }

    pub(crate) const fn text_presentation_work_trace_complete(&self) -> bool {
        self.text_presentation_work_trace_complete
    }

    pub(crate) fn authored_mounted_instances(
        &self,
    ) -> &[worth_ui_host_native::UiNativeClientAuthoredMountedInstanceObservation] {
        &self.authored_mounted_instances
    }

    pub(crate) const fn client_resource_peaks(&self) -> [usize; 2] {
        self.client_resource_peaks
    }
}
