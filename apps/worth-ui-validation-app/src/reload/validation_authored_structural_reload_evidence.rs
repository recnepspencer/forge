use worth_ui::facade::{
    WorthUiAuthoredDeltaChangePosture, WorthUiAuthoredSemanticSubject, WorthUiRuntimeFactFamily,
    WorthUiRuntimeFactId, WorthUiSemanticSliceId, WorthUiValidationChangedFactMappingReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAuthoredStructuralReloadEvidence {
    authored_delta_digest: u64,
    rows: Vec<ValidationAuthoredStructuralChangedFactRowEvidence>,
    previous_slots: Vec<ValidationAuthoredStructuralSlotEvidence>,
    current_slots: Vec<ValidationAuthoredStructuralSlotEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAuthoredStructuralChangedFactRowEvidence {
    slice_id: WorthUiSemanticSliceId,
    subject_label: String,
    change_posture: WorthUiAuthoredDeltaChangePosture,
    changed_fact_labels: Vec<String>,
    changed_fact_families: Vec<WorthUiRuntimeFactFamily>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAuthoredStructuralSlotEvidence {
    slot_name: String,
    surface_id: String,
    component_id: String,
}

impl ValidationAuthoredStructuralReloadEvidence {
    pub fn from_mapping_receipt(
        receipt: &WorthUiValidationChangedFactMappingReceipt,
        previous_slots: Vec<ValidationAuthoredStructuralSlotEvidence>,
        current_slots: Vec<ValidationAuthoredStructuralSlotEvidence>,
    ) -> Self {
        Self {
            authored_delta_digest: receipt.authored_delta_summary().digest().as_u64(),
            rows: receipt
                .rows()
                .iter()
                .map(ValidationAuthoredStructuralChangedFactRowEvidence::from_row)
                .collect(),
            previous_slots,
            current_slots,
        }
    }

    pub fn authored_delta_digest(&self) -> u64 {
        self.authored_delta_digest
    }

    pub fn rows(&self) -> &[ValidationAuthoredStructuralChangedFactRowEvidence] {
        &self.rows
    }

    pub fn previous_slots(&self) -> &[ValidationAuthoredStructuralSlotEvidence] {
        &self.previous_slots
    }

    pub fn current_slots(&self) -> &[ValidationAuthoredStructuralSlotEvidence] {
        &self.current_slots
    }
}

impl ValidationAuthoredStructuralChangedFactRowEvidence {
    fn from_row(row: &worth_ui::facade::WorthUiAuthoredStructuralChangedFactRow) -> Self {
        Self {
            slice_id: row.semantic_row().slice_id(),
            subject_label: subject_label(row.semantic_row().subject()),
            change_posture: row.semantic_row().change_posture(),
            changed_fact_labels: row
                .changed_facts()
                .facts()
                .map(runtime_fact_label)
                .collect(),
            changed_fact_families: row.changed_fact_families().to_vec(),
        }
    }

    pub fn slice_id(&self) -> WorthUiSemanticSliceId {
        self.slice_id
    }

    pub fn subject_label(&self) -> &str {
        &self.subject_label
    }

    pub fn change_posture(&self) -> WorthUiAuthoredDeltaChangePosture {
        self.change_posture
    }

    pub fn changed_fact_labels(&self) -> &[String] {
        &self.changed_fact_labels
    }

    pub fn changed_fact_families(&self) -> &[WorthUiRuntimeFactFamily] {
        &self.changed_fact_families
    }
}

impl ValidationAuthoredStructuralSlotEvidence {
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

fn subject_label(subject: &WorthUiAuthoredSemanticSubject) -> String {
    match subject {
        WorthUiAuthoredSemanticSubject::Workspace { workspace_name } => {
            format!("workspace:{workspace_name}")
        }
        WorthUiAuthoredSemanticSubject::Page { page_name } => format!("page:{page_name}"),
        WorthUiAuthoredSemanticSubject::PageSlot {
            page_name,
            slot_name,
        } => format!("page-slot:{page_name}:{slot_name}"),
        WorthUiAuthoredSemanticSubject::Surface { surface_id } => {
            format!("surface:{surface_id}")
        }
        WorthUiAuthoredSemanticSubject::AppearanceRecipe { recipe_name } => {
            format!("appearance:{recipe_name}")
        }
        WorthUiAuthoredSemanticSubject::RuntimeBinding { binding_name } => {
            format!("binding:{binding_name}")
        }
    }
}

fn runtime_fact_label(fact: &WorthUiRuntimeFactId) -> String {
    format!("{:?}({})", fact.family(), fact.identity())
}
