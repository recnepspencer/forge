use crate::scenario::application_authority_closure::fixed_host::FixedCertificationHostBinding;
use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;

use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::application_definition::application_builder_with_host;

impl FilesystemApplicationLifecycleScenario {
    pub fn prepare_application_with_host_and_visual_policy<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
        policy: worth_ui::facade::inspection::UiVisualInspectionPolicy,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        application_builder_with_host(&self.query, host)
            .with_visual_inspection_policy(policy)
            .with_candidate_submission(submission)
            .freeze()
            .expect("policy-bound filesystem visual application should prepare")
    }
}
