use forge_foundational::facade::AspectKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ViewShapeInvalidationPosture {
    OrderedCollectionMembershipAndOrdering,
    DetailProjectionFields,
    InspectorObservedNarrowDetail,
    InspectorFocusedAspect,
    KanbanGroupedMembershipAndAspect,
}

impl ViewShapeInvalidationPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OrderedCollectionMembershipAndOrdering => {
                "ordered_collection_membership_and_ordering"
            }
            Self::DetailProjectionFields => "detail_projection_fields",
            Self::InspectorObservedNarrowDetail => "inspector_observed_narrow_detail",
            Self::InspectorFocusedAspect => "inspector_focused_aspect",
            Self::KanbanGroupedMembershipAndAspect => "kanban_grouped_membership_and_aspect",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ViewShapePatchPosture {
    TableRowPatch,
    DetailFieldPatch,
    ObservedInspectorPatch,
    FocusedInspectorAspectPatch,
    KanbanGroupMembershipPatch,
}

impl ViewShapePatchPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TableRowPatch => "table_row_patch",
            Self::DetailFieldPatch => "detail_field_patch",
            Self::ObservedInspectorPatch => "observed_inspector_patch",
            Self::FocusedInspectorAspectPatch => "focused_inspector_aspect_patch",
            Self::KanbanGroupMembershipPatch => "kanban_group_membership_patch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeDeliveryMetadata {
    focus_aspect: Option<String>,
    grouping_aspect: Option<AspectKey>,
    identity_consumption: ViewShapeIdentityConsumption,
    projection_legality_matches_detail: bool,
    delivery_width_narrowed: bool,
    grouped_delivery: bool,
}

impl ViewShapeDeliveryMetadata {
    pub(crate) fn new(
        focus_aspect: Option<String>,
        grouping_aspect: Option<AspectKey>,
        identity_consumption: ViewShapeIdentityConsumption,
        projection_legality_matches_detail: bool,
        delivery_width_narrowed: bool,
        grouped_delivery: bool,
    ) -> Self {
        Self {
            focus_aspect,
            grouping_aspect,
            identity_consumption,
            projection_legality_matches_detail,
            delivery_width_narrowed,
            grouped_delivery,
        }
    }

    pub fn focus_aspect(&self) -> Option<&str> {
        self.focus_aspect.as_deref()
    }

    pub fn grouping_aspect(&self) -> Option<&str> {
        self.grouping_aspect.as_ref().map(AspectKey::as_str)
    }

    pub fn native_grouping_aspect_key(&self) -> Option<&AspectKey> {
        self.grouping_aspect.as_ref()
    }

    pub fn identity_consumption(&self) -> &ViewShapeIdentityConsumption {
        &self.identity_consumption
    }

    pub fn projection_legality_matches_detail(&self) -> bool {
        self.projection_legality_matches_detail
    }

    pub fn delivery_width_narrowed(&self) -> bool {
        self.delivery_width_narrowed
    }

    pub fn grouped_delivery(&self) -> bool {
        self.grouped_delivery
    }
}
use super::identity::ViewShapeIdentityConsumption;
