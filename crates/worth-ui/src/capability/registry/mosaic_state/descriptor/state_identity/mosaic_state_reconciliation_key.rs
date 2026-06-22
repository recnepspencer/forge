#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MosaicStateReconciliationKey {
    value: String,
}

impl MosaicStateReconciliationKey {
    pub(crate) fn from_digest_basis(value: String) -> Self {
        Self { value }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
