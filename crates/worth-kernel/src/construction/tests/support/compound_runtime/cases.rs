use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};
use crate::construction::tests::support::compound_lowering::{
    ConstructionMovePlan, ConstructionOffsetPlan, ConstructionReorientPlan,
    PrimitiveConstructionMotionLoweringError,
};
use worth_spatial::facade::refs::SpatialFrameRef;

use super::schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};

#[derive(Clone)]
pub(crate) struct PrimitiveConstructionCompoundScenario {
    pub(super) scenario_id: &'static str,
    pub(super) workload_family: PrimitiveConstructionCompoundWorkloadFamily,
    pub(super) topology_class: PrimitiveConstructionCompoundTopologyClass,
    pub(super) row_class: PrimitiveConstructionCompoundRowClass,
    base_intent: PrimitiveConstructionIntent,
    motion: Option<PrimitiveConstructionCompoundMotionPlan>,
    grazing: Option<PrimitiveConstructionCompoundGrazingPlan>,
}

#[derive(Clone)]
pub(crate) enum PrimitiveConstructionCompoundMotionPlan {
    Move { destination: [f64; 3] },
    Offset { offset: [f64; 3] },
    Reorient { facing: [f64; 3] },
}

impl PrimitiveConstructionCompoundMotionPlan {
    pub(crate) fn kind(&self) -> PrimitiveConstructionCompoundMotionKind {
        match self {
            Self::Move { .. } => PrimitiveConstructionCompoundMotionKind::Move,
            Self::Offset { .. } => PrimitiveConstructionCompoundMotionKind::Offset,
            Self::Reorient { .. } => PrimitiveConstructionCompoundMotionKind::Reorient,
        }
    }

    pub(crate) fn apply(
        &self,
        intent: PrimitiveConstructionIntent,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        match self {
            Self::Move { destination } => ConstructionMovePlan::shape(intent)
                .to(*destination)
                .finish(),
            Self::Offset { offset } => ConstructionOffsetPlan::shape(intent).by(*offset).finish(),
            Self::Reorient { facing } => ConstructionReorientPlan::shape(intent)
                .toward(*facing)
                .finish(),
        }
    }
}

#[derive(Clone)]
pub(crate) enum PrimitiveConstructionCompoundGrazingPlan {
    NearFrameNormal {
        frame: SpatialFrameRef,
        max_angle_radians: f64,
    },
    NearReferenceAnchor {
        reference_point: [f64; 3],
        max_distance: f64,
    },
}

impl PrimitiveConstructionCompoundGrazingPlan {
    pub(crate) fn kind(&self) -> PrimitiveConstructionCompoundGrazingKind {
        match self {
            Self::NearFrameNormal { .. } => {
                PrimitiveConstructionCompoundGrazingKind::NearFrameNormalAlignment
            }
            Self::NearReferenceAnchor { .. } => {
                PrimitiveConstructionCompoundGrazingKind::NearReferenceAnchorDistance
            }
        }
    }
}

impl PrimitiveConstructionCompoundScenario {
    pub(crate) fn scenario_id(&self) -> &'static str {
        self.scenario_id
    }

    pub(crate) fn workload_family(&self) -> PrimitiveConstructionCompoundWorkloadFamily {
        self.workload_family
    }

    pub(crate) fn row_class(&self) -> PrimitiveConstructionCompoundRowClass {
        self.row_class
    }

    pub(crate) fn resolved_intent(
        &self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let base = self.base_intent.clone();
        match &self.motion {
            Some(motion) => motion.apply(base),
            None => Ok(base),
        }
    }

    pub(crate) fn motion(&self) -> Option<&PrimitiveConstructionCompoundMotionPlan> {
        self.motion.as_ref()
    }

    pub(crate) fn motion_kind(&self) -> Option<PrimitiveConstructionCompoundMotionKind> {
        self.motion
            .as_ref()
            .map(PrimitiveConstructionCompoundMotionPlan::kind)
    }

    pub(crate) fn grazing(&self) -> Option<&PrimitiveConstructionCompoundGrazingPlan> {
        self.grazing.as_ref()
    }

    pub(crate) fn grazing_kind(&self) -> Option<PrimitiveConstructionCompoundGrazingKind> {
        self.grazing
            .as_ref()
            .map(PrimitiveConstructionCompoundGrazingPlan::kind)
    }
}

pub(crate) fn compound_scenarios() -> Vec<PrimitiveConstructionCompoundScenario> {
    let shell_frame = SpatialFrameRef::workplane(
        "sheet-grazing-plane",
        [2f64.powi(180), 0.0, -2f64.powi(180)],
        [0.0, 0.0, 1.0],
    );
    vec![
        scenario(
            "orthotope_direct_stable",
            PrimitiveConstructionCompoundWorkloadFamily::Orthotope,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::DirectStable,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0e-120, 2.0e-120, 3.0e-120],
            }),
            Some(PrimitiveConstructionCompoundMotionPlan::Move {
                destination: [2f64.powi(120), -2f64.powi(120), 2f64.powi(120)],
            }),
            None,
        ),
        scenario(
            "orthotope_boundary_neighbor_rejected",
            PrimitiveConstructionCompoundWorkloadFamily::Orthotope,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::BoundaryDriftGuardCase,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 0.0, 2.0],
            }),
            None,
            None,
        ),
        scenario(
            "regular_prism_direct_stable",
            PrimitiveConstructionCompoundWorkloadFamily::RegularPrism,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::DirectStable,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 3,
                radius: 1.0e-150,
                height: 1.0e-150,
            }),
            Some(PrimitiveConstructionCompoundMotionPlan::Move {
                destination: [2f64.powi(200), 2f64.powi(200), -2f64.powi(200)],
            }),
            None,
        ),
        scenario(
            "regular_prism_boundary_neighbor_rejected",
            PrimitiveConstructionCompoundWorkloadFamily::RegularPrism,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::BoundaryDriftGuardCase,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 3,
                radius: 0.0,
                height: 2.0,
            }),
            None,
            None,
        ),
        scenario(
            "pyramid_direct_stable_comparison",
            PrimitiveConstructionCompoundWorkloadFamily::RegularPyramid,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::DirectStable,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            }),
            None,
            None,
        ),
        scenario(
            "pyramid_threshold_admitted_exact_support",
            PrimitiveConstructionCompoundWorkloadFamily::RegularPyramid,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::EscalatedStableExactSupport,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0e-200,
                height: 1.0e-200,
            }),
            None,
            None,
        ),
        scenario(
            "pyramid_threshold_rejected_neighbor",
            PrimitiveConstructionCompoundWorkloadFamily::RegularPyramid,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::BoundaryDriftGuardCase,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 0.0,
            }),
            None,
            None,
        ),
        scenario(
            "pyramid_semantic_exhaustion",
            PrimitiveConstructionCompoundWorkloadFamily::RegularPyramid,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 0.0,
                height: 1.0,
            }),
            None,
            None,
        ),
        scenario(
            "simplex_world_collapsed_admitted_local_or_exact",
            PrimitiveConstructionCompoundWorkloadFamily::SimplexSolid,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::EscalatedStableExactSupport,
            PrimitiveConstructionIntent::simplex_solid(
                SimplexSolidSpec::new(1.0e-200).with_auxiliary_altitude_component(1.0e-220),
            )
            .at([2f64.powi(548), -2f64.powi(548), 2f64.powi(548)]),
            None,
            None,
        ),
        scenario(
            "simplex_world_collapsed_threshold_rejected",
            PrimitiveConstructionCompoundWorkloadFamily::SimplexSolid,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::BoundaryDriftGuardCase,
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(0.0)),
            Some(PrimitiveConstructionCompoundMotionPlan::Move {
                destination: [2f64.powi(548), -2f64.powi(548), 2f64.powi(548)],
            }),
            None,
        ),
        scenario(
            "simplex_world_collapsed_explicit_exhaustion",
            PrimitiveConstructionCompoundWorkloadFamily::SimplexSolid,
            PrimitiveConstructionCompoundTopologyClass::ClosedSolid,
            PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion,
            PrimitiveConstructionIntent::simplex_solid(
                SimplexSolidSpec::new(1.0e-240).with_auxiliary_altitude_component(1.0e-280),
            )
            .at([2f64.powi(548), -2f64.powi(548), 2f64.powi(548)]),
            None,
            None,
        ),
        scenario(
            "sheet_patch_reorient_grazing_workplane",
            PrimitiveConstructionCompoundWorkloadFamily::SheetPatch,
            PrimitiveConstructionCompoundTopologyClass::OpenShell,
            PrimitiveConstructionCompoundRowClass::MotionHostileReorientation,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 5,
                hole_loop_edge_counts: vec![3],
            }),
            Some(PrimitiveConstructionCompoundMotionPlan::Reorient {
                facing: [0.0, 1.0e-12, 1.0],
            }),
            Some(PrimitiveConstructionCompoundGrazingPlan::NearFrameNormal {
                frame: shell_frame,
                max_angle_radians: 1.0e-12,
            }),
        ),
        scenario(
            "wire_open_endpoint_graze",
            PrimitiveConstructionCompoundWorkloadFamily::WireOpen,
            PrimitiveConstructionCompoundTopologyClass::OpenWire,
            PrimitiveConstructionCompoundRowClass::PreBooleanGrazingCase,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 6 }),
            Some(PrimitiveConstructionCompoundMotionPlan::Offset {
                offset: [1.0e-14, 0.0, 0.0],
            }),
            Some(
                PrimitiveConstructionCompoundGrazingPlan::NearReferenceAnchor {
                    reference_point: [0.0, 0.0, 0.0],
                    max_distance: 1.0e-14,
                },
            ),
        ),
        scenario(
            "wire_open_motion_relocation",
            PrimitiveConstructionCompoundWorkloadFamily::WireOpen,
            PrimitiveConstructionCompoundTopologyClass::OpenWire,
            PrimitiveConstructionCompoundRowClass::MotionStableRelocation,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }),
            Some(PrimitiveConstructionCompoundMotionPlan::Move {
                destination: [12.0, -6.0, 4.0],
            }),
            None,
        ),
    ]
}

fn scenario(
    scenario_id: &'static str,
    workload_family: PrimitiveConstructionCompoundWorkloadFamily,
    topology_class: PrimitiveConstructionCompoundTopologyClass,
    row_class: PrimitiveConstructionCompoundRowClass,
    intent: PrimitiveConstructionIntent,
    motion: Option<PrimitiveConstructionCompoundMotionPlan>,
    grazing: Option<PrimitiveConstructionCompoundGrazingPlan>,
) -> PrimitiveConstructionCompoundScenario {
    PrimitiveConstructionCompoundScenario {
        scenario_id,
        workload_family,
        topology_class,
        row_class,
        base_intent: intent,
        motion,
        grazing,
    }
}
