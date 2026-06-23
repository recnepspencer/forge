use super::{
    WorthUiAuthoredDeltaCounters, WorthUiAuthoredDeltaDigest, WorthUiTouchedAuthoredDeclarationRow,
    WorthUiTouchedAuthoredSemanticSliceRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredDeltaSummary {
    digest: WorthUiAuthoredDeltaDigest,
    counters: WorthUiAuthoredDeltaCounters,
    touched_declaration_rows: Vec<WorthUiTouchedAuthoredDeclarationRow>,
    semantic_slice_rows: Vec<WorthUiTouchedAuthoredSemanticSliceRow>,
}

impl WorthUiAuthoredDeltaSummary {
    pub(crate) fn new(
        digest: WorthUiAuthoredDeltaDigest,
        counters: WorthUiAuthoredDeltaCounters,
        touched_declaration_rows: Vec<WorthUiTouchedAuthoredDeclarationRow>,
        semantic_slice_rows: Vec<WorthUiTouchedAuthoredSemanticSliceRow>,
    ) -> Self {
        Self {
            digest,
            counters,
            touched_declaration_rows,
            semantic_slice_rows,
        }
    }

    pub fn digest(&self) -> WorthUiAuthoredDeltaDigest {
        self.digest
    }

    pub fn counters(&self) -> &WorthUiAuthoredDeltaCounters {
        &self.counters
    }

    pub fn touched_declaration_rows(&self) -> &[WorthUiTouchedAuthoredDeclarationRow] {
        &self.touched_declaration_rows
    }

    pub fn semantic_slice_rows(&self) -> &[WorthUiTouchedAuthoredSemanticSliceRow] {
        &self.semantic_slice_rows
    }

    pub fn is_empty(&self) -> bool {
        self.touched_declaration_rows.is_empty() && self.semantic_slice_rows.is_empty()
    }
}
