use super::*;

use worth_query_declaration::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredCorrectionMechanism, DeclaredLoweringCorrespondenceRef,
    DeclaredPreImageDemand, DeclaredPreImageLocus, DeclaredRecordedInverse,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity,
    ApplicationFieldMarkerIdentity, ApplicationOperationDecisionReadTarget,
    ApplicationSchemaMember,
};

struct OtherEntity;
struct OtherEntityAspect;
struct OtherEntityField;
struct OtherAspect;
struct OtherAspectField;
struct OtherField;

impl ApplicationEntityMarkerIdentity for OtherEntity {
    type Schema = TestSchema;
    const IDENTIFIER: &'static str = "OtherEntity";
}

impl ApplicationAspectMarkerIdentity for OtherEntityAspect {
    type Schema = TestSchema;
    type Entity = OtherEntity;
    const IDENTIFIER: &'static str = "IdentityAspect";
}

impl ApplicationFieldMarkerIdentity for OtherEntityField {
    type Schema = TestSchema;
    type Entity = OtherEntity;
    type Aspect = OtherEntityAspect;
    const IDENTIFIER: &'static str = "PrincipalIdentityField";
}

impl ApplicationAspectMarkerIdentity for OtherAspect {
    type Schema = TestSchema;
    type Entity = TestEntity;
    const IDENTIFIER: &'static str = "OtherAspect";
}

impl ApplicationFieldMarkerIdentity for OtherAspectField {
    type Schema = TestSchema;
    type Entity = TestEntity;
    type Aspect = OtherAspect;
    const IDENTIFIER: &'static str = "PrincipalIdentityField";
}

impl ApplicationFieldMarkerIdentity for OtherField {
    type Schema = TestSchema;
    type Entity = TestEntity;
    type Aspect = FixtureIdentityAspect<TestSchema>;
    const IDENTIFIER: &'static str = "OtherField";
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

    let obligations = operation.graph_obligations();
    assert_eq!(obligations.rows().len(), 5);
    assert!(obligations.rows().iter().any(|row| {
        row.kind() == crate::facade::WorthQueryInstalledGraphObligationKind::InvariantExecution
            && row.invariant_requirement().is_some()
    }));
    assert_eq!(
        obligations
            .installation_evidence()
            .canonical_work()
            .digest_text_materializations(),
        0
    );

    let authorization = &operation.contracts().ability_requirements()[0];
    assert_ne!(authorization.identity().bytes(), &[0; 32]);
    assert!(
        authorization.canonical_work().digest_derivations() >= 2,
        "the installed policy identity and its path identity must both be phase-accounted"
    );
    assert_eq!(authorization.ability(), "TestAbility");
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
    assert_eq!(operation.contracts().decision_reads().len(), 1);
    let [precondition] = operation.contracts().mutation_preconditions() else {
        panic!("the exact declared mutation precondition must be installed");
    };
    assert_eq!(
        precondition.target().family(),
        worth_query_declaration::facade::application_schema::ApplicationMutationPreconditionFamily::ExpectedFact
    );
    assert_eq!(precondition.target().entity(), "TestEntity");
    assert_eq!(precondition.target().field_name(), "PrincipalIdentityField");
    assert_eq!(operation.contracts().projection_work_budget(), 32);
    assert!(matches!(
        operation.contracts().effects(),
        crate::facade::WorthQueryOperationEffectContract::Declared { effect_families }
            if effect_families == &[crate::facade::WorthQueryOperationEffectFamily::Mutation]
    ));
    assert_eq!(
        operation
            .contracts()
            .execution_strategy()
            .expect("compiled application operation must have one execution strategy")
            .name()
            .as_str(),
        "primary-application-atomic"
    );
}

#[test]
fn reinstallation_rejects_each_coherent_preimage_axis_and_bound_drift() {
    let declaration = test_schema_members::<TestSchema>(Some(recorded_inverse_at::<
        TestEntity,
        FixtureIdentityAspect<TestSchema>,
        FixturePrincipalIdentityField<TestSchema>,
    >(64)))
    .build()
    .unwrap();
    let index = installed_index_for(declaration.clone());
    let schema = index.bind_application_schema(declaration.clone()).unwrap();
    let operation = schema
        .installed_operation(ApplicationOperationRef::<
            TestSchema,
            TestOperation,
            TestInput,
        >::from_schema_identifier("TestOperation"))
        .unwrap();

    assert!(operation.meaning_matches(declaration.erased().members()));
    for changed in [
        coherent_candidate::<OtherEntity, OtherEntityAspect, OtherEntityField>(64),
        coherent_candidate::<TestEntity, OtherAspect, OtherAspectField>(64),
        coherent_candidate::<TestEntity, FixtureIdentityAspect<TestSchema>, OtherField>(64),
        coherent_candidate::<
            TestEntity,
            FixtureIdentityAspect<TestSchema>,
            FixturePrincipalIdentityField<TestSchema>,
        >(65),
    ] {
        assert!(
            !operation.meaning_matches(&changed),
            "the real reinstallation owner must reject one-axis semantic drift"
        );
    }
}

fn installed_index_for(
    declaration: ApplicationSchemaDeclaration<TestSchema>,
) -> WorthQueryInstalledPackageIndex {
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "typed-test",
        1,
        0,
    ))
    .application_schema(declaration)
    .validate()
    .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap()
}

fn coherent_candidate<Entity, Aspect, Field>(
    maximum_encoded_bytes: usize,
) -> Vec<ApplicationSchemaMember>
where
    Entity: ApplicationEntityMarkerIdentity<Schema = TestSchema>,
    Aspect: ApplicationAspectMarkerIdentity<Schema = TestSchema, Entity = Entity>,
    Field: ApplicationFieldMarkerIdentity<Schema = TestSchema, Entity = Entity, Aspect = Aspect>,
{
    let declaration =
        test_schema_members::<TestSchema>(Some(recorded_inverse_at::<Entity, Aspect, Field>(
            maximum_encoded_bytes,
        )))
        .build()
        .unwrap();
    declaration
        .erased()
        .members()
        .iter()
        .cloned()
        .map(|member| match member {
            ApplicationSchemaMember::OperationDecisionRead { operation, .. }
                if operation == "TestOperation" =>
            {
                ApplicationSchemaMember::OperationDecisionRead {
                    operation,
                    target: ApplicationOperationDecisionReadTarget::Field {
                        entity: Entity::IDENTIFIER.to_owned(),
                        aspect: Aspect::IDENTIFIER.to_owned(),
                        field: Field::IDENTIFIER.to_owned(),
                    },
                }
            }
            member => member,
        })
        .collect()
}

fn recorded_inverse_at<Entity, Aspect, Field>(
    maximum_encoded_bytes: usize,
) -> DeclaredApplicationAftermathContract<TestSchema>
where
    Entity: ApplicationEntityMarkerIdentity<Schema = TestSchema>,
    Aspect: ApplicationAspectMarkerIdentity<Schema = TestSchema, Entity = Entity>,
    Field: ApplicationFieldMarkerIdentity<Schema = TestSchema, Entity = Entity, Aspect = Aspect>,
{
    let field = ApplicationFieldRef::<TestSchema, Entity, Aspect, Field, u64>::from_schema_types();
    let inverse = DeclaredRecordedInverse::new(
        "restore-test-operation",
        DeclaredLoweringCorrespondenceRef::new("test-operation-inverse").unwrap(),
        DeclaredAftermathPostcondition::ExactPriorTruth,
        DeclaredPreImageDemand::new(
            [DeclaredPreImageLocus::from_field(field)],
            maximum_encoded_bytes,
        )
        .unwrap(),
    )
    .unwrap();
    DeclaredApplicationAftermathContract::runtime_alone(
        DeclaredCorrectionMechanism::RecordedInverse(inverse),
    )
}
