use worth_ui::facade::app::{
    WorthUiActiveApplicationSession, WorthUiApp, WorthUiApplicationCutoverReceipt,
};
use worth_ui::facade::diagnostics::CapabilitySnapshot;
use worth_ui::facade::host::WorthUiOperationalHostAdapter;
use worth_ui::facade::source::{WorthUiSettledSourceSnapshot, WorthUiWatchedCandidateSubmission};
use worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture;

use super::application_authority_closure::application_definition::{
    application_builder, application_builder_with_host, cross_lane_application_builder_with_host,
    preview_application_builder_with_host, CANDIDATE_COMPONENT, CROSS_LANE_CANVAS,
    CROSS_LANE_REALTIME, CURRENT_COMPONENT, IMPORTED_CANDIDATE_COMPONENT,
    IMPORTED_CURRENT_COMPONENT, PREVIEW_COMPONENT, PREVIEW_REGION, PREVIEW_SCROLL_STATE_SLOT,
    PREVIEW_SIZING, PREVIEW_STATE_SLOT, PREVIEW_SURFACE,
};
use super::application_authority_closure::authored_composition::{file_source, rust_submission};
use super::application_authority_closure::candidate_catalog::admit_candidate_catalog;

pub struct FilesystemApplicationLifecycleScenario {
    query: WorthUiInstalledQueryTestFixture,
}

impl FilesystemApplicationLifecycleScenario {
    pub fn new(responsibility: &str) -> Self {
        Self {
            query: WorthUiInstalledQueryTestFixture::new(responsibility),
        }
    }

    pub fn current_source_text() -> String {
        file_source(CURRENT_COMPONENT)
    }

    pub fn ordinary_execution_source_text() -> String {
        use super::application_authority_closure::application_definition::{
            CURRENT_COMPONENT, REGION, SIZING, STATE_SLOT, SURFACE, TOKEN,
        };
        format!(
            "component {CURRENT_COMPONENT} {{ region {REGION} {{ sizing {SIZING}; state {STATE_SLOT}; }} }}\n\
             surface {SURFACE} {{}}\n\
             token {TOKEN} = \"theme.text.authority_primary\";\n"
        )
    }

    pub fn reordered_ordinary_execution_source_text() -> String {
        use super::application_authority_closure::application_definition::{
            CURRENT_COMPONENT, REGION, SIZING, STATE_SLOT, SURFACE, TOKEN,
        };
        format!(
            "token {TOKEN} = \"theme.text.authority_primary\";\n\
             surface {SURFACE} {{}}\n\
             component {CURRENT_COMPONENT} {{ region {REGION} {{ state {STATE_SLOT}; sizing {SIZING}; }} }}\n"
        )
    }

    pub fn candidate_source_text() -> String {
        file_source(CANDIDATE_COMPONENT)
    }

    pub fn preview_source_text(include_successor: bool) -> String {
        Self::resizable_surface_source_text(PREVIEW_STATE_SLOT, include_successor)
    }

    pub fn resizable_non_splitter_source_text(include_successor: bool) -> String {
        Self::resizable_surface_source_text(PREVIEW_SCROLL_STATE_SLOT, include_successor)
    }

    fn resizable_surface_source_text(state_slot: &str, include_successor: bool) -> String {
        let successor = if include_successor {
            format!("component {CANDIDATE_COMPONENT} {{}}\n")
        } else {
            String::new()
        };
        format!(
            "component {PREVIEW_COMPONENT} {{}}\n\
             surface {PREVIEW_SURFACE} {{ region {PREVIEW_REGION} {{ sizing {PREVIEW_SIZING}; state {state_slot}; }} }}\n\
             {successor}"
        )
    }

    pub fn preview_sizing_contract_id() -> &'static str {
        PREVIEW_SIZING
    }

    pub fn cross_lane_source_text() -> String {
        format!(
            "{}\ncomponent {CROSS_LANE_CANVAS} {{}}\ncomponent {CROSS_LANE_REALTIME} {{}}\n\
             binding inspector.measurements {{}}\n",
            Self::ordinary_execution_source_text()
        )
    }

    pub fn scaled_canvas_source_text(canvas_count: usize, omit_first: bool) -> String {
        let mut source = Self::ordinary_execution_source_text();
        for index in usize::from(omit_first)..canvas_count {
            source.push_str(&format!(
                "component workspace.component.scaled_canvas_{index:04} {{}}\n"
            ));
        }
        source
    }

    pub fn imported_current_source_text() -> String {
        format!("component {IMPORTED_CURRENT_COMPONENT} {{}}")
    }

    pub fn imported_candidate_source_text() -> String {
        format!("component {IMPORTED_CANDIDATE_COMPONENT} {{}}")
    }

    pub fn capability_application(&self) -> WorthUiApp {
        application_builder(&self.query)
            .freeze()
            .expect("filesystem scenario capabilities should prepare")
    }

    pub fn prepare_application(&self, submission: WorthUiWatchedCandidateSubmission) -> WorthUiApp {
        application_builder(&self.query)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored scenario application should prepare")
    }

    pub fn prepare_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        application_builder_with_host(&self.query, host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored scenario application should prepare")
    }

    pub fn prepare_application_with_host_and_retention_budget<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
        retention_budget: worth_ui::facade::mounted::UiMountedFrameRetentionBudget,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        application_builder_with_host(&self.query, host)
            .with_mounted_frame_retention_budget(retention_budget)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored scenario application should prepare")
    }

    pub fn prepare_application_with_host_and_capacities<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
        retention_budget: worth_ui::facade::mounted::UiMountedFrameRetentionBudget,
        observation_capacity: worth_ui::facade::observation_report::UiHostObservationCapacity,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        application_builder_with_host(&self.query, host)
            .with_mounted_frame_retention_budget(retention_budget)
            .with_host_observation_capacity(observation_capacity)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored scenario application should prepare")
    }

    pub fn prepare_preview_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        preview_application_builder_with_host(&self.query, host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored splitter preview application should prepare")
    }

    pub fn cross_lane_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        cross_lane_application_builder_with_host(&self.query, host)
            .freeze()
            .expect("cross-lane capabilities should prepare")
    }

    pub fn preview_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        preview_application_builder_with_host(&self.query, host)
            .freeze()
            .expect("splitter preview capabilities should prepare")
    }

    pub fn scaled_canvas_capability_application<Host>(
        &self,
        host: Host,
        canvas_count: usize,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        super::application_authority_closure::application_definition::scaled_canvas_application_builder_with_host(
            &self.query,
            host,
            canvas_count,
        )
        .freeze()
        .expect("scaled canvas capabilities should prepare")
    }

    pub fn prepare_cross_lane_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        cross_lane_application_builder_with_host(&self.query, host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored cross-lane application should prepare")
    }

    pub fn prepare_scaled_canvas_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
        canvas_count: usize,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        super::application_authority_closure::application_definition::scaled_canvas_application_builder_with_host(
            &self.query,
            host,
            canvas_count,
        )
        .with_candidate_submission(submission)
        .freeze()
        .expect("filesystem-authored scaled canvas application should prepare")
    }

    pub fn settled_query_projection(
        &mut self,
    ) -> worth_ui_query_binding::WorthUiSettledSnapshotProjection {
        self.query.settle_snapshot()
    }

    pub fn close_query_retirement(
        &mut self,
        retirement: worth_ui_query_binding::WorthUiOperationLiveRetirement,
    ) -> worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome {
        self.query.close_retirement(retirement)
    }

    pub fn current_rust_submission(
        capabilities: &CapabilitySnapshot,
    ) -> WorthUiWatchedCandidateSubmission {
        rust_submission(
            CURRENT_COMPONENT,
            "filesystem-equivalent-rust",
            capabilities,
        )
    }

    pub fn lower_snapshot(
        snapshot: WorthUiSettledSourceSnapshot,
        capabilities: &CapabilitySnapshot,
    ) -> WorthUiWatchedCandidateSubmission {
        snapshot
            .lower_to_candidate_submission(capabilities)
            .expect("stable filesystem source should lower")
    }

    pub fn activate_submission(
        session: &mut WorthUiActiveApplicationSession,
        submission: WorthUiWatchedCandidateSubmission,
    ) -> WorthUiApplicationCutoverReceipt {
        let prepared = session
            .prepare_replacement(submission)
            .expect("filesystem replacement should prepare");
        let mut prepared = prepared;
        let catalog = admit_candidate_catalog(session, &mut prepared);
        let lowered = session
            .lower_prepared_replacement(*prepared)
            .expect("filesystem replacement should lower");
        let pending = session
            .stage_prepared_replacement(lowered)
            .expect("filesystem replacement should stage");
        let boundary = session
            .execute_framework_turn(|_| {})
            .expect("no mounted presentation lease is active")
            .into_completion()
            .into_execution()
            .expect("empty turn should publish an activation boundary")
            .into_activation_boundary();
        session
            .activate_prepared_replacement(pending, catalog, boundary, None)
            .expect("filesystem replacement should activate")
            .into_activation()
            .expect("changed filesystem meaning publishes a successor")
    }
}
