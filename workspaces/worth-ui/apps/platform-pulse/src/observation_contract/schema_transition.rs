use serde::{Deserialize, Serialize};
use worth_ui::facade::rebind::{
    UiProjectionPredecessorValuePolicy, UiProjectionSchemaRequirement,
    UiProjectionSchemaTransition, UiProjectionSchemaTransitionKind,
};

use super::PlatformPulseLifecycleObservationProjectionDenial;

const COMPONENT: &str = "component:platform.pulse.component.projected_status";
const PROJECTION: &str = "platform.pulse.status";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseProjectionSchemaTransitionObservation {
    kind: PlatformPulseProjectionSchemaTransitionKind,
    predecessor_selected_field: PlatformPulseProjectionSchemaField,
    candidate_selected_field: PlatformPulseProjectionSchemaField,
    installed_selected_field: PlatformPulseProjectionSchemaField,
    predecessor_value_preserved: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseProjectionSchemaTransitionKind {
    Stopped,
    Recovered,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseProjectionSchemaField {
    Status,
    Revision,
}

impl PlatformPulseProjectionSchemaTransitionObservation {
    pub(super) fn from_transition(
        transition: &UiProjectionSchemaTransition,
    ) -> Result<Self, PlatformPulseLifecycleObservationProjectionDenial> {
        if transition.component_identity() != COMPONENT
            || transition.declaration_identity() != PROJECTION
            || transition.view_identity().as_str() != PROJECTION
        {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::
                    UnexpectedSchemaTransitionIdentity,
            );
        }
        Ok(Self {
            kind: match transition.kind() {
                UiProjectionSchemaTransitionKind::Stopped => {
                    PlatformPulseProjectionSchemaTransitionKind::Stopped
                }
                UiProjectionSchemaTransitionKind::Recovered => {
                    PlatformPulseProjectionSchemaTransitionKind::Recovered
                }
            },
            predecessor_selected_field: scalar_field(transition.predecessor())?,
            candidate_selected_field: scalar_field(transition.candidate())?,
            installed_selected_field: scalar_field(transition.installed())?,
            predecessor_value_preserved: transition.predecessor_policy()
                == UiProjectionPredecessorValuePolicy::Preserve,
        })
    }

    pub const fn kind(&self) -> PlatformPulseProjectionSchemaTransitionKind {
        self.kind
    }

    pub const fn predecessor_selected_field(&self) -> PlatformPulseProjectionSchemaField {
        self.predecessor_selected_field
    }

    pub const fn candidate_selected_field(&self) -> PlatformPulseProjectionSchemaField {
        self.candidate_selected_field
    }

    pub const fn installed_selected_field(&self) -> PlatformPulseProjectionSchemaField {
        self.installed_selected_field
    }

    pub const fn predecessor_value_preserved(&self) -> bool {
        self.predecessor_value_preserved
    }
}

fn scalar_field(
    requirement: &UiProjectionSchemaRequirement,
) -> Result<PlatformPulseProjectionSchemaField, PlatformPulseLifecycleObservationProjectionDenial> {
    match requirement {
        UiProjectionSchemaRequirement::Scalar(requirement) => match requirement
            .selected_field()
            .declared_name()
        {
            "status" => Ok(PlatformPulseProjectionSchemaField::Status),
            "revision" => Ok(PlatformPulseProjectionSchemaField::Revision),
            _ => Err(
                PlatformPulseLifecycleObservationProjectionDenial::UnsupportedSchemaTransitionField,
            ),
        },
        UiProjectionSchemaRequirement::Collection(_) => {
            Err(PlatformPulseLifecycleObservationProjectionDenial::UnsupportedSchemaTransitionShape)
        }
    }
}
