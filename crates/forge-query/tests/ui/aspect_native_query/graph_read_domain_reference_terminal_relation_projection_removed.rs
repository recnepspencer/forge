use forge_query::facade::ForgeQueryAdmittedGraphReadDomainOperationReference;

fn main() {
    let reference = ForgeQueryAdmittedGraphReadDomainOperationReference::relation("manager").unwrap();
    let _ = reference.terminal_relation_projection();
}
