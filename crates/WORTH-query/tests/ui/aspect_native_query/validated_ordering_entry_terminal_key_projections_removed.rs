use worth_query::facade::ValidatedOrderingEntry;

fn main() {
    let entry: ValidatedOrderingEntry = unreachable!();
    let _ = entry.terminal_aspect_projection();
    let _ = entry.terminal_field_projection();
}
