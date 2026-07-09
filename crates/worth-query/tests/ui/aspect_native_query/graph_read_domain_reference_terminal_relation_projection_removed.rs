use worth_query::facade::WorthQueryAdmittedGraphReadDomainOperationReference;

fn main() {
    let reference = WorthQueryAdmittedGraphReadDomainOperationReference::relation("manager").unwrap();
    let _ = reference.terminal_relation_projection();
}
