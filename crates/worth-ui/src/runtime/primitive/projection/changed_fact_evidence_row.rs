use crate::runtime::{
    WorthUiAuthoredDeltaChangePosture, WorthUiAuthoredSemanticSubject,
    WorthUiAuthoredStructuralChangedFactRow, WorthUiRuntimeFactId, WorthUiSemanticSliceId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveChangedFactEvidenceRow {
    semantic_slice: WorthUiSemanticSliceId,
    subject_surface_id: String,
    change_posture: WorthUiAuthoredDeltaChangePosture,
    changed_facts: Vec<WorthUiRuntimeFactId>,
}

impl WorthUiPrimitiveChangedFactEvidenceRow {
    pub(crate) fn from_changed_fact_row(
        row: &WorthUiAuthoredStructuralChangedFactRow,
        subject_surface_id: &str,
    ) -> Option<Self> {
        let WorthUiAuthoredSemanticSubject::Surface { surface_id } = row.semantic_row().subject()
        else {
            return None;
        };
        if surface_id != subject_surface_id {
            return None;
        }
        if !primitive_projection_slice(row.semantic_row().slice_id()) {
            return None;
        }
        Some(Self {
            semantic_slice: row.semantic_row().slice_id(),
            subject_surface_id: surface_id.clone(),
            change_posture: row.semantic_row().change_posture(),
            changed_facts: row.changed_facts().facts().cloned().collect(),
        })
    }

    pub fn semantic_slice(&self) -> WorthUiSemanticSliceId {
        self.semantic_slice
    }

    pub fn subject_surface_id(&self) -> &str {
        &self.subject_surface_id
    }

    pub fn change_posture(&self) -> WorthUiAuthoredDeltaChangePosture {
        self.change_posture
    }

    pub fn changed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.changed_facts
    }
}

fn primitive_projection_slice(slice_id: WorthUiSemanticSliceId) -> bool {
    matches!(
        slice_id,
        WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps
            | WorthUiSemanticSliceId::PrimitiveContent
            | WorthUiSemanticSliceId::PrimitiveContainer
            | WorthUiSemanticSliceId::PrimitiveMeasurement
            | WorthUiSemanticSliceId::PrimitiveAppearance
            | WorthUiSemanticSliceId::PrimitiveAppearanceState
            | WorthUiSemanticSliceId::PrimitiveInteraction
            | WorthUiSemanticSliceId::PrimitiveMotion
            | WorthUiSemanticSliceId::PrimitiveFlowLayout
            | WorthUiSemanticSliceId::PrimitiveEventGeometry
    )
}
