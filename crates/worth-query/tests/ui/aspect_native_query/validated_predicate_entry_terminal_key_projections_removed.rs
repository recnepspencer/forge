use worth_query::facade::runtime::ValidatedPredicateEntry;

fn main() {
    let entry: ValidatedPredicateEntry = unreachable!();
    let _ = entry.terminal_aspect_projection();
    let _ = entry.terminal_field_projection();
}
