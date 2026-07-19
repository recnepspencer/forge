use crate::runtime::WorthQueryRuntimeFacadeFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQuerySupportPinFindingKind {
    SchemaMismatch,
    VocabularyMismatch,
    SourceMatrixDigestChanged,
    RequiredRowMissing,
    StatusMismatch,
    TeachingPostureMismatch,
    LiveRowDigestMismatch,
    ObservedRowMissing,
    ObservedStatusChanged,
    ObservedTeachingPostureChanged,
    ObservedLiveRowDigestChanged,
}

impl WorthQuerySupportPinFindingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SchemaMismatch => "schema-mismatch",
            Self::VocabularyMismatch => "vocabulary-mismatch",
            Self::SourceMatrixDigestChanged => "source-matrix-digest-changed",
            Self::RequiredRowMissing => "required-row-missing",
            Self::StatusMismatch => "status-mismatch",
            Self::TeachingPostureMismatch => "teaching-posture-mismatch",
            Self::LiveRowDigestMismatch => "live-row-digest-mismatch",
            Self::ObservedRowMissing => "observed-row-missing",
            Self::ObservedStatusChanged => "observed-status-changed",
            Self::ObservedTeachingPostureChanged => "observed-teaching-posture-changed",
            Self::ObservedLiveRowDigestChanged => "observed-live-row-digest-changed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySupportPinFinding {
    kind: WorthQuerySupportPinFindingKind,
    family: Option<WorthQueryRuntimeFacadeFamily>,
    surface: String,
    expected: Option<String>,
    found: Option<String>,
    blocking: bool,
    finding_digest: String,
}

impl WorthQuerySupportPinFinding {
    pub(crate) fn new(
        kind: WorthQuerySupportPinFindingKind,
        family: Option<WorthQueryRuntimeFacadeFamily>,
        surface: impl Into<String>,
        expected: Option<String>,
        found: Option<String>,
        blocking: bool,
        finding_digest: String,
    ) -> Self {
        Self {
            kind,
            family,
            surface: surface.into(),
            expected,
            found,
            blocking,
            finding_digest,
        }
    }

    pub fn kind(&self) -> WorthQuerySupportPinFindingKind {
        self.kind
    }

    pub fn family(&self) -> Option<WorthQueryRuntimeFacadeFamily> {
        self.family
    }

    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub fn expected(&self) -> Option<&str> {
        self.expected.as_deref()
    }

    pub fn found(&self) -> Option<&str> {
        self.found.as_deref()
    }

    pub fn blocking(&self) -> bool {
        self.blocking
    }

    pub fn finding_digest(&self) -> &str {
        &self.finding_digest
    }
}
