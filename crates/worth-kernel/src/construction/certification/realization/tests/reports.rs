use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionWitnessKind, PrimitiveRealizationStrategy,
    PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

use crate::construction::certification::{
    prepare_primitive_construction_conditioning_witness_report,
    prepare_primitive_construction_realization_exhaustion_report,
    prepare_primitive_construction_realization_exhaustion_witness_report,
    prepare_primitive_construction_realization_report_bundle,
    prepare_primitive_construction_realization_strategy_report,
    prepare_primitive_construction_stability_class_report,
    PrimitiveConstructionRealizationExhaustionStatus,
};
use crate::construction::{
    OrthotopeSpec, PrimitiveConstructionFamily, PrimitiveConstructionIntent, RegularPyramidSpec,
    WireBodySpec,
};

#[test]
fn realization_reports_certify_direct_stable_orthotope() {
    let strategy = prepare_primitive_construction_realization_strategy_report(
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
    );
    let witness = prepare_primitive_construction_conditioning_witness_report(
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
    );
    let stability = prepare_primitive_construction_stability_class_report(
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
    );
    let exhaustion = prepare_primitive_construction_realization_exhaustion_report(
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
    );

    assert_eq!(strategy.family(), PrimitiveConstructionFamily::Orthotope);
    assert!(strategy.admitted());
    assert_eq!(
        strategy.selected_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        strategy.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_eq!(strategy.attempted_strategies().len(), 1);
    assert!(strategy.canonical_artifact_digest().is_some());

    assert!(witness.admitted());
    assert!(witness.conditioning_witness().is_some());
    assert_eq!(witness.exhaustion_reason(), None);
    assert_eq!(
        witness
            .conditioning_witness()
            .expect("conditioning witness")
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::WorldSpaceSufficient
    );

    assert!(stability.admitted());
    assert_eq!(
        stability.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_eq!(stability.attempted_realization_strategy_count(), 1);

    assert_eq!(
        exhaustion.status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
    );
    assert_eq!(exhaustion.exhaustion_reason(), None);
}

#[test]
fn realization_reports_certify_escalated_pyramid_truth() {
    let request = PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
        sides: 3,
        radius: 1.0e-200,
        height: 1.0e-200,
    });
    let strategy = prepare_primitive_construction_realization_strategy_report(request.clone());
    let witness = prepare_primitive_construction_conditioning_witness_report(request.clone());
    let stability = prepare_primitive_construction_stability_class_report(request.clone());
    let exhaustion = prepare_primitive_construction_realization_exhaustion_report(request);

    assert!(strategy.admitted());
    assert_eq!(
        strategy.selected_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        strategy.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        stability.stability_class(),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_eq!(stability.attempted_realization_strategy_count(), 2);
    assert!(witness.conditioning_witness().is_some());
    assert_eq!(
        witness
            .conditioning_witness()
            .expect("conditioning witness")
            .support_normal_class(),
        PrimitiveSupportNormalClass::Degenerate
    );
    assert_eq!(
        witness
            .conditioning_witness()
            .expect("conditioning witness")
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert_eq!(
        exhaustion.status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotExhausted
    );
}

#[test]
fn realization_report_bundle_preserves_one_shared_certified_view() {
    let bundle = prepare_primitive_construction_realization_report_bundle(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0e-200,
            height: 1.0e-200,
        }),
    );

    assert_eq!(
        bundle.strategy_report().selected_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        bundle.stability_class_report().stability_class(),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_eq!(
        bundle.exhaustion_report().status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotExhausted
    );
    assert!(bundle
        .conditioning_witness_report()
        .conditioning_witness()
        .is_some());
}

#[test]
fn realization_reports_certify_world_collapsed_pyramid_salvage() {
    let request = PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
        sides: 3,
        radius: 1.0,
        height: 1.0,
    })
    .at([1.0e308, 1.0e308, 1.0e308]);
    let strategy = prepare_primitive_construction_realization_strategy_report(request.clone());
    let witness = prepare_primitive_construction_conditioning_witness_report(request.clone());
    let stability = prepare_primitive_construction_stability_class_report(request.clone());
    let exhaustion = prepare_primitive_construction_realization_exhaustion_report(request);

    assert!(strategy.admitted());
    assert_eq!(
        strategy.selected_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        strategy.attempted_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );
    assert!(strategy.canonical_artifact_digest().is_some());

    assert!(witness.admitted());
    assert_eq!(witness.exhaustion_reason(), None);
    assert!(!witness
        .conditioning_witness()
        .expect("conditioning witness")
        .feature_size_collapsed());
    assert_eq!(
        witness
            .conditioning_witness()
            .expect("conditioning witness")
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::WorldSpaceSufficient
    );

    assert_eq!(
        stability.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_eq!(
        stability.attempted_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );

    assert_eq!(
        exhaustion.status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
    );
    assert_eq!(exhaustion.exhaustion_reason(), None);
}

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
    let row = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse)
        .expect("zero-radius exhaustion witness");

    assert_eq!(row.family(), PrimitiveConstructionFamily::RegularPyramid);
    assert_eq!(
        row.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        row.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        row.exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        row.conditioning_witness().normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert!(!report.report_digest().is_empty());
}
