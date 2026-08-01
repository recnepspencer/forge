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
    assert_eq!(
        operation
            .contracts()
            .obligations()
            .rows()
            .iter()
            .map(crate::facade::WorthQueryInstalledGraphObligation::kind)
            .collect::<Vec<_>>(),
        [
            crate::facade::WorthQueryInstalledGraphObligationKind::GraphRead,
            crate::facade::WorthQueryInstalledGraphObligationKind::AuthorizationObservation,
            crate::facade::WorthQueryInstalledGraphObligationKind::MutationTouch,
            crate::facade::WorthQueryInstalledGraphObligationKind::EffectApplication,
            crate::facade::WorthQueryInstalledGraphObligationKind::InvariantExecution,
        ]
    );
    let obligations = operation.contracts().obligations();
    let installation = obligations.installation_evidence();
    assert_eq!(installation.obligation_rows(), 5);
    assert_eq!(installation.selector_index_entries(), 5);
    assert_eq!(installation.canonical_work().digest_derivations(), 1);
    assert!(installation.canonical_work().canonical_encoded_bytes() > 0);
    for kind in crate::facade::WorthQueryInstalledGraphObligationKind::ALL {
        let lookup = obligations.inspect_kind(kind);
        assert_eq!(lookup.selector_index_probes(), 1);
        assert_eq!(lookup.rows().len(), 1);
        assert_eq!(
            lookup.canonical_work(),
            crate::canonical_work::WorthQueryCanonicalWorkEvidence::zero()
        );
    }
    let routes = obligations
        .rows()
        .iter()
        .map(|row| row.owner_progression())
        .collect::<Vec<_>>();
    use crate::facade::WorthQueryInstalledGraphObligationOwner as Owner;
    assert_eq!(routes[0], [Owner::Relational, Owner::QueryExecution]);
    assert_eq!(
        routes[1],
        [
            Owner::Relational,
            Owner::RuntimeBridge,
            Owner::Signal,
            Owner::QueryAdmission,
        ]
    );
    assert_eq!(routes[2], [Owner::Relational, Owner::QueryAdmission]);
    assert_eq!(routes[3], [Owner::QueryExecution, Owner::Relational]);
    assert_eq!(routes[4], [Owner::QueryExecution, Owner::Relational]);
    assert!(matches!(
        obligations.rows()[1].authorization_requirement(),
        Some(crate::facade::WorthQueryInstalledGraphAuthorizationRequirement::Abilities(
            requirements
        )) if requirements.len() == 1 && requirements[0].ability() == "TestAbility"
    ));
}

#[test]
fn installed_operation_rejects_foreign_runtime_and_successor_generation_substitution() {
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

    let foreign = installed_index()
        .validate_application_operation(&operation)
        .unwrap_err();
    assert_eq!(
        foreign.kind(),
        crate::facade::WorthQueryApplicationOperationInstallationDenialKind::ForeignRuntime
    );
    let stale = index
        .successor_generation()
        .validate_application_operation(&operation)
        .unwrap_err();
    assert_eq!(
        stale.kind(),
        crate::facade::WorthQueryApplicationOperationInstallationDenialKind::StaleGeneration
    );
}
