use worth_query::facade::runtime::WorthQueryAdmittedBooleanPredicateLeaf;

fn main() {
    let leaf: WorthQueryAdmittedBooleanPredicateLeaf = unreachable!();
    let _ = leaf.aspect();
    let _ = leaf.field();
}
