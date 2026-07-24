use std::collections::BTreeSet;

use worth_query_declaration::facade::domain_computation::WorthQueryResourceDimension;

use super::WorthQueryExecutionResourceContract;

pub(super) fn validate_resource_contract(
    contract: &WorthQueryExecutionResourceContract,
) -> Result<(), &'static str> {
    let WorthQueryExecutionResourceContract::Declared { strategies } = contract else {
        return Err("undeclared-execution-resource-contract");
    };
    if strategies.is_empty() {
        return Err("empty-execution-resource-strategy-set");
    }
    let mut names = BTreeSet::new();
    for strategy in strategies {
        if !names.insert(strategy.name().as_str()) {
            return Err("duplicate-execution-resource-strategy");
        }
        if strategy
            .envelope()
            .resource_ceiling(WorthQueryResourceDimension::CancellationPollingInterval)
            == 0
        {
            return Err("zero-cancellation-polling-interval");
        }
        if strategy
            .envelope()
            .resource_ceiling(WorthQueryResourceDimension::CleanupBudget)
            == 0
        {
            return Err("zero-cleanup-budget");
        }
    }
    Ok(())
}
