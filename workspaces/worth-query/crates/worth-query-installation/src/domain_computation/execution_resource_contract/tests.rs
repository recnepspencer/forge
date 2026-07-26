use super::WorthQueryExecutionResourceContract;

#[test]
fn undeclared_resource_contract_cannot_become_installed_semantics() {
    assert_eq!(
        WorthQueryExecutionResourceContract::default().validate(),
        Err("undeclared-execution-resource-contract")
    );
}

#[test]
fn declared_resource_contract_requires_at_least_one_bounded_strategy() {
    assert_eq!(
        WorthQueryExecutionResourceContract::declared(std::iter::empty()),
        Err("empty-execution-resource-strategy-set")
    );
}
