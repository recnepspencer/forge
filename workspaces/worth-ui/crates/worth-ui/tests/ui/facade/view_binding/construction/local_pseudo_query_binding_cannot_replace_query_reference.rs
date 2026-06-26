use worth_ui::facade::{ViewBindingDescriptor, ViewBindingFamily, ViewBindingId};

fn main() {
    let descriptor = ViewBindingDescriptor::query_owned(
        ViewBindingId::new("workspace.view_binding.tasks").unwrap(),
        ViewBindingFamily::collection(),
    );

    let _ = descriptor.with_query_capability_posture("ui_cache.tasks");
}
