use crate::scenario::application_authority_closure::fixed_host::FixedCertificationHostBinding;
use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;

use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::application_definition::{
    platform_pulse_application_builder_with_host, PLATFORM_PULSE_BACKGROUND_COMPONENT,
    PLATFORM_PULSE_BLUE_TOKEN, PLATFORM_PULSE_FILL_TOKEN, PLATFORM_PULSE_GREEN_TOKEN,
    PLATFORM_PULSE_IDENTITY_TARGET_COMPONENT, PLATFORM_PULSE_IDENTITY_TARGET_FILL_TOKEN,
    PLATFORM_PULSE_SURFACE, PLATFORM_PULSE_YELLOW_TOKEN,
};

impl FilesystemApplicationLifecycleScenario {
    pub fn platform_pulse_source_text() -> String {
        Self::platform_pulse_source_text_with_color(PLATFORM_PULSE_BLUE_TOKEN)
    }

    pub fn platform_pulse_green_source_text() -> String {
        Self::platform_pulse_source_text_with_color(PLATFORM_PULSE_GREEN_TOKEN)
    }

    pub(super) fn platform_pulse_source_text_with_color(color_token: &str) -> String {
        format!(
            "component {PLATFORM_PULSE_BACKGROUND_COMPONENT} {{}}\n\
             component {PLATFORM_PULSE_IDENTITY_TARGET_COMPONENT} {{}}\n\
             surface {PLATFORM_PULSE_SURFACE} {{}}\n\
             token {PLATFORM_PULSE_FILL_TOKEN} = \"{color_token}\";\n\
             token {PLATFORM_PULSE_IDENTITY_TARGET_FILL_TOKEN} = \"{PLATFORM_PULSE_YELLOW_TOKEN}\";\n"
        )
    }

    pub fn platform_pulse_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        platform_pulse_application_builder_with_host(host)
            .freeze()
            .expect("platform pulse capabilities should prepare without Query")
    }

    pub fn prepare_platform_pulse_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        platform_pulse_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored platform pulse application should prepare")
    }
}
