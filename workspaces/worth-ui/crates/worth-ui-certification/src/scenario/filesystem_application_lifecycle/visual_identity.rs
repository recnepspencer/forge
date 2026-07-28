use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;
use worth_ui_runtime::facade::host::WorthUiOperationalHostAdapter;

use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::visual_identity_application::{
    duplicate_hit_order_application_builder_with_host,
    region_identity_application_builder_with_host, visual_identity_application_builder_with_host,
    VISUAL_HIT_ONLY_COMPONENT, VISUAL_IDENTITY_SURFACE, VISUAL_NEITHER_COMPONENT,
    VISUAL_PAINT_AND_HIT_COMPONENT, VISUAL_PAINT_AND_HIT_TOKEN, VISUAL_PAINT_ONLY_COMPONENT,
    VISUAL_PAINT_ONLY_TOKEN, VISUAL_PURPLE_TOKEN, VISUAL_RED_TOKEN,
};

impl FilesystemApplicationLifecycleScenario {
    pub fn visual_identity_source_text() -> String {
        format!(
            "component {VISUAL_PAINT_ONLY_COMPONENT} {{}}\n\
             component {VISUAL_HIT_ONLY_COMPONENT} {{}}\n\
             component {VISUAL_PAINT_AND_HIT_COMPONENT} {{}}\n\
             component {VISUAL_NEITHER_COMPONENT} {{}}\n\
             surface {VISUAL_IDENTITY_SURFACE} {{}}\n\
             token {VISUAL_PAINT_ONLY_TOKEN} = \"{VISUAL_RED_TOKEN}\";\n\
             token {VISUAL_PAINT_AND_HIT_TOKEN} = \"{VISUAL_PURPLE_TOKEN}\";\n"
        )
    }

    pub fn visual_identity_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        visual_identity_application_builder_with_host(host)
            .freeze()
            .expect("visual identity capabilities should prepare")
    }

    pub fn duplicate_hit_order_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        duplicate_hit_order_application_builder_with_host(host)
            .freeze()
            .expect("duplicate-order capabilities should prepare")
    }

    pub fn region_identity_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        region_identity_application_builder_with_host(host)
            .freeze()
            .expect("region identity capabilities should prepare")
    }

    pub fn prepare_visual_identity_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        visual_identity_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored visual identity application should prepare")
    }

    pub fn prepare_visual_identity_application_with_policy_and_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        policy: worth_ui::facade::inspection::UiVisualInspectionPolicy,
        host: Host,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        visual_identity_application_builder_with_host(host)
            .with_visual_inspection_policy(policy)
            .with_candidate_submission(submission)
            .freeze()
            .expect("policy-bounded filesystem visual identity application should prepare")
    }

    pub fn prepare_region_identity_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        region_identity_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored region identity application should prepare")
    }

    pub fn prepare_region_identity_application_with_policy_and_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        policy: worth_ui::facade::inspection::UiVisualInspectionPolicy,
        host: Host,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        region_identity_application_builder_with_host(host)
            .with_visual_inspection_policy(policy)
            .with_candidate_submission(submission)
            .freeze()
            .expect("policy-bounded filesystem region identity application should prepare")
    }

    pub fn prepare_duplicate_hit_order_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        duplicate_hit_order_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("duplicate-order world should prepare before mounted projection")
    }
}
