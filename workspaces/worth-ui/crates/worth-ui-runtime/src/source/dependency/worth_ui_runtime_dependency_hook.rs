use std::cmp::Ordering;

use crate::capability::{QueryDenialPresentation, ViewBindingId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiRuntimeDependencyHookKind {
    LiveView,
    RegionScopedInvalidation,
    SignalContinuation,
    AsyncResultState,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiRuntimeQuerySurface {
    LiveView,
    RegionScopedLiveInvalidation,
    SignalCompatibilityAndContinuation,
    AsyncResourcesAndResultState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiRuntimeDependencyHook {
    kind: WorthUiRuntimeDependencyHookKind,
    view_binding_id: ViewBindingId,
    definition: worth_ui_query_binding::WorthUiQueryViewDefinition,
    denial_presentation: QueryDenialPresentation,
}

impl WorthUiRuntimeDependencyHook {
    pub(crate) fn new(
        kind: WorthUiRuntimeDependencyHookKind,
        view_binding_id: ViewBindingId,
        definition: worth_ui_query_binding::WorthUiQueryViewDefinition,
        denial_presentation: QueryDenialPresentation,
    ) -> Self {
        Self {
            kind,
            view_binding_id,
            definition,
            denial_presentation,
        }
    }

    pub(crate) fn kind(&self) -> WorthUiRuntimeDependencyHookKind {
        self.kind
    }
    pub(crate) fn view_binding_id(&self) -> &ViewBindingId {
        &self.view_binding_id
    }
    pub(crate) fn definition(&self) -> &worth_ui_query_binding::WorthUiQueryViewDefinition {
        &self.definition
    }
    pub(crate) fn denial_presentation(&self) -> &QueryDenialPresentation {
        &self.denial_presentation
    }
    pub(crate) fn artifact_identity_material(&self) -> String {
        format!(
            "kind:{:?}|view_binding:{}|binding_contract:{}|denial:{}",
            self.kind,
            self.view_binding_id.as_str(),
            self.definition.digest().as_u64(),
            self.denial_presentation.digest_basis(),
        )
    }
    #[cfg(test)]
    pub(crate) fn uses_query_surface(&self, surface: WorthUiRuntimeQuerySurface) -> bool {
        matches!(
            (self.kind, surface),
            (
                WorthUiRuntimeDependencyHookKind::LiveView,
                WorthUiRuntimeQuerySurface::LiveView
            ) | (
                WorthUiRuntimeDependencyHookKind::RegionScopedInvalidation,
                WorthUiRuntimeQuerySurface::RegionScopedLiveInvalidation
            ) | (
                WorthUiRuntimeDependencyHookKind::SignalContinuation,
                WorthUiRuntimeQuerySurface::SignalCompatibilityAndContinuation
            ) | (
                WorthUiRuntimeDependencyHookKind::AsyncResultState,
                WorthUiRuntimeQuerySurface::AsyncResourcesAndResultState
            )
        )
    }
}

impl Ord for WorthUiRuntimeDependencyHook {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind
            .cmp(&other.kind)
            .then_with(|| self.view_binding_id.cmp(&other.view_binding_id))
            .then_with(|| self.definition.digest().cmp(&other.definition.digest()))
    }
}

impl PartialOrd for WorthUiRuntimeDependencyHook {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
