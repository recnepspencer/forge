use worth_query::facade::runtime::WorthQueryRetainedScalarFieldFact;

fn main() {
    let fact: WorthQueryRetainedScalarFieldFact = unreachable!();
    let _ = fact.field_key();
}
