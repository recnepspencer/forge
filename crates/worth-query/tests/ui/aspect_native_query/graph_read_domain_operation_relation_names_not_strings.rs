use worth_query::facade::{
    WorthQueryAdmittedGraphReadDomainOperationReference, WorthQueryGraphReadOperationRegistration,
};

fn main() {
    let reference =
        WorthQueryAdmittedGraphReadDomainOperationReference::relation("manager").unwrap();
    let _: &str = reference.relation_name();

    let registration =
        WorthQueryGraphReadOperationRegistration::domain("frontier", 1, "people")
            .accepts_relation("manager")
            .unwrap();
    let _: &[String] = registration.accepted_relation_names();
}
