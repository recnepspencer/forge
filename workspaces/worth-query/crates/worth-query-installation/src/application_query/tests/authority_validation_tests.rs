use super::*;

#[test]
fn installation_resolves_only_the_package_declared_typed_reference() {
    let schema = installed_schema();
    struct HostAuthoredQuery;
    struct HostParameters;
    struct HostResult;
    let changed = ApplicationQueryReference::<
        QueryTestSchema,
        HostAuthoredQuery,
        ActivityQueryParameters,
        ActivityQueryResult,
        Account,
    >::from_schema_identifier("account_activity");
    let missing = ApplicationQueryReference::<
        QueryTestSchema,
        ActivityQuery,
        ActivityQueryParameters,
        ActivityQueryResult,
        Account,
    >::from_schema_identifier("host_query");
    let changed_parameters = ApplicationQueryReference::<
        QueryTestSchema,
        ActivityQuery,
        HostParameters,
        ActivityQueryResult,
        Account,
    >::from_schema_identifier("account_activity");
    let changed_result = ApplicationQueryReference::<
        QueryTestSchema,
        ActivityQuery,
        ActivityQueryParameters,
        HostResult,
        Account,
    >::from_schema_identifier("account_activity");
    let changed_scope = ApplicationQueryReference::<
        QueryTestSchema,
        ActivityQuery,
        ActivityQueryParameters,
        ActivityQueryResult,
        Activity,
    >::from_schema_identifier("account_activity");

    for denial in [
        schema.application_query(changed).err().unwrap(),
        schema.application_query(changed_parameters).err().unwrap(),
        schema.application_query(changed_result).err().unwrap(),
        schema.application_query(changed_scope).err().unwrap(),
    ] {
        assert_eq!(
            denial.kind(),
            WorthQueryApplicationQueryInstallationDenialKind::QueryMeaningChanged
        );
    }
    assert_eq!(
        schema.application_query(missing).err().unwrap().kind(),
        WorthQueryApplicationQueryInstallationDenialKind::QueryNotInstalled
    );
}

#[test]
fn installed_query_authority_is_exact_to_runtime_and_generation() {
    let current = installed_index();
    let current_schema = current
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap();
    let query = current_schema.application_query(query_reference()).unwrap();

    let rebuilt_schema = current
        .rebuild()
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap();
    rebuilt_schema.validate_installed_query(&query).unwrap();

    let foreign_schema = installed_index()
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap();
    assert_eq!(
        foreign_schema
            .validate_installed_query(&query)
            .unwrap_err()
            .kind(),
        WorthQueryApplicationQueryInstallationDenialKind::ForeignRuntime
    );

    let successor_schema = current
        .successor_generation()
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap();
    assert_eq!(
        successor_schema
            .validate_installed_query(&query)
            .unwrap_err()
            .kind(),
        WorthQueryApplicationQueryInstallationDenialKind::StaleGeneration
    );
}

#[test]
fn installed_query_rejects_same_runtime_package_identity_drift() {
    let runtime = WorthQueryInstallationRuntimeIdentity::fresh();
    let current = installed_index_with(runtime.retained(), false);
    let current_schema = current
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap();
    let query = current_schema.application_query(query_reference()).unwrap();
    let drifted_schema = installed_index_with(runtime, true)
        .bind_application_schema(QueryTestSchema::declaration().unwrap())
        .unwrap();

    assert_eq!(
        drifted_schema
            .validate_installed_query(&query)
            .unwrap_err()
            .kind(),
        WorthQueryApplicationQueryInstallationDenialKind::PackageIdentityChanged
    );
}
