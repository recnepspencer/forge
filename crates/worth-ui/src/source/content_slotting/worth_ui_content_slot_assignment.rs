#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiContentSlotAssignment {
    slot_name: String,
    surface_id: String,
}

impl WorthUiContentSlotAssignment {
    pub(crate) fn from_prepared_mount(
        slot_name: impl Into<String>,
        surface_id: impl Into<String>,
    ) -> Self {
        Self {
            slot_name: slot_name.into(),
            surface_id: surface_id.into(),
        }
    }

    pub fn slot_name(&self) -> &str {
        &self.slot_name
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }
}
