use crate::declaration::stable_text_digest;

use super::{
    UiConstraintAxisScope, UiConstraintBoundedMinMaxRequirement, UiConstraintNormalizationPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintAvailableSpacePosture {
    DeclaredExtentUnknown,
    AdmittedZeroExtent,
    AdmittedPositiveExtent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiConstraintParentAvailableSpace {
    axis_scope: UiConstraintAxisScope,
    available_space_posture: UiConstraintAvailableSpacePosture,
    bounded_min_max_requirement: UiConstraintBoundedMinMaxRequirement,
    normalization_posture: UiConstraintNormalizationPosture,
}

impl UiConstraintParentAvailableSpace {
    pub const fn new(
        axis_scope: UiConstraintAxisScope,
        available_space_posture: UiConstraintAvailableSpacePosture,
        bounded_min_max_requirement: UiConstraintBoundedMinMaxRequirement,
        normalization_posture: UiConstraintNormalizationPosture,
    ) -> Self {
        Self {
            axis_scope,
            available_space_posture,
            bounded_min_max_requirement,
            normalization_posture,
        }
    }

    pub const fn axis_scope(self) -> UiConstraintAxisScope {
        self.axis_scope
    }

    pub const fn available_space_posture(self) -> UiConstraintAvailableSpacePosture {
        self.available_space_posture
    }

    pub const fn bounded_min_max_requirement(self) -> UiConstraintBoundedMinMaxRequirement {
        self.bounded_min_max_requirement
    }

    pub const fn normalization_posture(self) -> UiConstraintNormalizationPosture {
        self.normalization_posture
    }

    pub(crate) fn identity_digest(self) -> u64 {
        stable_text_digest("worth-ui.constraint-parent-available-space")
            ^ axis_scope_digest(self.axis_scope).rotate_left(7)
            ^ available_space_posture_digest(self.available_space_posture).rotate_left(13)
            ^ bounded_requirement_digest(self.bounded_min_max_requirement).rotate_left(19)
            ^ self.normalization_posture.identity_digest().rotate_left(23)
    }
}

fn axis_scope_digest(axis_scope: UiConstraintAxisScope) -> u64 {
    match axis_scope {
        UiConstraintAxisScope::Primary => stable_text_digest("worth-ui.constraint-axis.primary"),
        UiConstraintAxisScope::Cross => stable_text_digest("worth-ui.constraint-axis.cross"),
        UiConstraintAxisScope::Both => stable_text_digest("worth-ui.constraint-axis.both"),
    }
}

fn bounded_requirement_digest(requirement: UiConstraintBoundedMinMaxRequirement) -> u64 {
    match requirement {
        UiConstraintBoundedMinMaxRequirement::None => {
            stable_text_digest("worth-ui.constraint-bounds.none")
        }
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis => {
            stable_text_digest("worth-ui.constraint-bounds.primary-axis")
        }
        UiConstraintBoundedMinMaxRequirement::BothAxes => {
            stable_text_digest("worth-ui.constraint-bounds.both-axes")
        }
    }
}

fn available_space_posture_digest(posture: UiConstraintAvailableSpacePosture) -> u64 {
    stable_text_digest(match posture {
        UiConstraintAvailableSpacePosture::DeclaredExtentUnknown => {
            "worth-ui.constraint-parent-available-space.unknown"
        }
        UiConstraintAvailableSpacePosture::AdmittedZeroExtent => {
            "worth-ui.constraint-parent-available-space.zero"
        }
        UiConstraintAvailableSpacePosture::AdmittedPositiveExtent => {
            "worth-ui.constraint-parent-available-space.positive"
        }
    })
}
