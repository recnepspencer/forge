use super::{
    WorthQueryInspection, WorthQueryInspectionTarget, WorthQueryLiveArtifactTarget,
    WorthQueryRuntime, WorthQueryRuntimeError, WorthQueryRuntimeFacadeFamily,
    WorthQueryUnifiedInspectionResult, WorthQueryWorkspace,
};
use crate::intent_admission::{
    WorthQueryGenericInspectionIntentTarget, WorthQueryRuntimeInspectionIntentAuthoring,
};

pub struct WorthQueryWorkspaceInspectionLane<'a> {
    runtime: &'a WorthQueryRuntime,
}

impl<'a> WorthQueryWorkspaceInspectionLane<'a> {
    pub(crate) fn new(runtime: &'a WorthQueryRuntime) -> Self {
        Self { runtime }
    }

    pub fn inspect_live_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Result<WorthQueryUnifiedInspectionResult, WorthQueryRuntimeError> {
        let resolved = self
            .runtime
            .resolve_live_artifact_target(target.terminal_view_name_projection())?;
        self.runtime
            .inspect_live_view_name_result(resolved.terminal_view_name_projection())
    }

    pub fn inspect<T>(&self, target: T) -> Result<WorthQueryInspection, WorthQueryRuntimeError>
    where
        T: Into<WorthQueryInspectionTarget<'a>>,
    {
        self.runtime.inspect(target)
    }

    pub fn inspect_intent<T>(&self, target: T) -> WorthQueryRuntimeInspectionIntentAuthoring<'a>
    where
        T: WorthQueryGenericInspectionIntentTarget<'a>,
    {
        self.runtime.inspect_intent(target)
    }
}

impl WorthQueryWorkspace {
    pub fn inspections(
        &self,
    ) -> Result<WorthQueryWorkspaceInspectionLane<'_>, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(WorthQueryRuntimeFacadeFamily::Inspect)?;
        Ok(WorthQueryWorkspaceInspectionLane::new(&self.runtime))
    }
}
