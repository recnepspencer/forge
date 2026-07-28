use worth_query_declaration::facade::application_schema::{
    ApplicationAbilityRef, ApplicationAspectRef, ApplicationAuthorizationPathBuilder,
    ApplicationEntityRef, ApplicationFieldRef, ApplicationOperationRef, ApplicationPolicyRef,
    ApplicationPrincipalBindingRef, ApplicationRelationRef, ApplicationSchema,
    ApplicationSchemaDeclaration, ApplicationSchemaDeclarationBuilder, EqualityPredicate,
    NoEqualityPredicate, OperationCreates, OperationRequiresAbility, ReadOnly, ReadWrite,
};
use worth_query_declaration::facade::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};

use crate::facade::{
    WorthQueryAbilityInstallationDenialKind, WorthQueryInstallationAdmissionProfile,
    WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity,
    WorthQueryInstalledApplicationSchemaDenialKind, WorthQueryInstalledPackageIndex,
    WorthQueryPortableDefinition, WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage, WorthQueryPortablePackageValidationDenialKind,
};

struct TestSchema;
struct DriftedSchema;
struct TestEntity;
struct AddedEntity;
struct TestAbility;
struct TestOperation;
struct TestInput;
struct IdentityAspect;
struct ExternalIdentityField;
struct MappingStatusField;
struct PrincipalIdentityField;
struct MappingTarget;
struct PrincipalBinding;
struct TestPolicy;

impl OperationCreates<TestOperation> for TestEntity {}
impl OperationRequiresAbility<TestOperation> for TestAbility {}

impl ApplicationSchema for TestSchema {
    const OWNER: &'static str = "typed-test";
    const NAME: &'static str = "TestSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        test_schema_members::<Self>().build()
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
        test_schema_members::<Self>()
            .entity(
                ApplicationEntityRef::<Self, AddedEntity>::from_schema_identifier("AddedEntity"),
            )
            .build()
    }
}

fn test_schema_members<Schema>() -> ApplicationSchemaDeclarationBuilder<Schema>
where
    Schema: ApplicationSchema,
{
    let entity = ApplicationEntityRef::<Schema, TestEntity>::from_schema_identifier("TestEntity");
    let ability = ApplicationAbilityRef::<Schema, TestAbility, TestEntity>::from_schema_identifiers(
        "TestAbility",
        "TestEntity",
    );
    let operation =
        ApplicationOperationRef::<Schema, TestOperation, TestInput>::from_schema_identifier(
            "TestOperation",
        );
    ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(entity)
        .aspect(
            entity,
            ApplicationAspectRef::<Schema, TestEntity, IdentityAspect>::from_schema_identifier(
                "IdentityAspect",
            ),
        )
        .field(
            entity,
            ApplicationFieldRef::<
                Schema,
                TestEntity,
                IdentityAspect,
                ExternalIdentityField,
                WorthQueryExternalPrincipalIdentity,
                ReadOnly,
                EqualityPredicate,
            >::from_schema_identifiers("TestEntity", "IdentityAspect", "ExternalIdentityField"),
        )
        .field(
            entity,
            ApplicationFieldRef::<
                Schema,
                TestEntity,
                IdentityAspect,
                MappingStatusField,
                WorthQueryPrincipalMappingStatus,
                ReadWrite,
                NoEqualityPredicate,
            >::from_schema_identifiers("TestEntity", "IdentityAspect", "MappingStatusField"),
        )
        .field(
            entity,
            ApplicationFieldRef::<
                Schema,
                TestEntity,
                IdentityAspect,
                PrincipalIdentityField,
                u64,
                ReadOnly,
                EqualityPredicate,
            >::from_schema_identifiers("TestEntity", "IdentityAspect", "PrincipalIdentityField"),
        )
        .relation(
            ApplicationRelationRef::<Schema, MappingTarget, TestEntity, TestEntity>::from_schema_identifiers(
                "MappingTarget",
                "TestEntity",
                "TestEntity",
            ),
            entity,
            entity,
        )
        .principal_binding(
            ApplicationPrincipalBindingRef::<
                Schema,
                PrincipalBinding,
                TestEntity,
                TestEntity,
                u64,
            >::from_schema_identifiers(
                "PrincipalBinding",
                "TestEntity",
                "IdentityAspect",
                "ExternalIdentityField",
                "IdentityAspect",
                "MappingStatusField",
                "MappingTarget",
                "TestEntity",
                "IdentityAspect",
                "PrincipalIdentityField",
            ),
        )
        .policy(ApplicationPolicyRef::<Schema, TestPolicy>::from_schema_identifier(
            "TestPolicy",
        ))
        .ability(ability)
        .ability_policy(
            ability,
            ApplicationPolicyRef::<Schema, TestPolicy>::from_schema_identifier("TestPolicy"),
            [ApplicationAuthorizationPathBuilder::from_principal(entity).allow(entity)],
        )
        .operation(operation)
        .operation_requires_ability(operation, ability)
        .operation_create(operation, entity)
}

#[test]
fn installed_ability_is_runtime_generation_and_meaning_affine() {
    let index = installed_index();
    let schema = index
        .bind_application_schema(TestSchema::declaration().unwrap())
        .unwrap();
    let ability = schema
        .ability(
            ApplicationAbilityRef::<TestSchema, TestAbility, TestEntity>::from_schema_identifiers(
                "TestAbility",
                "TestEntity",
            ),
        )
        .unwrap();
    index.validate_ability(&ability).unwrap();

    let denial = installed_index().validate_ability(&ability).unwrap_err();
    assert_eq!(
        denial.kind(),
        crate::facade::WorthQueryAbilityInstallationDenialKind::ForeignRuntime
    );
    let denial = index
        .successor_generation()
        .validate_ability(&ability)
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        crate::facade::WorthQueryAbilityInstallationDenialKind::StaleGeneration
    );
}

#[test]
fn fabricated_ability_identity_cannot_bind_installed_authority() {
    let index = installed_index();
    let schema = index
        .bind_application_schema(TestSchema::declaration().unwrap())
        .unwrap();

    let unknown = schema
        .ability(
            ApplicationAbilityRef::<TestSchema, TestAbility, TestEntity>::from_schema_identifiers(
                "FabricatedAbility",
                "TestEntity",
            ),
        )
        .expect_err("fabricated ability name must not bind installed authority");
    assert_eq!(
        unknown.kind(),
        WorthQueryAbilityInstallationDenialKind::AbilityNotInstalled
    );

    let changed_scope = schema
        .ability(
            ApplicationAbilityRef::<TestSchema, TestAbility, TestEntity>::from_schema_identifiers(
                "TestAbility",
                "FabricatedScope",
            ),
        )
        .expect_err("fabricated ability scope must not bind installed authority");
    assert_eq!(
        changed_scope.kind(),
        WorthQueryAbilityInstallationDenialKind::AbilityMeaningChanged
    );
}

#[test]
fn installed_application_operation_compiles_existing_authority_contract_families() {
    let index = installed_index();
    let schema = index
        .bind_application_schema(TestSchema::declaration().unwrap())
        .unwrap();
    let operation = schema
        .installed_operation(ApplicationOperationRef::<
            TestSchema,
            TestOperation,
            TestInput,
        >::from_schema_identifier("TestOperation"))
        .unwrap();
    index.validate_application_operation(&operation).unwrap();

    assert_eq!(
        operation.contracts().ability_requirements()[0].ability(),
        "TestAbility"
    );
    assert!(matches!(
        operation.contracts().graph_reads(),
        crate::facade::WorthQueryOperationGraphReadContract::Declared { roles }
            if roles.len() == 1 && roles[0].role == "primary"
    ));
    assert!(matches!(
        operation.contracts().touches(),
        crate::facade::WorthQueryOperationTouchContract::Declared { scopes, .. }
            if scopes == &["create:TestEntity"]
    ));
    assert!(matches!(
        operation.contracts().effects(),
        crate::facade::WorthQueryOperationEffectContract::Declared { effect_families }
            if effect_families == &[crate::facade::WorthQueryOperationEffectFamily::Mutation]
    ));
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
