#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphSlotTopology {
    slot_name: Box<str>,
}

impl UiGraphSlotTopology {
    pub(in crate::graph::topology) fn new(slot_name: Box<str>) -> Self {
        Self { slot_name }
    }

    pub fn slot_name(&self) -> &str {
        &self.slot_name
    }
}
