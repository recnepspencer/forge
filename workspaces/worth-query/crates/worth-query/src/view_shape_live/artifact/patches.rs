use crate::identity_evolution::InspectorIdentityArtifact;
use crate::live::{RefreshFallback, SuppressionReason};
use worth_foundational::facade::AspectKey;

use super::super::family::LiveViewShapeFamily;
use super::super::grouped_delta::GroupedDeltaArtifact;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ViewShapePatchFamily {
    TableRowPatch,
    DetailFieldPatch,
    ObservedInspectorPatch,
    FocusedInspectorAspectPatch,
    KanbanGroupMembershipPatch,
}

impl ViewShapePatchFamily {
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
pub struct TableRowPatchArtifact {
    digest: String,
    row_delta_count: usize,
}

impl TableRowPatchArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn row_delta_count(&self) -> usize {
        self.row_delta_count
    }
    #[cfg(test)]
    pub(crate) fn new(digest: impl Into<String>, row_delta_count: usize) -> Self {
        Self {
            digest: digest.into(),
            row_delta_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailFieldPatchArtifact {
    digest: String,
    field_delta_count: usize,
}

impl DetailFieldPatchArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn field_delta_count(&self) -> usize {
        self.field_delta_count
    }
    #[cfg(test)]
    pub(crate) fn new(digest: impl Into<String>, field_delta_count: usize) -> Self {
        Self {
            digest: digest.into(),
            field_delta_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedInspectorPatchArtifact {
    digest: String,
    field_delta_count: usize,
    delivery_width: usize,
    inspector_identity: Option<InspectorIdentityArtifact>,
}

impl ObservedInspectorPatchArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn field_delta_count(&self) -> usize {
        self.field_delta_count
    }

    pub fn delivery_width(&self) -> usize {
        self.delivery_width
    }

    pub fn inspector_identity(&self) -> Option<&InspectorIdentityArtifact> {
        self.inspector_identity.as_ref()
    }
    #[cfg(test)]
    pub(crate) fn new(
        digest: impl Into<String>,
        field_delta_count: usize,
        delivery_width: usize,
        inspector_identity: Option<InspectorIdentityArtifact>,
    ) -> Self {
        Self {
            digest: digest.into(),
            field_delta_count,
            delivery_width,
            inspector_identity,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusedInspectorAspectPatchArtifact {
    digest: String,
    focus_aspect: AspectKey,
    field_delta_count: usize,
    inspector_identity: Option<InspectorIdentityArtifact>,
}

impl FocusedInspectorAspectPatchArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn native_focus_aspect_key(&self) -> &AspectKey {
        &self.focus_aspect
    }

    pub fn field_delta_count(&self) -> usize {
        self.field_delta_count
    }

    pub fn inspector_identity(&self) -> Option<&InspectorIdentityArtifact> {
        self.inspector_identity.as_ref()
    }
    #[cfg(test)]
    pub(crate) fn new(
        digest: impl Into<String>,
        focus_aspect: AspectKey,
        field_delta_count: usize,
        inspector_identity: Option<InspectorIdentityArtifact>,
    ) -> Self {
        Self {
            digest: digest.into(),
            focus_aspect,
            field_delta_count,
            inspector_identity,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapePatchPayload {
    TableRowPatch(TableRowPatchArtifact),
    DetailFieldPatch(DetailFieldPatchArtifact),
    ObservedInspectorPatch(ObservedInspectorPatchArtifact),
    FocusedInspectorAspectPatch(FocusedInspectorAspectPatchArtifact),
    KanbanGroupMembershipPatch(GroupedDeltaArtifact),
    Refresh(ViewShapeRefreshDisposition),
    Suppressed(ViewShapeSuppressionDisposition),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapeRefreshDisposition {
    Admitted {
        family: LiveViewShapeFamily,
        fallback: RefreshFallback,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapeSuppressionDisposition {
    SuppressedByCore(SuppressionReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapePatchEnvelope {
    family: LiveViewShapeFamily,
    patch_family: Option<ViewShapePatchFamily>,
    delivery_digest: String,
    replay_digest: String,
    payload: ViewShapePatchPayload,
}

impl ViewShapePatchEnvelope {
    pub fn family(&self) -> LiveViewShapeFamily {
        self.family
    }

    pub fn patch_family(&self) -> Option<ViewShapePatchFamily> {
        self.patch_family
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn payload(&self) -> &ViewShapePatchPayload {
        &self.payload
    }
    #[cfg(test)]
    pub(crate) fn new(
        family: LiveViewShapeFamily,
        patch_family: Option<ViewShapePatchFamily>,
        delivery_digest: impl Into<String>,
        replay_digest: impl Into<String>,
        payload: ViewShapePatchPayload,
    ) -> Self {
        Self {
            family,
            patch_family,
            delivery_digest: delivery_digest.into(),
            replay_digest: replay_digest.into(),
            payload,
        }
    }
}
