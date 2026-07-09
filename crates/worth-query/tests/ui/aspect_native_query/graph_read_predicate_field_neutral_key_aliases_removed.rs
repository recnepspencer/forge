use worth_query::facade::runtime::WorthQueryAdmittedGraphReadPredicateField;

fn main() {
    let field: WorthQueryAdmittedGraphReadPredicateField = unreachable!();
    let _ = field.aspect();
    let _ = field.field();
}
