use crate::reload::{
    ValidationAuthoredStructuralChangedFactRowEvidence, ValidationPageHostRebindEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPageSlotInteractionRenderPlan {
    page_name: String,
    slots: Vec<ValidationPageSlotInteractionSlotRow>,
    previous_slots: Vec<ValidationPageSlotInteractionSlotRow>,
    shadow_dependency: ValidationPageSlotAppearanceDependencyProof,
    padding_dependency: ValidationPageSlotDensityDependencyProof,
    authored_structural_rows: Vec<ValidationAuthoredStructuralChangedFactRowEvidence>,
    latest_rebind: Option<ValidationPageHostRebindEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPageSlotInteractionSlotRow {
    slot_name: String,
    surface_id: String,
    component_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPageSlotAppearanceDependencyProof {
    token_id: String,
    offset_x_points: i32,
    offset_y_points: i32,
    blur_points: i32,
    spread_points: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPageSlotDensityDependencyProof {
    token_id: String,
    top_points: i32,
    right_points: i32,
    bottom_points: i32,
    left_points: i32,
}

impl ValidationPageSlotInteractionRenderPlan {
    pub fn new(
        page_name: String,
        slots: Vec<ValidationPageSlotInteractionSlotRow>,
        previous_slots: Vec<ValidationPageSlotInteractionSlotRow>,
        shadow_dependency: ValidationPageSlotAppearanceDependencyProof,
        padding_dependency: ValidationPageSlotDensityDependencyProof,
        authored_structural_rows: Vec<ValidationAuthoredStructuralChangedFactRowEvidence>,
        latest_rebind: Option<ValidationPageHostRebindEvidence>,
    ) -> Self {
        Self {
            page_name,
            slots,
            previous_slots,
            shadow_dependency,
            padding_dependency,
            authored_structural_rows,
            latest_rebind,
        }
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn slots(&self) -> &[ValidationPageSlotInteractionSlotRow] {
        &self.slots
    }

    pub fn previous_slots(&self) -> &[ValidationPageSlotInteractionSlotRow] {
        &self.previous_slots
    }

    pub fn shadow_dependency(&self) -> &ValidationPageSlotAppearanceDependencyProof {
        &self.shadow_dependency
    }

    pub fn padding_dependency(&self) -> &ValidationPageSlotDensityDependencyProof {
        &self.padding_dependency
    }

    pub fn authored_structural_rows(
        &self,
    ) -> &[ValidationAuthoredStructuralChangedFactRowEvidence] {
        &self.authored_structural_rows
    }

    pub fn shadow_summary(&self) -> String {
        self.shadow_dependency.summary()
    }

    pub fn padding_summary(&self) -> String {
        self.padding_dependency.summary()
    }

    pub fn latest_rebind(&self) -> Option<&ValidationPageHostRebindEvidence> {
        self.latest_rebind.as_ref()
    }
}

impl ValidationPageSlotInteractionSlotRow {
    pub fn new(slot_name: String, surface_id: String, component_id: String) -> Self {
        Self {
            slot_name,
            surface_id,
            component_id,
        }
    }

    pub fn slot_name(&self) -> &str {
        &self.slot_name
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }
}

impl ValidationPageSlotAppearanceDependencyProof {
    pub fn new(
        token_id: String,
        offset_x_points: i32,
        offset_y_points: i32,
        blur_points: i32,
        spread_points: i32,
    ) -> Self {
        Self {
            token_id,
            offset_x_points,
            offset_y_points,
            blur_points,
            spread_points,
        }
    }

    pub fn token_id(&self) -> &str {
        &self.token_id
    }

    pub fn offset_x_points(&self) -> i32 {
        self.offset_x_points
    }

    pub fn offset_y_points(&self) -> i32 {
        self.offset_y_points
    }

    pub fn blur_points(&self) -> i32 {
        self.blur_points
    }

    pub fn spread_points(&self) -> i32 {
        self.spread_points
    }

    pub fn summary(&self) -> String {
        format!(
            "{}px {}px blur {}px spread {}px",
            self.offset_x_points, self.offset_y_points, self.blur_points, self.spread_points
        )
    }
}

impl ValidationPageSlotDensityDependencyProof {
    pub fn new(
        token_id: String,
        top_points: i32,
        right_points: i32,
        bottom_points: i32,
        left_points: i32,
    ) -> Self {
        Self {
            token_id,
            top_points,
            right_points,
            bottom_points,
            left_points,
        }
    }

    pub fn token_id(&self) -> &str {
        &self.token_id
    }

    pub fn top_points(&self) -> i32 {
        self.top_points
    }

    pub fn right_points(&self) -> i32 {
        self.right_points
    }

    pub fn bottom_points(&self) -> i32 {
        self.bottom_points
    }

    pub fn left_points(&self) -> i32 {
        self.left_points
    }

    pub fn summary(&self) -> String {
        format!(
            "top {}px right {}px bottom {}px left {}px",
            self.top_points, self.right_points, self.bottom_points, self.left_points
        )
    }
}
