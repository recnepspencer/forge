use worth_query::facade::runtime::WorthQueryExistingTruthProbeField;

fn main() {
    let field: WorthQueryExistingTruthProbeField = unreachable!();
    let _ = field.terminal_json_projection_string();
}
