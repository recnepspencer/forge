use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsDenialKind,
};

use super::contract_subject::{projection_consumed_planar_parts, stray_projection};
use super::runtime_handles::projection_consumption_handle;

#[test]
fn projection_consumed_planar_facts_preserve_denials_without_summary_upgrade() {
    let world = "projection-consumed-denials";
    let parts = projection_consumed_planar_parts(world);
    let contracts =
        ProjectionConsumedPlanarFactsContracts::new(projection_consumption_handle(world));

    let mut with_stray = parts.projections.clone();
    with_stray.push(stray_projection(world, &parts));
    let stray_denial =
        match ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
            .consume_bundle_projection_receipts(with_stray)
            .compile(&contracts)
        {
            Ok(_) => panic!("stray projection receipt must deny projection consumption"),
            Err(error) => error,
        };
    assert_eq!(
        stray_denial.kind(),
        ProjectionConsumedPlanarFactsDenialKind::MismatchedProjectionClosure
    );

    let mut duplicate = parts.projections.clone();
    duplicate.push(parts.projections[0].clone());
    let duplicate_denial =
        match ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
            .consume_bundle_projection_receipts(duplicate)
            .compile(&contracts)
        {
            Ok(_) => panic!("duplicate projection receipt must deny projection consumption"),
            Err(error) => error,
        };
    assert_eq!(
        duplicate_denial.kind(),
        ProjectionConsumedPlanarFactsDenialKind::DuplicateProjectionReceipt
    );

    let missing_denial =
        match ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained)
            .consume_bundle_projection_receipts(Vec::new())
            .compile(&contracts)
        {
            Ok(_) => panic!("summary-only projection consumption must deny"),
            Err(error) => error,
        };
    assert_eq!(
        missing_denial.kind(),
        ProjectionConsumedPlanarFactsDenialKind::MissingProjectionReceipts
    );
}

#[test]
fn projection_consumed_planar_facts_reject_blank_materialization_binding() {
    let world = "projection-consumed-blank-materialization";
    let parts = projection_consumed_planar_parts(world);
    let contracts =
        ProjectionConsumedPlanarFactsContracts::new(projection_consumption_handle(world));

    let denial =
        match ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
            .consume_bundle_projection_receipts(parts.projections.clone())
            .materialize_as("   ")
            .compile(&contracts)
        {
            Ok(_) => panic!("blank materialization identity must deny projection consumption"),
            Err(error) => error,
        };

    assert_eq!(
        denial.kind(),
        ProjectionConsumedPlanarFactsDenialKind::InvalidMaterializationBasis
    );
}
