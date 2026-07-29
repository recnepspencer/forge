use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::application_definition::{
    application_builder, application_builder_with_change_profile,
};
use worth_ui_test_support::WorthUiApplicationBuilderCertificationExt;

impl FilesystemApplicationLifecycleScenario {
    pub fn prepare_application_with_runtime_instance_bases(
        &self,
        submission: worth_ui::facade::source::WorthUiWatchedCandidateSubmission,
        admissions: impl IntoIterator<Item = worth_ui::facade::graph::UiRuntimeInstanceBasisAdmission>,
    ) -> worth_ui::facade::app::WorthUiApp {
        application_builder(&self.query)
            .with_runtime_instance_basis_admissions(admissions)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored repeated-instance application should prepare")
    }

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
