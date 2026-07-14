use worth_query::facade::foundation::WorthQueryAdmittedGraphReadDomainOperationReference;

fn main() {
    let reference = WorthQueryAdmittedGraphReadDomainOperationReference::relation("manager").unwrap();
    let _ = reference.terminal_relation_projection();
}
