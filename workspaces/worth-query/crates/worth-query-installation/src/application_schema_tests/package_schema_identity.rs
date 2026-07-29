use worth_query_declaration::facade::application_schema::ApplicationSchema;

use crate::facade::{
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
    WorthQueryPortablePackageValidationDenialKind,
};

use super::TestSchema;

#[test]
fn package_rejects_schema_identity_that_does_not_match_its_domain() {
    let denial = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "another-owner",
        1,
        0,
    ))
    .application_schema(TestSchema::declaration().unwrap())
    .validate()
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryPortablePackageValidationDenialKind::ApplicationSchemaIdentityMismatch
    );
}
