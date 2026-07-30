use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;
use worth_ui_runtime::facade::host::WorthUiOperationalHostAdapter;

use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::application_definition::scaled_canvas_application_builder_with_host;

impl FilesystemApplicationLifecycleScenario {
    pub fn scaled_canvas_source_text(canvas_count: usize, omit_first: bool) -> String {
        let mut source = Self::ordinary_execution_source_text();
        append_canvas_components(&mut source, canvas_count, usize::from(omit_first));
        source
    }

    pub fn exact_width_canvas_graph_source_text(graph_width: usize) -> String {
        assert!(graph_width > 0, "a mounted graph must contain a node");
        let mut source = String::new();
        append_canvas_components(&mut source, graph_width - 1, 0);
        source
    }

    pub fn scaled_canvas_capability_application<Host>(
        &self,
        host: Host,
        canvas_count: usize,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        scaled_canvas_application_builder_with_host(&self.query, host, canvas_count)
            .freeze()
            .expect("scaled canvas capabilities should prepare")
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
        scaled_canvas_application_builder_with_host(&self.query, host, canvas_count)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored scaled canvas application should prepare")
    }

    pub fn prepare_scaled_canvas_application_with_host_and_retention_budget<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
        canvas_count: usize,
        retention_budget: worth_ui_runtime::facade::mounted::UiMountedFrameRetentionBudget,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        scaled_canvas_application_builder_with_host(&self.query, host, canvas_count)
            .with_mounted_frame_retention_budget(retention_budget)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored scaled canvas application should prepare")
    }
}

fn append_canvas_components(source: &mut String, canvas_count: usize, first_index: usize) {
    for index in first_index..canvas_count {
        source.push_str(&format!(
            "component workspace.component.scaled_canvas_{index:04} {{}}\n"
        ));
    }
}
