use worth_query_installation::facade::WorthQueryExecutionStrategyContract;

use super::WorthQueryExecutionResourceSupportSnapshot;

pub(super) fn admitted_plan_identity(
    binding_identity: &str,
    request_identity: &str,
    support: &WorthQueryExecutionResourceSupportSnapshot,
    strategy: &WorthQueryExecutionStrategyContract,
) -> String {
    crate::identity::hash_parts(&[
        "worth_query_admitted_execution_resource_plan_v1".into(),
        format!("binding:{binding_identity}"),
        format!("request:{request_identity}"),
        format!("support:{}", support.identity()),
        format!("strategy:{}", strategy.name().as_str()),
        format!("mode:{}", strategy.envelope().mode().as_str()),
        format!(
            "safe-point:{}",
            strategy.envelope().cancellation_safe_point().as_str()
        ),
        format!(
            "degradation:{}",
            strategy
                .envelope()
                .degradation()
                .map_or("complete", |degradation| degradation.as_str())
        ),
        format!(
            "scale:{}",
            strategy
                .envelope()
                .scale_ceilings()
                .iter()
                .map(|(axis, value)| format!("{}={value}", axis.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "resources:{}",
            strategy
                .envelope()
                .resource_ceilings()
                .iter()
                .map(|(dimension, value)| format!("{}={value}", dimension.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    ])
}
