use crate::source::{WorthUiRuntimeDependencyHook, WorthUiRuntimeDependencyHookKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiQueryDependencySurface {
    LiveView,
    RegionScopedLiveInvalidation,
    SignalCompatibilityAndContinuation,
    AsyncResourcesAndResultState,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiQueryDependencyInvalidation {
    surface: WorthUiQueryDependencySurface,
    view_binding_id: String,
}

impl WorthUiQueryDependencyInvalidation {
    pub(crate) fn from_runtime_hook(hook: &WorthUiRuntimeDependencyHook) -> Self {
        Self {
            surface: surface_from_hook_kind(hook.kind()),
            view_binding_id: hook.view_binding_id().as_str().to_owned(),
        }
    }

    pub fn surface(&self) -> WorthUiQueryDependencySurface {
        self.surface
    }

    pub fn view_binding_id(&self) -> &str {
        &self.view_binding_id
    }
}

fn surface_from_hook_kind(kind: WorthUiRuntimeDependencyHookKind) -> WorthUiQueryDependencySurface {
    match kind {
        WorthUiRuntimeDependencyHookKind::QueryLiveView => WorthUiQueryDependencySurface::LiveView,
        WorthUiRuntimeDependencyHookKind::QueryRegionScopedInvalidation => {
            WorthUiQueryDependencySurface::RegionScopedLiveInvalidation
        }
        WorthUiRuntimeDependencyHookKind::QuerySignalContinuation => {
            WorthUiQueryDependencySurface::SignalCompatibilityAndContinuation
        }
        WorthUiRuntimeDependencyHookKind::QueryAsyncResultState => {
            WorthUiQueryDependencySurface::AsyncResourcesAndResultState
        }
    }
}
