use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_construction_query_intent_arbitration_inspection_parity_report,
    prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report,
    prepare_primitive_intent_arbitration_policy_report,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationQueryFactProvenance,
    PrimitiveConstructionIntentArbitrationQueryReadSurface, PrimitiveConstructionIntentChosenTruth,
};
use worth_spatial::facade::SpatialIntentCandidate;

#[test]
fn kernel_public_facade_exports_query_intent_arbitration_parity_surface() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api-intent-query".to_string(),
    )
    .expect("workspace");
    let policy = prepare_primitive_intent_arbitration_policy_report().expect("policy");
    let chosen = prepare_primitive_chosen_intent_resolution_report().expect("chosen");

    let inspection =
        prepare_primitive_construction_query_intent_arbitration_inspection_parity_report(
            &mut workspace,
            policy
                .row(PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict)
                .expect("grazing row")
                .clone(),
            None,
        )
        .expect("inspection");
    let projection = prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report(
        &mut workspace,
        policy
            .row(PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict)
            .expect("grazing row")
            .clone(),
        Some(
            chosen
                .row(PrimitiveConstructionChosenIntentResolutionCase::ExplicitSnapFlush)
                .expect("chosen row")
                .clone(),
        ),
    )
    .expect("projection");

    assert_eq!(
        inspection.read_surface(),
        PrimitiveConstructionIntentArbitrationQueryReadSurface::IntentArbitrationPolicyInspection
    );
    assert_eq!(
        inspection.chosen_truth(),
        PrimitiveConstructionIntentChosenTruth::Unresolved
    );
    assert_eq!(
        projection.fact_provenance(),
        PrimitiveConstructionIntentArbitrationQueryFactProvenance::EquivalentProjectionConsumptionFacts
    );
    assert_eq!(
        projection.chosen_truth(),
        PrimitiveConstructionIntentChosenTruth::Resolved {
            candidate: SpatialIntentCandidate::SnapFlush,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
        }
    );
    assert!(inspection.parity_verified());
    assert!(projection.parity_verified());
}
