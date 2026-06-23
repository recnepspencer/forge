#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQuerySupportSnapshotErrorKind {
    JsonDecodeFailed,
    JsonEncodeFailed,
    SchemaVersionMismatch,
    SchemaIdentityMismatch,
    InvalidBackendPosture,
    InvalidFacadeFamily,
    InvalidSupportStatus,
    InvalidTeachingPosture,
    InvalidRequiredField,
    SnapshotDigestMismatch,
    RowDigestMismatch,
    SourceMatrixDigestMismatch,
    RowCountMismatch,
    RowMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportSnapshotError {
    kind: ForgeQuerySupportSnapshotErrorKind,
    message: String,
    expected: Option<String>,
    found: Option<String>,
    surface: Option<String>,
    row_index: Option<usize>,
    field: Option<&'static str>,
}

impl ForgeQuerySupportSnapshotError {
    pub(crate) fn new(
        kind: ForgeQuerySupportSnapshotErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            expected: None,
            found: None,
            surface: None,
            row_index: None,
            field: None,
        }
    }

    pub(crate) fn with_expected_found(
        kind: ForgeQuerySupportSnapshotErrorKind,
        message: impl Into<String>,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            expected: Some(expected.into()),
            found: Some(found.into()),
            surface: None,
            row_index: None,
            field: None,
        }
    }

    pub(crate) fn with_surface_found(
        kind: ForgeQuerySupportSnapshotErrorKind,
        message: impl Into<String>,
        surface: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            expected: None,
            found: Some(found.into()),
            surface: Some(surface.into()),
            row_index: None,
            field: None,
        }
    }

    pub(crate) fn with_row_field_mismatch(
        kind: ForgeQuerySupportSnapshotErrorKind,
        message: impl Into<String>,
        row_index: usize,
        field: &'static str,
        surface: impl Into<String>,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            expected: Some(expected.into()),
            found: Some(found.into()),
            surface: Some(surface.into()),
            row_index: Some(row_index),
            field: Some(field),
        }
    }

    pub fn kind(&self) -> &ForgeQuerySupportSnapshotErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn expected(&self) -> Option<&str> {
        self.expected.as_deref()
    }

    pub fn found(&self) -> Option<&str> {
        self.found.as_deref()
    }

    pub fn surface(&self) -> Option<&str> {
        self.surface.as_deref()
    }

    pub fn row_index(&self) -> Option<usize> {
        self.row_index
    }

    pub fn field(&self) -> Option<&'static str> {
        self.field
    }
}

impl std::fmt::Display for ForgeQuerySupportSnapshotError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ForgeQuerySupportSnapshotError {}
