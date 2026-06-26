use forge_query::facade::ForgeQueryRetainedScalarFieldFact;

fn main() {
    let fact: ForgeQueryRetainedScalarFieldFact = unreachable!();
    let _ = fact.terminal_json_projection();
}
