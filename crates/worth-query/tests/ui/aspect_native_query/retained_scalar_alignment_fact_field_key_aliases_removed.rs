use worth_query::facade::runtime::WorthQueryRetainedScalarAlignmentFact;

fn main() {
    let fact: WorthQueryRetainedScalarAlignmentFact = unreachable!();
    let _ = fact.left_field_key();
    let _ = fact.right_field_key();
}
