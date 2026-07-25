use worth_query_installation::facade::{
    WorthQueryExecutionResourceEnvelope, WorthQueryExecutionStrategyContract,
};

use crate::admission_digest::hash_parts;

use super::WorthQueryExecutionResourceSupportSnapshot;

pub(super) fn admitted_plan_identity(
    binding_identity: &str,
    contract_identity: &str,
    request_identity: &str,
    support: &WorthQueryExecutionResourceSupportSnapshot,
    strategy: &WorthQueryExecutionStrategyContract,
) -> String {
    let envelope_identity = admitted_envelope_identity(strategy.envelope());
    hash_parts(&[
        "worth_query_admitted_execution_resource_plan_v1".into(),
        format!("binding:{binding_identity}"),
        format!("contract:{contract_identity}"),
        format!("request:{request_identity}"),
        format!("support:{}", support.identity()),
        format!("strategy:{}", strategy.name().as_str()),
        format!("envelope:{envelope_identity}"),
    ])
}

pub(super) fn admitted_envelope_identity(envelope: &WorthQueryExecutionResourceEnvelope) -> String {
    hash_parts(&[
        "worth_query_admitted_execution_resource_envelope_v1".into(),
        format!("mode:{}", envelope.mode().as_str()),
        format!("safe-point:{}", envelope.cancellation_safe_point().as_str()),
        format!(
            "degradation:{}",
            envelope
                .degradation()
                .map_or("complete", |degradation| degradation.as_str())
        ),
        format!(
            "partial-effect:{}",
            envelope.partial_effect_posture().as_str()
        ),
        format!(
            "scale:{}",
            envelope
                .scale_ceilings()
                .iter()
                .map(|(axis, value)| format!("{}={value}", axis.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "resources:{}",
            envelope
                .resource_ceilings()
                .iter()
                .map(|(dimension, value)| format!("{}={value}", dimension.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    ])
}
