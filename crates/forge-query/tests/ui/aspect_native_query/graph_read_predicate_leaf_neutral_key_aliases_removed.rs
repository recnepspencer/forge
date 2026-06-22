use forge_query::facade::runtime::ForgeQueryAdmittedBooleanPredicateLeaf;

fn main() {
    let leaf: ForgeQueryAdmittedBooleanPredicateLeaf = unreachable!();
    let _ = leaf.aspect();
    let _ = leaf.field();
}
