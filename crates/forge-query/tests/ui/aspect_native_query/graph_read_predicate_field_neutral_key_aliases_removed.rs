use forge_query::facade::runtime::ForgeQueryAdmittedGraphReadPredicateField;

fn main() {
    let field: ForgeQueryAdmittedGraphReadPredicateField = unreachable!();
    let _ = field.aspect();
    let _ = field.field();
}
