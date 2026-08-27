use super::{
    UiMountedSemanticTextMechanic, UiMountedSemanticTextReference, UiMountedSemanticTextTable,
    UiMountedSemanticTextTableDenial, UiMountedTextSchemaVersion,
};

impl UiMountedSemanticTextTable {
    pub const MAX_ROWS: usize = 8_192;
    pub const MAX_BYTES: usize = 8 * 1_024 * 1_024;

    pub fn empty() -> Self {
        Self {
            schema: UiMountedTextSchemaVersion::current(),
            rows: std::sync::Arc::from([]),
        }
    }

    #[doc(hidden)]
    pub fn from_runtime_mounting(
        rows: Vec<UiMountedSemanticTextMechanic>,
    ) -> Result<Self, UiMountedSemanticTextTableDenial> {
        if rows.len() > Self::MAX_ROWS {
            return Err(UiMountedSemanticTextTableDenial::CapacityExceeded);
        }
        let bytes = rows
            .iter()
            .try_fold(0usize, |total, row| total.checked_add(row.text.len()));
        if bytes.is_none_or(|bytes| bytes > Self::MAX_BYTES) {
            return Err(UiMountedSemanticTextTableDenial::ByteCapacityExceeded);
        }
        Ok(Self {
            schema: UiMountedTextSchemaVersion::current(),
            rows: rows.into(),
        })
    }

    pub const fn schema(&self) -> UiMountedTextSchemaVersion {
        self.schema
    }

    pub fn rows(&self) -> &[UiMountedSemanticTextMechanic] {
        &self.rows
    }

    pub fn resolve(
        &self,
        reference: UiMountedSemanticTextReference,
    ) -> Option<&UiMountedSemanticTextMechanic> {
        self.rows.get(usize::from(reference.index()))
    }
}
