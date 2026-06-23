use forge_query::facade::runtime::{
    ForgeQueryGraphReadOrderingFieldAuthority, ForgeQueryGraphReadPredicateFieldAuthority,
};

fn main() {
    let predicate: ForgeQueryGraphReadPredicateFieldAuthority = unreachable!();
    let _ = predicate.aspect();
    let _ = predicate.field();

    let ordering: ForgeQueryGraphReadOrderingFieldAuthority = unreachable!();
    let _ = ordering.aspect();
    let _ = ordering.field();
}
