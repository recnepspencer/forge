use super::*;

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
