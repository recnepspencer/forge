use worth_query::facade::runtime::{
    WorthQueryGraphReadOrderingFieldAuthority, WorthQueryGraphReadPredicateFieldAuthority,
};

fn main() {
    let predicate: WorthQueryGraphReadPredicateFieldAuthority = unreachable!();
    let _ = predicate.aspect();
    let _ = predicate.field();

    let ordering: WorthQueryGraphReadOrderingFieldAuthority = unreachable!();
    let _ = ordering.aspect();
    let _ = ordering.field();
}
