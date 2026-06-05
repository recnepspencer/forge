use hadwiger_research::facade::{
    query_aspect_contract_for_hadwiger_kind, HadwigerAspectKind,
};

fn query_mapping_dx() {
    let contract =
        query_aspect_contract_for_hadwiger_kind(HadwigerAspectKind::UnitDistanceEmbedding);

    assert!(
        contract
            .required()
            .contains(&"hadwiger.embedding.unit_distance".to_string())
    );
}

fn main() {
    let _ = query_mapping_dx as fn();
}
