use crate::capability::ComponentDescriptor;
use crate::capability::ComponentId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiComponentCompatibility {
    Equivalent,
    CompatiblePreserveState(WorthUiComponentStatePreservation),
    CompatibleDropState(WorthUiComponentStateDropReason),
    Denied(WorthUiComponentShapeDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiComponentStatePreservation {
    component_id: ComponentId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiComponentStateDropReason {
    PropSchemaIncompatible {
        component_id: ComponentId,
        previous_schema_key: String,
        candidate_schema_key: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiComponentShapeDenial {
    MissingComponent(ComponentId),
    MissingPropSchema(ComponentId),
    MissingStateOwnership(ComponentId),
    StateOwnershipChanged {
        component_id: ComponentId,
        previous: &'static str,
        candidate: &'static str,
    },
    ChildPolicyChanged {
        component_id: ComponentId,
        previous: &'static str,
        candidate: &'static str,
    },
    FocusChanged {
        component_id: ComponentId,
        previous: &'static str,
        candidate: &'static str,
    },
    AccessibilityChanged {
        component_id: ComponentId,
        previous: &'static str,
        candidate: &'static str,
    },
    ExecutionLaneChanged {
        component_id: ComponentId,
        previous: &'static str,
        candidate: &'static str,
    },
    IllegalChildPolicy(ComponentId),
    UntypedPropSchema(ComponentId),
}

impl WorthUiComponentStatePreservation {
    pub(crate) fn new(component_id: ComponentId) -> Self {
        Self { component_id }
    }

    pub fn component_id(&self) -> &ComponentId {
        &self.component_id
    }
}

impl WorthUiComponentStateDropReason {
    pub fn component_id(&self) -> &ComponentId {
        match self {
            Self::PropSchemaIncompatible { component_id, .. } => component_id,
        }
    }
}

impl WorthUiComponentShapeDenial {
    pub fn component_id(&self) -> &ComponentId {
        match self {
            Self::MissingComponent(component_id)
            | Self::MissingPropSchema(component_id)
            | Self::MissingStateOwnership(component_id)
            | Self::IllegalChildPolicy(component_id)
            | Self::UntypedPropSchema(component_id) => component_id,
            Self::StateOwnershipChanged { component_id, .. }
            | Self::ChildPolicyChanged { component_id, .. }
            | Self::FocusChanged { component_id, .. }
            | Self::AccessibilityChanged { component_id, .. }
            | Self::ExecutionLaneChanged { component_id, .. } => component_id,
        }
    }

    pub(crate) fn detail(&self) -> String {
        match self {
            Self::MissingComponent(component_id) => {
                format!("unknown component `{}`", component_id.as_str())
            }
            Self::MissingPropSchema(component_id) => format!(
                "component `{}` requires a typed prop schema",
                component_id.as_str()
            ),
            Self::MissingStateOwnership(component_id) => format!(
                "component `{}` requires state ownership classification",
                component_id.as_str()
            ),
            Self::StateOwnershipChanged {
                component_id,
                previous,
                candidate,
            } => format!(
                "component `{}` changed state ownership from `{previous}` to `{candidate}`",
                component_id.as_str()
            ),
            Self::ChildPolicyChanged {
                component_id,
                previous,
                candidate,
            } => format!(
                "component `{}` changed child policy from `{previous}` to `{candidate}`",
                component_id.as_str()
            ),
            Self::FocusChanged {
                component_id,
                previous,
                candidate,
            } => format!(
                "component `{}` changed focus posture from `{previous}` to `{candidate}`",
                component_id.as_str()
            ),
            Self::AccessibilityChanged {
                component_id,
                previous,
                candidate,
            } => format!(
                "component `{}` changed accessibility posture from `{previous}` to `{candidate}`",
                component_id.as_str()
            ),
            Self::ExecutionLaneChanged {
                component_id,
                previous,
                candidate,
            } => format!(
                "component `{}` changed execution lane from `{previous}` to `{candidate}`",
                component_id.as_str()
            ),
            Self::IllegalChildPolicy(component_id) => format!(
                "component `{}` cannot claim shell layout authority",
                component_id.as_str()
            ),
            Self::UntypedPropSchema(component_id) => format!(
                "component `{}` prop schema must remain typed",
                component_id.as_str()
            ),
        }
    }
}

pub(crate) fn classify_component_compatibility(
    previous: &ComponentDescriptor,
    candidate: &ComponentDescriptor,
) -> Result<WorthUiComponentCompatibility, WorthUiComponentShapeDenial> {
    let component_id = candidate.id().clone();
    let Some(previous_schema) = previous.prop_schema() else {
        return Err(WorthUiComponentShapeDenial::MissingPropSchema(component_id));
    };
    let Some(candidate_schema) = candidate.prop_schema() else {
        return Err(WorthUiComponentShapeDenial::MissingPropSchema(component_id));
    };
    let Some(previous_state_ownership) = previous.state_ownership() else {
        return Err(WorthUiComponentShapeDenial::MissingStateOwnership(
            component_id,
        ));
    };
    let Some(candidate_state_ownership) = candidate.state_ownership() else {
        return Err(WorthUiComponentShapeDenial::MissingStateOwnership(
            component_id,
        ));
    };
    if !candidate_schema.is_typed() {
        return Err(WorthUiComponentShapeDenial::UntypedPropSchema(component_id));
    }
    if candidate.child_policy().is_illegal() {
        return Err(WorthUiComponentShapeDenial::IllegalChildPolicy(
            component_id,
        ));
    }
    if previous_state_ownership != candidate_state_ownership {
        return Err(WorthUiComponentShapeDenial::StateOwnershipChanged {
            component_id,
            previous: previous_state_ownership.as_str(),
            candidate: candidate_state_ownership.as_str(),
        });
    }
    if previous.child_policy() != candidate.child_policy() {
        return Err(WorthUiComponentShapeDenial::ChildPolicyChanged {
            component_id,
            previous: previous.child_policy().as_str(),
            candidate: candidate.child_policy().as_str(),
        });
    }
    if previous.focus() != candidate.focus() {
        return Err(WorthUiComponentShapeDenial::FocusChanged {
            component_id,
            previous: previous.focus().as_str(),
            candidate: candidate.focus().as_str(),
        });
    }
    if previous.accessibility() != candidate.accessibility() {
        return Err(WorthUiComponentShapeDenial::AccessibilityChanged {
            component_id,
            previous: previous.accessibility().as_str(),
            candidate: candidate.accessibility().as_str(),
        });
    }
    if previous.execution_lane() != candidate.execution_lane() {
        return Err(WorthUiComponentShapeDenial::ExecutionLaneChanged {
            component_id,
            previous: previous.execution_lane().as_str(),
            candidate: candidate.execution_lane().as_str(),
        });
    }
    if previous == candidate {
        return Ok(WorthUiComponentCompatibility::Equivalent);
    }
    if previous_schema.schema_key() == candidate_schema.schema_key() {
        return Ok(WorthUiComponentCompatibility::CompatiblePreserveState(
            WorthUiComponentStatePreservation::new(candidate.id().clone()),
        ));
    }
    Ok(WorthUiComponentCompatibility::CompatibleDropState(
        WorthUiComponentStateDropReason::PropSchemaIncompatible {
            component_id: candidate.id().clone(),
            previous_schema_key: previous_schema.schema_key().to_owned(),
            candidate_schema_key: candidate_schema.schema_key().to_owned(),
        },
    ))
}

pub(crate) fn merge_component_compatibility(
    current: WorthUiComponentCompatibility,
    next: WorthUiComponentCompatibility,
) -> WorthUiComponentCompatibility {
    match (current, next) {
        (WorthUiComponentCompatibility::Equivalent, next) => next,
        (current, WorthUiComponentCompatibility::Equivalent) => current,
        (
            WorthUiComponentCompatibility::CompatiblePreserveState(_),
            WorthUiComponentCompatibility::CompatibleDropState(reason),
        ) => WorthUiComponentCompatibility::CompatibleDropState(reason),
        (current @ WorthUiComponentCompatibility::CompatibleDropState(_), _) => current,
        (current, _) => current,
    }
}
