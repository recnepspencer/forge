use worth_query::facade::runtime::WorthQueryRetainedScalarFieldFact;

fn main() {
    let fact: WorthQueryRetainedScalarFieldFact = unreachable!();
    let _ = fact.terminal_json_projection();
}
