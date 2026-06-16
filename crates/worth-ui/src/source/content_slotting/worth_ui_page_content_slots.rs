use super::WorthUiContentSlotAssignment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageContentSlots {
    page_name: String,
    assignments: Vec<WorthUiContentSlotAssignment>,
}

impl WorthUiPageContentSlots {
    pub(crate) fn from_prepared_assignments(
        page_name: impl Into<String>,
        assignments: Vec<WorthUiContentSlotAssignment>,
    ) -> Self {
        Self {
            page_name: page_name.into(),
            assignments,
        }
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn assignments(&self) -> &[WorthUiContentSlotAssignment] {
        &self.assignments
    }

    pub fn assignment_for_slot(&self, slot_name: &str) -> Option<&WorthUiContentSlotAssignment> {
        self.assignments
            .iter()
            .find(|assignment| assignment.slot_name() == slot_name)
    }
}
