use super::{
    WorthQueryLiveArtifactTarget, WorthQueryRuntime, WorthQueryRuntimeError,
    WorthQueryRuntimeFacadeFamily, WorthQueryUnifiedInspectionResult, WorthQueryWorkspace,
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
