use forge_query::facade::runtime::ForgeQueryBooleanPredicateSelectivityRow;

fn main() {
    let row: ForgeQueryBooleanPredicateSelectivityRow = unreachable!();
    let _ = row.aspect();
    let _ = row.field();
}
