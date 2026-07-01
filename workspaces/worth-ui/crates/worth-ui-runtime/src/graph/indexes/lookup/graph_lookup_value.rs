use crate::graph::indexes::UiGraphLookupReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphLookup<T> {
    receipt: UiGraphLookupReceipt,
    value: T,
}

impl<T> UiGraphLookup<T> {
    pub const fn new(receipt: UiGraphLookupReceipt, value: T) -> Self {
        Self { receipt, value }
    }

    pub const fn receipt(&self) -> UiGraphLookupReceipt {
        self.receipt
    }

    pub fn value(&self) -> T
    where
        T: Clone,
    {
        self.value.clone()
    }

    pub fn value_ref(&self) -> &T {
        &self.value
    }
}
