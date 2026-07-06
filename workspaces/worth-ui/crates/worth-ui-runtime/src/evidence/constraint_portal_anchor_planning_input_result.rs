use crate::declaration::stable_text_digest;

use super::{
    UiConstraintPropagationEdgeFamily, UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPortalAnchorPlanningInputSolveOrder {
    BeforeDerivedConstraintFamilies,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPortalAnchorPlanningInputPosture {
    AdmittedPlanningTimeOnly,
    MissingRequiredEvidence,
    IncompatibleMeasurementPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConstraintPortalAnchorPlanningInputResult {
    neighborhood_identity_digest: u64,
    solve_order: UiPortalAnchorPlanningInputSolveOrder,
    posture: UiPortalAnchorPlanningInputPosture,
    source_evidence_identity_digest: Option<u64>,
    source_generation_digest: Option<u64>,
    unit_posture: Option<UiMeasurementUnitPosture>,
    coordinate_space: Option<UiMeasurementCoordinateSpace>,
    rounding_posture: Option<UiMeasurementRoundingPosture>,
    planning_time_only: bool,
    identity_digest: u64,
}

impl UiConstraintPortalAnchorPlanningInputResult {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        neighborhood_identity_digest: u64,
        solve_order: UiPortalAnchorPlanningInputSolveOrder,
        posture: UiPortalAnchorPlanningInputPosture,
        source_evidence_identity_digest: Option<u64>,
        source_generation_digest: Option<u64>,
        unit_posture: Option<UiMeasurementUnitPosture>,
        coordinate_space: Option<UiMeasurementCoordinateSpace>,
        rounding_posture: Option<UiMeasurementRoundingPosture>,
        planning_time_only: bool,
    ) -> Self {
        let identity_digest =
            stable_text_digest("worth-ui.constraint-portal-anchor-planning-input")
                ^ neighborhood_identity_digest.rotate_left(7)
                ^ solve_order_digest(solve_order).rotate_left(13)
                ^ posture_digest(posture).rotate_left(19)
                ^ optional_digest(
                    source_evidence_identity_digest,
                    "worth-ui.constraint-portal-anchor.no-source",
                )
                .rotate_left(23)
                ^ optional_digest(
                    source_generation_digest,
                    "worth-ui.constraint-portal-anchor.no-generation",
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
        UiConstraintPropagationEdgeFamily::PortalAnchorInput
    }
    pub fn neighborhood_identity_digest(&self) -> u64 { self.neighborhood_identity_digest }
    pub fn solve_order(&self) -> UiPortalAnchorPlanningInputSolveOrder { self.solve_order }
    pub fn posture(&self) -> UiPortalAnchorPlanningInputPosture { self.posture }
    pub fn source_evidence_identity_digest(&self) -> Option<u64> { self.source_evidence_identity_digest }
    pub fn source_generation_digest(&self) -> Option<u64> { self.source_generation_digest }
    pub fn unit_posture(&self) -> Option<UiMeasurementUnitPosture> { self.unit_posture }
    pub fn coordinate_space(&self) -> Option<UiMeasurementCoordinateSpace> { self.coordinate_space }
    pub fn rounding_posture(&self) -> Option<UiMeasurementRoundingPosture> { self.rounding_posture }
    pub fn is_planning_time_only(&self) -> bool { self.planning_time_only }
    pub fn identity_digest(&self) -> u64 { self.identity_digest }
}

fn solve_order_digest(order: UiPortalAnchorPlanningInputSolveOrder) -> u64 {
    match order {
        UiPortalAnchorPlanningInputSolveOrder::BeforeDerivedConstraintFamilies => {
            stable_text_digest("worth-ui.constraint-portal-anchor.solve-order.before-derived")
        }
    }
}

fn posture_digest(posture: UiPortalAnchorPlanningInputPosture) -> u64 {
    stable_text_digest(match posture {
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly => {
            "worth-ui.constraint-portal-anchor.posture.admitted-planning-time-only"
        }
        UiPortalAnchorPlanningInputPosture::MissingRequiredEvidence => {
            "worth-ui.constraint-portal-anchor.posture.missing-required-evidence"
        }
        UiPortalAnchorPlanningInputPosture::IncompatibleMeasurementPosture => {
            "worth-ui.constraint-portal-anchor.posture.incompatible-measurement-posture"
        }
    })
}

fn unit_posture_digest(posture: Option<UiMeasurementUnitPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementUnitPosture::LogicalPx) => {
            "worth-ui.constraint-portal-anchor.unit.logical-px"
        }
        Some(UiMeasurementUnitPosture::PhysicalPx) => {
            "worth-ui.constraint-portal-anchor.unit.physical-px"
        }
        Some(UiMeasurementUnitPosture::UnitlessScale) => {
            "worth-ui.constraint-portal-anchor.unit.unitless-scale"
        }
        None => "worth-ui.constraint-portal-anchor.unit.none",
    })
}

fn coordinate_space_digest(space: Option<UiMeasurementCoordinateSpace>) -> u64 {
    stable_text_digest(match space {
        Some(UiMeasurementCoordinateSpace::Viewport) => {
            "worth-ui.constraint-portal-anchor.coordinate.viewport"
        }
        Some(UiMeasurementCoordinateSpace::Window) => {
            "worth-ui.constraint-portal-anchor.coordinate.window"
        }
        Some(UiMeasurementCoordinateSpace::GraphNodeLocal) => {
            "worth-ui.constraint-portal-anchor.coordinate.graph-node-local"
        }
        Some(UiMeasurementCoordinateSpace::HostSurface) => {
            "worth-ui.constraint-portal-anchor.coordinate.host-surface"
        }
        Some(UiMeasurementCoordinateSpace::PortalLayer) => {
            "worth-ui.constraint-portal-anchor.coordinate.portal-layer"
        }
        None => "worth-ui.constraint-portal-anchor.coordinate.none",
    })
}

fn rounding_posture_digest(posture: Option<UiMeasurementRoundingPosture>) -> u64 {
    stable_text_digest(match posture {
        Some(UiMeasurementRoundingPosture::ExactFloat) => {
            "worth-ui.constraint-portal-anchor.rounding.exact-float"
        }
        Some(UiMeasurementRoundingPosture::HostRounded) => {
            "worth-ui.constraint-portal-anchor.rounding.host-rounded"
        }
        Some(UiMeasurementRoundingPosture::RuntimeRounded) => {
            "worth-ui.constraint-portal-anchor.rounding.runtime-rounded"
        }
        Some(UiMeasurementRoundingPosture::DeferredToAllocation) => {
            "worth-ui.constraint-portal-anchor.rounding.deferred"
        }
        None => "worth-ui.constraint-portal-anchor.rounding.none",
    })
}

fn optional_digest(value: Option<u64>, none_text: &str) -> u64 {
    value.unwrap_or_else(|| stable_text_digest(none_text))
}

fn bool_digest(value: bool) -> u64 {
    stable_text_digest(if value {
        "worth-ui.constraint-portal-anchor.planning-time-only"
    } else {
        "worth-ui.constraint-portal-anchor.not-planning-time-only"
    })
}
