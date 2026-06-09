use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::specs::WireBodySpec;
use crate::construction::tests::support::realization::{
    prepare_primitive_construction_conditioning_witness_report,
    prepare_primitive_construction_realization_exhaustion_report,
    prepare_primitive_construction_realization_exhaustion_witness_report,
    prepare_primitive_construction_realization_strategy_report,
    prepare_primitive_construction_stability_class_report,
    PrimitiveConstructionRealizationExhaustionStatus,
};
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionWitnessKind, PrimitiveRealizationStrategy,
    PrimitiveStabilityClass,
};

#[test]
fn realization_reports_do_not_invent_conditioning_truth_for_admission_rejection() {
    let request = PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 });
    let strategy = prepare_primitive_construction_realization_strategy_report(request.clone());
    let witness = prepare_primitive_construction_conditioning_witness_report(request.clone());
    let stability = prepare_primitive_construction_stability_class_report(request.clone());
    let exhaustion = prepare_primitive_construction_realization_exhaustion_report(request);

    assert!(!strategy.admitted());
    assert!(strategy.attempted_strategies().is_empty());
    assert_eq!(witness.conditioning_witness(), None);
    assert_eq!(stability.stability_class(), None);
    assert_eq!(
        exhaustion.status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
    );
}

#[test]
fn realization_exhaustion_witness_report_exposes_live_lower_layer_exhaustion_truth() {
    let report = prepare_primitive_construction_realization_exhaustion_witness_report();
    let pyramid = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse)
        .expect("zero-radius exhaustion witness");
    let simplex = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse)
        .expect("zero-scale exhaustion witness");
    let squeezed_simplex = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse)
        .expect("altitude-squeezed exhaustion witness");

    assert_eq!(
        pyramid.family(),
        PrimitiveConstructionFamily::RegularPyramid
    );
    assert_eq!(
        pyramid.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        pyramid.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        pyramid.exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        pyramid.conditioning_witness().normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert_eq!(simplex.family(), PrimitiveConstructionFamily::SimplexSolid);
    assert_eq!(
        simplex.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        simplex.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        simplex.exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        simplex.conditioning_witness().normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert_eq!(
        squeezed_simplex.family(),
        PrimitiveConstructionFamily::SimplexSolid
    );
    assert_eq!(
        squeezed_simplex.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        squeezed_simplex.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        squeezed_simplex.exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        squeezed_simplex
            .conditioning_witness()
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert_ne!(report.report_digest(), simplex.row_digest());
}
