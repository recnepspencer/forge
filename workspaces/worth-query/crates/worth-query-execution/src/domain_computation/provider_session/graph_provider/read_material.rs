use super::WorthQueryGraphReadRow;

#[derive(Debug, PartialEq)]
pub struct WorthQueryGraphReadMaterial {
    rows: Vec<WorthQueryGraphReadRow>,
}

impl WorthQueryGraphReadMaterial {
    pub fn new(rows: impl IntoIterator<Item = WorthQueryGraphReadRow>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }

    pub fn rows(&self) -> &[WorthQueryGraphReadRow] {
        &self.rows
    }

    pub(super) fn into_rows(self) -> Vec<WorthQueryGraphReadRow> {
        self.rows
    }
}
