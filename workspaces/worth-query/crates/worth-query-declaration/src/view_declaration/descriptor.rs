use super::family::ViewShapeFamily;
use super::identity::ViewShapeIdentityConsumption;
use worth_foundational::facade::AspectKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeDescriptor {
    family: ViewShapeFamily,
    focused_aspect: Option<AspectKey>,
    grouping_aspect: Option<AspectKey>,
    identity_consumption: ViewShapeIdentityConsumption,
}

impl ViewShapeDescriptor {
    pub fn table() -> Self {
        Self {
            family: ViewShapeFamily::Table,
            focused_aspect: None,
            grouping_aspect: None,
            identity_consumption: ViewShapeIdentityConsumption::none(),
        }
    }

    pub fn detail() -> Self {
        Self {
            family: ViewShapeFamily::Detail,
            focused_aspect: None,
            grouping_aspect: None,
            identity_consumption: ViewShapeIdentityConsumption::none(),
        }
    }

    pub fn inspector_detail_observed() -> Self {
        Self {
            family: ViewShapeFamily::InspectorDetailObserved,
            focused_aspect: None,
            grouping_aspect: None,
            identity_consumption: ViewShapeIdentityConsumption::none(),
        }
    }

    pub fn inspector_detail_focused(focused_aspect: AspectKey) -> Self {
        Self {
            family: ViewShapeFamily::InspectorDetailFocused,
            focused_aspect: Some(focused_aspect),
            grouping_aspect: None,
            identity_consumption: ViewShapeIdentityConsumption::none(),
        }
    }

    pub fn identity_aware_inspector_detail_observed() -> Self {
        Self {
            family: ViewShapeFamily::InspectorDetailObserved,
            focused_aspect: None,
            grouping_aspect: None,
            identity_consumption: ViewShapeIdentityConsumption::inspector_identity_summary(),
        }
    }

    pub fn identity_aware_inspector_detail_focused(
        focused_aspect: AspectKey,
        classification: super::identity::InspectorIdentityClassification,
    ) -> Self {
        Self {
            family: ViewShapeFamily::InspectorDetailFocused,
            focused_aspect: Some(focused_aspect),
            grouping_aspect: None,
            identity_consumption:
                ViewShapeIdentityConsumption::focused_inspector_identity_classification(
                    classification,
                ),
        }
    }

    pub fn kanban_grouped(grouping_aspect: AspectKey) -> Self {
        Self {
            family: ViewShapeFamily::KanbanGrouped,
            focused_aspect: None,
            grouping_aspect: Some(grouping_aspect),
            identity_consumption: ViewShapeIdentityConsumption::none(),
        }
    }

    pub fn family(&self) -> ViewShapeFamily {
        self.family
    }

    pub fn native_focused_aspect_key(&self) -> Option<&AspectKey> {
        self.focused_aspect.as_ref()
    }

    pub fn native_grouping_aspect_key(&self) -> Option<&AspectKey> {
        self.grouping_aspect.as_ref()
    }

    pub fn identity_consumption(&self) -> &ViewShapeIdentityConsumption {
        &self.identity_consumption
    }
}
