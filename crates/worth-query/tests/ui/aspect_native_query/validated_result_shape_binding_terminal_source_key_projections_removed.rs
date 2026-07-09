use worth_query::facade::ValidatedResultShapeBinding;

fn main() {
    let binding: ValidatedResultShapeBinding = unreachable!();
    let _ = binding.terminal_source_aspect_projection();
    let _ = binding.terminal_source_field_projection();
}
