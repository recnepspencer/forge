use worth_query::facade::runtime::WorthQueryContinuityMutationEvidence;

fn main() {
    let _ = WorthQueryContinuityMutationEvidence {
        family: todo!(),
        outcome_class: todo!(),
        prior_authoritative_identity: String::new(),
        successor_authoritative_identities: Vec::new(),
        basis_binding_digest: None,
        resolved_target_entity_identity: None,
        target_collection: None,
        lineage_digest: String::new(),
        continuity_resolution_digest: String::new(),
    };
}
