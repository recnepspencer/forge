#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditSourceSite {
    source_label: String,
    source_path: Option<String>,
    line: usize,
    column: usize,
}

impl ForgeQueryBoundaryAuditSourceSite {
    pub(crate) fn new(
        source_label: impl Into<String>,
        source_path: Option<&str>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            source_label: source_label.into(),
            source_path: source_path.map(str::to_string),
            line,
            column,
        }
    }

    pub fn source_label(&self) -> &str {
        &self.source_label
    }

    pub fn source_path(&self) -> Option<&str> {
        self.source_path.as_deref()
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
