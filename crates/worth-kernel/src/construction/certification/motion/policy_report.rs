use forge_query::facade::ForgeQueryWorkspace;
use worth_geom::ParameterSpacePoint;

use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::{
    prepare_primitive_construction_move_motion_report_bundle,
    prepare_primitive_construction_points_toward_motion_report_bundle,
    prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog,
    prepare_primitive_construction_reorient_motion_report_bundle,
    prepare_primitive_construction_reorient_motion_report_bundle_with_catalog,
    prepare_primitive_construction_rotate_motion_report_bundle_with_catalog,
    PrimitiveConstructionFamily, PrimitiveConstructionIntent,
    PrimitiveConstructionMotionReportBundleError, PrimitiveConstructionMotionRuntimeSurfaceStatus,
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionStatus,
    PrimitiveConstructionRequestedMotionWitness, PrimitiveConstructionVerifiedMotionReportBundle,
    RegularPyramidSpec, WireBodySpec,
};
use crate::spatial_intent::{MoveSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent};
use crate::test_support::SpatialFixtureWitnessCatalog;
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
    SpatialDirectionWitnessRef, SpatialFrameRef, SpatialPointWitnessRef,
};
use worth_spatial::facade::witness_catalog::{
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass,
};
use worth_spatial::facade::witness_resolution::{
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionResolutionPolicyCase {
    DirectMove,
    FrameReorient,
    CarrierReorient,
    FallbackPointsToward,
    AmbiguousMove,
    UndefinedReorient,
    UnsupportedReorient,
    ExhaustedRotate,
    CoincidentPointsToward,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionResolutionPolicyRow {
    case: PrimitiveConstructionMotionResolutionPolicyCase,
    subject_family: PrimitiveConstructionFamily,
    anchor: SpatialAnchorRef,
    kind: PrimitiveConstructionMotionWitnessResolutionKind,
    requested_witness: PrimitiveConstructionRequestedMotionWitness,
    status: PrimitiveConstructionMotionWitnessResolutionStatus,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    failure_kind: Option<PrimitiveConstructionMotionWitnessResolutionFailureKind>,
    runtime_surface_status: PrimitiveConstructionMotionRuntimeSurfaceStatus,
    row_digest: String,
}

impl PrimitiveConstructionMotionResolutionPolicyRow {
    fn new(
        case: PrimitiveConstructionMotionResolutionPolicyCase,
        bundle: PrimitiveConstructionVerifiedMotionReportBundle,
    ) -> Self {
        let witness_report = bundle.witness_report();
        let runtime_surface_status = bundle
            .branch_preview_runtime_report()
            .runtime_surface_status();
        let row_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &[
                format!("{case:?}"),
                witness_report.subject_family().as_str().to_string(),
                format!("{:?}", witness_report.anchor()),
                format!("{:?}", witness_report.kind()),
                format!("{:?}", witness_report.requested_witness()),
                format!("{:?}", witness_report.status()),
                format!("{:?}", witness_report.resolution_class()),
                format!("{:?}", witness_report.failure_kind()),
                format!("{runtime_surface_status:?}"),
                bundle.truth().truth_digest().to_string(),
                bundle.witness_report().report_digest().to_string(),
                bundle.replay_parity_report().report_digest().to_string(),
                bundle
                    .query_inspection_parity_report()
                    .report_digest()
                    .to_string(),
                bundle
                    .query_projection_receipt_report()
                    .report_digest()
                    .to_string(),
                bundle
                    .branch_preview_runtime_report()
                    .report_digest()
                    .to_string(),
            ],
        );
        Self {
            case,
            subject_family: witness_report.subject_family(),
            anchor: witness_report.anchor().clone(),
            kind: witness_report.kind(),
            requested_witness: witness_report.requested_witness().clone(),
            status: witness_report.status(),
            resolution_class: witness_report.resolution_class(),
            failure_kind: witness_report.failure_kind(),
            runtime_surface_status,
            row_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionMotionResolutionPolicyCase {
        self.case
    }

    pub fn subject_family(&self) -> PrimitiveConstructionFamily {
        self.subject_family
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn kind(&self) -> PrimitiveConstructionMotionWitnessResolutionKind {
        self.kind
    }

    pub fn requested_witness(&self) -> &PrimitiveConstructionRequestedMotionWitness {
        &self.requested_witness
    }

    pub fn status(&self) -> PrimitiveConstructionMotionWitnessResolutionStatus {
        self.status
    }

    pub fn resolution_class(&self) -> Option<SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn failure_kind(&self) -> Option<PrimitiveConstructionMotionWitnessResolutionFailureKind> {
        self.failure_kind
    }

    pub fn runtime_surface_status(&self) -> PrimitiveConstructionMotionRuntimeSurfaceStatus {
        self.runtime_surface_status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionResolutionPolicyReport {
    rows: Vec<PrimitiveConstructionMotionResolutionPolicyRow>,
    report_digest: String,
}

impl PrimitiveConstructionMotionResolutionPolicyReport {
    fn new(rows: Vec<PrimitiveConstructionMotionResolutionPolicyRow>) -> Self {
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionMotionResolutionPolicyRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionMotionResolutionPolicyCase,
    ) -> Option<&PrimitiveConstructionMotionResolutionPolicyRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionMotionResolutionPolicyReportError {
    Bundle(PrimitiveConstructionMotionReportBundleError),
}

impl std::fmt::Display for PrimitiveConstructionMotionResolutionPolicyReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bundle(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionMotionResolutionPolicyReportError {}

pub fn prepare_primitive_construction_motion_resolution_policy_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionMotionResolutionPolicyReport,
    PrimitiveConstructionMotionResolutionPolicyReportError,
> {
    let rows = vec![
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::DirectMove,
            prepare_primitive_construction_move_motion_report_bundle(
                workspace,
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 6,
                }))
                .to([10.0, 0.0, 3.0]),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::FrameReorient,
            prepare_primitive_construction_reorient_motion_report_bundle(
                workspace,
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 1.0,
                        height: 2.0,
                    },
                ))
                .parallel_to(SpatialFrameRef::workplane(
                    "policy-workplane",
                    [0.0, 0.0, 5.0],
                    [0.0, 0.0, 1.0],
                )),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::CarrierReorient,
            prepare_primitive_construction_reorient_motion_report_bundle_with_catalog(
                workspace,
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 1.0,
                        height: 2.0,
                    },
                ))
                .toward_witness(SpatialDirectionWitnessRef::curve_tangent(
                    "policy-curve",
                    0.25,
                )),
                &SpatialFixtureWitnessCatalog::new().with_parameter_space_direction(
                    SpatialCarrierKind::Curve,
                    "policy-curve",
                    ParameterSpacePoint::try_new([0.25, 0.0]).unwrap(),
                    SpatialCarrierDirectionRole::Tangent,
                    Ok(SpatialCatalogResolvedDirectionWitness::new(
                        [0.0, 1.0, 0.0],
                        SpatialCatalogWitnessResolutionClass::CarrierDerived,
                    )),
                ),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::FallbackPointsToward,
            prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog(
                workspace,
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 3,
                        radius: 1.0,
                        height: 1.0,
                    },
                ))
                .so(SpatialAnchorRef::shape_origin())
                .points_toward_witness(SpatialPointWitnessRef::feature_origin("policy-feature")),
                &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
                    "policy-feature",
                    SpatialCarrierPointRole::Origin,
                    Ok(SpatialCatalogResolvedPointWitness::new(
                        [4.0, 5.0, 6.0],
                        SpatialCatalogWitnessResolutionClass::FallbackDerived,
                    )),
                ),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::AmbiguousMove,
            prepare_primitive_construction_move_motion_report_bundle(
                workspace,
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 4,
                }))
                .to_witness(SpatialPointWitnessRef::ambiguous_curve_point(
                    "policy-curve",
                )),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::UndefinedReorient,
            prepare_primitive_construction_reorient_motion_report_bundle_with_catalog(
                workspace,
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 1.0,
                        height: 2.0,
                    },
                ))
                .toward_witness(SpatialDirectionWitnessRef::feature_axis(
                    "policy-undefined-feature",
                )),
                &SpatialFixtureWitnessCatalog::new().with_feature_owned_direction(
                    "policy-undefined-feature",
                    SpatialCarrierDirectionRole::Axis,
                    Err(SpatialWitnessFailureClass::Undefined),
                ),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::UnsupportedReorient,
            prepare_primitive_construction_reorient_motion_report_bundle(
                workspace,
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 1.0,
                        height: 2.0,
                    },
                ))
                .toward_witness(SpatialDirectionWitnessRef::surface_normal(
                    "policy-surface",
                    0.5,
                    0.5,
                )),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::ExhaustedRotate,
            prepare_primitive_construction_rotate_motion_report_bundle_with_catalog(
                workspace,
                RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 4,
                }))
                .around_witness(SpatialDirectionWitnessRef::feature_axis(
                    "policy-exhausted-feature",
                ))
                .by_radians(0.5),
                &SpatialFixtureWitnessCatalog::new().with_feature_owned_direction(
                    "policy-exhausted-feature",
                    SpatialCarrierDirectionRole::Axis,
                    Err(SpatialWitnessFailureClass::Exhausted),
                ),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
        PrimitiveConstructionMotionResolutionPolicyRow::new(
            PrimitiveConstructionMotionResolutionPolicyCase::CoincidentPointsToward,
            prepare_primitive_construction_points_toward_motion_report_bundle(
                workspace,
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 3,
                        radius: 1.0,
                        height: 1.0,
                    },
                ))
                .so(SpatialAnchorRef::shape_origin())
                .points_toward([0.0, 0.0, 0.0]),
            )
            .map_err(PrimitiveConstructionMotionResolutionPolicyReportError::Bundle)?,
        ),
    ];
    Ok(PrimitiveConstructionMotionResolutionPolicyReport::new(rows))
}
