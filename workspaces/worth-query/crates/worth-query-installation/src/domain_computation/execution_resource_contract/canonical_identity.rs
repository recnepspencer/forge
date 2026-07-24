use super::WorthQueryExecutionResourceContract;

pub(super) fn canonical_resource_contract_token(
    contract: &WorthQueryExecutionResourceContract,
) -> String {
    let WorthQueryExecutionResourceContract::Declared { strategies } = contract else {
        return "undeclared".into();
    };
    let mut token = String::from("declared");
    for strategy in strategies {
        token.push('|');
        token.push_str(strategy.name().as_str());
        token.push('|');
        token.push_str(strategy.provider_requirements().provider().as_str());
        token.push('|');
        token.push_str(strategy.provider_requirements().access_product().as_str());
        token.push('|');
        token.push_str(strategy.provider_requirements().allocator().as_str());
        token.push('|');
        token.push_str(strategy.envelope().mode().as_str());
        token.push('|');
        token.push_str(strategy.envelope().cancellation_safe_point().as_str());
        token.push('|');
        token.push_str(
            strategy
                .envelope()
                .degradation()
                .map_or("complete", |degradation| degradation.as_str()),
        );
        for (axis, value) in strategy.envelope().scale_ceilings().iter() {
            token.push('|');
            token.push_str(axis.as_str());
            token.push('=');
            token.push_str(&value.to_string());
        }
        for (dimension, value) in strategy.envelope().resource_ceilings().iter() {
            token.push('|');
            token.push_str(dimension.as_str());
            token.push('=');
            token.push_str(&value.to_string());
        }
    }
    token
}
