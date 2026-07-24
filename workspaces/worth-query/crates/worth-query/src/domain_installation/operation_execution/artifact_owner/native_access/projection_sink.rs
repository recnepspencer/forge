use worth_foundational::facade::{AspectContract, AspectValue};

use super::provider_port::value_matches_contract;
use super::WorthQueryArtifactProviderAccessDenial;

pub struct WorthQueryArtifactProjectionSink {
    fields: Vec<AspectContract>,
    max_rows: usize,
    values: Vec<AspectValue>,
}

impl WorthQueryArtifactProjectionSink {
    pub(crate) fn new(fields: Vec<AspectContract>, max_rows: usize) -> Self {
        let value_capacity = max_rows.saturating_mul(fields.len());
        Self {
            fields,
            max_rows,
            values: Vec::with_capacity(value_capacity),
        }
    }

    pub fn push_row(
        &mut self,
        values: impl IntoIterator<Item = AspectValue>,
    ) -> Result<(), WorthQueryArtifactProviderAccessDenial> {
        if self.row_count() >= self.max_rows {
            return Err(WorthQueryArtifactProviderAccessDenial::BoundsExceeded);
        }
        let values = values.into_iter().collect::<Vec<_>>();
        if values.len() != self.fields.len()
            || values
                .iter()
                .zip(&self.fields)
                .any(|(value, contract)| !value_matches_contract(value, contract))
        {
            return Err(WorthQueryArtifactProviderAccessDenial::ShapeMismatch);
        }
        self.values.extend(values);
        Ok(())
    }

    pub fn row_count(&self) -> usize {
        if self.fields.is_empty() {
            0
        } else {
            self.values.len() / self.fields.len()
        }
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    pub fn row(&self, row: usize) -> Option<&[AspectValue]> {
        let start = row.checked_mul(self.fields.len())?;
        self.values.get(start..start + self.fields.len())
    }

    pub fn allocated_capacity_bytes(&self) -> usize {
        self.values
            .capacity()
            .saturating_mul(std::mem::size_of::<AspectValue>())
    }

    pub(crate) fn result_semantic_bytes(&self) -> usize {
        self.values
            .iter()
            .map(AspectValue::semantic_byte_width)
            .sum()
    }
}
