use hadwiger_research::facade::{
    query_aspect_contract_for_hadwiger_kind, HadwigerAspectKind,
};
use worth_query::facade::foundation::AspectFieldKey;

fn query_mapping_dx() {
    let contract =
        query_aspect_contract_for_hadwiger_kind(HadwigerAspectKind::UnitDistanceEmbedding);

    let unit_distance =
        AspectFieldKey::from_authoring_parts("hadwiger.embedding", "unit_distance").unwrap();
    assert!(contract.required().contains(&unit_distance));
}

fn main() {
    let _ = query_mapping_dx as fn();
}
