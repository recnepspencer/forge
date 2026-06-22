use forge_query::facade::{
    ForgeQueryAdmittedGraphReadDomainOperationReference, ForgeQueryGraphReadOperationRegistration,
};

fn main() {
    let reference =
        ForgeQueryAdmittedGraphReadDomainOperationReference::relation("manager").unwrap();
    let _: &str = reference.relation_name();

    let registration =
        ForgeQueryGraphReadOperationRegistration::domain("frontier", 1, "people")
            .accepts_relation("manager")
            .unwrap();
    let _: &[String] = registration.accepted_relation_names();
}
