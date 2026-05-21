use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionWitnessKind,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};
use worth_kernel::facade::{authoring::construction::*, certification::corpus::*};

#[test]
fn kernel_public_facade_exports_direct_simplex_ladder_and_witness_artifacts() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-simplex-ladder".to_string(),
    )
    .expect("workspace");
    let ladder =
        prepare_primitive_construction_simplex_realization_strategy_ladder_report(&mut workspace)
            .expect("simplex ladder");
    let witnesses = prepare_primitive_construction_simplex_realization_exhaustion_witness_report();
    let drift = prepare_primitive_construction_family_boundary_drift_report(&mut workspace)
        .expect("family boundary drift report");

    assert_eq!(ladder.rows().len(), 3);
    assert_eq!(
        ladder
            .row_for("simplex_world_collapsed_admitted_local_or_exact")
            .expect("admitted simplex row")
            .direct_selected_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        ladder
            .row_for("simplex_world_collapsed_admitted_local_or_exact")
            .expect("admitted simplex row")
            .stability_class(),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_eq!(
        ladder
            .row_for("simplex_world_collapsed_explicit_exhaustion")
            .expect("exhausted simplex row")
            .exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_eq!(
        ladder
            .row_for("simplex_world_collapsed_threshold_rejected")
            .expect("threshold rejected simplex row")
            .query_surface_status(),
        PrimitiveConstructionSimplexQuerySurfaceStatus::UnavailableByTypedAdmissionRejection
    );
    assert_eq!(
        ladder
            .row_for("simplex_world_collapsed_explicit_exhaustion")
            .expect("exhausted simplex row")
            .query_surface_status(),
        PrimitiveConstructionSimplexQuerySurfaceStatus::UnavailableByRealizationExhaustion
    );
    assert_eq!(witnesses.rows().len(), 2);
    assert_eq!(
        witnesses
            .row_for(
                PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse
            )
            .expect("altitude-squeezed witness")
            .linked_parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion
    );
    assert_eq!(
        drift
            .row_for(PrimitiveConstructionFamily::SimplexSolid)
            .expect("simplex drift row")
            .lower_layer_exhaustion_witnesses()
            .len(),
        2
    );
}
