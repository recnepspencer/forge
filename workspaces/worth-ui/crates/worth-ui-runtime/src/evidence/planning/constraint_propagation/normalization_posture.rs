use crate::declaration::stable_text_digest;

use crate::evidence::{
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintNormalizationPosture {
    Deferred,
    Explicit {
        unit_posture: UiMeasurementUnitPosture,
        coordinate_space: UiMeasurementCoordinateSpace,
        rounding_posture: UiMeasurementRoundingPosture,
    },
}

impl UiConstraintNormalizationPosture {
    pub const fn deferred() -> Self {
        Self::Deferred
    }

    pub const fn explicit(
        unit_posture: UiMeasurementUnitPosture,
        coordinate_space: UiMeasurementCoordinateSpace,
        rounding_posture: UiMeasurementRoundingPosture,
    ) -> Self {
        Self::Explicit {
            unit_posture,
            coordinate_space,
            rounding_posture,
        }
    }

    pub const fn unit_posture(self) -> Option<UiMeasurementUnitPosture> {
        match self {
            Self::Deferred => None,
            Self::Explicit { unit_posture, .. } => Some(unit_posture),
        }
    }

    pub const fn coordinate_space(self) -> Option<UiMeasurementCoordinateSpace> {
        match self {
            Self::Deferred => None,
            Self::Explicit {
                coordinate_space, ..
            } => Some(coordinate_space),
        }
    }

    pub const fn rounding_posture(self) -> Option<UiMeasurementRoundingPosture> {
        match self {
            Self::Deferred => None,
            Self::Explicit {
                rounding_posture, ..
            } => Some(rounding_posture),
        }
    }

    pub(crate) fn identity_digest(self) -> u64 {
        match self {
            Self::Deferred => stable_text_digest("worth-ui.constraint-normalization.deferred"),
            Self::Explicit {
                unit_posture,
                coordinate_space,
                rounding_posture,
            } => {
                stable_text_digest("worth-ui.constraint-normalization.explicit")
                    ^ unit_posture_digest(unit_posture).rotate_left(7)
                    ^ coordinate_space_digest(coordinate_space).rotate_left(13)
                    ^ rounding_posture_digest(rounding_posture).rotate_left(19)
            }
        }
    }
}

fn unit_posture_digest(posture: UiMeasurementUnitPosture) -> u64 {
    stable_text_digest(match posture {
        UiMeasurementUnitPosture::LogicalPx => "worth-ui.constraint-unit.logical-px",
        UiMeasurementUnitPosture::PhysicalPx => "worth-ui.constraint-unit.physical-px",
        UiMeasurementUnitPosture::UnitlessScale => "worth-ui.constraint-unit.unitless-scale",
    })
}

fn coordinate_space_digest(space: UiMeasurementCoordinateSpace) -> u64 {
    stable_text_digest(match space {
        UiMeasurementCoordinateSpace::Viewport => "worth-ui.constraint-coordinate.viewport",
        UiMeasurementCoordinateSpace::Window => "worth-ui.constraint-coordinate.window",
        UiMeasurementCoordinateSpace::GraphNodeLocal => {
            "worth-ui.constraint-coordinate.graph-node-local"
        }
        UiMeasurementCoordinateSpace::HostSurface => "worth-ui.constraint-coordinate.host-surface",
        UiMeasurementCoordinateSpace::PortalLayer => "worth-ui.constraint-coordinate.portal-layer",
    })
}

fn rounding_posture_digest(posture: UiMeasurementRoundingPosture) -> u64 {
    stable_text_digest(match posture {
        UiMeasurementRoundingPosture::ExactFloat => "worth-ui.constraint-rounding.exact-float",
        UiMeasurementRoundingPosture::HostRounded => "worth-ui.constraint-rounding.host-rounded",
        UiMeasurementRoundingPosture::RuntimeRounded => {
            "worth-ui.constraint-rounding.runtime-rounded"
        }
        UiMeasurementRoundingPosture::DeferredToAllocation => {
            "worth-ui.constraint-rounding.deferred-to-allocation"
        }
    })
}
