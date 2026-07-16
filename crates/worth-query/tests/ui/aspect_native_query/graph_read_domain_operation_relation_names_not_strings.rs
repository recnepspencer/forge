use worth_query::facade::foundation::WorthQueryAdmittedGraphReadDomainOperationReference;
use worth_query::facade::runtime::WorthQueryDomainRegisteredGraphReadOperation;

fn main() {
    let reference =
        WorthQueryAdmittedGraphReadDomainOperationReference::relation("manager").unwrap();
    let _: &str = reference.relation_name();

    let registration = registered_operation();
    let _: &[String] = registration.accepted_relation_names();
}

fn registered_operation() -> &'static WorthQueryDomainRegisteredGraphReadOperation {
    panic!("fixture only")
}
