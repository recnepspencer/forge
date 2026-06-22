use forge_query::facade::ForgeQueryExistingTruthProbeField;

fn main() {
    let field: ForgeQueryExistingTruthProbeField = unreachable!();
    let _ = field.terminal_json_projection_string();
}
