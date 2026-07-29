use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::application_definition::application_builder_with_change_profile;

impl FilesystemApplicationLifecycleScenario {
    pub fn prepare_application_with_change_profile(
        &self,
        submission: worth_ui::facade::source::WorthUiWatchedCandidateSubmission,
        profile: worth_ui::facade::rebind::UiChangeProfile,
    ) -> worth_ui::facade::app::WorthUiApp {
        application_builder_with_change_profile(&self.query, profile)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem application with certification profile should prepare")
    }
}
