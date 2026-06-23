use crate::reload::{
    ValidationAuthoredStructuralChangedFactRowEvidence, ValidationPageHostProjectionRowEvidence,
    ValidationProjectionRebindRowEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationVisibleStructuralEvidence {
    authored_structural_rows: Vec<ValidationAuthoredStructuralChangedFactRowEvidence>,
    header_projection_rows: Vec<ValidationProjectionRebindRowEvidence>,
    page_host_projection_rows: Vec<ValidationPageHostProjectionRowEvidence>,
}

impl ValidationVisibleStructuralEvidence {
    pub fn new(
        authored_structural_rows: Vec<ValidationAuthoredStructuralChangedFactRowEvidence>,
        header_projection_rows: Vec<ValidationProjectionRebindRowEvidence>,
        page_host_projection_rows: Vec<ValidationPageHostProjectionRowEvidence>,
    ) -> Self {
        Self {
            authored_structural_rows,
            header_projection_rows,
            page_host_projection_rows,
        }
    }

    pub fn authored_structural_rows(
        &self,
    ) -> &[ValidationAuthoredStructuralChangedFactRowEvidence] {
        &self.authored_structural_rows
    }

    pub fn header_projection_rows(&self) -> &[ValidationProjectionRebindRowEvidence] {
        &self.header_projection_rows
    }

    pub fn page_host_projection_rows(&self) -> &[ValidationPageHostProjectionRowEvidence] {
        &self.page_host_projection_rows
    }
}
