use crate::logic::runtime::RelationalRuntime;
use forge_foundational::facade::AspectKey;

pub(super) fn declared_aspects_for_entity_kind(
    runtime: &RelationalRuntime,
    kind_id: crate::identity::data::KindId,
) -> Vec<AspectKey> {
    runtime
        .schema_contract_runtime
        .aspect_contract_plans
        .entity_plans
        .get(&kind_id)
        .map(plan_aspect_keys)
        .unwrap_or_default()
}

pub(super) fn declared_aspects_for_relation_kind(
    runtime: &RelationalRuntime,
    kind_id: crate::identity::data::KindId,
) -> Vec<AspectKey> {
    runtime
        .schema_contract_runtime
        .aspect_contract_plans
        .relation_plans
        .get(&kind_id)
        .map(plan_aspect_keys)
        .unwrap_or_default()
}

fn plan_aspect_keys(plan: &crate::schema::data::LoweredAspectContractPlan) -> Vec<AspectKey> {
    let mut aspects = plan
        .executable_bindings
        .iter()
        .map(|binding| binding.aspect_key().clone())
        .collect::<Vec<_>>();
    if !aspects.windows(2).all(|window| window[0] < window[1]) {
        aspects.sort();
        aspects.dedup();
    }
    aspects
}
