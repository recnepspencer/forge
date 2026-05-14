use crate::memory_workspace::ForgeQueryEntity;

use super::ForgeQueryReadReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryReadResult {
    payload: Vec<ForgeQueryEntity>,
    receipt: ForgeQueryReadReceipt,
}

impl ForgeQueryReadResult {
    pub fn payload(&self) -> &[ForgeQueryEntity] {
        &self.payload
    }

    pub fn rows(&self) -> &[ForgeQueryEntity] {
        &self.payload
    }

    pub fn receipt(&self) -> &ForgeQueryReadReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        payload: Vec<ForgeQueryEntity>,
        receipt: ForgeQueryReadReceipt,
    ) -> Self {
        Self { payload, receipt }
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        payload: Vec<ForgeQueryEntity>,
        receipt: ForgeQueryReadReceipt,
    ) -> Self {
        Self { payload, receipt }
    }
}
