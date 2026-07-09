use worth_query::facade::WorthQueryExistingTruthProbeField;

fn main() {
    let field: WorthQueryExistingTruthProbeField = unreachable!();
    let _ = field.terminal_json_projection_string();
}
