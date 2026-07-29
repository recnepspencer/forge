use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;
use worth_ui_runtime::facade::host::WorthUiOperationalHostAdapter;

use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::application_definition::{
    platform_pulse_application_builder_with_host_and_unrelated_width, unrelated_component_id,
    PLATFORM_PULSE_BLUE_TOKEN, PLATFORM_PULSE_GREEN_TOKEN,
};

impl FilesystemApplicationLifecycleScenario {
    pub fn platform_pulse_source_text_with_unrelated_width(unrelated_width: usize) -> String {
        scaled_source(PLATFORM_PULSE_BLUE_TOKEN, unrelated_width)
    }

    pub fn platform_pulse_green_source_text_with_unrelated_width(unrelated_width: usize) -> String {
        scaled_source(PLATFORM_PULSE_GREEN_TOKEN, unrelated_width)
    }

    pub fn platform_pulse_capability_application_with_unrelated_width<Host>(
        &self,
        host: Host,
        unrelated_width: usize,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        platform_pulse_application_builder_with_host_and_unrelated_width(host, unrelated_width)
            .freeze()
            .expect("scaled Platform Pulse capabilities should prepare")
    }

    pub fn prepare_platform_pulse_application_with_unrelated_width<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
        unrelated_width: usize,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        platform_pulse_application_builder_with_host_and_unrelated_width(host, unrelated_width)
            .with_candidate_submission(submission)
            .freeze()
            .expect("scaled filesystem-authored Platform Pulse should prepare")
    }
}

fn scaled_source(color_token: &str, unrelated_width: usize) -> String {
    let mut source =
        FilesystemApplicationLifecycleScenario::platform_pulse_source_text_with_color(color_token);
    for index in 0..unrelated_width {
        source.push_str(&format!(
            "component {} {{}}\n",
            unrelated_component_id(index)
        ));
    }
    source
}
