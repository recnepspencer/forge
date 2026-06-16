#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageHostFrameReceipt {
    page_name: String,
    slots: Vec<WorthUiPageHostSlotReceipt>,
    frame_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageHostSlotReceipt {
    slot_name: String,
    surface_id: String,
}

impl WorthUiPageHostFrameReceipt {
    pub(crate) fn new(
        page_name: impl Into<String>,
        slots: Vec<WorthUiPageHostSlotReceipt>,
        frame_digest: u64,
    ) -> Self {
        Self {
            page_name: page_name.into(),
            slots,
            frame_digest,
        }
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn slots(&self) -> &[WorthUiPageHostSlotReceipt] {
        &self.slots
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }
}

impl WorthUiPageHostSlotReceipt {
    pub(crate) fn new(slot_name: impl Into<String>, surface_id: impl Into<String>) -> Self {
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
