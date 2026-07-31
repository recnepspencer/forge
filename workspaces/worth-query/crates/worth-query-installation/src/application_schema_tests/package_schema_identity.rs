use worth_foundational::facade::{CanonicalDigestDerivationDenial, CanonicalDigestWorkBudget};
use worth_query_declaration::facade::application_schema::ApplicationSchema;

use crate::application_schema::derive_installed_schema_identity_with_budget;
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

#[test]
fn installed_schema_identity_denies_entry_and_encoded_byte_overflow() {
    let declaration = TestSchema::declaration().unwrap();
    let entry_denial = derive_installed_schema_identity_with_budget(
        declaration.identity(),
        CanonicalDigestWorkBudget::new(1, 1024 * 1024).unwrap(),
    )
    .unwrap_err();
    assert!(matches!(
        entry_denial,
        CanonicalDigestDerivationDenial::EntryLimitExceeded { maximum: 1, .. }
    ));

    let byte_denial = derive_installed_schema_identity_with_budget(
        declaration.identity(),
        CanonicalDigestWorkBudget::new(32_768, 1).unwrap(),
    )
    .unwrap_err();
    assert!(matches!(
        byte_denial,
        CanonicalDigestDerivationDenial::EncodedByteLimitExceeded { maximum: 1, .. }
    ));
}
