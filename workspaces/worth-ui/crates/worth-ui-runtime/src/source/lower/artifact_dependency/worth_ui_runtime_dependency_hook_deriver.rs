use crate::source::{
    WorthUiBoundViewBindingReference, WorthUiRuntimeDependencyHook,
    WorthUiRuntimeDependencyHookKind,
};

pub(super) fn hooks_for_view_binding(
    view_binding: &WorthUiBoundViewBindingReference,
) -> Vec<WorthUiRuntimeDependencyHook> {
    [
        WorthUiRuntimeDependencyHookKind::LiveView,
        WorthUiRuntimeDependencyHookKind::RegionScopedInvalidation,
        WorthUiRuntimeDependencyHookKind::SignalContinuation,
        WorthUiRuntimeDependencyHookKind::AsyncResultState,
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
        query.definition().clone(),
        *query.denial_presentation(),
    )
}
