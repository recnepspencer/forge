use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
};

use super::contract_subject::projection_consumed_planar_parts;
use super::runtime_handles::projection_consumption_handle;

#[test]
fn projection_consumed_planar_facts_match_live_and_retained_basis() {
    let world = "projection-consumed-parity";
    let parts = projection_consumed_planar_parts(world);
    let contracts =
        ProjectionConsumedPlanarFactsContracts::new(projection_consumption_handle(world));
    let plan = ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
        .consume_bundle_projection_receipts(parts.projections.clone())
        .materialize_as("materialization:projection-consumed-parity")
        .compile(&contracts)
        .expect("projection-consumed plan");

    assert_eq!(
        plan.inspected_projection_consumption_rows(),
        parts.projections.len() + 5
    );
    let receipt = plan.consume().expect("projection-consumed receipt");

    assert_eq!(
        receipt.retained_planar_fact_digest(),
        parts.retained.retained_fact_digest()
    );
    assert_eq!(
        receipt.structural_identity_digest(),
        parts
            .retained
            .basis()
            .structural_identity_receipt()
            .structural_identity_digest()
    );
    assert_eq!(
        receipt.motion_posture_digest(),
        parts
            .retained
            .basis()
            .motion_posture_receipt()
            .retained_motion_digest()
    );
    assert_eq!(
        receipt.topology_contract_digest(),
        parts
            .retained
            .basis()
            .topology_contract_receipt()
            .fact_digest()
    );
    assert_eq!(
        receipt.counters().projection_receipts_consumed(),
        parts.projections.len()
    );
    assert_eq!(
        receipt.counters().retained_source_rows_inspected(),
        parts.readiness.basis().family_rows().len()
    );
    assert_eq!(receipt.counters().materialization_binding_rows(), 1);
    assert_eq!(receipt.counters().rejected_projection_rows(), 0);
    assert_eq!(
        receipt.counters().projection_consumption_breadth(),
        parts.projections.len() + 5
    );
    assert!(!receipt.materialization_digest().is_empty());
    assert!(!receipt.projection_consumption_digest().is_empty());
}

#[test]
fn projection_consumed_planar_fact_parity_matches_retained_basis() {
    let world = "projection-consumed-parity";
    let parts = projection_consumed_planar_parts(world);
    let receipt = ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
        .consume_bundle_projection_receipts(parts.projections.clone())
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(world),
        ))
        .expect("MB-M6-7 projection-consumption plan")
        .consume()
        .expect("MB-M6-7 projection-consumed receipt");

    assert_eq!(
        receipt.basis().projection_receipts().len(),
        parts.readiness.basis().projection_receipts().len()
    );
    assert_eq!(
        receipt.retained_planar_fact_digest(),
        parts.retained.retained_fact_digest()
    );
    assert_eq!(
        receipt.structural_identity_digest(),
        parts
            .retained
            .basis()
            .structural_identity_receipt()
            .structural_identity_digest()
    );
    assert_eq!(
        receipt.motion_posture_digest(),
        parts
            .retained
            .basis()
            .motion_posture_receipt()
            .retained_motion_digest()
    );
}

#[test]
fn projection_consumed_planar_facts_canonicalize_projection_receipt_order() {
    let world = "projection-consumed-canonical-order";
    let parts = projection_consumed_planar_parts(world);
    let contracts =
        ProjectionConsumedPlanarFactsContracts::new(projection_consumption_handle(world));
    let retained = parts.retained.clone();
    let materialization = "materialization:projection-consumed-canonical-order";
    let ordinary_receipt =
        ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
            .consume_bundle_projection_receipts(parts.projections.clone())
            .materialize_as(materialization)
            .compile(&contracts)
            .expect("ordinary projection-consumption plan")
            .consume()
            .expect("ordinary projection-consumed receipt");
    let mut reversed_projections = parts.projections.clone();
    reversed_projections.reverse();
    let reversed_receipt = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained)
        .consume_bundle_projection_receipts(reversed_projections)
        .materialize_as(materialization)
        .compile(&contracts)
        .expect("reversed projection-consumption plan")
        .consume()
        .expect("reversed projection-consumed receipt");

    assert_eq!(
        ordinary_receipt.declaration_digest(),
        reversed_receipt.declaration_digest()
    );
    assert_eq!(
        ordinary_receipt.materialization_digest(),
        reversed_receipt.materialization_digest()
    );
    assert_eq!(
        ordinary_receipt.projection_consumption_digest(),
        reversed_receipt.projection_consumption_digest()
    );
}
