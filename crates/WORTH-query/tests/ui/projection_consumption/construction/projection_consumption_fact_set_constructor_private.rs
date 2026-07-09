use worth_query::facade::{
    ConsumedProjectionFactSet, ProjectionContractSupportPosture, ProjectionFactExtractionCounters,
    ProjectionSourceFamily,
};

fn main() {
    let _ = ConsumedProjectionFactSet {
        declaration_digest: "declaration:test".to_string(),
        contract_digest: "contract:test".to_string(),
        source_family: ProjectionSourceFamily::QueryWriteReceipt,
        source_identity: "commit:test".to_string().into(),
        support_posture: ProjectionContractSupportPosture::Admitted,
        counters: ProjectionFactExtractionCounters::default(),
        fact_set_digest: "fact-set:test".to_string(),
        entity_identities: Vec::new(),
        view_local_identities: Vec::new(),
        memberships: Vec::new(),
        display_fields: Vec::new(),
        derived_scalar_fields: Vec::new(),
        target_identities: Vec::new(),
        source_references: Vec::new(),
        effect_continuity_facts: Vec::new(),
        relation_endpoints: Vec::new(),
    };
}
