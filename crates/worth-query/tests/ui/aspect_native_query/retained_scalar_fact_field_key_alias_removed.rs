use worth_query::facade::WorthQueryRetainedScalarFieldFact;

fn main() {
    let fact: WorthQueryRetainedScalarFieldFact = unreachable!();
    let _ = fact.field_key();
}
