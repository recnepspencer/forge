#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDiagnosticSupportReport {
    section_count: usize,
    materialized: bool,
}

impl WorthUiDiagnosticSupportReport {
    pub(crate) fn elided() -> Self {
        Self {
            section_count: 0,
            materialized: false,
        }
    }

    pub(crate) fn materialized(section_count: usize) -> Self {
        Self {
            section_count,
            materialized: true,
        }
    }

    pub fn section_count(&self) -> usize {
        self.section_count
    }

    pub fn is_materialized(&self) -> bool {
        self.materialized
    }
}
