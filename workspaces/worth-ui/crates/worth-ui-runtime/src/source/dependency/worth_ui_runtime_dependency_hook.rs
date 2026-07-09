use std::cmp::Ordering;

use worth_query::facade::ViewShapeDescriptor;

use crate::capability::{
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiRuntimeDependencyHookKind {
    QueryLiveView,
    QueryRegionScopedInvalidation,
    QuerySignalContinuation,
    QueryAsyncResultState,
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
    query_capability: QueryViewCapabilityReference,
    query_composition_profile_digest: String,
    view_shape: ViewShapeDescriptor,
    result_shape: QueryResultShapeReference,
    basis_posture: QueryBasisPostureReference,
    live_compatibility: QueryLiveCompatibility,
    denial_presentation: QueryDenialPresentation,
}

impl WorthUiRuntimeDependencyHook {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        kind: WorthUiRuntimeDependencyHookKind,
        view_binding_id: ViewBindingId,
        query_capability: QueryViewCapabilityReference,
        query_composition_profile_digest: impl Into<String>,
        view_shape: ViewShapeDescriptor,
        result_shape: QueryResultShapeReference,
        basis_posture: QueryBasisPostureReference,
        live_compatibility: QueryLiveCompatibility,
        denial_presentation: QueryDenialPresentation,
    ) -> Self {
        Self {
            kind,
            view_binding_id,
            query_capability,
            query_composition_profile_digest: query_composition_profile_digest.into(),
            view_shape,
            result_shape,
            basis_posture,
            live_compatibility,
            denial_presentation,
        }
    }

    pub(crate) fn kind(&self) -> WorthUiRuntimeDependencyHookKind {
        self.kind
    }

    pub(crate) fn view_binding_id(&self) -> &ViewBindingId {
        &self.view_binding_id
    }

    pub(crate) fn query_capability(&self) -> &QueryViewCapabilityReference {
        &self.query_capability
    }

    pub(crate) fn query_composition_profile_digest(&self) -> &str {
        &self.query_composition_profile_digest
    }

    #[cfg(test)]
    pub(crate) fn view_shape(&self) -> &ViewShapeDescriptor {
        &self.view_shape
    }

    pub(crate) fn result_shape(&self) -> &QueryResultShapeReference {
        &self.result_shape
    }

    pub(crate) fn basis_posture(&self) -> &QueryBasisPostureReference {
        &self.basis_posture
    }

    pub(crate) fn live_compatibility(&self) -> &QueryLiveCompatibility {
        &self.live_compatibility
    }

    pub(crate) fn denial_presentation(&self) -> &QueryDenialPresentation {
        &self.denial_presentation
    }

    pub(crate) fn digest_basis(&self) -> String {
        [
            format!("kind:{:?}", self.kind),
            format!("view_binding:{}", self.view_binding_id.as_str()),
            format!("capability:{}", self.query_capability.digest_basis()),
            format!("composition:{}", self.query_composition_profile_digest),
            format!("view_shape:{:?}", self.view_shape),
            format!("result_shape:{}", self.result_shape.digest_basis()),
            format!("basis_posture:{}", self.basis_posture.digest_basis()),
            format!("live:{}", self.live_compatibility.digest_basis()),
            format!("denial:{}", self.denial_presentation.digest_basis()),
        ]
        .join("|")
    }

    #[cfg(test)]
    pub(crate) fn uses_query_surface(&self, surface: WorthUiRuntimeQuerySurface) -> bool {
        matches!(
            (self.kind, surface),
            (
                WorthUiRuntimeDependencyHookKind::QueryLiveView,
                WorthUiRuntimeQuerySurface::LiveView
            ) | (
                WorthUiRuntimeDependencyHookKind::QueryRegionScopedInvalidation,
                WorthUiRuntimeQuerySurface::RegionScopedLiveInvalidation
            ) | (
                WorthUiRuntimeDependencyHookKind::QuerySignalContinuation,
                WorthUiRuntimeQuerySurface::SignalCompatibilityAndContinuation
            ) | (
                WorthUiRuntimeDependencyHookKind::QueryAsyncResultState,
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
            .then_with(|| self.digest_basis().cmp(&other.digest_basis()))
    }
}

impl PartialOrd for WorthUiRuntimeDependencyHook {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
