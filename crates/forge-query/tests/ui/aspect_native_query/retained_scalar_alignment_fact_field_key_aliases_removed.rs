use forge_query::facade::ForgeQueryRetainedScalarAlignmentFact;

fn main() {
    let fact: ForgeQueryRetainedScalarAlignmentFact = unreachable!();
    let _ = fact.left_field_key();
    let _ = fact.right_field_key();
}
