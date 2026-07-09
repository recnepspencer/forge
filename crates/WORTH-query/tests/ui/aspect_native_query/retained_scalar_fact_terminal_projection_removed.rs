use worth_query::facade::WorthQueryRetainedScalarFieldFact;

fn main() {
    let fact: WorthQueryRetainedScalarFieldFact = unreachable!();
    let _ = fact.terminal_json_projection();
}
