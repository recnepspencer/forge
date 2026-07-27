use worth_query_declaration::facade::application_schema::{
    ApplicationEntityRef, ApplicationSchema, ApplicationSchemaDeclaration,
    ApplicationSchemaDeclarationBuilder,
};

use crate::facade::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstallationRuntimeIdentity, WorthQueryInstalledApplicationSchemaDenialKind,
    WorthQueryInstalledPackageIndex, WorthQueryPortableDefinition,
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
    WorthQueryPortablePackageValidationDenialKind,
};

struct TestSchema;
struct DriftedSchema;
struct TestEntity;
struct AddedEntity;

impl ApplicationSchema for TestSchema {
    const OWNER: &'static str = "typed-test";
    const NAME: &'static str = "TestSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema()
            .entity(ApplicationEntityRef::<Self, TestEntity>::from_schema_identifier("TestEntity"))
            .build()
    }
}

impl ApplicationSchema for DriftedSchema {
    const OWNER: &'static str = "typed-test";
    const NAME: &'static str = "TestSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema()
            .entity(ApplicationEntityRef::<Self, TestEntity>::from_schema_identifier("TestEntity"))
            .entity(
                ApplicationEntityRef::<Self, AddedEntity>::from_schema_identifier("AddedEntity"),
            )
            .build()
    }
}

#[test]
fn installed_schema_binding_is_runtime_generation_and_meaning_affine() {
    let index = installed_index();
    let binding = index
        .bind_application_schema(TestSchema::declaration().unwrap())
        .unwrap();
    index.validate_application_schema(&binding).unwrap();

    let foreign = installed_index();
    let denial = foreign.validate_application_schema(&binding).unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryInstalledApplicationSchemaDenialKind::ForeignRuntime
    );

    let successor = index.successor_generation();
    let denial = successor.validate_application_schema(&binding).unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryInstalledApplicationSchemaDenialKind::StaleGeneration
    );

    let denial = index
        .bind_application_schema(DriftedSchema::declaration().unwrap())
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryInstalledApplicationSchemaDenialKind::SchemaMeaningChanged
    );
}

#[test]
fn installed_schema_binding_rejects_package_and_admission_identity_drift() {
    let runtime = WorthQueryInstallationRuntimeIdentity::fresh();
    let same_runtime_for_package_drift = runtime.retained();
    let same_runtime_for_admission_drift = runtime.retained();
    let index = installed_index_with(runtime, false, "support");
    let binding = index
        .bind_application_schema(TestSchema::declaration().unwrap())
        .unwrap();

    let package_drift = installed_index_with(same_runtime_for_package_drift, true, "support");
    let denial = package_drift
        .validate_application_schema(&binding)
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryInstalledApplicationSchemaDenialKind::PackageIdentityChanged
    );

    let admission_drift =
        installed_index_with(same_runtime_for_admission_drift, false, "other-support");
    let denial = admission_drift
        .validate_application_schema(&binding)
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryInstalledApplicationSchemaDenialKind::AdmissionIdentityChanged
    );
}

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

fn installed_index() -> WorthQueryInstalledPackageIndex {
    installed_index_with(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        false,
        "support",
    )
}

fn installed_index_with(
    runtime: WorthQueryInstallationRuntimeIdentity,
    package_drift: bool,
    support: &str,
) -> WorthQueryInstalledPackageIndex {
    let mut package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "typed-test",
        1,
        0,
    ))
    .application_schema(TestSchema::declaration().unwrap());
    if package_drift {
        package = package.definition(WorthQueryPortableDefinition::declaration_family(
            "extra",
            "package-drift",
        ));
    }
    let package = package.validate().unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new(support, "configuration")
        .admit(package)
        .unwrap();
    WorthQueryInstalledPackageIndex::build(
        runtime,
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap()
}
