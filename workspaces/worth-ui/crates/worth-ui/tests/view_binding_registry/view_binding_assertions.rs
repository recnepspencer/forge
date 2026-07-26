use worth_ui_runtime::facade::registry::snapshot::FrozenViewBindingCapabilities;

pub(crate) fn assert_registered_view_binding_ids(
    view_bindings: &FrozenViewBindingCapabilities,
    expected_binding_ids: &[&str],
) {
    let actual_binding_ids = view_bindings
        .entries()
        .iter()
        .map(|entry| entry.descriptor().id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_binding_ids, expected_binding_ids);
}
