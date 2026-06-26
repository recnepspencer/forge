#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateImpactReceipts {
    preserve_receipts: usize,
    replace_receipts: usize,
    drop_receipts: usize,
    create_receipts: usize,
}

impl WorthUiDurableStateImpactReceipts {
    pub fn is_complete(self) -> bool {
        self.preserve_receipts > 0
            || self.replace_receipts > 0
            || self.drop_receipts > 0
            || self.create_receipts > 0
    }
}
