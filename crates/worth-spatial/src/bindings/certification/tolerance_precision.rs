use forge_query::facade::ForgeQueryApplicationFacade;
use worth_geom::facade::{
    realize_tetrahedron_support, Plane, PrimitiveFeatureConditioningClass,
    PrimitiveRealizationError, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};
use worth_primitives::PrimitiveConstructionFamilyKey;

use crate::bindings::query_native_tolerance_precision::{
    ToleranceAndPrecisionCertificationQueryDomain, ToleranceAndPrecisionCertificationQueryWorld,
};
use crate::bindings::query_native_tolerance_precision_authoring::{
    primitive_construction_tolerance_and_precision_certification_entry,
    primitive_construction_tolerance_and_precision_certification_facts,
    ToleranceAndPrecisionCertificationCase, ToleranceAndPrecisionCertificationPosture,
    ToleranceAndPrecisionRealizationPosture, ToleranceAndPrecisionToleranceBasis,
};

fn admitted_tolerance_precision_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    ToleranceAndPrecisionCertificationQueryDomain,
    ToleranceAndPrecisionCertificationQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ToleranceAndPrecisionCertificationQueryDomain)
        .with_operating_context(ToleranceAndPrecisionCertificationQueryWorld::new(world))
        .validate()
        .expect("tolerance/precision query handle should validate")
        .admit()
        .expect("tolerance/precision query handle should admit")
}

#[test]
fn tolerance_precision_certification_preserves_direct_realization_truth_as_typed_receipt() {
    let posture = ToleranceAndPrecisionRealizationPosture::from_direct_planar_support(
        "orthotope",
        &[
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        &[Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")],
    );
    let entry = primitive_construction_tolerance_and_precision_certification_entry(
        ToleranceAndPrecisionCertificationCase::primitive_construction_birth(
            PrimitiveConstructionFamilyKey::Orthotope,
            "policy:direct",
            ToleranceAndPrecisionToleranceBasis::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            posture.clone(),
        ),
    );
    let receipt = primitive_construction_tolerance_and_precision_certification_facts(
        &entry,
        &admitted_tolerance_precision_handle("tolerance-direct"),
    )
    .expect("tolerance facts");

    assert_eq!(
        receipt.certification_posture(),
        ToleranceAndPrecisionCertificationPosture::CertifiedStable
    );
    assert_eq!(
        receipt.certified_bound().precision_headroom_ratio(),
        posture.conditioning_witness().precision_headroom_ratio()
    );
    assert_eq!(
        receipt.certified_bound().support_normal_headroom_ratio(),
        posture
            .conditioning_witness()
            .support_normal_headroom_ratio()
    );
    assert!(receipt.escalation_trace().is_empty());
    assert_eq!(receipt.unsupported_reason(), None);
}

#[test]
fn tolerance_precision_certification_marks_degenerate_support_normals_as_unsupported() {
    let posture = match realize_tetrahedron_support([0.0, 0.0, 0.0], 1.0e-200)
        .expect_err("tiny simplex should exhaust")
    {
        PrimitiveRealizationError::Exhausted(report) => {
            ToleranceAndPrecisionRealizationPosture::from_exhaustion_report(report)
        }
        PrimitiveRealizationError::Geometry(error) => {
            panic!("expected exhausted realization report, found geometry error: {error}")
        }
    };
    let entry = primitive_construction_tolerance_and_precision_certification_entry(
        ToleranceAndPrecisionCertificationCase::primitive_construction_birth(
            PrimitiveConstructionFamilyKey::SimplexSolid,
            "policy:degenerate",
            ToleranceAndPrecisionToleranceBasis::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            posture.clone(),
        ),
    );
    let receipt = primitive_construction_tolerance_and_precision_certification_facts(
        &entry,
        &admitted_tolerance_precision_handle("tolerance-degenerate"),
    )
    .expect("tolerance facts");

    assert_eq!(
        receipt.certification_posture(),
        ToleranceAndPrecisionCertificationPosture::Unsupported
    );
    assert!(matches!(
        receipt.unsupported_reason(),
        Some(
            crate::bindings::query_native_tolerance_precision_authoring::ToleranceAndPrecisionUnsupportedReason::CollapsedFeatureScale
        ) | Some(
            crate::bindings::query_native_tolerance_precision_authoring::ToleranceAndPrecisionUnsupportedReason::DegenerateSupportNormals
        )
    ));
    assert!(matches!(
        posture.stability_class(),
        PrimitiveStabilityClass::StableDirect
            | PrimitiveStabilityClass::StableAfterEscalation
            | PrimitiveStabilityClass::RejectedBelowConditioningFloor
    ));
    assert!(matches!(
        posture.conditioning_witness().feature_conditioning_class(),
        PrimitiveFeatureConditioningClass::Collapsed
            | PrimitiveFeatureConditioningClass::NearThreshold
            | PrimitiveFeatureConditioningClass::Healthy
    ));
    assert!(matches!(
        posture.conditioning_witness().support_normal_class(),
        PrimitiveSupportNormalClass::Degenerate
            | PrimitiveSupportNormalClass::NearDegenerate
            | PrimitiveSupportNormalClass::Robust
    ));
    assert!(matches!(
        posture.selected_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
            | PrimitiveRealizationStrategy::LocalNormalized
            | PrimitiveRealizationStrategy::ExactSupport
    ));
}
