use worth_ui::facade::WorthUiActiveRuntimeObservation;

use crate::sample::ValidationAuthoringSample;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationWorkbenchSnapshot {
    app_name: &'static str,
    workspace_name: &'static str,
    page_count: usize,
    dynamic_page_count: usize,
    artifact_digest: u64,
    active_plan_digest: u64,
}

impl ValidationWorkbenchSnapshot {
    pub fn from_launch(
        sample: ValidationAuthoringSample,
        observation: WorthUiActiveRuntimeObservation,
    ) -> Self {
        Self {
            app_name: sample.app_name(),
            workspace_name: sample.workspace_name(),
            page_count: sample.pages().len(),
            dynamic_page_count: sample.dynamic_pages().len(),
            artifact_digest: observation.artifact_digest(),
            active_plan_digest: observation.active_plan_digest(),
        }
    }

    pub fn app_name(self) -> &'static str {
        self.app_name
    }

    pub fn workspace_name(self) -> &'static str {
        self.workspace_name
    }

    pub fn page_count(self) -> usize {
        self.page_count
    }

    pub fn dynamic_page_count(self) -> usize {
        self.dynamic_page_count
    }

    pub fn artifact_digest(self) -> u64 {
        self.artifact_digest
    }

    pub fn active_plan_digest(self) -> u64 {
        self.active_plan_digest
    }
}
