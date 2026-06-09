use worth_geom::facade::Plane;
use worth_primitives::PrimitiveConstructionFamilyKey;
use worth_spatial::facade::tolerance::{
    primitive_construction_tolerance_and_precision_certification_entry, CertifiedToleranceBound,
    PrimitiveConstructionToleranceAndPrecisionCertificationEntry,
    ToleranceAndPrecisionCertificateKind, ToleranceAndPrecisionCertificationCase,
    ToleranceAndPrecisionCertificationDeclarationFamily,
    ToleranceAndPrecisionCertificationFactReceipt, ToleranceAndPrecisionCertificationPosture,
    ToleranceAndPrecisionCertificationQueryDomain, ToleranceAndPrecisionCertificationQueryWorld,
    ToleranceAndPrecisionRealizationPosture, ToleranceAndPrecisionToleranceBasis,
};

#[test]
fn spatial_public_facade_exports_tolerance_precision_family_surface() {
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
    let case = ToleranceAndPrecisionCertificationCase::primitive_construction_birth(
        PrimitiveConstructionFamilyKey::Orthotope,
        "policy:public",
        ToleranceAndPrecisionToleranceBasis::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        posture,
    );
    let entry = primitive_construction_tolerance_and_precision_certification_entry(case);

    let _: PrimitiveConstructionToleranceAndPrecisionCertificationEntry = entry;
    let _: ToleranceAndPrecisionCertificationDeclarationFamily =
        ToleranceAndPrecisionCertificationDeclarationFamily;
    let _: ToleranceAndPrecisionCertificationQueryDomain =
        ToleranceAndPrecisionCertificationQueryDomain;
    let _: ToleranceAndPrecisionCertificationQueryWorld =
        ToleranceAndPrecisionCertificationQueryWorld::new("public");
    let _: ToleranceAndPrecisionCertificateKind =
        ToleranceAndPrecisionCertificateKind::PrimitiveConstructionBirth;
    let _: ToleranceAndPrecisionCertificationPosture =
        ToleranceAndPrecisionCertificationPosture::CertifiedStable;
    let _: CertifiedToleranceBound = CertifiedToleranceBound::new(1.0, 1.0, 1.0);
    let _: Option<ToleranceAndPrecisionCertificationFactReceipt> = None;
}
