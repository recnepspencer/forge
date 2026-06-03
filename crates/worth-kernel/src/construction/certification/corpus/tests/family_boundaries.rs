use super::support::siege_workspace;
use crate::construction::certification::prepare_primitive_construction_family_boundary_report;
use crate::construction::certification::PrimitiveConstructionFamilyBoundaryTransitionClass;
use crate::construction::{
    PrimitiveConstructionBlockingBoundary, PrimitiveConstructionFamily,
    PrimitiveConstructionRejectionLocality,
};
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionWitnessKind, PrimitiveRealizationStrategy,
    PrimitiveStabilityClass,
};

#[test]
fn family_boundary_report_certifies_every_threshold_pair() {
    let mut workspace = siege_workspace("corpus-family-boundaries.threshold-pairs");
    let report = prepare_primitive_construction_family_boundary_report(&mut workspace)
        .expect("family boundary report");

    assert_eq!(report.rows().len(), 6);
    assert!(report
        .row_for(PrimitiveConstructionFamily::SimplexSolid)
        .is_some());
    assert!(report
        .row_for(PrimitiveConstructionFamily::Orthotope)
        .is_some());
    assert!(report
        .row_for(PrimitiveConstructionFamily::RegularPrism)
        .is_some());
    assert!(report
        .row_for(PrimitiveConstructionFamily::RegularPyramid)
        .is_some());
    assert!(report
        .row_for(PrimitiveConstructionFamily::WireBody)
        .is_some());
    assert!(report
        .row_for(PrimitiveConstructionFamily::ShellWithHole)
        .is_some());
    assert_ne!(
        report.report_digest(),
        report
            .row_for(PrimitiveConstructionFamily::SimplexSolid)
            .expect("simplex row")
            .row_digest()
    );
}

#[test]
fn family_boundary_report_distinguishes_direct_and_escalated_boundary_classes() {
    let mut workspace = siege_workspace("corpus-family-boundaries.transition-classes");
    let report = prepare_primitive_construction_family_boundary_report(&mut workspace)
        .expect("family boundary report");
    let simplex = report
        .row_for(PrimitiveConstructionFamily::SimplexSolid)
        .expect("simplex row");
    let prism = report
        .row_for(PrimitiveConstructionFamily::RegularPrism)
        .expect("prism row");
    let pyramid = report
        .row_for(PrimitiveConstructionFamily::RegularPyramid)
        .expect("pyramid row");

    assert_eq!(
        prism.transition_class(),
        PrimitiveConstructionFamilyBoundaryTransitionClass::DirectStableToTypedRejection
    );
    assert_eq!(
        prism.admitted_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        prism.admitted_stability_class(),
        PrimitiveStabilityClass::StableDirect
    );

    assert_eq!(
        simplex.transition_class(),
        PrimitiveConstructionFamilyBoundaryTransitionClass::EscalatedStableToTypedRejection
    );
    assert_eq!(
        simplex.admitted_strategy(),
        PrimitiveRealizationStrategy::ExactSupport
    );
    assert_eq!(
        simplex.admitted_attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        simplex.admitted_stability_class(),
        PrimitiveStabilityClass::StableAfterEscalation
    );
    assert_eq!(
        simplex.admitted_normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );

    assert_eq!(
        pyramid.transition_class(),
        PrimitiveConstructionFamilyBoundaryTransitionClass::EscalatedStableToTypedRejection
    );
    assert_eq!(
        pyramid.admitted_strategy(),
        PrimitiveRealizationStrategy::ExactSupport
    );
    assert_eq!(
        pyramid.admitted_attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        pyramid.admitted_stability_class(),
        PrimitiveStabilityClass::StableAfterEscalation
    );
    assert_eq!(
        pyramid.admitted_normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
}

#[test]
fn family_boundary_report_binds_lower_layer_exhaustion_truth_for_pyramid_floor() {
    let mut workspace = siege_workspace("corpus-family-boundaries.lower-layer-exhaustion");
    let report = prepare_primitive_construction_family_boundary_report(&mut workspace)
        .expect("family boundary report");
    let simplex = report
        .row_for(PrimitiveConstructionFamily::SimplexSolid)
        .expect("simplex row");
    let pyramid = report
        .row_for(PrimitiveConstructionFamily::RegularPyramid)
        .expect("pyramid row");
    let orthotope = report
        .row_for(PrimitiveConstructionFamily::Orthotope)
        .expect("orthotope row");

    assert_eq!(
        pyramid.rejected_rejection_locality(),
        PrimitiveConstructionRejectionLocality::Admission
    );
    assert_eq!(
        pyramid.rejected_blocking_boundary(),
        PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission
    );
    assert_eq!(
        pyramid.lower_layer_exhaustion_witness_kind(),
        Some(PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse)
    );
    assert_eq!(pyramid.lower_layer_exhaustion_witnesses().len(), 1);
    assert_eq!(
        pyramid.lower_layer_exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_eq!(
        pyramid.lower_layer_exhaustion_attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert!(!pyramid.lower_layer_exhaustion_witnesses()[0]
        .row_digest()
        .is_empty());
    assert_eq!(orthotope.lower_layer_exhaustion_witness_kind(), None);
    assert!(orthotope
        .lower_layer_exhaustion_attempted_strategies()
        .is_empty());
    assert!(orthotope.lower_layer_exhaustion_witnesses().is_empty());

    assert_eq!(simplex.lower_layer_exhaustion_witness_kind(), None);
    assert_eq!(simplex.lower_layer_exhaustion_reason(), None);
    assert!(simplex
        .lower_layer_exhaustion_attempted_strategies()
        .is_empty());
    assert_eq!(simplex.lower_layer_exhaustion_witnesses().len(), 2);
    assert_eq!(
        simplex
            .lower_layer_exhaustion_witnesses()
            .iter()
            .map(|witness| witness.witness_kind())
            .collect::<Vec<_>>(),
        vec![
            PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse,
            PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
        ]
    );
    assert!(simplex
        .lower_layer_exhaustion_witnesses()
        .iter()
        .all(|witness| witness.exhaustion_reason()
            == PrimitiveRealizationExhaustionReason::DegenerateSupportNormals));
    assert!(simplex
        .lower_layer_exhaustion_witnesses()
        .iter()
        .all(|witness| witness.attempted_strategies()
            == &[
                PrimitiveRealizationStrategy::DirectWorld,
                PrimitiveRealizationStrategy::ExactSupport,
            ]));
}
