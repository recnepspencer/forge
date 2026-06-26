use crate::source::{
    WorthUiBoundViewBindingReference, WorthUiRuntimeDependencyHook,
    WorthUiRuntimeDependencyHookKind,
};

pub(super) fn hooks_for_view_binding(
    view_binding: &WorthUiBoundViewBindingReference,
) -> Vec<WorthUiRuntimeDependencyHook> {
    [
        WorthUiRuntimeDependencyHookKind::QueryLiveView,
        WorthUiRuntimeDependencyHookKind::QueryRegionScopedInvalidation,
        WorthUiRuntimeDependencyHookKind::QuerySignalContinuation,
        WorthUiRuntimeDependencyHookKind::QueryAsyncResultState,
    ]
    .into_iter()
    .map(|kind| hook_from_query_semantics(kind, view_binding))
    .collect()
}

fn hook_from_query_semantics(
    kind: WorthUiRuntimeDependencyHookKind,
    view_binding: &WorthUiBoundViewBindingReference,
) -> WorthUiRuntimeDependencyHook {
    let query = view_binding.query_semantics();
    WorthUiRuntimeDependencyHook::new(
        kind,
        view_binding.view_binding().id().clone(),
        query.query_capability().clone(),
        query.query_composition_profile_digest(),
        query.view_shape().clone(),
        query.result_shape().clone(),
        query.basis_posture().clone(),
        query.live_compatibility().clone(),
        query.denial_presentation().clone(),
    )
}
