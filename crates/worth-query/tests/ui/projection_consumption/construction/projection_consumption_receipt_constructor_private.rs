use worth_query::facade::{ProjectionConsumptionReceipt, ProjectionContractSupportPosture};

fn main() {
    let _ = ProjectionConsumptionReceipt {
        declaration_digest: String::new(),
        contract_digest: String::new(),
        fact_set_digest: String::new(),
        source_family: worth_query::facade::ProjectionSourceFamily::RelationalRowSet,
        source_identity: String::new(),
        support_posture: ProjectionContractSupportPosture::Admitted,
        admitted_fact_family_count: 0,
        extracted_fact_count: 0,
        authority_reopen_count: 0,
        deferred_neighbors: Vec::new(),
        counter_snapshot_digest: String::new(),
        integrity_digest: String::new(),
        receipt_digest: String::new(),
    };
}
