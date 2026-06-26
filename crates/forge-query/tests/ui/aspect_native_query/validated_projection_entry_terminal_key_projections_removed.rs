use forge_query::facade::ValidatedProjectionEntry;

fn main() {
    let entry: ValidatedProjectionEntry = unreachable!();
    let _ = entry.terminal_aspect_projection();
    let _ = entry.terminal_field_projection();
}
