use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFactKind, ProjectionConsumedPlanarFactsCounters,
    ProjectionConsumedPlanarFactsReceipt,
};

fn main() {
    let _receipt = ProjectionConsumedPlanarFactsReceipt {
        basis: panic!("private basis"),
        projected_fact_kind: ProjectionConsumedPlanarFactKind::RetainedPlanarClassification,
        declaration_digest: String::new(),
        progression_digest: String::new(),
        route_plan_digest: String::new(),
        query_receipt_digest: String::new(),
        envelope_digest: String::new(),
        retained_planar_fact_digest: String::new(),
        structural_identity_digest: String::new(),
        motion_posture_digest: String::new(),
        topology_contract_digest: String::new(),
        materialization_digest: String::new(),
        projection_consumption_digest: String::new(),
        counters: ProjectionConsumedPlanarFactsCounters::default(),
    };
}
