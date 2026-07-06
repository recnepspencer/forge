use crate::declaration::stable_text_digest;

use super::{
    UiConstraintPropagationEdgeFamily, UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiViewportPlanningInputSolveOrder {
    BeforeDerivedConstraintFamilies,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiViewportPlanningInputPosture {
    AdmittedPlanningTimeOnly,
    MissingRequiredEvidence,
    IncompatibleMeasurementPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConstraintViewportPlanningInputResult {
    neighborhood_identity_digest: u64,
    solve_order: UiViewportPlanningInputSolveOrder,
    posture: UiViewportPlanningInputPosture,
    source_evidence_identity_digest: Option<u64>,
    source_generation_digest: Option<u64>,
    unit_posture: Option<UiMeasurementUnitPosture>,
    coordinate_space: Option<UiMeasurementCoordinateSpace>,
    rounding_posture: Option<UiMeasurementRoundingPosture>,
    planning_time_only: bool,
    identity_digest: u64,
}

impl UiConstraintViewportPlanningInputResult {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        neighborhood_identity_digest: u64,
        solve_order: UiViewportPlanningInputSolveOrder,
        posture: UiViewportPlanningInputPosture,
        source_evidence_identity_digest: Option<u64>,
        source_generation_digest: Option<u64>,
        unit_posture: Option<UiMeasurementUnitPosture>,
        coordinate_space: Option<UiMeasurementCoordinateSpace>,
        rounding_posture: Option<UiMeasurementRoundingPosture>,
        planning_time_only: bool,
    ) -> Self {
        let identity_digest = stable_text_digest("worth-ui.constraint-viewport-planning-input")
            ^ neighborhood_identity_digest.rotate_left(7)
            ^ solve_order_digest(solve_order).rotate_left(13)
            ^ posture_digest(posture).rotate_left(19)
            ^ optional_digest(
                source_evidence_identity_digest,
                "worth-ui.constraint-viewport.no-source",
            )
            .rotate_left(23)
            ^ optional_digest(
                source_generation_digest,
                "worth-ui.constraint-viewport.no-generation",
            )
            .rotate_left(29)
            ^ unit_posture_digest(unit_posture).rotate_left(31)
            ^ coordinate_space_digest(coordinate_space).rotate_left(37)
            ^ rounding_posture_digest(rounding_posture).rotate_left(41)
            ^ bool_digest(planning_time_only).rotate_left(43);
        Self {
            neighborhood_identity_digest,
            solve_order,
            posture,
            source_evidence_identity_digest,
            source_generation_digest,
            unit_posture,
            coordinate_space,
            rounding_posture,
            planning_time_only,
            identity_digest,
        }
    }

    pub fn edge_family(&self) -> UiConstraintPropagationEdgeFamily {
        UiConstraintPropagationEdgeFamily::ViewportInput
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub fn solve_order(&self) -> UiViewportPlanningInputSolveOrder {
        self.solve_order
    }

    pub fn posture(&self) -> UiViewportPlanningInputPosture {
        self.posture
    }

    pub fn source_evidence_identity_digest(&self) -> Option<u64> {
        self.source_evidence_identity_digest
    }

    pub fn source_generation_digest(&self) -> Option<u64> {
        self.source_generation_digest
    }

    pub fn unit_posture(&self) -> Option<UiMeasurementUnitPosture> {
        self.unit_posture
    }

    pub fn coordinate_space(&self) -> Option<UiMeasurementCoordinateSpace> {
        self.coordinate_space
    }

    pub fn rounding_posture(&self) -> Option<UiMeasurementRoundingPosture> {
        self.rounding_posture
    }

    pub fn is_planning_time_only(&self) -> bool {
        self.planning_time_only
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

fn solve_order_digest(order: UiViewportPlanningInputSolveOrder) -> u64 {
    match order {
        UiViewportPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-viewport.solve-order.before-derived")
        }
    }
}

fn posture_digest(posture: UiViewportPlanningInputPosture) -> u64 {
    stable_text_digest(match posture {
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly => {
            "worth-ui.constraint-viewport.posture.admitted-planning-time-only"
        }
        UiViewportPlanningInputPosture::MissingRequiredEvidence => {
            "worth-ui.constraint-viewport.posture.missing-required-evidence"
        }
        UiViewportPlanningInputPosture::IncompatibleMeasurementPosture => {
            "worth-ui.constraint-viewport.posture.incompatible-measurement-posture"
        }
    })
}

fn unit_posture_digest(posture: Option<UiMeasurementUnitPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementUnitPosture::LogicalPx) => "worth-ui.constraint-viewport.unit.logical-px",
        Some(UiMeasurementUnitPosture::PhysicalPx) => {
            "worth-ui.constraint-viewport.unit.physical-px"
        }
        Some(UiMeasurementUnitPosture::UnitlessScale) => {
            "worth-ui.constraint-viewport.unit.unitless-scale"
        }
        None => "worth-ui.constraint-viewport.unit.none",
    })
}

fn coordinate_space_digest(space: Option<UiMeasurementCoordinateSpace>) -> u64 {
    stable_text_digest(match space {
        Some(UiMeasurementCoordinateSpace::Viewport) => {
            "worth-ui.constraint-viewport.coordinate.viewport"
        }
        Some(UiMeasurementCoordinateSpace::Window) => {
            "worth-ui.constraint-viewport.coordinate.window"
        }
        Some(UiMeasurementCoordinateSpace::GraphNodeLocal) => {
            "worth-ui.constraint-viewport.coordinate.graph-node-local"
        }
        Some(UiMeasurementCoordinateSpace::HostSurface) => {
            "worth-ui.constraint-viewport.coordinate.host-surface"
        }
        Some(UiMeasurementCoordinateSpace::PortalLayer) => {
            "worth-ui.constraint-viewport.coordinate.portal-layer"
        }
        None => "worth-ui.constraint-viewport.coordinate.none",
    })
}

fn rounding_posture_digest(posture: Option<UiMeasurementRoundingPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementRoundingPosture::ExactFloat) => {
            "worth-ui.constraint-viewport.rounding.exact-float"
        }
        Some(UiMeasurementRoundingPosture::HostRounded) => {
            "worth-ui.constraint-viewport.rounding.host-rounded"
        }
        Some(UiMeasurementRoundingPosture::RuntimeRounded) => {
            "worth-ui.constraint-viewport.rounding.runtime-rounded"
        }
        Some(UiMeasurementRoundingPosture::DeferredToAllocation) => {
            "worth-ui.constraint-viewport.rounding.deferred"
        }
        None => "worth-ui.constraint-viewport.rounding.none",
    })
}

fn optional_digest(value: Option<u64>, none_text: &str) -> u64 {
    value.unwrap_or_else(|| stable_text_digest(none_text))
}

fn bool_digest(value: bool) -> u64 {
    stable_text_digest(if value {
        "worth-ui.constraint-viewport.planning-time-only"
    } else {
        "worth-ui.constraint-viewport.not-planning-time-only"
    })
}
