use crate::runtime::WorthUiRuntimeFactId;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageHostSlotMountReceipt {
    page_name: String,
    slot: WorthUiPageHostSlotReceipt,
    frame_digest: u64,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
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

impl WorthUiPageHostSlotMountReceipt {
    pub(crate) fn new(
        page_name: impl Into<String>,
        slot: WorthUiPageHostSlotReceipt,
        frame_digest: u64,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
    ) -> Self {
        let mut consumed_facts = consumed_facts;
        consumed_facts.sort();
        consumed_facts.dedup();
        Self {
            page_name: page_name.into(),
            slot,
            frame_digest,
            consumed_facts,
        }
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn slot_name(&self) -> &str {
        self.slot.slot_name()
    }

    pub fn surface_id(&self) -> &str {
        self.slot.surface_id()
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }
}
