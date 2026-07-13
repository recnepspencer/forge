use worth_query::facade::{
    load_projection_authority_contract_document, ProjectionAuthorityContract,
};

fn round_trip() {
    let contract = ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
        .require_target_identity()
        .require_source_references();
    let document = contract
        .to_terminal_json_document()
        .expect("contract document");
    let replayed = load_projection_authority_contract_document(&document.to_external())
        .expect("replayed contract");
    assert_eq!(contract, replayed);
}

fn main() {
    round_trip();
}
