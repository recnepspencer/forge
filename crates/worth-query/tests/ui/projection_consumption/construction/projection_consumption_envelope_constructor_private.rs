use worth_query::facade::foundation::{ProjectionContractSupportPosture, SelfDescribingProjectionConsumptionEnvelope};

fn main() {
    let _ = SelfDescribingProjectionConsumptionEnvelope {
        source_family: worth_query::facade::foundation::ProjectionSourceFamily::RelationalRowSet,
        source_identity: String::new(),
        support_posture: ProjectionContractSupportPosture::Admitted,
        admitted_fact_family_count: 0,
        extracted_fact_count: 0,
        authority_reopen_count: 0,
        transition_rules_digest: String::new(),
        deferred_neighbors: Vec::new(),
        integrity_digest: String::new(),
        performance_digest: String::new().into(),
        boundary_digest: String::new(),
        sources: panic!(),
        envelope_digest: String::new(),
    };
}
