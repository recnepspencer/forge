use crate::runtime::WorthUiSemanticSliceId;

use super::WorthUiAuthoredDeltaChangePosture;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiAuthoredSemanticSubject {
    Workspace {
        workspace_name: String,
    },
    Page {
        page_name: String,
    },
    PageSlot {
        page_name: String,
        slot_name: String,
    },
    Surface {
        surface_id: String,
    },
    AppearanceRecipe {
        recipe_name: String,
    },
    RuntimeBinding {
        binding_name: String,
    },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiTouchedAuthoredSemanticSliceRow {
    slice_id: WorthUiSemanticSliceId,
    subject: WorthUiAuthoredSemanticSubject,
    change_posture: WorthUiAuthoredDeltaChangePosture,
}

impl WorthUiTouchedAuthoredSemanticSliceRow {
    pub(crate) fn new(
        slice_id: WorthUiSemanticSliceId,
        subject: WorthUiAuthoredSemanticSubject,
        change_posture: WorthUiAuthoredDeltaChangePosture,
    ) -> Self {
        Self {
            slice_id,
            subject,
            change_posture,
        }
    }

    pub fn slice_id(&self) -> WorthUiSemanticSliceId {
        self.slice_id
    }

    pub fn subject(&self) -> &WorthUiAuthoredSemanticSubject {
        &self.subject
    }

    pub fn change_posture(&self) -> WorthUiAuthoredDeltaChangePosture {
        self.change_posture
    }
}
