use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryDerivedPatchFamily {
    Incremental,
    RefreshFallback,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDerivedPatchPayload {
    kind: WorthQueryDerivedPatchPayloadKind,
}

#[derive(Clone, Debug, PartialEq)]
enum WorthQueryDerivedPatchPayloadKind {
    Empty,
    RetainedRows(Vec<WorthQueryRetainedMaterializedRow>),
}

impl WorthQueryDerivedPatchPayload {
    pub fn empty() -> Self {
        Self {
            kind: WorthQueryDerivedPatchPayloadKind::Empty,
        }
    }

    pub(in crate::runtime) fn from_retained_row(row: WorthQueryRetainedMaterializedRow) -> Self {
        Self::from_retained_rows([row])
    }

    pub(in crate::runtime) fn from_retained_rows(
        rows: impl IntoIterator<Item = WorthQueryRetainedMaterializedRow>,
    ) -> Self {
        Self {
            kind: WorthQueryDerivedPatchPayloadKind::RetainedRows(rows.into_iter().collect()),
        }
    }

    pub fn from_retained_scalar_values(
        scalar_values: impl IntoIterator<Item = (WorthQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<Self, String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        Ok(Self::from_retained_row(row))
    }

    pub(in crate::runtime) fn empty_refresh_fallback() -> Self {
        Self::empty()
    }

    #[cfg(test)]
    pub fn retained_rows(&self) -> &[WorthQueryRetainedMaterializedRow] {
        match &self.kind {
            WorthQueryDerivedPatchPayloadKind::RetainedRows(rows) => rows,
            WorthQueryDerivedPatchPayloadKind::Empty => &[],
        }
    }
}

pub(super) fn retained_materialized_row_from_scalar_values(
    scalar_values: impl IntoIterator<Item = (WorthQueryRetainedFieldPath, AspectValue)>,
) -> Result<WorthQueryRetainedMaterializedRow, String> {
    WorthQueryRetainedMaterializedRow::from_scalar_values(BTreeMap::from_iter(scalar_values))
}
