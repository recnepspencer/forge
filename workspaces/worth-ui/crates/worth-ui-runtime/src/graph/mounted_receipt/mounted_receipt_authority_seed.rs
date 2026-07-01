#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountedReceiptAuthoritySeed {
    graph_owned_slot_reserved: bool,
}

impl UiGraphMountedReceiptAuthoritySeed {
    pub(crate) const fn reserved() -> Self {
        Self {
            graph_owned_slot_reserved: true,
        }
    }

    pub fn graph_owned_slot_reserved(self) -> bool {
        self.graph_owned_slot_reserved
    }
}
