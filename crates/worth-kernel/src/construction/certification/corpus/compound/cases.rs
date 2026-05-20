use crate::construction::{
    OrthotopeSpec, PrimitiveConstructionIntent, RegularPrismSpec, RegularPyramidSpec,
    ShellWithHoleSpec, WireBodySpec,
};
use crate::facade::{
    MoveSpatialIntent, OffsetSpatialIntent, PrimitiveConstructionSpatialIntentError,
    ReorientSpatialIntent,
};
use worth_spatial::facade::SpatialFrameRef;

use super::schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};

#[derive(Clone)]
pub(super) struct PrimitiveConstructionCompoundScenario {
    pub(super) scenario_id: &'static str,
    pub(super) workload_family: PrimitiveConstructionCompoundWorkloadFamily,
    pub(super) topology_class: PrimitiveConstructionCompoundTopologyClass,
    pub(super) row_class: PrimitiveConstructionCompoundRowClass,
    base_intent: PrimitiveConstructionIntent,
    motion: Option<PrimitiveConstructionCompoundMotionPlan>,
    grazing: Option<PrimitiveConstructionCompoundGrazingPlan>,
}

#[derive(Clone)]
pub(super) enum PrimitiveConstructionCompoundMotionPlan {
    Move { destination: [f64; 3] },
    Offset { offset: [f64; 3] },
    Reorient { facing: [f64; 3] },
}

impl PrimitiveConstructionCompoundMotionPlan {
    pub(super) fn kind(&self) -> PrimitiveConstructionCompoundMotionKind {
        match self {
            Self::Move { .. } => PrimitiveConstructionCompoundMotionKind::Move,
            Self::Offset { .. } => PrimitiveConstructionCompoundMotionKind::Offset,
            Self::Reorient { .. } => PrimitiveConstructionCompoundMotionKind::Reorient,
        }
    }

    pub(super) fn apply(
        &self,
        intent: PrimitiveConstructionIntent,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        match self {
            Self::Move { destination } => {
                MoveSpatialIntent::shape(intent).to(*destination).finish()
            }
            Self::Offset { offset } => OffsetSpatialIntent::shape(intent).by(*offset).finish(),
            Self::Reorient { facing } => ReorientSpatialIntent::shape(intent)
                .toward(*facing)
                .finish(),
        }
    }
}

#[derive(Clone)]
pub(super) enum PrimitiveConstructionCompoundGrazingPlan {
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
    pub(super) fn kind(&self) -> PrimitiveConstructionCompoundGrazingKind {
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
    pub(super) fn resolved_intent(
        &self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let base = self.base_intent.clone();
        match &self.motion {
            Some(motion) => motion.apply(base),
            None => Ok(base),
        }
    }

    pub(super) fn motion(&self) -> Option<&PrimitiveConstructionCompoundMotionPlan> {
        self.motion.as_ref()
    }

    pub(super) fn grazing(&self) -> Option<&PrimitiveConstructionCompoundGrazingPlan> {
        self.grazing.as_ref()
    }
}

pub(super) fn compound_scenarios() -> Vec<PrimitiveConstructionCompoundScenario> {
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
            })
            .created()
            .at([2f64.powi(120), -2f64.powi(120), 2f64.powi(120)])
            .finish(),
            None,
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
            })
            .created()
            .at([2f64.powi(200), 2f64.powi(200), -2f64.powi(200)])
            .finish(),
            None,
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
            "regular_pyramid_threshold_exact_support",
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
            "regular_pyramid_threshold_rejected_neighbor",
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
            "sheet_patch_reorient_grazing_workplane",
            PrimitiveConstructionCompoundWorkloadFamily::SheetPatch,
            PrimitiveConstructionCompoundTopologyClass::OpenShell,
            PrimitiveConstructionCompoundRowClass::MotionHostileReorientation,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 5,
                hole_loop_edge_counts: vec![3],
            })
            .created()
            .relative_to(shell_frame.clone())
            .aligned_with(shell_frame.clone())
            .finish(),
            Some(PrimitiveConstructionCompoundMotionPlan::Reorient {
                facing: [0.0, 1.0e-12, 1.0],
            }),
            Some(PrimitiveConstructionCompoundGrazingPlan::NearFrameNormal {
                frame: shell_frame,
                max_angle_radians: 1.0e-12,
            }),
        ),
        scenario(
            "wire_open_origin_graze",
            PrimitiveConstructionCompoundWorkloadFamily::WireOpen,
            PrimitiveConstructionCompoundTopologyClass::OpenWire,
            PrimitiveConstructionCompoundRowClass::PreBooleanGrazingCase,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 6 })
                .created()
                .at([0.0, 0.0, 0.0])
                .finish(),
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
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 })
                .created()
                .at([1.0, 2.0, 3.0])
                .finish(),
            Some(PrimitiveConstructionCompoundMotionPlan::Move {
                destination: [12.0, -6.0, 4.0],
            }),
            None,
        ),
    ]
}

pub(super) fn canonical_order(
    scenarios: &[PrimitiveConstructionCompoundScenario],
) -> Vec<PrimitiveConstructionCompoundScenario> {
    scenarios.to_vec()
}

pub(super) fn reversed_order(
    scenarios: &[PrimitiveConstructionCompoundScenario],
) -> Vec<PrimitiveConstructionCompoundScenario> {
    let mut rows = scenarios.to_vec();
    rows.reverse();
    rows
}

pub(super) fn topology_clustered_order(
    scenarios: &[PrimitiveConstructionCompoundScenario],
) -> Vec<PrimitiveConstructionCompoundScenario> {
    let mut rows = scenarios.to_vec();
    rows.sort_by_key(|scenario| {
        (
            scenario.topology_class as u8,
            scenario.workload_family as u8,
            scenario.scenario_id,
        )
    });
    rows
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
