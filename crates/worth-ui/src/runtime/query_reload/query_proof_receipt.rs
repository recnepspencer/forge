#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryProjectionFactReceipt {
    receipt_identity: String,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryStateSnapshotReceipt {
    receipt_identity: String,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryEffectPostureReceipt {
    receipt_identity: String,
    receipt_digest: u64,
}

impl WorthUiQueryProjectionFactReceipt {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(receipt_identity: impl Into<String>, receipt_digest: u64) -> Self {
        Self {
            receipt_identity: receipt_identity.into(),
            receipt_digest,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiQueryEffectPostureReceipt {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(receipt_identity: impl Into<String>, receipt_digest: u64) -> Self {
        Self {
            receipt_identity: receipt_identity.into(),
            receipt_digest,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiQueryStateSnapshotReceipt {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(receipt_identity: impl Into<String>, receipt_digest: u64) -> Self {
        Self {
            receipt_identity: receipt_identity.into(),
            receipt_digest,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
