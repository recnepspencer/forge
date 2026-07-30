use worth_ui::facade::query_binding::UiProjectionBinding;

fn invalid(binding: &UiProjectionBinding) {
    let _copy: UiProjectionBinding = binding.clone();
}

fn main() {}
